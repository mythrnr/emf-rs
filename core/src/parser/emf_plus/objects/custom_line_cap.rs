//! Custom line cap objects (MS-EMFPLUS 2.2.1.2 EmfPlusCustomLineCap,
//! 2.2.2.13 EmfPlusCustomLineCapData, 2.2.2.12
//! EmfPlusCustomLineCapArrowData, 2.2.2.14
//! EmfPlusCustomLineCapOptionalData, 2.2.2.17 EmfPlusFillPath,
//! 2.2.2.26 EmfPlusLinePath).

use crate::parser::emf_plus::objects::{
    EmfPlusGraphicsVersion, EmfPlusPath, EmfPlusPointF,
    path::skip_object_padding,
};

/// The EmfPlusCustomLineCap object specifies the shape to use at the
/// ends of a line drawn by a graphics pen (MS-EMFPLUS 2.2.1.2).
///
/// This object is generic and is used to specify different types of
/// custom line cap data, including:
///
/// - An EmfPlusCustomLineCapArrowData object (section 2.2.2.12); and
/// - An EmfPlusCustomLineCapData object (section 2.2.2.13).
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusCustomLineCap {
    /// Version (4 bytes): An EmfPlusGraphicsVersion object (section
    /// 2.2.2.19) that specifies the version of operating system
    /// graphics that was used to create this object.
    pub version: EmfPlusGraphicsVersion,
    /// CustomLineCapData (variable): Variable-length data that defines
    /// the custom line cap data object specified in the Type field.
    /// The content and format of the data can be different for every
    /// custom line cap type.
    ///
    /// The Type field of the wire format is implied by the variant.
    pub cap_data: EmfPlusCustomLineCapKind,
}

/// The CustomLineCapData field of an EmfPlusCustomLineCap object.
#[derive(Clone, Debug, PartialEq)]
pub enum EmfPlusCustomLineCapKind {
    /// A default custom cap (MS-EMFPLUS 2.2.2.13).
    Default(EmfPlusCustomLineCapData),
    /// An adjustable arrow cap (MS-EMFPLUS 2.2.2.12).
    AdjustableArrow(EmfPlusCustomLineCapArrowData),
}

impl EmfPlusCustomLineCap {
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
        let version =
            read_with(buf, &mut consumed_bytes, EmfPlusGraphicsVersion::parse)?;
        let cap_type = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::CustomLineCapDataType::parse,
        )?;

        use crate::parser::emf_plus::CustomLineCapDataType;

        let cap_data = match cap_type {
            CustomLineCapDataType::CustomLineCapDataTypeDefault => {
                EmfPlusCustomLineCapKind::Default(read_with(
                    buf,
                    &mut consumed_bytes,
                    EmfPlusCustomLineCapData::parse,
                )?)
            }
            CustomLineCapDataType::CustomLineCapDataTypeAdjustableArrow => {
                EmfPlusCustomLineCapKind::AdjustableArrow(read_with(
                    buf,
                    &mut consumed_bytes,
                    EmfPlusCustomLineCapArrowData::parse,
                )?)
            }
        };

        Ok((Self { version, cap_data }, consumed_bytes))
    }
}

