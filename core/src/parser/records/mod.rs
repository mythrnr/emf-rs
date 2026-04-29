//! Implementation of the definitions in Section 2.3 of the EMF specifications.

mod bitmap;
mod clipping;
mod comment;
mod control;
mod drawing;
mod escape;
mod object_creation;
mod object_manipulation;
mod open_gl;
mod path_bracket;
mod state;
mod transform;

pub use self::{
    bitmap::*, clipping::*, comment::*, control::*, drawing::*, escape::*,
    object_creation::*, object_manipulation::*, open_gl::*, path_bracket::*,
    state::*, transform::*,
};

/// Upper bound on the total number of points per drawing record.
///
/// Derived from `MAX_RECORD_BYTES` divided by the smallest plausible
/// per-point size (`sizeof(PointS) = 4`). At 16 Mi points the bound is
/// far above any realistic figure (a 50,000-vertex map outline is
/// typical even for atlases) yet still rejects crafted inputs that
/// would push `Vec::with_capacity` into hundreds of megabytes.
pub(crate) const MAX_TOTAL_POINTS: u32 = 16 * 1024 * 1024;

/// Guards a record's reported total point count against
/// `MAX_TOTAL_POINTS`. Call immediately after reading the `count`
/// field, before iterating the points array.
pub(crate) fn check_total_points(
    count: u32,
) -> Result<(), crate::parser::ParseError> {
    crate::parser::ParseError::expect_le(
        "total point count",
        count,
        MAX_TOTAL_POINTS,
    )
}

/// Validates that the sum of `polygon_point_count` does not overflow
/// `u32` and does not exceed the declared total `count`. PR #40 of
/// wmf-rs introduced the same guard for `META_POLYPOLYGON`; without it,
/// a crafted multi-polygon record could declare per-polygon counts
/// whose sum overflows or wraps `count`, steering the points-array
/// allocation toward an oversized state.
pub(crate) fn check_polygon_point_count_sum(
    polygon_point_count: &[u32],
    count: u32,
) -> Result<(), crate::parser::ParseError> {
    let mut sum: u32 = 0;
    for entry in polygon_point_count {
        sum = sum.checked_add(*entry).ok_or_else(|| {
            crate::parser::ParseError::UnexpectedPattern {
                cause: alloc::format!(
                    "polygon_point_count sum overflow at entry {entry:#010X}",
                ),
            }
        })?;
    }
    crate::parser::ParseError::expect_le("polygon_point_count sum", sum, count)
}

/// Read a fixed-width little-endian integer field, advance the `tracker`
/// by the number of bytes consumed, and return the value.
///
/// Collapses the recurring three-step pattern in record parsers
/// (call `read_<ty>_from_le_bytes`, capture both value and byte count,
/// then advance the byte counter) so the byte-count bookkeeping cannot
/// drift from the actual read. The tracker can be either a `Size`
/// (record parsers) or a plain `usize` counter (object parsers).
/// The output type is selected via type inference from the binding,
/// e.g. `let v: u32 = read_field(...)?;`.
pub(crate) fn read_field<R, T>(
    buf: &mut R,
    tracker: &mut impl crate::parser::ConsumeTracker,
) -> Result<T, crate::parser::ParseError>
where
    R: crate::Read,
    T: crate::parser::ReadLeField,
{
    let (v, c) = T::read_le(buf)?;
    tracker.track(c);
    Ok(v)
}

/// Run a composite parser that returns `(value, consumed_bytes)` and
/// advance the `tracker` accordingly. Used for sub-object parsers
/// (e.g. `RectL::parse`, `RecordType::parse`) that already follow the
/// `(T, usize)` convention but cannot satisfy the `ReadLeField` bound.
pub(crate) fn read_with<R, T, F, E>(
    buf: &mut R,
    tracker: &mut impl crate::parser::ConsumeTracker,
    parser: F,
) -> Result<T, crate::parser::ParseError>
where
    R: crate::Read,
    F: FnOnce(&mut R) -> Result<(T, usize), E>,
    crate::parser::ParseError: From<E>,
{
    let (v, c) = parser(buf)?;
    tracker.track(c);
    Ok(v)
}

