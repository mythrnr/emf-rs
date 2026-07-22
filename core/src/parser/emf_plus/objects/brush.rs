use crate::{
    imports::*,
    parser::emf_plus::objects::{
        EmfPlusARGB, EmfPlusGraphicsVersion, EmfPlusLinearGradientBrushData,
        EmfPlusPathGradientBrushData, EmfPlusTextureBrushData,
    },
};

/// The EmfPlusBrush object specifies a graphics brush for filling
/// regions (MS-EMFPLUS 2.2.1.1).
///
/// This object is generic and is used to specify different types of
/// brush data, including the following objects:
///
/// - EmfPlusHatchBrushData object (section 2.2.2.20)
/// - EmfPlusLinearGradientBrushData object (section 2.2.2.24)
/// - EmfPlusPathGradientBrushData object (section 2.2.2.29)
/// - EmfPlusSolidBrushData object (section 2.2.2.43)
/// - EmfPlusTextureBrushData object (section 2.2.2.45)
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusBrush {
    /// Version (4 bytes): An EmfPlusGraphicsVersion object (section
    /// 2.2.2.19) that specifies the version of operating system
    /// graphics that was used to create this object.
    pub version: EmfPlusGraphicsVersion,
    /// BrushData (variable): Variable-length data that defines the
    /// brush object specified in the Type field. The content and
    /// format of the data can be different for every brush type.
    ///
    /// The BrushType field of the wire format is implied by the
    /// variant.
    pub brush_data: EmfPlusBrushData,
}

/// The BrushData field of an EmfPlusBrush object.
#[derive(Clone, Debug, PartialEq)]
pub enum EmfPlusBrushData {
    /// A solid color fill (MS-EMFPLUS 2.2.2.43).
    SolidColor(EmfPlusSolidBrushData),
    /// A hatch pattern fill (MS-EMFPLUS 2.2.2.20).
    HatchFill(EmfPlusHatchBrushData),
    /// A texture image fill (MS-EMFPLUS 2.2.2.45).
    TextureFill(EmfPlusTextureBrushData),
    /// A path gradient fill (MS-EMFPLUS 2.2.2.29).
    PathGradient(EmfPlusPathGradientBrushData),
    /// A linear gradient fill (MS-EMFPLUS 2.2.2.24).
    LinearGradient(EmfPlusLinearGradientBrushData),
}

impl EmfPlusBrush {
    /// Parses a brush from at most `available` bytes of object data.
    /// The budget bounds the variable-length brush data (texture
    /// images, gradient blend patterns).
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        available: usize,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::read_with;

        let mut consumed_bytes: usize = 0;
        let version =
            read_with(buf, &mut consumed_bytes, EmfPlusGraphicsVersion::parse)?;
        let brush_type = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::BrushType::parse,
        )?;

        let remaining = available.saturating_sub(consumed_bytes);
        let brush_data = match brush_type {
            crate::parser::emf_plus::BrushType::BrushTypeSolidColor => {
                EmfPlusBrushData::SolidColor(read_with(
                    buf,
                    &mut consumed_bytes,
                    EmfPlusSolidBrushData::parse,
                )?)
            }
            crate::parser::emf_plus::BrushType::BrushTypeHatchFill => {
                EmfPlusBrushData::HatchFill(read_with(
                    buf,
                    &mut consumed_bytes,
                    EmfPlusHatchBrushData::parse,
                )?)
            }
            crate::parser::emf_plus::BrushType::BrushTypeTextureFill => {
                let (v, c) = EmfPlusTextureBrushData::parse(buf, remaining)?;
                consumed_bytes += c;
                EmfPlusBrushData::TextureFill(v)
            }
            crate::parser::emf_plus::BrushType::BrushTypePathGradient => {
                let (v, c) =
                    EmfPlusPathGradientBrushData::parse(buf, remaining)?;
                consumed_bytes += c;
                EmfPlusBrushData::PathGradient(v)
            }
            crate::parser::emf_plus::BrushType::BrushTypeLinearGradient => {
                let (v, c) =
                    EmfPlusLinearGradientBrushData::parse(buf, remaining)?;
                consumed_bytes += c;
                EmfPlusBrushData::LinearGradient(v)
            }
        };

        Ok((Self { version, brush_data }, consumed_bytes))
    }
}

/// The EmfPlusSolidBrushData object specifies a solid color for a
/// graphics brush (MS-EMFPLUS 2.2.2.43).
///
/// Graphics brushes are specified by EmfPlusBrush objects (section
/// 2.2.1.1). A solid color brush paints a background in a solid color.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmfPlusSolidBrushData {
    /// SolidColor (4 bytes): An EmfPlusARGB object (section 2.2.2.1)
    /// that specifies the color of the brush.
    pub solid_color: EmfPlusARGB,
}

