/// The EMR_SETPIXELV record defines the color of the pixel at the specified
/// logical coordinates.
#[derive(Clone, Debug)]
pub struct EMR_SETPIXELV {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETPIXELV. This value is 0x0000000F.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Pixel (8 bytes): A PointL object ([MS-WMF] section 2.2.2.15) that
    /// specifies the logical coordinates for the pixel.
    pub pixel: wmf_core::parser::PointL,
    /// Color (4 bytes): A 32-bit ColorRef object ([MS-WMF] section 2.2.2.8)
    /// that specifies the pixel color.
    pub color: wmf_core::parser::ColorRef,
}

impl EMR_SETPIXELV {
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
        if record_type != crate::parser::RecordType::EMR_SETPIXELV {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETPIXELV as u32,
                    record_type as u32
                ),
            });
        }

        let ((pixel, pixel_bytes), (color, color_bytes)) = (
            wmf_core::parser::PointL::parse(buf)?,
            wmf_core::parser::ColorRef::parse(buf)?,
        );

        size.consume(pixel_bytes + color_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, pixel, color })
    }
}
