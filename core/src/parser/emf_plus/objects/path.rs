use crate::parser::emf_plus::objects::{EmfPlusPathPointTypes, EmfPlusPoints};

/// The EmfPlusPath object specifies a series of line and curve segments
/// that form a graphics path (MS-EMFPLUS 2.2.1.6). The order for Bezier
/// data points is the start point, control point 1, control point 2,
/// and end point. For more information see [MSDN-DrawBeziers].
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusPath {
    /// Version (4 bytes): An EmfPlusGraphicsVersion object (section
    /// 2.2.2.19) that specifies the version of operating system
    /// graphics that was used to create this object.
    pub version: crate::parser::emf_plus::objects::EmfPlusGraphicsVersion,
    /// PathPoints (variable): An array of PathPointCount points that
    /// specify the path. The type of objects in this array is
    /// specified by the PathPointFlags field, as follows:
    ///
    /// - If the R flag is set, the points are relative locations specified by
    ///   EmfPlusPointR objects (section 2.2.2.37).
    /// - If the R flag is clear and the C flag is set, the points are absolute
    ///   locations specified by EmfPlusPoint objects (section 2.2.2.35).
    /// - If the R flag is clear and the C flag is clear, the points are
    ///   absolute locations specified by EmfPlusPointF objects (section
    ///   2.2.2.36).
    pub path_points: EmfPlusPoints,
    /// PathPointTypes (variable): An array of PathPointCount objects
    /// that specifies how the points in the PathPoints field are used
    /// to draw the path. The type of objects in this array is
    /// specified by the PathPointFlags field, as follows:
    ///
    /// - If the R flag is set, the point types are specified by
    ///   EmfPlusPathPointTypeRLE objects (section 2.2.2.32), which use
    ///   run-length encoding (RLE) compression ([MS-WMF] section 3.1.6).
    /// - If the R flag is clear, the point types are specified by
    ///   EmfPlusPathPointType objects (section 2.2.2.31).
    pub path_point_types: EmfPlusPathPointTypes,
}

impl EmfPlusPath {
    /// C (1 bit): If the R flag is clear, this flag specifies the type
    /// of objects in the PathPoints array. See PathPoints and
    /// PathPointTypes for details.
    ///
    /// PathPointFlags bit: points are 16-bit EmfPlusPoint objects.
    /// Undefined and ignored when FLAG_RELATIVE is set.
    const FLAG_COMPRESSED: u32 = 0x0000_4000;
    /// R (1 bit): If this flag is set, the C flag is undefined and
    /// MUST be ignored. The R flag specifies whether the PathPoints
    /// are relative or absolute locations in the coordinate space, and
    /// whether the PathPointTypes are run-length encoded. See
    /// PathPoints and PathPointTypes for details.
    ///
    /// PathPointFlags bit: points are EmfPlusPointR deltas.
    ///
    /// The specification prose describes a single R flag that controls
    /// both relative points and RLE-compressed types, but its own bit
    /// diagram and every mainstream implementation (GDI+ output,
    /// LibreOffice, libUEMF) treat "relative points" (0x0800, same
    /// position as the P flag of drawing records) and "RLE types"
    /// (0x1000) as independent bits. Real files produced by GDI+
    /// combine relative points with plain (non-RLE) type bytes.
    const FLAG_RELATIVE: u32 = 0x0000_0800;
    /// PathPointFlags bit: type values are RLE-compressed.
    const FLAG_RLE_TYPES: u32 = 0x0000_1000;

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
        let version = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::objects::EmfPlusGraphicsVersion::parse,
        )?;
        let path_point_count: u32 = read_field(buf, &mut consumed_bytes)?;
        let path_point_flags: u32 = read_field(buf, &mut consumed_bytes)?;

        crate::parser::emf_plus::check_element_count(
            "PathPointCount",
            path_point_count,
        )?;

        let relative = path_point_flags & Self::FLAG_RELATIVE != 0;
        let compressed = path_point_flags & Self::FLAG_COMPRESSED != 0;
        let rle_types = path_point_flags & Self::FLAG_RLE_TYPES != 0;

        let path_points = EmfPlusPoints::parse(
            buf,
            &mut consumed_bytes,
            path_point_count,
            relative,
            compressed,
        )?;
        let path_point_types = EmfPlusPathPointTypes::parse(
            buf,
            &mut consumed_bytes,
            path_point_count,
            rle_types,
        )?;

        // AlignmentPadding (up to 3 bytes) is NOT consumed here: when
        // the path is nested inside another object it is followed by a
        // caller-known size boundary, and when it is a whole object
        // record the framing discards the remainder.
        Ok((Self { version, path_points, path_point_types }, consumed_bytes))
    }
}

