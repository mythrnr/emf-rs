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
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::read_with;

        let mut consumed_bytes: usize = 0;
        let pen_style = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::PenStyle::parse,
        )?;
        let width = read_with(
            buf,
            &mut consumed_bytes,
            wmf_core::parser::PointL::parse,
        )?;
        let color_ref = read_with(
            buf,
            &mut consumed_bytes,
            wmf_core::parser::ColorRef::parse,
        )?;

        Ok((Self { pen_style, width, color_ref }, consumed_bytes))
    }
}
