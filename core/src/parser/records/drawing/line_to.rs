/// The EMR_LINETO record specifies a line from the current drawing position up
/// to, but not including, the specified point. It resets the current position
/// to the specified point.
#[derive(Clone, Debug)]
pub struct EMR_LINETO {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_LINETO. This value is 0x00000036.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Point (8 bytes): A PointL object ([MS-WMF] section 2.2.2.15), which
    /// specifies the coordinates of the line's endpoint.
    pub point: wmf_core::parser::PointL,
}

impl EMR_LINETO {
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
        if record_type != crate::parser::RecordType::EMR_LINETO {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_LINETO as u32,
                    record_type as u32
                ),
            });
        }

        let (point, point_bytes) = wmf_core::parser::PointL::parse(buf)?;

        size.consume(point_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, point })
    }
}
