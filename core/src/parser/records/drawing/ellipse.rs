/// The EMR_ELLIPSE record specifies an ellipse. The center of the ellipse is
/// the center of the specified bounding rectangle. The ellipse is outlined by
/// using the current pen and is filled by using the current brush.
#[derive(Clone, Debug)]
pub struct EMR_ELLIPSE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_ELLIPSE. This value is 0x0000002A.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Box (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19), which
    /// specifies the inclusive-inclusive bounding rectangle in logical units.
    pub bx: wmf_core::parser::RectL,
}

impl EMR_ELLIPSE {
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
        if record_type != crate::parser::RecordType::EMR_ELLIPSE {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_ELLIPSE as u32,
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
