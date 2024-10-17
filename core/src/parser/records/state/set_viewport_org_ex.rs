/// The EMR_SETVIEWPORTORGEX record defines the viewport origin.
#[derive(Clone, Debug)]
pub struct EMR_SETVIEWPORTORGEX {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETVIEWPORTORGEX. This value is 0x0000000C.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Origin (8 bytes): A PointL object ([MS-WMF] section 2.2.2.15) that
    /// specifies the window horizontal and vertical origin in device units.
    pub origin: wmf_core::parser::PointL,
}

impl EMR_SETVIEWPORTORGEX {
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
        if record_type != crate::parser::RecordType::EMR_SETVIEWPORTORGEX {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETVIEWPORTORGEX as u32,
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
