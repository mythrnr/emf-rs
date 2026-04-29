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
    #[snafu(display("not supported: {cause}"))]
    NotSupported { cause: String },
    #[snafu(display("unexpected enum value: {cause}"))]
    UnexpectedEnumValue { cause: String },
    #[snafu(display("unexpected bytes pattern: {cause}"))]
    UnexpectedPattern { cause: String },
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
                Self::NotSupported { cause: err.to_string() }
            }
            ParseError::UnexpectedEnumValue { .. } => {
                Self::UnexpectedEnumValue { cause: err.to_string() }
            }
            ParseError::UnexpectedPattern { .. } => {
                Self::UnexpectedPattern { cause: err.to_string() }
            }
        }
    }
}

impl ParseError {
    /// Returns `Ok(())` when `got == expected`, otherwise an
    /// `UnexpectedPattern` error with a uniform diagnostic format.
    ///
    /// Hex width is selected from `size_of::<T>()` so that 8/16/32-bit
    /// fields print the natural number of digits (e.g. `0x04` /
    /// `0x0004` / `0x00000004`).
    pub(crate) fn expect_eq<T>(
        field: &str,
        got: T,
        expected: T,
    ) -> Result<(), Self>
    where
        T: Copy + PartialEq + core::fmt::UpperHex,
    {
        if got == expected {
            return Ok(());
        }

        let width = match core::mem::size_of::<T>() {
            1 => 4,  // "0xNN"   width 4 includes "0x"
            2 => 6,  // "0xNNNN"
            _ => 10, // "0xNNNNNNNN" and wider
        };

        Err(Self::UnexpectedPattern {
            cause: alloc::format!(
                "{field} must be `{expected:#0width$X}`, but got \
                 `{got:#0width$X}`",
            ),
        })
    }

    /// Returns `Ok(())` when `got <= max`, otherwise an
    /// `UnexpectedPattern` error. Used to enforce upper bounds on
    /// counts and offsets that would otherwise drive oversized
    /// allocations.
    pub(crate) fn expect_le<T>(field: &str, got: T, max: T) -> Result<(), Self>
    where
        T: Copy + PartialOrd + core::fmt::UpperHex,
    {
        if got <= max {
            return Ok(());
        }

        let width = match core::mem::size_of::<T>() {
            1 => 4,
            2 => 6,
            _ => 10,
        };

        Err(Self::UnexpectedPattern {
            cause: alloc::format!(
                "{field} must be `<= {max:#0width$X}`, but got \
                 `{got:#0width$X}`",
            ),
        })
    }

    /// Returns `Ok(())` when `got != forbidden`, otherwise an
    /// `UnexpectedPattern` error. Used for spec-mandated non-zero /
    /// non-equal invariants (e.g. `EMR_SCALEWINDOWEXTEX::xNum MUST NOT
    /// be zero`).
    pub(crate) fn expect_ne<T>(
        field: &str,
        got: T,
        forbidden: T,
    ) -> Result<(), Self>
    where
        T: Copy + PartialEq + core::fmt::UpperHex,
    {
        if got != forbidden {
            return Ok(());
        }

        let width = match core::mem::size_of::<T>() {
            1 => 4,
            2 => 6,
            _ => 10,
        };

        Err(Self::UnexpectedPattern {
            cause: alloc::format!(
                "{field} must not be `{forbidden:#0width$X}`",
            ),
        })
    }

    /// Returns `Ok(())` when `got > min`, otherwise an
    /// `UnexpectedPattern` error. Used for spec-mandated strictly
    /// positive invariants (e.g. `EMR_ALPHABLEND::cxDest MUST be
    /// greater than zero`).
    pub(crate) fn expect_gt<T>(field: &str, got: T, min: T) -> Result<(), Self>
    where
        T: Copy + PartialOrd + core::fmt::UpperHex,
    {
        if got > min {
            return Ok(());
        }

        let width = match core::mem::size_of::<T>() {
            1 => 4,
            2 => 6,
            _ => 10,
        };

        Err(Self::UnexpectedPattern {
            cause: alloc::format!(
                "{field} must be `> {min:#0width$X}`, but got \
                 `{got:#0width$X}`",
            ),
        })
    }

    /// Returns `Ok(())` when `min <= got <= max`, otherwise an
    /// `UnexpectedPattern` error. Used for spec-mandated inclusive
    /// range invariants (e.g. `Gamma` ∈ [2500, 65000], `Adjustment`
    /// ∈ [-100, 100]).
    pub(crate) fn expect_in_range<T>(
        field: &str,
        got: T,
        min: T,
        max: T,
    ) -> Result<(), Self>
    where
        T: Copy + PartialOrd + core::fmt::UpperHex,
    {
        if got >= min && got <= max {
            return Ok(());
        }

        let width = match core::mem::size_of::<T>() {
            1 => 4,
            2 => 6,
            _ => 10,
        };

        Err(Self::UnexpectedPattern {
            cause: alloc::format!(
                "{field} must be within `{min:#0width$X}..={max:#0width$X}`, \
                 but got `{got:#0width$X}`",
            ),
        })
    }
}

