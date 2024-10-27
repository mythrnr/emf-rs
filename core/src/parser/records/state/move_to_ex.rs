/// The EMR_MOVETOEX record specifies the coordinates of s new drawing position
/// in logical units.
#[derive(Clone, Debug)]
pub struct EMR_MOVETOEX {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_MOVETOEX. This value is 0x0000001B.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Offset (8 bytes): A PointL object ([MS-WMF] section 2.2.2.15), which
    /// specifies coordinates of the new drawing position in logical units.
    pub offset: wmf_core::parser::PointL,
}

impl EMR_MOVETOEX {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_MOVETOEX {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_MOVETOEX as u32,
                    record_type as u32
                ),
            });
        }

        let (offset, offset_bytes) = wmf_core::parser::PointL::parse(buf)?;

        size.consume(offset_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, offset })
    }
}
