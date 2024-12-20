/// The EMR_ROUNDRECT record specifies a rectangle with rounded corners. The
/// rectangle is outlined by using the current pen and filled by using the
/// current brush.
#[derive(Clone, Debug)]
pub struct EMR_ROUNDRECT {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_ROUNDRECT. This value is 0x0000002C.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Box (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19), which
    /// specifies the inclusive-inclusive bounding rectangle, in logical
    /// coordinates.
    pub bx: wmf_core::parser::RectL,
    /// Corner (8 bytes): A 64-bit SizeL object ([MS-WMF] section 2.2.2.22),
    /// which specifies the width and height, in logical coordinates, of the
    /// ellipse used to draw the rounded corners.
    pub corner: wmf_core::parser::SizeL,
}

impl EMR_ROUNDRECT {
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
        if record_type != crate::parser::RecordType::EMR_ROUNDRECT {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_ROUNDRECT as u32,
                    record_type as u32
                ),
            });
        }

        let ((bx, bx_bytes), (corner, corner_bytes)) = (
            wmf_core::parser::RectL::parse(buf)?,
            wmf_core::parser::SizeL::parse(buf)?,
        );

        size.consume(bx_bytes + corner_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, bx, corner })
    }
}
