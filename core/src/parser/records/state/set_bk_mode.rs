/// The EMR_SETBKMODE record specifies the background mix mode to use with text,
/// hatched brushes, and pens that are not solid lines.
#[derive(Clone, Debug)]
pub struct EMR_SETBKMODE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETBKMODE. This value is 0x00000012.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// BackgroundMode (4 bytes): An unsigned integer that specifies the
    /// background mode, from the BackgroundMode enumeration.
    pub background_mode: crate::parser::BackgroundMode,
}

impl EMR_SETBKMODE {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = ?record_type),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{consume_remaining_bytes, read_with};

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_SETBKMODE as u32,
        )?;

        let background_mode =
            read_with(buf, &mut size, crate::parser::BackgroundMode::parse)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, background_mode })
    }
}
