use crate::parser::emf_plus::objects::{
    EmfPlusARGB, EmfPlusBlendColors, EmfPlusBlendFactors, EmfPlusRectF,
    EmfPlusTransformMatrix, brush::check_blend_pattern_flags,
};

/// The EmfPlusLinearGradientBrushData object specifies a linear
/// gradient for a graphics brush (MS-EMFPLUS 2.2.2.24).
///
/// Graphics brushes are specified by EmfPlusBrush objects (section
/// 2.2.1.1). A linear gradient brush paints a color gradient in which
/// the color changes gradually along a gradient line from a starting
/// boundary point to an ending boundary point, which are specified by
/// the diagonal of a rectangle in the RectF field.
///
/// Gamma correction controls the overall brightness and intensity of
/// an image. Uncorrected images can look either bleached out or too
/// dark. Varying the amount of gamma correction changes not only the
/// brightness but also the ratios of red to green to blue. The need
/// for gamma correction arises because an output device might not
/// render colors in the same intensity as the input image.
///
/// The OptionalData field (an EmfPlusLinearGradientBrushOptionalData
/// object, section 2.2.2.25) is inlined into this struct as the
/// `transform_matrix` and `blend_pattern_*` fields. Per the
/// specification, its BlendPattern field is an optional blend pattern
/// for the linear gradient brush; if present, it MUST contain either
/// an EmfPlusBlendColors object (section 2.2.2.4), or one or two
/// EmfPlusBlendFactors objects (section 2.2.2.5), but it MUST NOT
/// contain both. The valid combinations of flags in the BrushDataFlags
/// field and the corresponding blend patterns are:
///
/// - PresetColors clear, BlendFactorsH clear, BlendFactorsV clear: This field
///   MUST NOT be present in the EmfPlusLinearGradientBrushOptionalData object.
/// - PresetColors set, BlendFactorsH clear, BlendFactorsV clear: An
///   EmfPlusBlendColors object MUST be present.
/// - PresetColors clear, BlendFactorsH set, BlendFactorsV clear: An
///   EmfPlusBlendFactors object along the horizontal gradient line MUST be
///   present.
/// - PresetColors clear, BlendFactorsH clear, BlendFactorsV set: An
///   EmfPlusBlendFactors object along the vertical gradient line MUST be
///   present.
/// - PresetColors clear, BlendFactorsH set, BlendFactorsV set: An
///   EmfPlusBlendFactors object along the vertical gradient line and an
///   EmfPlusBlendFactors object along the horizontal gradient line MUST be
///   present.
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusLinearGradientBrushData {
    /// BrushDataFlags (4 bytes): An unsigned integer that specifies
    /// the data in the OptionalData field. This value MUST be composed
    /// of BrushData flags (section 2.1.2.1). The following flags are
    /// relevant to a linear gradient brush:
    ///
    /// | Name | Value |
    /// |:-|:-|
    /// | BrushDataTransform | 0x00000002 |
    /// | BrushDataPresetColors | 0x00000004 |
    /// | BrushDataBlendFactorsH | 0x00000008 |
    /// | BrushDataBlendFactorsV | 0x00000010 |
    /// | BrushDataIsGammaCorrected | 0x00000080 |
    pub brush_data_flags: crate::parser::emf_plus::BrushDataFlags,
    /// WrapMode (4 bytes): A signed integer from the WrapMode
    /// enumeration (section 2.1.1.33) that specifies whether to paint
    /// the area outside the boundary of the brush. When painting
    /// outside the boundary, the wrap mode specifies how the color
    /// gradient is repeated.
    pub wrap_mode: crate::parser::emf_plus::WrapMode,
    /// RectF (16 bytes): An EmfPlusRectF object (section 2.2.2.39)
    /// that specifies the starting and ending points of the gradient
    /// line. The upper-left corner of the rectangle is the starting
    /// point. The lower-right corner is the ending point.
    pub rect: EmfPlusRectF,
    /// StartColor (4 bytes): An EmfPlusARGB object (section 2.2.2.1)
    /// that specifies the color at the starting boundary point of the
    /// linear gradient brush.
    pub start_color: EmfPlusARGB,
    /// EndColor (4 bytes): An EmfPlusARGB object that specifies the
    /// color at the ending boundary point of the linear gradient
    /// brush.
    pub end_color: EmfPlusARGB,
    /// Reserved1 (4 bytes): This field is reserved and SHOULD be
    /// ignored.
    pub reserved_1: u32,
    /// Reserved2 (4 bytes): This field is reserved and SHOULD be
    /// ignored.
    pub reserved_2: u32,
    /// TransformMatrix (24 bytes): An optional EmfPlusTransformMatrix
    /// object (section 2.2.2.47) that specifies a world space to
    /// device space transform for the linear gradient brush. This
    /// field MUST be present if the BrushDataTransform flag is set in
    /// the BrushDataFlags field of the EmfPlusLinearGradientBrushData
    /// object.
    pub transform_matrix: Option<EmfPlusTransformMatrix>,
    /// The EmfPlusBlendColors alternative of the BlendPattern field,
    /// present when the BrushDataPresetColors flag is set.
    pub blend_pattern_preset_colors: Option<EmfPlusBlendColors>,
    /// The horizontal EmfPlusBlendFactors alternative of the
    /// BlendPattern field, present when the BrushDataBlendFactorsH
    /// flag is set.
    pub blend_pattern_factors_h: Option<EmfPlusBlendFactors>,
    /// The vertical EmfPlusBlendFactors alternative of the
    /// BlendPattern field, present when the BrushDataBlendFactorsV
    /// flag is set. GDI+ is not known to write this pattern; it is
    /// parsed for completeness.
    pub blend_pattern_factors_v: Option<EmfPlusBlendFactors>,
}

