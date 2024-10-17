/// The LogPen object defines the style, width, and color of a logical pen.
#[derive(Clone, Debug)]
pub struct LogPen {
    /// PenStyle (4 bytes): An unsigned integer that specifies a value from the
    /// PenStyle enumeration.
    pub pen_style: crate::parser::PenStyle,
    /// Width (8 bytes): A PointL object ([MS-WMF] section 2.2.2.15) that
    /// specifies the width of the pen by the value of its x field. The value
    /// of its y field MUST be ignored.
    ///
    /// If the pen type in the PenStyle field is PS_GEOMETRIC, this value is
    /// the width in logical units; otherwise, the width is specified in device
    /// units. If the pen type in the PenStyle field is PS_COSMETIC, this value
    /// MUST be 0x00000001.
    pub width: wmf_core::parser::PointL,
    /// ColorRef (4 bytes): A ColorRef object ([MS-WMF] section 2.2.2.8) that
    /// specifies the pen color value.
    pub color_ref: wmf_core::parser::ColorRef,
}

impl LogPen {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (pen_style, pen_style_bytes),
            (width, width_bytes),
            (color_ref, color_ref_bytes),
        ) = (
            crate::parser::PenStyle::parse(buf)?,
            wmf_core::parser::PointL::parse(buf)?,
            wmf_core::parser::ColorRef::parse(buf)?,
        );

        Ok((
            Self { pen_style, width, color_ref },
            pen_style_bytes + width_bytes + color_ref_bytes,
        ))
    }
}