/// Read a fixed-size byte array `[u8; N]` directly and advance the
/// `tracker`. The compile-time `N` removes the runtime length check
/// that `Vec::try_into::<[u8; N]>()` would otherwise need, eliminating
/// the `.expect("should be N bytes")` panic site for fixed-length
/// fields (e.g. `dwLayerMask`, `Reserved`, ...). The byte array lives
/// on the stack, so this avoids the intermediate `Vec` allocation that
/// `read_bytes_field` performs.
pub(in crate::parser) fn read_array_field<R, const N: usize>(
    buf: &mut R,
    tracker: &mut impl crate::parser::ConsumeTracker,
) -> Result<[u8; N], crate::parser::ParseError>
where
    R: crate::Read,
{
    let (bytes, c) = crate::parser::read::<R, N>(buf)?;
    tracker.track(c);
    Ok(bytes)
}

/// Read a variable-length byte buffer of `len` bytes and advance the
/// `tracker` accordingly.
pub(in crate::parser) fn read_bytes_field<R>(
    buf: &mut R,
    tracker: &mut impl crate::parser::ConsumeTracker,
    len: usize,
) -> Result<crate::imports::Vec<u8>, crate::parser::ParseError>
where
    R: crate::Read,
{
    let (v, c) = crate::parser::read_variable(buf, len)?;
    tracker.track(c);
    Ok(v)
}

fn consume_remaining_bytes<R: crate::Read>(
    buf: &mut R,
    remaining_bytes: usize,
) -> Result<(), crate::parser::ParseError> {
    if remaining_bytes == 0 {
        return Ok(());
    }

    // Discard remaining bytes in fixed-size chunks. Allocating a single
    // `Vec<u8>` of `remaining_bytes` would let a malformed record drive
    // a multi-megabyte allocation just to throw the data away.
    let mut discarded = 0;
    let mut chunk = [0_u8; 4096];

    while discarded < remaining_bytes {
        let to_read = core::cmp::min(remaining_bytes - discarded, chunk.len());
        crate::parser::read_exact(buf, &mut chunk[..to_read])?;
        discarded += to_read;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn check_total_points_accepts_at_max() {
        assert!(check_total_points(MAX_TOTAL_POINTS).is_ok());
    }

    #[test]
    fn check_total_points_rejects_over_max() {
        let err = check_total_points(MAX_TOTAL_POINTS + 1).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("total point count"), "msg = {msg}");
    }

    #[test]
    fn read_field_advances_size_tracker() {
        let data = 0x12345678_u32.to_le_bytes();
        let mut reader = &data[..];
        let mut size = crate::parser::Size::from(8);
        size.consume(2);

        let v: u32 = read_field(&mut reader, &mut size).unwrap();
        assert_eq!(v, 0x12345678);
        assert_eq!(size.consumed_bytes(), 2 + 4);
    }

    #[test]
    fn read_field_advances_usize_tracker() {
        let data = 0xABCD_u16.to_le_bytes();
        let mut reader = &data[..];
        let mut consumed: usize = 5;

        let v: u16 = read_field(&mut reader, &mut consumed).unwrap();
        assert_eq!(v, 0xABCD);
        assert_eq!(consumed, 5 + 2);
    }

    #[test]
    fn consume_remaining_bytes_zero_is_noop() {
        let mut empty: &[u8] = &[];
        assert!(consume_remaining_bytes(&mut empty, 0).is_ok());
    }

    #[test]
    fn consume_remaining_bytes_drains_input() {
        let data = vec![0_u8; 8192];
        let mut reader = &data[..];
        // Drain a length larger than the 4 KiB chunk to exercise the
        // loop boundary.
        assert!(consume_remaining_bytes(&mut reader, 5000).is_ok());
        assert_eq!(reader.len(), 8192 - 5000);
    }

    #[test]
    fn consume_remaining_bytes_short_input_errors() {
        let data = [0_u8; 100];
        let mut reader = &data[..];
        assert!(consume_remaining_bytes(&mut reader, 200).is_err());
    }
}
