use crate::{
    imports::*,
    parser::emf_plus::objects::{
        EmfPlusARGB, EmfPlusBlendColors, EmfPlusBlendFactors, EmfPlusPath,
        EmfPlusPointF, EmfPlusTransformMatrix,
        brush::check_blend_pattern_flags, path::skip_object_padding,
    },
};

/// The EmfPlusPathGradientBrushData object specifies a path gradient
/// for a graphics brush (MS-EMFPLUS 2.2.2.29).
///
/// Graphics brushes are specified by EmfPlusBrush objects (section
/// 2.2.1.1). A path gradient brush paints a color gradient in which
/// the color changes gradually along a gradient line from the center
/// point outward to the boundary, which are specified by either a
/// closed cardinal spline or a path in the BoundaryData field.
///
/// Gamma correction controls the overall brightness and intensity of
/// an image. Uncorrected images can look either bleached out or too
/// dark. Varying the amount of gamma correction changes not only the
/// brightness but also the ratios of red to green to blue. The need
/// for gamma correction arises because an output device might not
/// render colors in the same intensity as the input image.
///
/// The OptionalData field (an EmfPlusPathGradientBrushOptionalData
/// object, section 2.2.2.30) is inlined into this struct as the
/// `transform_matrix`, `blend_pattern_*`, and `focus_scale_data`
/// fields. Per the specification, its BlendPattern field is an
/// optional blend pattern for the path gradient brush; if present, it
/// MUST contain either an EmfPlusBlendColors object (section 2.2.2.4),
/// or an EmfPlusBlendFactors object (section 2.2.2.5), but it MUST NOT
/// contain both. The valid combinations of flags in the BrushDataFlags
/// field and the corresponding blend patterns are:
///
/// - PresetColors clear, BlendFactorsH clear: This field MUST NOT be present.
/// - PresetColors set, BlendFactorsH clear: An EmfPlusBlendColors object MUST
///   be present.
/// - PresetColors clear, BlendFactorsH set: An EmfPlusBlendFactors object MUST
///   be present.
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusPathGradientBrushData {
    /// BrushDataFlags (4 bytes): An unsigned integer that specifies
    /// the data in the OptionalData field. This value is composed of
    /// BrushData flags (section 2.1.2.1). The following flags are
    /// relevant to a path gradient brush:
    ///
    /// | Name | Value |
    /// |:-|:-|
    /// | BrushDataPath | 0x00000001 |
    /// | BrushDataTransform | 0x00000002 |
    /// | BrushDataPresetColors | 0x00000004 |
    /// | BrushDataBlendFactorsH | 0x00000008 |
    /// | BrushDataFocusScales | 0x00000040 |
    /// | BrushDataIsGammaCorrected | 0x00000080 |
    pub brush_data_flags: crate::parser::emf_plus::BrushDataFlags,
    /// WrapMode (4 bytes): A signed integer from the WrapMode
    /// enumeration (section 2.1.1.33) that specifies whether to paint
    /// the area outside the boundary of the brush. When painting
    /// outside the boundary, the wrap mode specifies how the color
    /// gradient is repeated.
    pub wrap_mode: crate::parser::emf_plus::WrapMode,
    /// CenterColor (4 bytes): An EmfPlusARGB object (section 2.2.2.1)
    /// that specifies the center color of the path gradient brush,
    /// which is the color that appears at the center point of the
    /// brush. The color of the brush changes gradually from the
    /// boundary color to the center color as it moves from the
    /// boundary to the center point.
    pub center_color: EmfPlusARGB,
    /// CenterPointF (8 bytes): An EmfPlusPointF object (section
    /// 2.2.2.36) that specifies the center point of the path gradient
    /// brush, which can be any location inside or outside the
    /// boundary. The color of the brush changes gradually from the
    /// boundary color to the center color as it moves from the
    /// boundary to the center point.
    pub center_point: EmfPlusPointF,
    /// SurroundingColor (variable): An array of SurroundingColorCount
    /// EmfPlusARGB objects that specify the colors for discrete points
    /// on the boundary of the brush.
    ///
    /// The SurroundingColorCount field (4 bytes), which specifies the
    /// number of colors specified in the SurroundingColor field, is
    /// represented by the length of the vector.
    pub surrounding_colors: Vec<EmfPlusARGB>,
    /// BoundaryData (variable): The boundary of the path gradient
    /// brush, which is specified by either a path or a closed cardinal
    /// spline. If the BrushDataPath flag is set in the BrushDataFlags
    /// field, this field MUST contain an EmfPlusBoundaryPathData
    /// object (section 2.2.2.6); otherwise, this field MUST contain an
    /// EmfPlusBoundaryPointData object (section 2.2.2.7).
    pub boundary_data: EmfPlusBoundaryData,
    /// TransformMatrix (24 bytes): An optional EmfPlusTransformMatrix
    /// object (section 2.2.2.47) that specifies a world space to
    /// device space transform for the path gradient brush. This field
    /// MUST be present if the BrushDataTransform flag is set in the
    /// BrushDataFlags field of the EmfPlusPathGradientBrushData
    /// object.
    pub transform_matrix: Option<EmfPlusTransformMatrix>,
    /// The EmfPlusBlendColors alternative of the BlendPattern field,
    /// present when the BrushDataPresetColors flag is set.
    pub blend_pattern_preset_colors: Option<EmfPlusBlendColors>,
    /// The EmfPlusBlendFactors alternative of the BlendPattern field,
    /// present when the BrushDataBlendFactorsH flag is set.
    pub blend_pattern_factors: Option<EmfPlusBlendFactors>,
    /// FocusScaleData (12 bytes): An optional EmfPlusFocusScaleData
    /// object (section 2.2.2.18) that specifies focus scales for the
    /// path gradient brush. This field MUST be present if the
    /// BrushDataFocusScales flag is set in the BrushDataFlags field of
    /// the EmfPlusPathGradientBrushData object.
    pub focus_scale_data: Option<EmfPlusFocusScaleData>,
}