/// The EmfPlusCustomLineCapData object specifies default data for a
/// custom line cap (MS-EMFPLUS 2.2.2.13), with its optional data
/// (EmfPlusCustomLineCapOptionalData, 2.2.2.14) inlined.
///
/// Custom line caps are specified by EmfPlusCustomLineCap objects
/// (section 2.2.1.2).
///
/// The EmfPlusCustomLineCapOptionalData object specifies optional fill
/// and outline data for a custom line cap. Note: Each field specified
/// for this object is optional and might not be present in the
/// OptionalData field of an EmfPlusCustomLineCapData object (section
/// 2.2.2.13), depending on the CustomLineCapData flags (section
/// 2.1.2.2) set in its CustomLineCapDataFlags field. Although it is
/// not practical to represent every possible combination of fields
/// present or absent, this section specifies their relative order in
/// the object. The implementer is responsible for determining which
/// fields are actually present in a given metafile record, and for
/// unmarshaling the data for individual fields separately and
/// appropriately.
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusCustomLineCapData {
    /// CustomLineCapDataFlags (4 bytes): An unsigned integer that
    /// specifies the data in the OptionalData field. This value is
    /// composed of CustomLineCapData flags (section 2.1.2.2).
    pub custom_line_cap_data_flags:
        crate::parser::emf_plus::CustomLineCapDataFlags,
    /// BaseCap (4 bytes): An unsigned integer that specifies the value
    /// from the LineCapType enumeration (section 2.1.1.17) on which
    /// the custom line cap is based.
    pub base_cap: crate::parser::emf_plus::LineCapType,
    /// BaseInset (4 bytes): A floating-point value that specifies the
    /// distance between the beginning of the line cap and the end of
    /// the line.
    pub base_inset: f32,
    /// StrokeStartCap (4 bytes): An unsigned integer that specifies
    /// the value in the LineCapType enumeration that indicates the
    /// line cap used at the start of the line to be drawn.
    pub stroke_start_cap: crate::parser::emf_plus::LineCapType,
    /// StrokeEndCap (4 bytes): An unsigned integer that specifies the
    /// value in the LineCapType enumeration that indicates what line
    /// cap is to be used at the end of the line to be drawn.
    pub stroke_end_cap: crate::parser::emf_plus::LineCapType,
    /// StrokeJoin (4 bytes): An unsigned integer that specifies the
    /// value in the LineJoinType enumeration (section 2.1.1.18) that
    /// specifies how to join two lines that are drawn by the same pen
    /// and whose ends meet. At the intersection of the two line ends,
    /// a line join makes the connection look more continuous.
    pub stroke_join: crate::parser::emf_plus::LineJoinType,
    /// StrokeMiterLimit (4 bytes): A floating-point value that
    /// contains the limit of the thickness of the join on a mitered
    /// corner by setting the maximum allowed ratio of miter length to
    /// line width.
    pub stroke_miter_limit: f32,
    /// WidthScale (4 bytes): A floating-point value that specifies the
    /// amount by which to scale the custom line cap with respect to
    /// the width of the EmfPlusPen object (section 2.2.1.7) that is
    /// used to draw the lines.
    pub width_scale: f32,
    /// FillHotSpot (8 bytes): An EmfPlusPointF object (section
    /// 2.2.2.36) that is not currently used. It MUST be set to
    /// {0.0, 0.0}.
    pub fill_hot_spot: EmfPlusPointF,
    /// StrokeHotSpot (8 bytes): An EmfPlusPointF object that is not
    /// currently used. It MUST be set to {0.0, 0.0}.
    pub stroke_hot_spot: EmfPlusPointF,
    /// FillData (variable): An optional EmfPlusFillPath object
    /// (section 2.2.2.17) that specifies the path for filling a custom
    /// graphics line cap. This field MUST be present if the
    /// CustomLineCapDataFillPath flag is set in the
    /// CustomLineCapDataFlags field of the EmfPlusCustomLineCapData
    /// object.
    ///
    /// The length-prefixed EmfPlusFillPath envelope is unwrapped at
    /// parse time; only the inner EmfPlusPath is kept.
    pub fill_path: Option<EmfPlusPath>,
    /// OutlineData (variable): An optional EmfPlusLinePath object
    /// (section 2.2.2.26) that specifies the path for outlining a
    /// custom graphics line cap. This field MUST be present if the
    /// CustomLineCapDataLinePath flag is set in the
    /// CustomLineCapDataFlags field of the EmfPlusCustomLineCapData
    /// object.
    ///
    /// The length-prefixed EmfPlusLinePath envelope is unwrapped at
    /// parse time; only the inner EmfPlusPath is kept.
    pub line_path: Option<EmfPlusPath>,
}

