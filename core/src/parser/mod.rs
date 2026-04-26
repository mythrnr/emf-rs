mod enums;
mod objects;
mod primitive;
mod records;

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

pub fn read<R: crate::Read, const N: usize>(
    buf: &mut R,
) -> Result<([u8; N], usize), ReadError> {
    let mut buffer = [0u8; N];

    read_exact(buf, &mut buffer)?;

    Ok((buffer, N))
}

pub fn read_variable<R: crate::Read>(
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

/// `embedded_io::Read` may return short reads, so loop until `buffer`
/// is fully populated or the stream signals EOF via `Ok(0)`.
fn read_exact<R: crate::Read>(
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

macro_rules! impl_from_le_bytes {
    ($(($t:ty, $n:expr)),+) => {
        paste::paste!{
            $(
                pub fn [<read_ $t _from_le_bytes>]<R: $crate::Read>(
                    buf: &mut R,
                ) -> Result<($t, usize), ReadError> {
                    let (bytes, consumed_bytes) = read::<R, $n>(buf)?;

                    Ok((<$t>::from_le_bytes(bytes), consumed_bytes))
                }
            )*
        }
    };
}

impl_from_le_bytes! {
    (i8, 1), (i16, 2), (i32, 4),
    (u8, 1), (u16, 2), (u32, 4),
    (f32, 4)
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
    fn read_i16_roundtrip() {
        let data = (-1234_i16).to_le_bytes();
        let mut reader = &data[..];
        let (val, consumed) = read_i16_from_le_bytes(&mut reader).unwrap();
        assert_eq!(val, -1234);
        assert_eq!(consumed, 2);
    }

    #[test]
    fn read_u16_roundtrip() {
        let data = 0xABCD_u16.to_le_bytes();
        let mut reader = &data[..];
        let (val, consumed) = read_u16_from_le_bytes(&mut reader).unwrap();
        assert_eq!(val, 0xABCD);
        assert_eq!(consumed, 2);
    }

    #[test]
    fn read_i32_roundtrip() {
        let data = (-99999_i32).to_le_bytes();
        let mut reader = &data[..];
        let (val, consumed) = read_i32_from_le_bytes(&mut reader).unwrap();
        assert_eq!(val, -99999);
        assert_eq!(consumed, 4);
    }

    #[test]
    fn read_u32_roundtrip() {
        let data = 0xDEADBEEF_u32.to_le_bytes();
        let mut reader = &data[..];
        let (val, consumed) = read_u32_from_le_bytes(&mut reader).unwrap();
        assert_eq!(val, 0xDEADBEEF);
        assert_eq!(consumed, 4);
    }

    #[test]
    fn read_u32_from_short_buffer_errors() {
        let data = [0_u8; 2];
        let mut reader = &data[..];
        assert!(read_u32_from_le_bytes(&mut reader).is_err());
    }

    #[test]
    fn read_f32_roundtrip() {
        let data = 1.5_f32.to_le_bytes();
        let mut reader = &data[..];
        let (val, consumed) = read_f32_from_le_bytes(&mut reader).unwrap();
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
