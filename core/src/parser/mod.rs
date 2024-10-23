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
        use wmf_core::parser::ParseError::*;

        match err {
            FailedReadBuffer { cause } => {
                Self::FailedReadBuffer { cause: cause.into() }
            }
            NotSupported { .. } => {
                Self::NotSupported { cause: err.to_string() }
            }
            UnexpectedEnumValue { .. } => {
                Self::UnexpectedEnumValue { cause: err.to_string() }
            }
            UnexpectedPattern { .. } => {
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

    match buf.read(&mut buffer) {
        Ok(bytes_read) if bytes_read == N => Ok((buffer, N)),
        Ok(bytes_read) => Err(ReadError::new(format!(
            "expected {N} bytes read, but {bytes_read} bytes read"
        ))),
        Err(err) => Err(ReadError::new(format!("{err:?}"))),
    }
}

pub fn read_variable<R: crate::Read>(
    buf: &mut R,
    len: usize,
) -> Result<(Vec<u8>, usize), ReadError> {
    if len == 0 {
        return Ok((vec![0u8; 0], 0));
    }

    let mut buffer = vec![0u8; len];

    match buf.read(&mut buffer) {
        Ok(bytes_read) if bytes_read == len => Ok((buffer, len)),
        Ok(bytes_read) => Err(ReadError::new(format!(
            "expected {len} bytes read, but {bytes_read} bytes read"
        ))),
        Err(err) => Err(ReadError::new(format!("{err:?}"))),
    }
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