/// Skips the gap between the bytes a nested object actually consumed
/// and the size its length-prefix declared. Rejects the layout where
/// the object consumed more bytes than the declared size, which would
/// mean the parse ran past the nested boundary into sibling data.
pub(in crate::parser::emf_plus) fn skip_object_padding<R: crate::Read>(
    buf: &mut R,
    tracker: &mut impl crate::parser::ConsumeTracker,
    declared_size: usize,
    consumed: usize,
) -> Result<(), crate::parser::ParseError> {
    if consumed > declared_size {
        return Err(crate::parser::ParseError::UnexpectedPattern {
            cause: alloc::format!(
                "nested object consumed {consumed} bytes, but its declared \
                 size is {declared_size}",
            )
            .into(),
        });
    }

    crate::parser::records::discard_bytes_field(
        buf,
        tracker,
        declared_size - consumed,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::{
        PathPointType,
        objects::{EmfPlusPoint, EmfPlusPointF},
    };

    fn version_bytes() -> [u8; 4] {
        0xDBC0_1002_u32.to_le_bytes()
    }

    #[test]
    fn parses_float_points_with_plain_types() {
        let mut data = vec![];
        data.extend(version_bytes());
        data.extend(2_u32.to_le_bytes());
        data.extend(0_u32.to_le_bytes());
        for v in [1.0_f32, 2.0, 3.0, 4.0] {
            data.extend(v.to_le_bytes());
        }
        data.extend([0x00, 0x01]);

        let mut buf: &[u8] = &data;
        let (path, consumed) = EmfPlusPath::parse(&mut buf).unwrap();

        assert_eq!(consumed, data.len());
        assert_eq!(
            path.path_points,
            EmfPlusPoints::AbsoluteF(vec![
                EmfPlusPointF { x: 1.0, y: 2.0 },
                EmfPlusPointF { x: 3.0, y: 4.0 },
            ]),
        );

        let types = path.path_point_types.expand();
        assert_eq!(types[0].point_type, PathPointType::PathPointTypeStart);
        assert_eq!(types[1].point_type, PathPointType::PathPointTypeLine);
    }

    #[test]
    fn parses_compressed_points() {
        let mut data = vec![];
        data.extend(version_bytes());
        data.extend(2_u32.to_le_bytes());
        data.extend(0x4000_u32.to_le_bytes());
        for v in [1_i16, 2, 3, 4] {
            data.extend(v.to_le_bytes());
        }
        data.extend([0x00, 0x01]);

        let mut buf: &[u8] = &data;
        let (path, consumed) = EmfPlusPath::parse(&mut buf).unwrap();

        assert_eq!(consumed, data.len());
        assert_eq!(
            path.path_points,
            EmfPlusPoints::Absolute(vec![
                EmfPlusPoint { x: 1, y: 2 },
                EmfPlusPoint { x: 3, y: 4 },
            ]),
        );
    }

    #[test]
    fn relative_flag_overrides_compressed_flag() {
        let mut data = vec![];
        data.extend(version_bytes());
        data.extend(2_u32.to_le_bytes());
        // C is set but MUST be ignored because the relative flag wins.
        data.extend(0x4800_u32.to_le_bytes());
        // Deltas (1, 1) and (2, 2) as EmfPlusInteger7 values.
        data.extend([0x01, 0x01, 0x02, 0x02]);
        data.extend([0x00, 0x01]);

        let mut buf: &[u8] = &data;
        let (path, consumed) = EmfPlusPath::parse(&mut buf).unwrap();

        assert_eq!(consumed, data.len());
        assert_eq!(path.path_points.as_points_f(), vec![
            EmfPlusPointF { x: 1.0, y: 1.0 },
            EmfPlusPointF { x: 3.0, y: 3.0 },
        ],);
    }
}