#[derive(Clone, Debug, snafu::prelude::Snafu)]
#[snafu(display("failed to read buffer: {cause}"))]
pub struct ReadError {
    cause: String,
}

impl ReadError {
    pub fn new(err: impl core::fmt::Display) -> Self {
        Self { cause: err.to_string() }
    }
}

impl From<wmf_core::parser::ReadError> for ReadError {
    fn from(err: wmf_core::parser::ReadError) -> Self {
        Self { cause: err.to_string() }
    }
}

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
    fn track(&mut self, consumed_bytes: usize) {
        *self += consumed_bytes;
    }
}

impl ConsumeTracker for crate::parser::Size {
    fn track(&mut self, consumed_bytes: usize) {
        self.consume(consumed_bytes);
    }
}

impl ReadLeField for i8 {
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 1>(buf)?;
        Ok((i8::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for i16 {
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 2>(buf)?;
        Ok((i16::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for i32 {
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 4>(buf)?;
        Ok((i32::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for u8 {
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 1>(buf)?;
        Ok((u8::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for u16 {
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 2>(buf)?;
        Ok((u16::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for u32 {
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 4>(buf)?;
        Ok((u32::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for f32 {
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
fn utf16le_bytes_to_string(bytes: &[u8]) -> Result<String, ParseError> {
    if bytes.len() % 2 != 0 {
        return Err(ParseError::UnexpectedPattern {
            cause: "Byte array length must be even".to_owned(),
        });
    }

    let u16_vec = bytes
        .chunks_exact(2)
        .map(|chunk| {
            u16::from_le_bytes(chunk.try_into().expect("should be converted"))
        })
        .collect::<Vec<_>>();

    String::from_utf16(&u16_vec)
        .map_err(|err| ParseError::UnexpectedPattern { cause: err.to_string() })
}

fn null_terminated_utf16le_string(bytes: &[u8]) -> Result<String, ParseError> {
    if bytes.len() % 2 != 0 {
        return Err(ParseError::UnexpectedPattern {
            cause: "Byte array length must be even".to_owned(),
        });
    }

    // Find the position of the first null byte (0)
    let len = bytes
        .chunks(2)
        .position(|chunk| {
            u16::from_le_bytes(chunk.try_into().expect("should be converted"))
                == 0
        })
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
    fn expect_eq_fails_on_mismatch_with_u16_width() {
        let err =
            ParseError::expect_eq("foo", 0x1234_u16, 0x5678_u16).unwrap_err();
        let msg = err.to_string();
        // Diagnostic should reflect 16-bit width as `0xNNNN`.
        assert!(msg.contains("0x5678"), "msg = {msg}");
        assert!(msg.contains("0x1234"), "msg = {msg}");
    }

    #[test]
    fn expect_eq_records_u32_width() {
        let err = ParseError::expect_eq("magic", 0x0_u32, 0x12345678_u32)
            .unwrap_err();
        let msg = err.to_string();
        // Width selected from size_of::<u32>() must show 8 hex digits.
        assert!(msg.contains("0x12345678"), "msg = {msg}");
        assert!(msg.contains("0x00000000"), "msg = {msg}");
    }

    #[test]
    fn expect_le_allows_equal() {
        assert!(ParseError::expect_le("bar", 100_u32, 100_u32).is_ok());
    }

    #[test]
    fn expect_le_rejects_overflow() {
        let err = ParseError::expect_le("bar", 101_u32, 100_u32).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("bar"), "msg = {msg}");
        assert!(msg.contains("0x00000064"), "msg = {msg}");
        assert!(msg.contains("0x00000065"), "msg = {msg}");
    }

    #[test]
    fn expect_ne_allows_distinct() {
        assert!(ParseError::expect_ne("baz", 1_i32, 0_i32).is_ok());
    }

    #[test]
    fn expect_ne_rejects_forbidden() {
        let err = ParseError::expect_ne("baz", 0_i32, 0_i32).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("baz"), "msg = {msg}");
        assert!(msg.contains("0x00000000"), "msg = {msg}");
    }

    #[test]
    fn expect_gt_accepts_above() {
        assert!(ParseError::expect_gt("cx", 1_i32, 0_i32).is_ok());
    }

    #[test]
    fn expect_gt_rejects_at_or_below() {
        assert!(ParseError::expect_gt("cx", 0_i32, 0_i32).is_err());
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
        let msg = err.to_string();
        assert!(msg.contains("g must be within"), "msg = {msg}");
    }

    #[test]
    fn expect_in_range_rejects_above() {
        let err = ParseError::expect_in_range("g", 65_001_u16, 2_500, 65_000)
            .unwrap_err();
        assert!(err.to_string().contains("g must be within"));
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
