/// The LogBrushEx object defines the style, color, and pattern of a
/// device-independent brush.
#[derive(Clone, Debug)]
pub enum LogBrushEx {
    Solid {
        /// Color (4 bytes): A 32-bit ColorRef object ([MS-WMF] section
        /// 2.2.2.8) that specifies a color. The interpretation of this field
        /// depends on the value of BrushStyle, as explained in the following
        /// table.
        color: wmf_core::parser::ColorRef,
    },
    Null,
    Hatched {
        /// Color (4 bytes): A 32-bit ColorRef object ([MS-WMF] section
        /// 2.2.2.8) that specifies a color. The interpretation of this field
        /// depends on the value of BrushStyle, as explained in the following
        /// table.
        color: wmf_core::parser::ColorRef,
        /// BrushHatch (4 bytes): A 32-bit unsigned field that contains the
        /// brush hatch data. Its interpretation depends on the value of
        /// BrushStyle, as explained in the following table.
        brush_hatch: crate::parser::HatchStyle,
    },
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

        let v = match brush_style {
            wmf_core::parser::BrushStyle::BS_SOLID => Self::Solid { color },
            wmf_core::parser::BrushStyle::BS_NULL => Self::Null,
            wmf_core::parser::BrushStyle::BS_HATCHED => {
                Self::Hatched { color, brush_hatch }
            }
            _ => {
                return Err(crate::parser::ParseError::NotSupported {
                    cause: format!("Unsupported BrushStyle {brush_style:?}"),
                })
            }
        };

        Ok((v, brush_style_bytes + color_bytes + brush_hatch_bytes))
    }
}