impl EmfPlusSolidBrushData {
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
        let solid_color =
            read_with(buf, &mut consumed_bytes, EmfPlusARGB::parse)?;

        Ok((Self { solid_color }, consumed_bytes))
    }
}

/// The EmfPlusHatchBrushData object specifies a hatch pattern for a
/// graphics brush (MS-EMFPLUS 2.2.2.20).
///
/// Graphics brushes are specified by EmfPlusBrush objects (section
/// 2.2.1.1). A hatch brush paints a background and draws a pattern of
/// lines, dots, dashes, squares, and crosshatch lines over this
/// background. The hatch brush defines two colors: one for the
/// background and one for the pattern over the background. The color
/// of the background is called the background color, and the color of
/// the pattern is called the foreground color.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmfPlusHatchBrushData {
    /// HatchStyle (4 bytes): An unsigned integer that specifies the
    /// brush hatch style. It is defined in the HatchStyle enumeration
    /// (section 2.1.1.13).
    pub hatch_style: crate::parser::emf_plus::HatchStyle,
    /// ForeColor (4 bytes): An EmfPlusARGB object (section 2.2.2.1)
    /// that specifies the color used to draw the lines of the hatch
    /// pattern.
    pub fore_color: EmfPlusARGB,
    /// BackColor (4 bytes): An EmfPlusARGB object that specifies the
    /// color used to paint the background of the hatch pattern.
    pub back_color: EmfPlusARGB,
}

impl EmfPlusHatchBrushData {
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
        let hatch_style = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::HatchStyle::parse,
        )?;
        let fore_color =
            read_with(buf, &mut consumed_bytes, EmfPlusARGB::parse)?;
        let back_color =
            read_with(buf, &mut consumed_bytes, EmfPlusARGB::parse)?;

        Ok((Self { hatch_style, fore_color, back_color }, consumed_bytes))
    }
}

/// Rejects the specification-invalid combination of BrushDataPresetColors
/// and BrushDataBlendFactorsH, whose optional-data layout would be
/// ambiguous.
pub(in crate::parser::emf_plus) fn check_blend_pattern_flags(
    flags: crate::parser::emf_plus::BrushDataFlags,
) -> Result<(), crate::parser::ParseError> {
    use crate::parser::emf_plus::BrushDataFlags;

    if flags.contains(BrushDataFlags::PRESET_COLORS)
        && flags.contains(BrushDataFlags::BLEND_FACTORS_H)
    {
        return Err(crate::parser::ParseError::UnexpectedPattern {
            cause: Cow::from(
                "brush data must not contain both BrushDataPresetColors and \
                 BrushDataBlendFactorsH",
            ),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::HatchStyle;

    fn version_bytes() -> [u8; 4] {
        0xDBC0_1002_u32.to_le_bytes()
    }

    #[test]
    fn parses_solid_brush() {
        let mut data = vec![];
        data.extend(version_bytes());
        data.extend(0_u32.to_le_bytes());
        data.extend([0x10, 0x20, 0x30, 0xFF]);

        let mut buf: &[u8] = &data;
        let (brush, consumed) =
            EmfPlusBrush::parse(&mut buf, data.len()).unwrap();

        assert_eq!(consumed, data.len());
        assert_eq!(
            brush.brush_data,
            EmfPlusBrushData::SolidColor(EmfPlusSolidBrushData {
                solid_color: EmfPlusARGB {
                    blue: 0x10,
                    green: 0x20,
                    red: 0x30,
                    alpha: 0xFF,
                },
            }),
        );
    }

    #[test]
    fn parses_hatch_brush() {
        let mut data = vec![];
        data.extend(version_bytes());
        data.extend(1_u32.to_le_bytes());
        data.extend(0x0000_0004_u32.to_le_bytes()); // LargeGrid
        data.extend([0x00, 0x00, 0xFF, 0xFF]); // fore: red
        data.extend([0xFF, 0xFF, 0xFF, 0xFF]); // back: white

        let mut buf: &[u8] = &data;
        let (brush, consumed) =
            EmfPlusBrush::parse(&mut buf, data.len()).unwrap();

        assert_eq!(consumed, data.len());

        let EmfPlusBrushData::HatchFill(hatch) = brush.brush_data else {
            panic!("expected hatch brush data");
        };
        assert_eq!(hatch.hatch_style, HatchStyle::HatchStyleLargeGrid);
        assert_eq!(hatch.fore_color.red, 0xFF);
        assert_eq!(hatch.fore_color.blue, 0x00);
    }

    #[test]
    fn rejects_ambiguous_blend_pattern_flags() {
        use crate::parser::emf_plus::BrushDataFlags;

        let flags = BrushDataFlags::from_raw(
            BrushDataFlags::PRESET_COLORS | BrushDataFlags::BLEND_FACTORS_H,
        );

        assert!(check_blend_pattern_flags(flags).is_err());
    }
}