impl EmfPlusCustomLineCapData {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::{
            emf_plus::CustomLineCapDataFlags,
            records::{read_field, read_with},
        };

        let mut consumed_bytes: usize = 0;
        let custom_line_cap_data_flags =
            read_with(buf, &mut consumed_bytes, CustomLineCapDataFlags::parse)?;
        let base_cap = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::LineCapType::parse,
        )?;
        let base_inset = read_field(buf, &mut consumed_bytes)?;
        let stroke_start_cap = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::LineCapType::parse,
        )?;
        let stroke_end_cap = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::LineCapType::parse,
        )?;
        let stroke_join = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::LineJoinType::parse,
        )?;
        let stroke_miter_limit = read_field(buf, &mut consumed_bytes)?;
        let width_scale = read_field(buf, &mut consumed_bytes)?;
        let fill_hot_spot =
            read_with(buf, &mut consumed_bytes, EmfPlusPointF::parse)?;
        let stroke_hot_spot =
            read_with(buf, &mut consumed_bytes, EmfPlusPointF::parse)?;

        // EmfPlusFillPath / EmfPlusLinePath both wrap an EmfPlusPath in
        // a length-prefixed envelope.
        let fill_path = if custom_line_cap_data_flags
            .contains(CustomLineCapDataFlags::FILL_PATH)
        {
            Some(parse_length_prefixed_path(buf, &mut consumed_bytes)?)
        } else {
            None
        };
        let line_path = if custom_line_cap_data_flags
            .contains(CustomLineCapDataFlags::LINE_PATH)
        {
            Some(parse_length_prefixed_path(buf, &mut consumed_bytes)?)
        } else {
            None
        };

        Ok((
            Self {
                custom_line_cap_data_flags,
                base_cap,
                base_inset,
                stroke_start_cap,
                stroke_end_cap,
                stroke_join,
                stroke_miter_limit,
                width_scale,
                fill_hot_spot,
                stroke_hot_spot,
                fill_path,
                line_path,
            },
            consumed_bytes,
        ))
    }
}

/// The EmfPlusCustomLineCapArrowData object specifies adjustable arrow
/// data for a custom line cap (MS-EMFPLUS 2.2.2.12).
///
/// Custom line caps are specified by EmfPlusCustomLineCap objects
/// (section 2.2.1.2).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmfPlusCustomLineCapArrowData {
    /// Width (4 bytes): A floating-point value that specifies the
    /// width of the arrow cap.
    ///
    /// The width of the arrow cap is scaled by the width of the
    /// EmfPlusPen object (section 2.2.1.7) that is used to draw the
    /// line being capped. For example, when drawing a capped line with
    /// a pen that has a width of 5 pixels, and the adjustable arrow
    /// cap object has a width of 3, the actual arrow cap is drawn 15
    /// pixels wide.
    pub width: f32,
    /// Height (4 bytes): A floating-point value that specifies the
    /// height of the arrow cap.
    ///
    /// The height of the arrow cap is scaled by the width of the
    /// EmfPlusPen object that is used to draw the line being capped.
    /// For example, when drawing a capped line with a pen that has a
    /// width of 5 pixels, and the adjustable arrow cap object has a
    /// height of 3, the actual arrow cap is drawn 15 pixels high.
    pub height: f32,
    /// MiddleInset (4 bytes): A floating-point value that specifies
    /// the number of pixels between the outline of the arrow cap and
    /// the fill of the arrow cap.
    pub middle_inset: f32,
    /// FillState (4 bytes): A Boolean value that specifies whether the
    /// arrow cap is filled. If the arrow cap is not filled, only the
    /// outline is drawn.
    pub fill_state: bool,
    /// LineStartCap (4 bytes): An unsigned integer that specifies the
    /// value in the LineCapType enumeration (section 2.1.1.17) that
    /// indicates the line cap to be used at the start of the line to
    /// be drawn.
    pub line_start_cap: crate::parser::emf_plus::LineCapType,
    /// LineEndCap (4 bytes): An unsigned integer that specifies the
    /// value in the LineCapType enumeration that indicates the line
    /// cap to be used at the end of the line to be drawn.
    pub line_end_cap: crate::parser::emf_plus::LineCapType,
    /// LineJoin (4 bytes): An unsigned integer that specifies the
    /// value in the LineJoinType enumeration (section 2.1.1.18) that
    /// specifies how to join two lines that are drawn by the same pen
    /// and whose ends meet. At the intersection of the two line ends,
    /// a line join makes the connection look more continuous.
    pub line_join: crate::parser::emf_plus::LineJoinType,
    /// LineMiterLimit (4 bytes): A floating-point value that specifies
    /// the limit of the thickness of the join on a mitered corner by
    /// setting the maximum allowed ratio of miter length to line
    /// width.
    pub line_miter_limit: f32,
    /// WidthScale (4 bytes): A floating-point value that specifies the
    /// amount by which to scale an EmfPlusCustomLineCap object
    /// (section 2.2.1.2) with respect to the width of the graphics pen
    /// that is used to draw the lines.
    pub width_scale: f32,
    /// FillHotSpot (8 bytes): An EmfPlusPointF object (section
    /// 2.2.2.36) that is not currently used. It MUST be set to
    /// {0.0, 0.0}.
    pub fill_hot_spot: EmfPlusPointF,
    /// LineHotSpot (8 bytes): An EmfPlusPointF object that is not
    /// currently used. It MUST be set to {0.0, 0.0}.
    pub line_hot_spot: EmfPlusPointF,
}

