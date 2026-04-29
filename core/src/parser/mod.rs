mod enums;
mod objects;
mod primitive;
pub(crate) mod records;

pub use self::{enums::*, objects::*, primitive::*, records::*};
use crate::imports::*;

#[derive(Clone, Debug, snafu::prelude::Snafu)]
pub enum ParseError {
    #[snafu(display("failed to read buffer: {cause}"))]
    FailedReadBuffer { cause: ReadError },
    /// `cause` is a `Cow<'static, str>` so call sites can pass a
    /// `&'static str` literal without allocating, and still fall back to
    /// a formatted `String` (via `Cow::Owned`) when the message embeds
    /// runtime values.
    #[snafu(display("not supported: {cause}"))]
    NotSupported { cause: Cow<'static, str> },
    #[snafu(display("unexpected enum value: {cause}"))]
    UnexpectedEnumValue { cause: Cow<'static, str> },
    #[snafu(display("unexpected bytes pattern: {cause}"))]
    UnexpectedPattern { cause: Cow<'static, str> },
    /// A fixed-value field carried a value other than the one the
    /// specification mandates. `width_bits` records the source field
    /// width so the hex display preserves the leading zeros that the
    /// natural field width implies.
    #[snafu(display(
        "field `{field}` mismatched: actual {actual:#0w$x}, expected \
         {expected:#0w$x}",
        w = hex_width(*width_bits),
    ))]
    MismatchedField {
        field: &'static str,
        actual: u64,
        expected: u64,
        width_bits: u8,
    },
    /// A field exceeded its allowed maximum, either by specification or
    /// by an internal sanity bound. `width_bits` controls the hex
    /// display width as for `MismatchedField`.
    #[snafu(display(
        "field `{field}` out of range: actual {actual:#0w$x}, max \
         {max:#0w$x}",
        w = hex_width(*width_bits),
    ))]
    FieldOutOfRange {
        field: &'static str,
        actual: u64,
        max: u64,
        width_bits: u8,
    },
    /// A field carried a value the specification forbids (e.g. a handle
    /// that must not be zero, a scale denominator that must be
    /// non-zero).
    #[snafu(display(
        "field `{field}` must not be {forbidden:#0w$x}",
        w = hex_width(*width_bits),
    ))]
    ForbiddenField { field: &'static str, forbidden: u64, width_bits: u8 },
    /// A field violated a spec-mandated strictly-greater bound (e.g.
    /// `EMR_ALPHABLEND::cxDest MUST be greater than zero`). Display
    /// uses decimal because callers may pass signed integers.
    #[snafu(display(
        "field `{field}` must be greater than {min}, but actual {actual}"
    ))]
    FieldNotGreater { field: &'static str, actual: i64, min: i64 },
    /// A field fell outside an inclusive range (e.g. `Gamma` ∈
    /// `[2500, 65000]`, `Adjustment` ∈ `[-100, 100]`). Display uses
    /// decimal so signed and unsigned bounds render uniformly.
    #[snafu(display(
        "field `{field}` must be within {min}..={max}, but actual {actual}"
    ))]
    FieldOutOfInclusiveRange {
        field: &'static str,
        actual: i64,
        min: i64,
        max: i64,
    },
}

impl From<ReadError> for ParseError {
    fn from(err: ReadError) -> Self {
        Self::FailedReadBuffer { cause: err }
    }
}

impl From<wmf_core::parser::ParseError> for ParseError {
    fn from(err: wmf_core::parser::ParseError) -> Self {
        use wmf_core::parser::ParseError;

        match err {
            ParseError::FailedReadBuffer { cause } => {
                Self::FailedReadBuffer { cause: cause.into() }
            }
            ParseError::NotSupported { .. } => {
                Self::NotSupported { cause: err.to_string().into() }
            }
            ParseError::UnexpectedEnumValue { .. } => {
                Self::UnexpectedEnumValue { cause: err.to_string().into() }
            }
            ParseError::UnexpectedPattern { .. } => {
                Self::UnexpectedPattern { cause: err.to_string().into() }
            }
        }
    }
}

impl ParseError {
    /// Returns `Ok(())` when `actual == expected`, otherwise produces
    /// `MismatchedField` carrying the field name, both values, and the
    /// source bit width so the diagnostic display can keep zero-padded
    /// hex at the natural source width.
    pub(crate) fn expect_eq<T>(
        field: &'static str,
        actual: T,
        expected: T,
    ) -> Result<(), Self>
    where
        T: Copy + PartialEq + IntoU64Bits,
    {
        if actual == expected {
            return Ok(());
        }

        Err(Self::MismatchedField {
            field,
            actual: actual.into_u64_bits(),
            expected: expected.into_u64_bits(),
            width_bits: bits_of::<T>(),
        })
    }

