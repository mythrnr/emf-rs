//! Pen objects (MS-EMFPLUS 2.2.1.7 EmfPlusPen, 2.2.2.33
//! EmfPlusPenData, 2.2.2.34 EmfPlusPenOptionalData, 2.2.2.16
//! EmfPlusDashedLineData, 2.2.2.9 EmfPlusCompoundLineData, 2.2.2.15
//! EmfPlusCustomStartCapData, 2.2.2.11 EmfPlusCustomEndCapData).

use crate::{
    imports::*,
    parser::emf_plus::objects::{
        EmfPlusBrush, EmfPlusCustomLineCap, EmfPlusGraphicsVersion,
        EmfPlusTransformMatrix, path::skip_object_padding,
    },
};

/// The EmfPlusPen object specifies a graphics pen for the drawing of
/// lines (MS-EMFPLUS 2.2.1.7).
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusPen {
    /// Version (4 bytes): An EmfPlusGraphicsVersion object (section
    /// 2.2.2.19) that specifies the version of operating system
    /// graphics that was used to create this object.
    pub version: EmfPlusGraphicsVersion,
    /// PenData (variable): An EmfPlusPenData object (section 2.2.2.33)
    /// that specifies properties of the graphics pen.
    pub pen_data: EmfPlusPenData,
    /// BrushObject (variable): An EmfPlusBrush object (section
    /// 2.2.1.1) that specifies a graphics brush associated with the
    /// pen.
    pub brush_object: EmfPlusBrush,
}

impl EmfPlusPen {
    /// Parses a pen from at most `available` bytes of object data.
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        available: usize,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let version =
            read_with(buf, &mut consumed_bytes, EmfPlusGraphicsVersion::parse)?;
        let pen_type: u32 = read_field(buf, &mut consumed_bytes)?;

        // Type (4 bytes): MUST be zero.
        crate::parser::ParseError::expect_eq("pen Type", pen_type, 0)?;

        let pen_data =
            read_with(buf, &mut consumed_bytes, EmfPlusPenData::parse)?;

        let remaining = available.saturating_sub(consumed_bytes);
        let brush_object = {
            let (brush, c) = EmfPlusBrush::parse(buf, remaining)?;
            consumed_bytes += c;
            brush
        };

        Ok((Self { version, pen_data, brush_object }, consumed_bytes))
    }
}