impl EmfPlusLinearGradientBrushData {
    /// Parses linear gradient brush data. The `available` budget is
    /// accepted for interface symmetry with the other brush kinds; the
    /// layout is fully determined by the flags.
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        _available: usize,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::{
            emf_plus::BrushDataFlags,
            records::{read_field, read_with},
        };

        let mut consumed_bytes: usize = 0;
        let brush_data_flags =
            read_with(buf, &mut consumed_bytes, BrushDataFlags::parse)?;
        let wrap_mode = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::WrapMode::parse,
        )?;
        let rect = read_with(buf, &mut consumed_bytes, EmfPlusRectF::parse)?;
        let start_color =
            read_with(buf, &mut consumed_bytes, EmfPlusARGB::parse)?;
        let end_color =
            read_with(buf, &mut consumed_bytes, EmfPlusARGB::parse)?;
        let reserved_1: u32 = read_field(buf, &mut consumed_bytes)?;
        let reserved_2: u32 = read_field(buf, &mut consumed_bytes)?;

        check_blend_pattern_flags(brush_data_flags)?;

        let transform_matrix =
            if brush_data_flags.contains(BrushDataFlags::TRANSFORM) {
                Some(read_with(
                    buf,
                    &mut consumed_bytes,
                    EmfPlusTransformMatrix::parse,
                )?)
            } else {
                None
            };

        let blend_pattern_preset_colors =
            if brush_data_flags.contains(BrushDataFlags::PRESET_COLORS) {
                Some(read_with(
                    buf,
                    &mut consumed_bytes,
                    EmfPlusBlendColors::parse,
                )?)
            } else {
                None
            };

        // When both flag bits are set, the specification lists the
        // vertical blend factors ahead of the horizontal ones, so the
        // vertical object is read first in that case. GDI+ never
        // writes the vertical pattern, so the single-pattern cases are
        // the ones observed in practice.
        let has_factors_h =
            brush_data_flags.contains(BrushDataFlags::BLEND_FACTORS_H);
        let has_factors_v =
            brush_data_flags.contains(BrushDataFlags::BLEND_FACTORS_V);

        let mut blend_pattern_factors_h = None;
        let mut blend_pattern_factors_v = None;

        if has_factors_v && has_factors_h {
            blend_pattern_factors_v = Some(read_with(
                buf,
                &mut consumed_bytes,
                EmfPlusBlendFactors::parse,
            )?);
            blend_pattern_factors_h = Some(read_with(
                buf,
                &mut consumed_bytes,
                EmfPlusBlendFactors::parse,
            )?);
        } else if has_factors_h {
            blend_pattern_factors_h = Some(read_with(
                buf,
                &mut consumed_bytes,
                EmfPlusBlendFactors::parse,
            )?);
        } else if has_factors_v {
            blend_pattern_factors_v = Some(read_with(
                buf,
                &mut consumed_bytes,
                EmfPlusBlendFactors::parse,
            )?);
        }

        Ok((
            Self {
                brush_data_flags,
                wrap_mode,
                rect,
                start_color,
                end_color,
                reserved_1,
                reserved_2,
                transform_matrix,
                blend_pattern_preset_colors,
                blend_pattern_factors_h,
                blend_pattern_factors_v,
            },
            consumed_bytes,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::BrushDataFlags;

    #[test]
    fn parses_transform_and_horizontal_blend_factors() {
        let mut data = vec![];
        data.extend(
            (BrushDataFlags::TRANSFORM | BrushDataFlags::BLEND_FACTORS_H)
                .to_le_bytes(),
        );
        data.extend(0_u32.to_le_bytes()); // WrapModeTile
        for v in [1.0_f32, 2.0, 3.0, 4.0] {
            data.extend(v.to_le_bytes()); // RectF
        }
        data.extend([0x01, 0x02, 0x03, 0xFF]); // StartColor
        data.extend([0x04, 0x05, 0x06, 0xFF]); // EndColor
        data.extend(0_u32.to_le_bytes()); // Reserved1
        data.extend(0_u32.to_le_bytes()); // Reserved2
        for v in [1.0_f32, 0.0, 0.0, 1.0, 5.0, 6.0] {
            data.extend(v.to_le_bytes()); // TransformMatrix
        }
        data.extend(2_u32.to_le_bytes()); // PositionCount
        for v in [0.0_f32, 1.0, 0.25, 0.75] {
            data.extend(v.to_le_bytes()); // positions then factors
        }

        let mut buf: &[u8] = &data;
        let (brush, consumed) =
            EmfPlusLinearGradientBrushData::parse(&mut buf, data.len())
                .unwrap();

        assert_eq!(consumed, data.len());
        assert_eq!(
            brush.transform_matrix,
            Some(EmfPlusTransformMatrix {
                m11: 1.0,
                m12: 0.0,
                m21: 0.0,
                m22: 1.0,
                dx: 5.0,
                dy: 6.0,
            }),
        );
        assert!(brush.blend_pattern_preset_colors.is_none());
        assert_eq!(
            brush.blend_pattern_factors_h,
            Some(EmfPlusBlendFactors {
                blend_positions: vec![0.0, 1.0],
                blend_factors: vec![0.25, 0.75],
            }),
        );
        assert!(brush.blend_pattern_factors_v.is_none());
    }
}
