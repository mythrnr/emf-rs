/// The EMR_SETBKCOLOR record specifies the background color for text output.
#[derive(Clone, Debug)]
pub struct EMR_SETBKCOLOR {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETBKCOLOR. This value is 0x00000019.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Color (4 bytes): A ColorRef object ([MS-WMF] section 2.2.2.8), which
    /// specifies the background color value.
    pub color: wmf_core::parser::ColorRef,
}

impl EMR_SETBKCOLOR {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_SETBKCOLOR {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETBKCOLOR as u32,
                    record_type as u32
                ),
            });
        }

        let (color, color_bytes) = wmf_core::parser::ColorRef::parse(buf)?;

        size.consume(color_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, color })
    }
}