    /// Returns `Ok(())` when `actual <= max`, otherwise produces
    /// `FieldOutOfRange`. Used for spec-defined upper bounds and for
    /// internal allocation guards alike.
    pub(crate) fn expect_le<T>(
        field: &'static str,
        actual: T,
        max: T,
    ) -> Result<(), Self>
    where
        T: Copy + PartialOrd + IntoU64Bits,
    {
        if actual <= max {
            return Ok(());
        }

        Err(Self::FieldOutOfRange {
            field,
            actual: actual.into_u64_bits(),
            max: max.into_u64_bits(),
            width_bits: bits_of::<T>(),
        })
    }

    /// Returns `Ok(())` when `actual != forbidden`, otherwise produces
    /// `ForbiddenField`. Used for spec-mandated non-zero / non-equal
    /// invariants (e.g. `EMR_SCALEWINDOWEXTEX::xNum MUST NOT be zero`).
    pub(crate) fn expect_ne<T>(
        field: &'static str,
        actual: T,
        forbidden: T,
    ) -> Result<(), Self>
    where
        T: Copy + PartialEq + IntoU64Bits,
    {
        if actual != forbidden {
            return Ok(());
        }

        Err(Self::ForbiddenField {
            field,
            forbidden: forbidden.into_u64_bits(),
            width_bits: bits_of::<T>(),
        })
    }

    /// Returns `Ok(())` when `actual > min`, otherwise produces
    /// `FieldNotGreater`. Used for spec-mandated strictly-positive
    /// invariants (e.g. `EMR_ALPHABLEND::cxDest MUST be greater than
    /// zero`).
    pub(crate) fn expect_gt<T>(
        field: &'static str,
        actual: T,
        min: T,
    ) -> Result<(), Self>
    where
        T: Copy + PartialOrd + Into<i64>,
    {
        if actual > min {
            return Ok(());
        }

        Err(Self::FieldNotGreater {
            field,
            actual: actual.into(),
            min: min.into(),
        })
    }

    /// Returns `Ok(())` when `min <= actual <= max`, otherwise produces
    /// `FieldOutOfInclusiveRange`. Used for spec-mandated inclusive
    /// range invariants (`Gamma` ∈ `[2500, 65000]`, `Adjustment` ∈
    /// `[-100, 100]`).
    pub(crate) fn expect_in_range<T>(
        field: &'static str,
        actual: T,
        min: T,
        max: T,
    ) -> Result<(), Self>
    where
        T: Copy + PartialOrd + Into<i64>,
    {
        if actual >= min && actual <= max {
            return Ok(());
        }

        Err(Self::FieldOutOfInclusiveRange {
            field,
            actual: actual.into(),
            min: min.into(),
            max: max.into(),
        })
    }
}

/// Returns the bit width of `T` (8/16/32/64). Used to thread the source
/// integer width through to the structured `ParseError` variants so
/// `Display` can emit hex with the appropriate zero padding.
const fn bits_of<T>() -> u8 {
    (core::mem::size_of::<T>() * 8) as u8
}

/// Maps a bit width (8/16/32/64) to the formatter `width` argument
/// expected by `{:#0w$x}`. The `#` flag emits the `0x` prefix and the
/// `0` flag pads with zeros, so the total width includes those two
/// prefix characters and the underlying hex digits.
const fn hex_width(width_bits: u8) -> usize {
    (width_bits as usize) / 4 + 2
}

/// Widens `Self` into a `u64` while preserving the source bit pattern.
/// Required so `MismatchedField` / `FieldOutOfRange` / `ForbiddenField`
/// can display zero-padded hex at the original field width even for
/// signed inputs (e.g. an `i32` handle field rendered as `0xFFFFFFFC`).
pub(crate) trait IntoU64Bits: Copy {
    fn into_u64_bits(self) -> u64;
}

impl IntoU64Bits for u8 {
    #[inline]
    fn into_u64_bits(self) -> u64 {
        u64::from(self)
    }
}

impl IntoU64Bits for u16 {
    #[inline]
    fn into_u64_bits(self) -> u64 {
        u64::from(self)
    }
}

impl IntoU64Bits for u32 {
    #[inline]
    fn into_u64_bits(self) -> u64 {
        u64::from(self)
    }
}

