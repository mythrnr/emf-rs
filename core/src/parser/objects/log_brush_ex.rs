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
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_array_field, read_with};

        let mut consumed_bytes: usize = 0;
        let brush_style = read_with(
            buf,
            &mut consumed_bytes,
            wmf_core::parser::BrushStyle::parse,
        )?;
        // Skip 2 bytes; wmf_core::parser::BrushStyle is 2 bytes wide
        // and the on-disk layout pads the field to 4 bytes.
        let _: [u8; 2] = read_array_field(buf, &mut consumed_bytes)?;

        let v = match brush_style {
            wmf_core::parser::BrushStyle::BS_SOLID => {
                let color = read_with(
                    buf,
                    &mut consumed_bytes,
                    wmf_core::parser::ColorRef::parse,
                )?;
                let _: [u8; 4] = read_array_field(buf, &mut consumed_bytes)?;

                Self::Solid { color }
            }
            wmf_core::parser::BrushStyle::BS_NULL => {
                let _: [u8; 4] = read_array_field(buf, &mut consumed_bytes)?;
                let _: [u8; 4] = read_array_field(buf, &mut consumed_bytes)?;

                Self::Null
            }
            wmf_core::parser::BrushStyle::BS_HATCHED => {
                let color = read_with(
                    buf,
                    &mut consumed_bytes,
                    wmf_core::parser::ColorRef::parse,
                )?;
                let brush_hatch = read_with(
                    buf,
                    &mut consumed_bytes,
                    crate::parser::HatchStyle::parse,
                )?;

                Self::Hatched { color, brush_hatch }
            }
            _ => {
                return Err(crate::parser::ParseError::NotSupported {
                    cause: format!("Unsupported BrushStyle {brush_style:?}")
                        .into(),
                });
            }
        };

        Ok((v, consumed_bytes))
    }

    pub fn black_brush() -> Self {
        Self::Solid { color: wmf_core::parser::ColorRef::black() }
    }

    pub fn white_brush() -> Self {
        Self::Solid { color: wmf_core::parser::ColorRef::white() }
    }
}
