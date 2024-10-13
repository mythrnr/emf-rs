/// The EMR_FORCEUFIMAPPING record forces the font mapper to match fonts based
/// on their UniversalFontId in preference to their LogFont information.
#[derive(Clone, Debug)]
pub struct EMR_FORCEUFIMAPPING {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_FORCEUFIMAPPING. This value is 0x0000006D.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// ufi (8 bytes): The font id to use, specified as a UniversalFontId.
    pub ufi: crate::parser::UniversalFontId,
}

impl EMR_FORCEUFIMAPPING {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_FORCEUFIMAPPING {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_FORCEUFIMAPPING as u32,
                    record_type as u32
                ),
            });
        }

        let (ufi, ufi_bytes) = crate::parser::UniversalFontId::parse(buf)?;

        size.consume(ufi_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, ufi })
    }
}
