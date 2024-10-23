/// The EMR_RECTANGLE record draws a rectangle. The rectangle is outlined by
/// using the current pen and filled by using the current brush.
///
/// The current drawing position is neither used nor updated by this record.
///
/// If a PS_NULL pen is used, the dimensions of the rectangle are 1 pixel less
/// in height and 1 pixel less in width.
#[derive(Clone, Debug)]
pub struct EMR_RECTANGLE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_RECTANGLE. This value is 0x0000002B.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Box (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19), which
    /// specifies the inclusive-inclusive rectangle to draw.
    pub bx: wmf_core::parser::RectL,
}

impl EMR_RECTANGLE {
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
        if record_type != crate::parser::RecordType::EMR_RECTANGLE {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_RECTANGLE as u32,
                    record_type as u32
                ),
            });
        }

        let (bx, bx_bytes) = wmf_core::parser::RectL::parse(buf)?;

        size.consume(bx_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, bx })
    }
}