impl EmfPlusCustomLineCapArrowData {
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
        let width = read_field(buf, &mut consumed_bytes)?;
        let height = read_field(buf, &mut consumed_bytes)?;
        let middle_inset = read_field(buf, &mut consumed_bytes)?;
        let fill_state: u32 = read_field(buf, &mut consumed_bytes)?;
        let line_start_cap = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::LineCapType::parse,
        )?;
        let line_end_cap = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::LineCapType::parse,
        )?;
        let line_join = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::LineJoinType::parse,
        )?;
        let line_miter_limit = read_field(buf, &mut consumed_bytes)?;
        let width_scale = read_field(buf, &mut consumed_bytes)?;
        let fill_hot_spot =
            read_with(buf, &mut consumed_bytes, EmfPlusPointF::parse)?;
        let line_hot_spot =
            read_with(buf, &mut consumed_bytes, EmfPlusPointF::parse)?;

        Ok((
            Self {
                width,
                height,
                middle_inset,
                fill_state: fill_state != 0,
                line_start_cap,
                line_end_cap,
                line_join,
                line_miter_limit,
                width_scale,
                fill_hot_spot,
                line_hot_spot,
            },
            consumed_bytes,
        ))
    }
}

/// Reads a length-prefixed EmfPlusPath envelope (EmfPlusFillPath /
/// EmfPlusLinePath / EmfPlusRegionNodePath share this layout) and
/// skips the alignment padding the length prefix covers.
pub(in crate::parser::emf_plus) fn parse_length_prefixed_path<
    R: crate::Read,
