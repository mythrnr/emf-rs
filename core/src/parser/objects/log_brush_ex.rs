/// The LogBrushEx object defines the style, color, and pattern of a
/// device-independent brush.
#[derive(Clone, Debug)]
pub struct LogBrushEx {
    /// BrushStyle (4 bytes): An unsigned integer that specifies the brush
    /// style. The value MUST be an enumeration from BrushStyle enumeration
    /// ([MS-WMF] section 2.1.1.4). The style values that are supported in this
    /// structure are listed later in this section. The BS_NULL style SHOULD be
    /// used to specify a brush that has no effect.
    pub brush_style: wmf_core::parser::BrushStyle,
    /// Color (4 bytes): A 32-bit ColorRef object ([MS-WMF] section 2.2.2.8)
    /// that specifies a color. The interpretation of this field depends on the
    /// value of BrushStyle, as explained in the following table.
    pub color: wmf_core::parser::ColorRef,
    /// BrushHatch (4 bytes): A 32-bit unsigned field that contains the brush
    /// hatch data. Its interpretation depends on the value of BrushStyle, as
    /// explained in the following table.
    ///
    /// The following table shows the relationship between the BrushStyle,
    /// Color, and BrushHatch fields in a LogBrushEx object. Only supported
    /// brush styles are listed.
    ///
    /// | BrushStyle | Color | BrushHatch |
    /// |:-|:-|:-|
    /// | `BS_SOLID` | A ColorRef object, which specifies the color of the brush. | Not used and SHOULD be ignored. |
    /// | `BS_NULL` | Not used and SHOULD be ignored. | Not used and SHOULD be ignored. |
    /// | `BS_HATCHED` | A ColorRef object, which specifies the foreground color of the hatch pattern. | A value from the HatchStyle enumeration, which specifies the orientation of lines used to create the hatch. |
    pub brush_hatch: crate::parser::HatchStyle,
}

impl LogBrushEx {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (brush_style, brush_style_bytes),
            (color, color_bytes),
            (brush_hatch, brush_hatch_bytes),
        ) = (
            wmf_core::parser::BrushStyle::parse(buf)?,
            wmf_core::parser::ColorRef::parse(buf)?,
            crate::parser::HatchStyle::parse(buf)?,
        );

        Ok((
            Self { brush_style, color, brush_hatch },
            brush_style_bytes + color_bytes + brush_hatch_bytes,
        ))
    }
}