/// The BoundaryData field of an EmfPlusPathGradientBrushData object.
///
/// Boundary path data and boundary point data are specified in the
/// BoundaryData field of an EmfPlusPathGradientBrushData object
/// (section 2.2.2.29).
#[derive(Clone, Debug, PartialEq)]
pub enum EmfPlusBoundaryData {
    /// The EmfPlusBoundaryPathData object specifies a graphics path
    /// boundary for a gradient brush (MS-EMFPLUS 2.2.2.6). Its
    /// BoundaryPathData field is an EmfPlusPath object (section
    /// 2.2.1.6), which specifies the boundary of the brush.
    ///
    /// The BoundaryPathSize field (4 bytes), a signed integer that
    /// specifies the size in bytes of the BoundaryPathData field, is
    /// consumed at parse time and not stored.
    Path(EmfPlusPath),
    /// The EmfPlusBoundaryPointData object specifies a closed cardinal
    /// spline boundary for a gradient brush (MS-EMFPLUS 2.2.2.7). Its
    /// BoundaryPointData field is an array of BoundaryPointCount
    /// EmfPlusPointF objects (section 2.2.2.36) that specify the
    /// boundary of the brush.
    ///
    /// The BoundaryPointCount field (4 bytes), a signed integer that
    /// specifies the number of points in the BoundaryPointData field,
    /// is represented by the length of the vector.
    Points(Vec<EmfPlusPointF>),
}