/// The EmfPlusPenData object specifies properties of a graphics pen
/// (MS-EMFPLUS 2.2.2.33), with its optional data
/// (EmfPlusPenOptionalData, 2.2.2.34) inlined. Optional fields appear
/// on the wire in the order of the PenDataFlags bit values.
///
/// Graphics pens are specified by EmfPlusPen objects (section
/// 2.2.1.7).
///
/// The EmfPlusPenOptionalData object specifies optional data for a
/// graphics pen. Note: Each field of this object is optional and might
/// not be present in the OptionalData field of an EmfPlusPenData
/// object (section 2.2.2.33), depending on the PenData flags (section
/// 2.1.2.7) set in its PenDataFlags field. Although it is not
/// practical to represent every possible combination of fields present
/// or absent, this section specifies their relative order in the
/// object. The implementer is responsible for determining which fields
/// are actually present in a given metafile record, and for
/// unmarshaling the data for individual fields separately and
/// appropriately.
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusPenData {
    /// PenDataFlags (4 bytes): An unsigned integer that specifies the
    /// data in the OptionalData field. This value is composed of
    /// PenData flags (section 2.1.2.7).
    pub pen_data_flags: crate::parser::emf_plus::PenDataFlags,
    /// PenUnit (4 bytes): An unsigned integer that specifies the
    /// measuring units for the pen. The value is from the UnitType
    /// enumeration (section 2.1.1.32).
    pub pen_unit: crate::parser::emf_plus::UnitType,
    /// PenWidth (4 bytes): A floating-point value that specifies the
    /// width of the line drawn by the pen in the units specified by
    /// the PenUnit field. If a zero width is specified, a minimum
    /// value is used, which is determined by the units.
    pub pen_width: f32,
    /// TransformMatrix (24 bytes): An optional EmfPlusTransformMatrix
    /// object (section 2.2.2.47) that specifies a world space to
    /// device space transform for the pen. This field MUST be present
    /// if the PenDataTransform flag is set in the PenDataFlags field
    /// of the EmfPlusPenData object.
    pub transform_matrix: Option<EmfPlusTransformMatrix>,
    /// StartCap (4 bytes): An optional signed integer that specifies
    /// the shape for the start of a line in the CustomStartCapData
    /// field. This field MUST be present if the PenDataStartCap flag
    /// is set in the PenDataFlags field of the EmfPlusPenData object,
    /// and the value is defined in the LineCapType enumeration
    /// (section 2.1.1.17).
    pub start_cap: Option<crate::parser::emf_plus::LineCapType>,
    /// EndCap (4 bytes): An optional signed integer that specifies the
    /// shape for the end of a line in the CustomEndCapData field. This
    /// field MUST be present if the PenDataEndCap flag is set in the
    /// PenDataFlags field of the EmfPlusPenData object, and the value
    /// is defined in the LineCapType enumeration.
    pub end_cap: Option<crate::parser::emf_plus::LineCapType>,
    /// Join (4 bytes): An optional signed integer that specifies how
    /// to join two lines that are drawn by the same pen and whose ends
    /// meet. This field MUST be present if the PenDataJoin flag is set
    /// in the PenDataFlags field of the EmfPlusPenData object, and the
    /// value is defined in the LineJoinType enumeration (section
    /// 2.1.1.18).
    pub join: Option<crate::parser::emf_plus::LineJoinType>,
    /// MiterLimit (4 bytes): An optional floating-point value that
    /// specifies the miter limit, which is the maximum allowed ratio
    /// of miter length to line width. The miter length is the distance
    /// from the intersection of the line walls on the inside the join
    /// to the intersection of the line walls outside the join. The
    /// miter length can be large when the angle between two lines is
    /// small. This field MUST be present if the PenDataMiterLimit flag
    /// is set in the PenDataFlags field of the EmfPlusPenData object.
    pub miter_limit: Option<f32>,
    /// LineStyle (4 bytes): An optional signed integer that specifies
    /// the style used for lines drawn with this pen object. This field
    /// MUST be present if the PenDataLineStyle flag is set in the
    /// PenDataFlags field of the EmfPlusPenData object, and the value
    /// is defined in the LineStyle enumeration (section 2.1.1.19).
    pub line_style: Option<crate::parser::emf_plus::LineStyle>,
    /// DashedLineCapType (4 bytes): An optional signed integer that
    /// specifies the shape for both ends of each dash in a dashed
    /// line. This field MUST be present if the PenDataDashedLineCap
    /// flag is set in the PenDataFlags field of the EmfPlusPenData
    /// object, and the value is defined in the DashedLineCapType
    /// enumeration (section 2.1.1.10).
    pub dashed_line_cap_type:
        Option<crate::parser::emf_plus::DashedLineCapType>,
    /// DashOffset (4 bytes): An optional floating-point value that
    /// specifies the distance from the start of a line to the start of
    /// the first space in a dashed line pattern. This field MUST be
    /// present if the PenDataDashedLineOffset flag is set in the
    /// PenDataFlags field of the EmfPlusPenData object.
    pub dash_offset: Option<f32>,
    /// DashedLineData (variable): An optional EmfPlusDashedLineData
    /// object (section 2.2.2.16) that specifies the lengths of dashes
    /// and spaces in a custom dashed line. This field MUST be present
    /// if the PenDataDashedLine flag is set in the PenDataFlags field
    /// of the EmfPlusPenData object.
    ///
    /// The DashedLineDataSize prefix of the EmfPlusDashedLineData
    /// envelope is consumed at parse time; only the array of dash and
    /// space lengths is kept.
    pub dashed_line_data: Option<Vec<f32>>,
    /// PenAlignment (4 bytes): An optional signed integer that
    /// specifies the distribution of the pen width with respect to the
    /// coordinates of the line being drawn. This field MUST be present
    /// if the PenDataNonCenter flag is set in the PenDataFlags field
    /// of the EmfPlusPenData object, and the value is defined in the
    /// PenAlignment enumeration (section 2.1.1.23).
    ///
    /// For example, consider the placement of a line. If the starting
    /// and ending coordinates of the line are defined, it is possible
    /// to think of a theoretical line between the two points that is
    /// zero width. Center alignment means that the pen width is
    /// distributed as evenly as possible on either side of that
    /// theoretical line.
    pub pen_alignment: Option<crate::parser::emf_plus::PenAlignment>,
    /// CompoundLineData (variable): An optional
    /// EmfPlusCompoundLineData object (section 2.2.2.9) that specifies
    /// an array of 32-bit floating-point values that define the
    /// compound line of a pen, which is made up of parallel lines and
    /// spaces. This field MUST be present if the PenDataCompoundLine
    /// flag is set in the PenDataFlags field of the EmfPlusPenData
    /// object.
    ///
    /// The CompoundLineDataSize prefix of the EmfPlusCompoundLineData
    /// envelope is consumed at parse time; only the array of positions
    /// in the range [0.0, 1.0] is kept.
    pub compound_line_data: Option<Vec<f32>>,
    /// CustomStartCapData (variable): An optional
    /// EmfPlusCustomStartCapData object (section 2.2.2.15) that
    /// defines the custom start-cap shape, which is the shape to use
    /// at the start of a line drawn with this pen. It can be any of
    /// various shapes, such as a square, circle, or diamond. This
    /// field MUST be present if the PenDataCustomStartCap flag is set
    /// in the PenDataFlags field of the EmfPlusPenData object.
    ///
    /// The CustomStartCapSize prefix of the envelope is consumed at
    /// parse time; only the inner EmfPlusCustomLineCap is kept.
    pub custom_start_cap: Option<EmfPlusCustomLineCap>,
    /// CustomEndCapData (variable): An optional EmfPlusCustomEndCapData
    /// object (section 2.2.2.11) that defines the custom end-cap
    /// shape, which is the shape to use at the end of a line drawn
    /// with this pen. It can be any of various shapes, such as a
    /// square, circle, or diamond. This field MUST be present if the
    /// PenDataCustomEndCap flag is set in the PenDataFlags field of
    /// the EmfPlusPenData object.
    ///
    /// The CustomEndCapSize prefix of the envelope is consumed at
    /// parse time; only the inner EmfPlusCustomLineCap is kept.
    pub custom_end_cap: Option<EmfPlusCustomLineCap>,
}