impl IntoU64Bits for u64 {
    #[inline]
    fn into_u64_bits(self) -> u64 {
        self
    }
}

impl IntoU64Bits for i8 {
    #[inline]
    fn into_u64_bits(self) -> u64 {
        u64::from(self as u8)
    }
}

impl IntoU64Bits for i16 {
    #[inline]
    fn into_u64_bits(self) -> u64 {
        u64::from(self as u16)
    }
}

impl IntoU64Bits for i32 {
    #[inline]
    fn into_u64_bits(self) -> u64 {
        u64::from(self as u32)
    }
}

impl IntoU64Bits for usize {
    /// `usize` is `u32` on 32-bit targets and `u64` on 64-bit. The
    /// `as u64` cast widens losslessly on supported targets, and
    /// `bits_of::<usize>()` reports the matching source width so hex
    /// display zero-pads to the natural field width on either.
    #[inline]
    fn into_u64_bits(self) -> u64 {
        self as u64
    }
}

impl IntoU64Bits for isize {
    #[inline]
    fn into_u64_bits(self) -> u64 {
        (self as usize) as u64
    }
}

#[derive(Clone, Debug, snafu::prelude::Snafu)]
#[snafu(display("failed to read buffer: {cause}"))]
pub struct ReadError {
    /// `Cow<'static, str>` so static error literals stay borrowed and
    /// only the dynamic `format!`-built messages allocate.
    cause: Cow<'static, str>,
}

impl ReadError {
    pub fn new(err: impl core::fmt::Display) -> Self {
        Self { cause: err.to_string().into() }
    }
}

impl From<wmf_core::parser::ReadError> for ReadError {
    fn from(err: wmf_core::parser::ReadError) -> Self {
        Self { cause: err.to_string().into() }
    }
}

#[inline]
pub(in crate::parser) fn read<R: crate::Read, const N: usize>(
    buf: &mut R,
) -> Result<([u8; N], usize), ReadError> {
    let mut buffer = [0u8; N];

    read_exact(buf, &mut buffer)?;

    Ok((buffer, N))
}

pub(crate) fn read_variable<R: crate::Read>(
    buf: &mut R,
    len: usize,
) -> Result<(Vec<u8>, usize), ReadError> {
    if len == 0 {
        return Ok((Vec::new(), 0));
    }

    let mut buffer = vec![0u8; len];

    read_exact(buf, &mut buffer)?;

    Ok((buffer, len))
}

/// Type-driven dispatch for little-endian fixed-width integer reads.
///
/// Lets generic helpers (e.g. `records::read_field`) pick the right
/// integer width based on the requested type, while keeping the byte-count
/// returned by each impl bound to the type that produced it.
pub(crate) trait ReadLeField: Sized {
    fn read_le<R: crate::Read>(buf: &mut R)
    -> Result<(Self, usize), ReadError>;
}

/// Abstract interface for tracking how many bytes have been consumed from
/// a buffer. Implemented for both `Size` (used by record parsers that
/// have a known frame) and `usize` (used by object parsers that just
/// thread a counter through the call graph).
pub(crate) trait ConsumeTracker {
    fn track(&mut self, consumed_bytes: usize);
}

impl ConsumeTracker for usize {
    #[inline]
    fn track(&mut self, consumed_bytes: usize) {
        *self += consumed_bytes;
    }
}

impl ConsumeTracker for crate::parser::Size {
    #[inline]
    fn track(&mut self, consumed_bytes: usize) {
        self.consume(consumed_bytes);
    }
}