/// The EmfPlusFocusScaleData object specifies focus scales for the
/// blend pattern of a path gradient brush (MS-EMFPLUS 2.2.2.18).
///
/// By default, the center color of a path gradient brush is displayed
/// only at the center point of an area bounded by a path. Focus scales
/// specify an inner path inside that area, and the center color is
/// displayed everywhere inside it. The inner path is the boundary path
/// scaled by horizontal and vertical scale factors.
///
/// For example, focus scales of {0.2, 0.3} specifies a path that is
/// the boundary path scaled by a factor of 0.2 horizontally and 0.3
/// vertically. The area inside the scaled path MUST be filled with the
/// center color. Between the inner and outer boundaries, the color
/// MUST change gradually from the center color to the boundary color.
///
/// An EmfPlusFocusScaleData object MUST be present in the OptionalData
/// field of an EmfPlusPathGradientBrushData object (section 2.2.2.29),
/// if the BrushDataFocusScales flag (section 2.1.2.1) is set in its
/// BrushDataFlags field.
///
/// The FocusScaleCount field (4 bytes), an unsigned integer that
/// specifies the number of focus scales and MUST be 2, is validated at
/// parse time and not stored.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmfPlusFocusScaleData {
    /// FocusScaleX (4 bytes): A floating-point value that defines the
    /// horizontal focus scale. The focus scale MUST be a value between
    /// 0.0 and 1.0, exclusive.
    pub focus_scale_x: f32,
    /// FocusScaleY (4 bytes): A floating-point value that defines the
    /// vertical focus scale. The focus scale MUST be a value between
    /// 0.0 and 1.0, exclusive.
    pub focus_scale_y: f32,
}

impl EmfPlusPathGradientBrushData {
    /// Parses path gradient brush data. The `available` budget is
    /// accepted for interface symmetry with the other brush kinds; the
    /// layout is fully determined by the flags and embedded counts.
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
        let center_color =
            read_with(buf, &mut consumed_bytes, EmfPlusARGB::parse)?;
        let center_point =
            read_with(buf, &mut consumed_bytes, EmfPlusPointF::parse)?;
        let surrounding_color_count: u32 =
            read_field(buf, &mut consumed_bytes)?;

        crate::parser::emf_plus::check_element_count(
            "SurroundingColorCount",
            surrounding_color_count,
        )?;
        check_blend_pattern_flags(brush_data_flags)?;

        let mut surrounding_colors = vec![];
        for _ in 0..surrounding_color_count {
            surrounding_colors.push(read_with(
                buf,
                &mut consumed_bytes,
                EmfPlusARGB::parse,
            )?);
        }

        let boundary_data = if brush_data_flags.contains(BrushDataFlags::PATH) {
            // EmfPlusBoundaryPathData: BoundaryPathSize (4 bytes)
            // followed by that many bytes of EmfPlusPath.
            let boundary_path_size: i32 = read_field(buf, &mut consumed_bytes)?;
            let declared = u32::try_from(boundary_path_size).map_err(|_| {
                crate::parser::ParseError::UnexpectedPattern {
                    cause: alloc::format!(
                        "BoundaryPathSize is negative: {boundary_path_size}",
                    )
                    .into(),
                }
            })?;

            crate::parser::ParseError::expect_le(
                "BoundaryPathSize",
                declared,
                crate::parser::MAX_RECORD_BYTES,
            )?;

            let mut path_bytes: usize = 0;
            let path = {
                let (path, c) = EmfPlusPath::parse(buf)?;
                path_bytes += c;
                path
            };

            let consumed_by_path = path_bytes;
            skip_object_padding(
                buf,
                &mut path_bytes,
                declared as usize,
                consumed_by_path,
            )?;
            consumed_bytes += path_bytes;

            EmfPlusBoundaryData::Path(path)
        } else {
            // EmfPlusBoundaryPointData: BoundaryPointCount (4
            // bytes) followed by EmfPlusPointF objects.
            let boundary_point_count: u32 =
                read_field(buf, &mut consumed_bytes)?;

            crate::parser::emf_plus::check_element_count(
                "BoundaryPointCount",
                boundary_point_count,
            )?;

            let mut points = vec![];
            for _ in 0..boundary_point_count {
                points.push(read_with(
                    buf,
                    &mut consumed_bytes,
                    EmfPlusPointF::parse,
                )?);
            }

            EmfPlusBoundaryData::Points(points)
        };

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