>(
    buf: &mut R,
    tracker: &mut impl crate::parser::ConsumeTracker,
) -> Result<EmfPlusPath, crate::parser::ParseError> {
    use crate::parser::records::read_field;

    let mut consumed: usize = 0;
    let path_length: i32 = read_field(buf, &mut consumed)?;
    let declared = u32::try_from(path_length).map_err(|_| {
        crate::parser::ParseError::UnexpectedPattern {
            cause: alloc::format!(
                "nested path length is negative: {path_length}",
            )
            .into(),
        }
    })?;

    crate::parser::ParseError::expect_le(
        "nested path length",
        declared,
        crate::parser::MAX_RECORD_BYTES,
    )?;

    let mut path_bytes: usize = 0;
    let (path, c) = EmfPlusPath::parse(buf)?;
    path_bytes += c;

    let consumed_by_path = path_bytes;
    skip_object_padding(
        buf,
        &mut path_bytes,
        declared as usize,
        consumed_by_path,
    )?;

    tracker.track(consumed + path_bytes);

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::LineCapType;

    #[test]
    fn parses_adjustable_arrow_cap() {
        let mut data = vec![];
        data.extend(0xDBC0_1002_u32.to_le_bytes()); // Version
        data.extend(1_u32.to_le_bytes()); // AdjustableArrow
        data.extend(4.0_f32.to_le_bytes()); // Width
        data.extend(6.0_f32.to_le_bytes()); // Height
        data.extend(1.0_f32.to_le_bytes()); // MiddleInset
        data.extend(1_u32.to_le_bytes()); // FillState
        data.extend(0_u32.to_le_bytes()); // LineStartCap: Flat
        data.extend(0_u32.to_le_bytes()); // LineEndCap: Flat
        data.extend(0_u32.to_le_bytes()); // LineJoin: Miter
        data.extend(10.0_f32.to_le_bytes()); // LineMiterLimit
        data.extend(1.0_f32.to_le_bytes()); // WidthScale
        for v in [0.0_f32, 0.0, 0.0, 0.0] {
            data.extend(v.to_le_bytes()); // hot spots
        }

        let mut buf: &[u8] = &data;
        let (cap, consumed) = EmfPlusCustomLineCap::parse(&mut buf).unwrap();

        assert_eq!(consumed, data.len());

        let EmfPlusCustomLineCapKind::AdjustableArrow(arrow) = cap.cap_data
        else {
            panic!("expected an adjustable arrow cap");
        };
        assert!(arrow.fill_state);
        assert_eq!(arrow.line_start_cap, LineCapType::LineCapTypeFlat);
    }

    #[test]
    fn parses_default_cap_with_nested_fill_path() {
        // A minimal nested path: 2 float points forming a line.
        let mut path = vec![];
        path.extend(0xDBC0_1002_u32.to_le_bytes());
        path.extend(2_u32.to_le_bytes());
        path.extend(0_u32.to_le_bytes());
        for v in [0.0_f32, 0.0, 4.0, 0.0] {
            path.extend(v.to_le_bytes());
        }
        path.extend([0x00, 0x01]); // point types
        path.extend([0x00, 0x00]); // alignment padding

        let mut data = vec![];
        data.extend(0xDBC0_1002_u32.to_le_bytes()); // Version
        data.extend(0_u32.to_le_bytes()); // Default
        data.extend(
            crate::parser::emf_plus::CustomLineCapDataFlags::FILL_PATH
                .to_le_bytes(),
        );
        data.extend(0_u32.to_le_bytes()); // BaseCap: Flat
        data.extend(0.0_f32.to_le_bytes()); // BaseInset
        data.extend(0_u32.to_le_bytes()); // StrokeStartCap
        data.extend(0_u32.to_le_bytes()); // StrokeEndCap
        data.extend(0_u32.to_le_bytes()); // StrokeJoin
        data.extend(10.0_f32.to_le_bytes()); // StrokeMiterLimit
        data.extend(1.0_f32.to_le_bytes()); // WidthScale
        for v in [0.0_f32, 0.0, 0.0, 0.0] {
            data.extend(v.to_le_bytes()); // hot spots
        }
        data.extend(
            i32::try_from(path.len()).unwrap().to_le_bytes(), // FillPathLength
        );
        data.extend(&path);

        let mut buf: &[u8] = &data;
        let (cap, consumed) = EmfPlusCustomLineCap::parse(&mut buf).unwrap();

        assert_eq!(consumed, data.len());

        let EmfPlusCustomLineCapKind::Default(cap_data) = cap.cap_data else {
            panic!("expected a default cap");
        };
        let Some(fill_path) = cap_data.fill_path else {
            panic!("expected a fill path");
        };
        assert_eq!(fill_path.path_points.len(), 2);
        assert!(cap_data.line_path.is_none());
    }
}