impl EmfPlusPenData {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::{
            emf_plus::PenDataFlags,
            records::{read_field, read_with},
        };

        let mut consumed_bytes: usize = 0;
        let pen_data_flags =
            read_with(buf, &mut consumed_bytes, PenDataFlags::parse)?;
        let pen_unit = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::UnitType::parse,
        )?;
        let pen_width = read_field(buf, &mut consumed_bytes)?;

        let transform_matrix =
            if pen_data_flags.contains(PenDataFlags::TRANSFORM) {
                Some(read_with(
                    buf,
                    &mut consumed_bytes,
                    EmfPlusTransformMatrix::parse,
                )?)
            } else {
                None
            };

        let start_cap = if pen_data_flags.contains(PenDataFlags::START_CAP) {
            Some(read_with(
                buf,
                &mut consumed_bytes,
                crate::parser::emf_plus::LineCapType::parse,
            )?)
        } else {
            None
        };

        let end_cap = if pen_data_flags.contains(PenDataFlags::END_CAP) {
            Some(read_with(
                buf,
                &mut consumed_bytes,
                crate::parser::emf_plus::LineCapType::parse,
            )?)
        } else {
            None
        };

        let join = if pen_data_flags.contains(PenDataFlags::JOIN) {
            Some(read_with(
                buf,
                &mut consumed_bytes,
                crate::parser::emf_plus::LineJoinType::parse,
            )?)
        } else {
            None
        };

        let miter_limit = if pen_data_flags.contains(PenDataFlags::MITER_LIMIT)
        {
            Some(read_field(buf, &mut consumed_bytes)?)
        } else {
            None
        };

        let line_style = if pen_data_flags.contains(PenDataFlags::LINE_STYLE) {
            Some(read_with(
                buf,
                &mut consumed_bytes,
                crate::parser::emf_plus::LineStyle::parse,
            )?)
        } else {
            None
        };

        let dashed_line_cap_type =
            if pen_data_flags.contains(PenDataFlags::DASHED_LINE_CAP) {
                Some(read_with(
                    buf,
                    &mut consumed_bytes,
                    crate::parser::emf_plus::DashedLineCapType::parse,
                )?)
            } else {
                None
            };

        let dash_offset =
            if pen_data_flags.contains(PenDataFlags::DASHED_LINE_OFFSET) {
                Some(read_field(buf, &mut consumed_bytes)?)
            } else {
                None
            };

        let dashed_line_data =
            if pen_data_flags.contains(PenDataFlags::DASHED_LINE) {
                Some(parse_float_array(
                    buf,
                    &mut consumed_bytes,
                    "DashedLineDataSize",
                )?)
            } else {
                None
            };

        let pen_alignment = if pen_data_flags.contains(PenDataFlags::NON_CENTER)
        {
            Some(read_with(
                buf,
                &mut consumed_bytes,
                crate::parser::emf_plus::PenAlignment::parse,
            )?)
        } else {
            None
        };

        let compound_line_data =
            if pen_data_flags.contains(PenDataFlags::COMPOUND_LINE) {
                Some(parse_float_array(
                    buf,
                    &mut consumed_bytes,
                    "CompoundLineDataSize",
                )?)
            } else {
                None
            };

        let custom_start_cap =
            if pen_data_flags.contains(PenDataFlags::CUSTOM_START_CAP) {
                Some(parse_length_prefixed_cap(buf, &mut consumed_bytes)?)
            } else {
                None
            };

        let custom_end_cap =
            if pen_data_flags.contains(PenDataFlags::CUSTOM_END_CAP) {
                Some(parse_length_prefixed_cap(buf, &mut consumed_bytes)?)
            } else {
                None
            };

        Ok((
            Self {
                pen_data_flags,
                pen_unit,
                pen_width,
                transform_matrix,
                start_cap,
                end_cap,
                join,
                miter_limit,
                line_style,
                dashed_line_cap_type,
                dash_offset,
                dashed_line_data,
                pen_alignment,
                compound_line_data,
                custom_start_cap,
                custom_end_cap,
            },
            consumed_bytes,
        ))
    }
}