        let blend_pattern_factors =
            if brush_data_flags.contains(BrushDataFlags::BLEND_FACTORS_H) {
                Some(read_with(
                    buf,
                    &mut consumed_bytes,
                    EmfPlusBlendFactors::parse,
                )?)
            } else {
                None
            };

        let focus_scale_data =
            if brush_data_flags.contains(BrushDataFlags::FOCUS_SCALES) {
                Some(read_with(
                    buf,
                    &mut consumed_bytes,
                    EmfPlusFocusScaleData::parse,
                )?)
            } else {
                None
            };

        Ok((
            Self {
                brush_data_flags,
                wrap_mode,
                center_color,
                center_point,
                surrounding_colors,
                boundary_data,
                transform_matrix,
                blend_pattern_preset_colors,
                blend_pattern_factors,
                focus_scale_data,
            },
            consumed_bytes,
        ))
    }
}

impl EmfPlusFocusScaleData {
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
        let focus_scale_count: u32 = read_field(buf, &mut consumed_bytes)?;

        // FocusScaleCount MUST be 2: one horizontal and one vertical
        // scale value.
        crate::parser::ParseError::expect_eq(
            "FocusScaleCount",
            focus_scale_count,
            2,
        )?;

        let focus_scale_x = read_field(buf, &mut consumed_bytes)?;
        let focus_scale_y = read_field(buf, &mut consumed_bytes)?;

        Ok((Self { focus_scale_x, focus_scale_y }, consumed_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::BrushDataFlags;

    #[test]
    fn parses_point_boundary_with_focus_scales() {
        let mut data = vec![];
        data.extend(BrushDataFlags::FOCUS_SCALES.to_le_bytes());
        data.extend(0_u32.to_le_bytes()); // WrapModeTile
        data.extend([0x01, 0x02, 0x03, 0xFF]); // CenterColor
        data.extend(5.0_f32.to_le_bytes()); // CenterPoint.x
        data.extend(6.0_f32.to_le_bytes()); // CenterPoint.y
        data.extend(2_u32.to_le_bytes()); // SurroundingColorCount
        data.extend([0xAA, 0x00, 0x00, 0xFF]);
        data.extend([0x00, 0xBB, 0x00, 0xFF]);
        data.extend(3_i32.to_le_bytes()); // BoundaryPointCount
        for v in [0.0_f32, 0.0, 10.0, 0.0, 5.0, 8.0] {
            data.extend(v.to_le_bytes());
        }
        data.extend(2_u32.to_le_bytes()); // FocusScaleCount
        data.extend(0.5_f32.to_le_bytes());
        data.extend(0.25_f32.to_le_bytes());

        let mut buf: &[u8] = &data;
        let (brush, consumed) =
            EmfPlusPathGradientBrushData::parse(&mut buf, data.len()).unwrap();

        assert_eq!(consumed, data.len());
        assert_eq!(brush.surrounding_colors.len(), 2);
        assert_eq!(
            brush.boundary_data,
            EmfPlusBoundaryData::Points(vec![
                EmfPlusPointF { x: 0.0, y: 0.0 },
                EmfPlusPointF { x: 10.0, y: 0.0 },
                EmfPlusPointF { x: 5.0, y: 8.0 },
            ]),
        );
        assert_eq!(
            brush.focus_scale_data,
            Some(EmfPlusFocusScaleData {
                focus_scale_x: 0.5,
                focus_scale_y: 0.25,
            }),
        );
    }

    #[test]
    fn rejects_wrong_focus_scale_count() {
        let mut data = vec![];
        data.extend(3_u32.to_le_bytes());
        data.extend(0.5_f32.to_le_bytes());
        data.extend(0.25_f32.to_le_bytes());

        let mut buf: &[u8] = &data;

        assert!(EmfPlusFocusScaleData::parse(&mut buf).is_err());
    }
}
