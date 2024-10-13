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
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_REALIZEPALETTE {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_REALIZEPALETTE as u32,
                    record_type as u32
                ),
            });
        }

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size })
    }
}