/// Reads a count-prefixed float array (EmfPlusDashedLineData /
/// EmfPlusCompoundLineData share this layout).
fn parse_float_array<R: crate::Read>(
    buf: &mut R,
    tracker: &mut impl crate::parser::ConsumeTracker,
    count_field: &'static str,
) -> Result<Vec<f32>, crate::parser::ParseError> {
    use crate::parser::records::read_field;

    let mut consumed: usize = 0;
    let count: u32 = read_field(buf, &mut consumed)?;

    crate::parser::emf_plus::check_element_count(count_field, count)?;

    let mut values = vec![];
    for _ in 0..count {
        values.push(read_field(buf, &mut consumed)?);
    }

    tracker.track(consumed);

    Ok(values)
}

/// Reads a size-prefixed EmfPlusCustomLineCap envelope
/// (EmfPlusCustomStartCapData / EmfPlusCustomEndCapData share this
/// layout).
fn parse_length_prefixed_cap<R: crate::Read>(
    buf: &mut R,
    tracker: &mut impl crate::parser::ConsumeTracker,
) -> Result<EmfPlusCustomLineCap, crate::parser::ParseError> {
    use crate::parser::records::read_field;

    let mut consumed: usize = 0;
    let declared_size: u32 = read_field(buf, &mut consumed)?;

    crate::parser::ParseError::expect_le(
        "custom cap size",
        declared_size,
        crate::parser::MAX_RECORD_BYTES,
    )?;

    let mut cap_bytes: usize = 0;
    let (cap, c) = EmfPlusCustomLineCap::parse(buf)?;
    cap_bytes += c;

    let consumed_by_cap = cap_bytes;
    skip_object_padding(
        buf,
        &mut cap_bytes,
        declared_size as usize,
        consumed_by_cap,
    )?;

    tracker.track(consumed + cap_bytes);

    Ok(cap)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::{
        LineCapType, LineJoinType, PenDataFlags, UnitType,
        objects::{EmfPlusBrushData, EmfPlusSolidBrushData},
    };

    #[test]
    fn parses_pen_with_optional_fields_in_flag_order() {
        let mut data = vec![];
        data.extend(0xDBC0_1002_u32.to_le_bytes()); // Version
        data.extend(0_u32.to_le_bytes()); // Type (MUST be 0)
        data.extend(
            (PenDataFlags::START_CAP
                | PenDataFlags::END_CAP
                | PenDataFlags::JOIN
                | PenDataFlags::DASHED_LINE)
                .to_le_bytes(),
        );
        data.extend(0_u32.to_le_bytes()); // UnitTypeWorld
        data.extend(1.5_f32.to_le_bytes()); // PenWidth
        data.extend(2_u32.to_le_bytes()); // StartCap: Round
        data.extend(1_u32.to_le_bytes()); // EndCap: Square
        data.extend(2_u32.to_le_bytes()); // Join: Round
        data.extend(2_u32.to_le_bytes()); // DashedLineDataSize
        data.extend(3.0_f32.to_le_bytes());
        data.extend(1.0_f32.to_le_bytes());
        // Trailing brush: solid black.
        data.extend(0xDBC0_1002_u32.to_le_bytes());
        data.extend(0_u32.to_le_bytes());
        data.extend([0x00, 0x00, 0x00, 0xFF]);

        let mut buf: &[u8] = &data;
        let (pen, consumed) = EmfPlusPen::parse(&mut buf, data.len()).unwrap();

        assert_eq!(consumed, data.len());
        assert_eq!(pen.pen_data.pen_unit, UnitType::UnitTypeWorld);

        let start_cap = pen.pen_data.start_cap;
        let end_cap = pen.pen_data.end_cap;
        assert_eq!(start_cap, Some(LineCapType::LineCapTypeRound));
        assert_eq!(end_cap, Some(LineCapType::LineCapTypeSquare));
        assert_eq!(pen.pen_data.join, Some(LineJoinType::LineJoinTypeRound));
        assert_eq!(pen.pen_data.dashed_line_data, Some(vec![3.0, 1.0]));
        assert!(pen.pen_data.transform_matrix.is_none());
        assert!(pen.pen_data.custom_start_cap.is_none());
        assert_eq!(
            pen.brush_object.brush_data,
            EmfPlusBrushData::SolidColor(EmfPlusSolidBrushData {
                solid_color: crate::parser::emf_plus::objects::EmfPlusARGB {
                    blue: 0x00,
                    green: 0x00,
                    red: 0x00,
                    alpha: 0xFF,
                },
            }),
        );
    }

    #[test]
    fn rejects_nonzero_pen_type() {
        let mut data = vec![];
        data.extend(0xDBC0_1002_u32.to_le_bytes());
        data.extend(1_u32.to_le_bytes());

        let mut buf: &[u8] = &data;

        assert!(EmfPlusPen::parse(&mut buf, data.len()).is_err());
    }
}
