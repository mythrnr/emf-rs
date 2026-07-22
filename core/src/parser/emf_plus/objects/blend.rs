//! Blend pattern objects for gradient brushes (MS-EMFPLUS 2.2.2.4
//! EmfPlusBlendColors, 2.2.2.5 EmfPlusBlendFactors).

use crate::{imports::*, parser::emf_plus::objects::EmfPlusARGB};

/// The EmfPlusBlendColors object specifies positions and colors for the
/// blend pattern of a gradient brush (MS-EMFPLUS 2.2.2.4).
///
/// Gradient brushes are specified by EmfPlusLinearGradientBrushData
/// objects (section 2.2.2.24) and EmfPlusPathGradientBrushData objects
/// (section 2.2.2.29). Blend patterns are used to smoothly shade the
/// interiors of shapes filled by gradient brushes, and can be defined
/// by arrays of positions and colors or positions and factors.
/// Positions and factors are specified by EmfPlusBlendFactors objects
/// (section 2.2.2.5).
///
/// An EmfPlusBlendColors object MUST be present in the OptionalData
/// field of an EmfPlusLinearGradientBrushData object, if the
/// BrushDataPresetColors flag is set in its BrushDataFlags field.
///
/// An EmfPlusBlendColors object MUST be present in the OptionalData
/// field of an EmfPlusPathGradientBrushData object, if the
/// BrushDataPresetColors flag is set in its BrushDataFlags field.
///
/// The PositionCount field (4 bytes), which specifies the number of
/// positions in the BlendPositions field and colors in the BlendColors
/// field, is represented by the lengths of the two vectors.
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusBlendColors {
    /// BlendPositions (variable): An array of PositionCount 32-bit
    /// floating-point values that specify proportions of distance
    /// along the gradient line.
    ///
    /// Each element MUST be a number between 0.0 and 1.0 inclusive.
    /// For a linear gradient brush, 0.0 represents the starting point
    /// and 1.0 represents the ending point. For a path gradient brush,
    /// 0.0 represents the midpoint and 1.0 represents an endpoint.
    pub blend_positions: Vec<f32>,
    /// BlendColors (variable): An array of PositionCount EmfPlusARGB
    /// objects (section 2.2.2.1) that specify colors at the positions
    /// defined in the BlendPositions field.
    pub blend_colors: Vec<EmfPlusARGB>,
}

impl EmfPlusBlendColors {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let position_count: u32 = read_field(buf, &mut consumed_bytes)?;

        crate::parser::emf_plus::check_element_count(
            "PositionCount",
            position_count,
        )?;

        let mut blend_positions = vec![];
        for _ in 0..position_count {
            blend_positions.push(read_field(buf, &mut consumed_bytes)?);
        }

        let mut blend_colors = vec![];
        for _ in 0..position_count {
            blend_colors.push(read_with(
                buf,
                &mut consumed_bytes,
                EmfPlusARGB::parse,
            )?);
        }

        Ok((Self { blend_positions, blend_colors }, consumed_bytes))
    }
}

/// The EmfPlusBlendFactors object specifies positions and factors for
/// the blend pattern of a gradient brush (MS-EMFPLUS 2.2.2.5).
///
/// Gradient brushes are specified by EmfPlusLinearGradientBrushData
/// objects (section 2.2.2.24) and EmfPlusPathGradientBrushData objects
/// (section 2.2.2.29). Blend patterns are used to smoothly shade the
/// interiors of shapes filled by gradient brushes, and can be defined
/// by arrays of positions and colors or positions and factors.
/// Positions and colors are specified by EmfPlusBlendColors objects
/// (section 2.2.2.4).
///
/// An EmfPlusBlendFactors object MUST be present in the OptionalData
/// field of an EmfPlusLinearGradientBrushData or
/// EmfPlusPathGradientBrushData object if either of the flags
/// BrushDataBlendFactorsH or BrushDataBlendFactorsV is set in its
/// BrushDataFlags field.
///
/// The PositionCount field (4 bytes), which specifies the number of
/// positions in the BlendPositions field and factors in the
/// BlendFactors field, is represented by the lengths of the two
/// vectors.
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusBlendFactors {
    /// BlendPositions (variable): An array of PositionCount 32-bit
    /// floating-point values that specify proportions of distance
    /// along the gradient line.
    ///
    /// Each value MUST be a number between 0.0 and 1.0 inclusive.
    /// There MUST be at least two positions specified: the first
    /// position, which is always 0.0f, and the last position, which is
    /// always 1.0f. Each position in BlendPositions is generally
    /// greater than the preceding position. For a linear gradient
    /// brush, 0.0 represents the starting point and 1.0 represents the
    /// ending point. For a path gradient brush, 0.0 represents the
    /// midpoint and 1.0 represents an endpoint.
    pub blend_positions: Vec<f32>,
    /// BlendFactors (variable): An array of PositionCount 32-bit
    /// floating-point values that specify proportions of colors at the
    /// positions defined in the BlendPositions field. Each value MUST
    /// be a number between 0.0 and 1.0 inclusive.
    ///
    /// For a linear gradient brush, 0.0 represents 0% starting color
    /// and 100% ending color, and 1.0 represents 100% starting color
    /// and 0% ending color. For a path gradient brush, 0.0 represents
    /// 0% midpoint color and 100% endpoint color, and 1.0 represents
    /// 100% midpoint color and 0% endpoint color.
    ///
    /// For example, if a linear gradient brush specifies a position of
    /// 0.2 and a factor of 0.3 along a gradient line that is 100
    /// pixels long, the color that is 20 pixels along that line
    /// consists of 30 percent starting color and 70 percent ending
    /// color.
    pub blend_factors: Vec<f32>,
}

impl EmfPlusBlendFactors {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::read_field;

        let mut consumed_bytes: usize = 0;
        let position_count: u32 = read_field(buf, &mut consumed_bytes)?;

        crate::parser::emf_plus::check_element_count(
            "PositionCount",
            position_count,
        )?;

        let mut blend_positions = vec![];
        for _ in 0..position_count {
            blend_positions.push(read_field(buf, &mut consumed_bytes)?);
        }

        let mut blend_factors = vec![];
        for _ in 0..position_count {
            blend_factors.push(read_field(buf, &mut consumed_bytes)?);
        }

        Ok((Self { blend_positions, blend_factors }, consumed_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_positions_then_factors() {
        let mut data = vec![];
        data.extend(2_u32.to_le_bytes());
        data.extend(0.0_f32.to_le_bytes());
        data.extend(1.0_f32.to_le_bytes());
        data.extend(0.25_f32.to_le_bytes());
        data.extend(0.75_f32.to_le_bytes());

        let mut buf: &[u8] = &data;
        let (v, consumed) = EmfPlusBlendFactors::parse(&mut buf).unwrap();

        assert_eq!(consumed, data.len());
        assert_eq!(v.blend_positions, vec![0.0, 1.0]);
        assert_eq!(v.blend_factors, vec![0.25, 0.75]);
    }
}
