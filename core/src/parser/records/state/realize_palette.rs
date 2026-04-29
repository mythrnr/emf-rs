/// This record maps palette entries from the current LogPalette object to the
/// system_palette. This record specifies no parameters.
#[derive(Clone, Debug)]
pub struct EMR_REALIZEPALETTE {
    /// Type (4 bytes): An unsigned integer that identifies the record type as
    /// EMR_REALIZEPALETTE. This value is 0x00000034.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of the
    /// record in bytes.
    pub size: crate::parser::Size,
}

impl EMR_REALIZEPALETTE {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::consume_remaining_bytes;

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_REALIZEPALETTE as u32,
        )?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size })
    }
}
