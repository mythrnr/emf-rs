/// The EMR_SETWINDOWORGEX record defines the window origin.
#[derive(Clone, Debug)]
pub struct EMR_SETWINDOWORGEX {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETWINDOWORGEX. This value is 0x0000000A.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Origin (8 bytes): A PointL object ([MS-WMF] section 2.2.2.15) that
    /// specifies the window horizontal and vertical origin in logical units.
    pub origin: wmf_core::parser::PointL,
}

impl EMR_SETWINDOWORGEX {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_SETWINDOWORGEX {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETWINDOWORGEX as u32,
                    record_type as u32
                ),
            });
        }

        let (origin, origin_bytes) = wmf_core::parser::PointL::parse(buf)?;

        size.consume(origin_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, origin })
    }
}