impl ReadLeField for i8 {
    #[inline]
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 1>(buf)?;
        Ok((i8::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for i16 {
    #[inline]
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 2>(buf)?;
        Ok((i16::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for i32 {
    #[inline]
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 4>(buf)?;
        Ok((i32::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for u8 {
    #[inline]
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 1>(buf)?;
        Ok((u8::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for u16 {
    #[inline]
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 2>(buf)?;
        Ok((u16::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for u32 {
    #[inline]
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 4>(buf)?;
        Ok((u32::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for f32 {
    #[inline]
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 4>(buf)?;
        Ok((f32::from_le_bytes(bytes), c))
    }
}

/// `embedded_io::Read` may return short reads, so loop until `buffer`
/// is fully populated or the stream signals EOF via `Ok(0)`.
pub(in crate::parser) fn read_exact<R: crate::Read>(
    buf: &mut R,
    buffer: &mut [u8],
) -> Result<(), ReadError> {
    let total = buffer.len();
    let mut offset = 0;

    while offset < total {
        match buf.read(&mut buffer[offset..]) {
            Ok(0) => {
                return Err(ReadError::new(format!(
                    "expected {total} bytes read, but {offset} bytes read \
                     (unexpected end of stream)"
                )));
            }
            Ok(n) => offset += n,
            Err(err) => return Err(ReadError::new(format!("{err:?}"))),
        }
    }

    Ok(())
}

/// Convert UTF16-LE bytes to String.
///
/// Streams `u16` code units straight into `char::decode_utf16` so the
/// output `String` is built in a single pass. The previous version
/// collected a `Vec<u16>` first and handed it to `String::from_utf16`,
/// allocating twice as many bytes as the resulting string.
fn utf16le_bytes_to_string(bytes: &[u8]) -> Result<String, ParseError> {
    if bytes.len() % 2 != 0 {
        return Err(ParseError::UnexpectedPattern {
            cause: "Byte array length must be even".into(),
        });
    }

    let units = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]));

    // Worst-case: every BMP code unit yields a 1-3 byte UTF-8 char,
    // surrogate pairs collapse two units into one 4-byte char. Reserving
    // `bytes.len() / 2` chars (i.e. units count) is a safe lower bound.
    let mut out = String::with_capacity(bytes.len() / 2);
    for c in core::char::decode_utf16(units) {
        match c {
            Ok(ch) => out.push(ch),
            Err(err) => {
                return Err(ParseError::UnexpectedPattern {
                    cause: err.to_string().into(),
                });
            }
        }
    }

    Ok(out)
}

fn null_terminated_utf16le_string(bytes: &[u8]) -> Result<String, ParseError> {
    if bytes.len() % 2 != 0 {
        return Err(ParseError::UnexpectedPattern {
            cause: "Byte array length must be even".into(),
        });
    }

    // Find the position of the first null byte (0).
    let len = bytes
        .chunks_exact(2)
        .position(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]) == 0)
        .unwrap_or(bytes.len() / 2);

    utf16le_bytes_to_string(&bytes[..len * 2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_fixed_exact_boundary() {
        let data = [0xAA_u8, 0xBB];
        let mut reader = &data[..];
        let (bytes, consumed) = read::<&[u8], 2>(&mut reader).unwrap();
        assert_eq!(bytes, [0xAA, 0xBB]);
        assert_eq!(consumed, 2);
    }

    #[test]
    fn read_fixed_empty_buffer_errors() {
        let mut reader: &[u8] = &[];
        assert!(read::<&[u8], 2>(&mut reader).is_err());
    }

    #[test]
    fn read_variable_exact() {
        let data = [1_u8, 2, 3, 4, 5];
        let mut reader = &data[..];
        let (result, consumed) = read_variable(&mut reader, 5).unwrap();
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
        assert_eq!(consumed, 5);
    }

    #[test]
    fn read_variable_zero_length_returns_empty() {
        let mut reader: &[u8] = &[];
        let (data, consumed) = read_variable(&mut reader, 0).unwrap();
        assert!(data.is_empty());
        assert_eq!(consumed, 0);
    }

    #[test]
    fn read_variable_insufficient_data_errors() {
        let data = [0_u8; 3];
        let mut reader = &data[..];
        assert!(read_variable(&mut reader, 10).is_err());
    }

    #[test]
    fn expect_eq_passes_on_match() {
        assert!(ParseError::expect_eq("foo", 0x1234_u16, 0x1234_u16).is_ok());
    }

    #[test]
    fn expect_eq_fails_on_mismatch() {
        let err =
            ParseError::expect_eq("foo", 0x1234_u16, 0x5678_u16).unwrap_err();
        match err {
            ParseError::MismatchedField {
                field,
                actual,
                expected,
                width_bits,
            } => {
                assert_eq!(field, "foo");
                assert_eq!(actual, 0x1234);
                assert_eq!(expected, 0x5678);
                assert_eq!(width_bits, 16);
            }
            _ => panic!("unexpected variant"),
        }
    }

    /// Verifies the source bit width flows through to the variant so
    /// `Display` can pad hex to the original zero-extended width. A u32
    /// source must be tagged 32 bits, not the receiver's `u64` upper
    /// bound.
    #[test]
    fn expect_eq_records_u32_width() {
        let err = ParseError::expect_eq("magic", 0x0_u32, 0x12345678_u32)
            .unwrap_err();
        let ParseError::MismatchedField { width_bits, .. } = err else {
            panic!("unexpected variant");
        };
        assert_eq!(width_bits, 32);
    }

    /// `Display` must zero-pad both operands to the source bit width.
    /// For a u16 source that means 4 hex digits plus the `0x` prefix.
    #[test]
    fn mismatched_field_display_pads_to_source_width() {
        let err = ParseError::expect_eq("byte_count", 0x0_u16, 0x0004_u16)
            .unwrap_err();
        let s = err.to_string();
        assert!(
            s.contains("0x0000") && s.contains("0x0004"),
            "expected zero-padded u16 hex, got: {s}"
        );
    }

    /// Signed inputs are widened bit-for-bit, so `i32` `-1` must render
    /// as `0xffffffff` rather than the i64 widened `0xffffffffffffffff`.
    #[test]
    fn expect_eq_signed_renders_as_source_width() {
        let err = ParseError::expect_eq("delta", -1_i32, 0_i32).unwrap_err();
        let s = err.to_string();
        assert!(
            s.contains("0xffffffff") && s.contains("0x00000000"),
            "expected i32 bit pattern, got: {s}"
        );
    }

    #[test]
    fn expect_le_allows_equal() {
        assert!(ParseError::expect_le("bar", 100_u32, 100_u32).is_ok());
    }

    #[test]
    fn expect_le_rejects_overflow() {
        let err = ParseError::expect_le("bar", 101_u32, 100_u32).unwrap_err();
        match err {
            ParseError::FieldOutOfRange { field, actual, max, width_bits } => {
                assert_eq!(field, "bar");
                assert_eq!(actual, 101);
                assert_eq!(max, 100);
                assert_eq!(width_bits, 32);
            }
            _ => panic!("unexpected variant"),
        }
    }

    #[test]
    fn expect_ne_allows_distinct() {
        assert!(ParseError::expect_ne("baz", 1_i32, 0_i32).is_ok());
    }

    #[test]
    fn expect_ne_rejects_forbidden() {
        let err = ParseError::expect_ne("baz", 0_i32, 0_i32).unwrap_err();
        match err {
            ParseError::ForbiddenField { field, forbidden, width_bits } => {
                assert_eq!(field, "baz");
                assert_eq!(forbidden, 0);
                assert_eq!(width_bits, 32);
            }
            _ => panic!("unexpected variant"),
        }
    }

    #[test]
    fn expect_gt_accepts_above() {
        assert!(ParseError::expect_gt("cx", 1_i32, 0_i32).is_ok());
    }

    #[test]
    fn expect_gt_rejects_at_or_below() {
        let err = ParseError::expect_gt("cx", 0_i32, 0_i32).unwrap_err();
        match err {
            ParseError::FieldNotGreater { field, actual, min } => {
                assert_eq!(field, "cx");
                assert_eq!(actual, 0);
                assert_eq!(min, 0);
            }
            _ => panic!("unexpected variant"),
        }
        assert!(ParseError::expect_gt("cx", -1_i32, 0_i32).is_err());
    }

    #[test]
    fn expect_in_range_accepts_inclusive_bounds() {
        assert!(
            ParseError::expect_in_range("g", 2_500_u16, 2_500, 65_000).is_ok()
        );
        assert!(
            ParseError::expect_in_range("g", 65_000_u16, 2_500, 65_000).is_ok()
        );
        assert!(
            ParseError::expect_in_range("g", 30_000_u16, 2_500, 65_000).is_ok()
        );
    }

    #[test]
    fn expect_in_range_rejects_below() {
        let err = ParseError::expect_in_range("g", 2_499_u16, 2_500, 65_000)
            .unwrap_err();
        match err {
            ParseError::FieldOutOfInclusiveRange {
                field,
                actual,
                min,
                max,
            } => {
                assert_eq!(field, "g");
                assert_eq!(actual, 2_499);
                assert_eq!(min, 2_500);
                assert_eq!(max, 65_000);
            }
            _ => panic!("unexpected variant"),
        }
    }

    #[test]
    fn expect_in_range_rejects_above() {
        let err = ParseError::expect_in_range("g", 65_001_u16, 2_500, 65_000)
            .unwrap_err();
        let ParseError::FieldOutOfInclusiveRange { actual, .. } = err else {
            panic!("unexpected variant");
        };
        assert_eq!(actual, 65_001);
    }

    #[test]
    fn read_le_field_u16() {
        let data = 0xABCD_u16.to_le_bytes();
        let mut reader = &data[..];
        let (val, consumed) = u16::read_le(&mut reader).unwrap();
        assert_eq!(val, 0xABCD);
        assert_eq!(consumed, 2);
    }

    #[test]
    fn read_le_field_i32() {
        let data = (-99999_i32).to_le_bytes();
        let mut reader = &data[..];
        let (val, consumed) = i32::read_le(&mut reader).unwrap();
        assert_eq!(val, -99999);
        assert_eq!(consumed, 4);
    }

    #[test]
    fn read_le_field_f32() {
        let data = 1.5_f32.to_le_bytes();
        let mut reader = &data[..];
        let (val, consumed) = f32::read_le(&mut reader).unwrap();
        assert!((val - 1.5).abs() < f32::EPSILON);
        assert_eq!(consumed, 4);
    }

    #[test]
    fn consume_tracker_for_usize_accumulates() {
        let mut t: usize = 0;
        t.track(3);
        t.track(5);
        assert_eq!(t, 8);
    }

    #[test]
    fn read_le_field_i16_roundtrip() {
        let data = (-1234_i16).to_le_bytes();
        let mut reader = &data[..];
        let (val, consumed) = i16::read_le(&mut reader).unwrap();
        assert_eq!(val, -1234);
        assert_eq!(consumed, 2);
    }

    #[test]
    fn read_le_field_u32_roundtrip() {
        let data = 0xDEADBEEF_u32.to_le_bytes();
        let mut reader = &data[..];
        let (val, consumed) = u32::read_le(&mut reader).unwrap();
        assert_eq!(val, 0xDEADBEEF);
        assert_eq!(consumed, 4);
    }

    #[test]
    fn read_le_field_u32_short_buffer_errors() {
        let data = [0_u8; 2];
        let mut reader = &data[..];
        assert!(u32::read_le(&mut reader).is_err());
    }

    #[test]
    fn read_le_field_f32_roundtrip() {
        let data = 1.5_f32.to_le_bytes();
        let mut reader = &data[..];
        let (val, consumed) = f32::read_le(&mut reader).unwrap();
        assert!((val - 1.5).abs() < f32::EPSILON);
        assert_eq!(consumed, 4);
    }

    #[test]
    fn utf16le_bytes_to_string_roundtrip() {
        // UTF-16LE encoded bytes for "AB".
        let bytes = [0x41_u8, 0x00, 0x42, 0x00];
        let s = utf16le_bytes_to_string(&bytes).unwrap();
        assert_eq!(s, "AB");
    }

    #[test]
    fn utf16le_bytes_to_string_odd_length_errors() {
        let bytes = [0x41_u8, 0x00, 0x42];
        assert!(utf16le_bytes_to_string(&bytes).is_err());
    }

    #[test]
    fn null_terminated_utf16le_string_truncates_at_null() {
        // UTF-16LE encoded bytes for "AB\0CD"; bytes after NUL must be dropped.
        let bytes =
            [0x41_u8, 0x00, 0x42, 0x00, 0x00, 0x00, 0x43, 0x00, 0x44, 0x00];
        let s = null_terminated_utf16le_string(&bytes).unwrap();
        assert_eq!(s, "AB");
    }

    #[test]
    fn null_terminated_utf16le_string_without_null_uses_full_input() {
        let bytes = [0x41_u8, 0x00, 0x42, 0x00];
        let s = null_terminated_utf16le_string(&bytes).unwrap();
        assert_eq!(s, "AB");
    }

    /// `read_exact` must succeed even when the underlying `embedded_io::Read`
    /// implementation returns short reads.
    #[test]
    fn read_exact_handles_short_reads() {
        struct ChunkedReader<'a> {
            data: &'a [u8],
            chunk: usize,
        }

        impl embedded_io::ErrorType for ChunkedReader<'_> {
            type Error = embedded_io::ErrorKind;
        }

        impl embedded_io::Read for ChunkedReader<'_> {
            fn read(&mut self, out: &mut [u8]) -> Result<usize, Self::Error> {
                let n = self.chunk.min(self.data.len()).min(out.len());
                out[..n].copy_from_slice(&self.data[..n]);
                self.data = &self.data[n..];
                Ok(n)
            }
        }

        let bytes = [1_u8, 2, 3, 4, 5, 6, 7, 8];
        let mut reader = ChunkedReader { data: &bytes, chunk: 1 };
        let (got, consumed) = read::<_, 8>(&mut reader).unwrap();
        assert_eq!(got, bytes);
        assert_eq!(consumed, 8);
    }
}
