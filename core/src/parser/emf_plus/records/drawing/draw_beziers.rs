use crate::parser::emf_plus::records::{FLAG_C, FLAG_P, flag, object_id};

/// The EmfPlusDrawBeziers record specifies drawing a sequence of
/// connected Bezier curves. The order for Bezier data points is the
/// start point, control point 1, control point 2 and end point. For
/// more information see [MSDN-DrawBeziers].
#[derive(Clone, Debug)]
pub struct EmfPlusDrawBeziers {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusDrawBeziers from the RecordType enumeration. The
    /// value MUST be 0x4019.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    ///
    /// C (1 bit): This bit indicates whether the PointData field
    /// specifies compressed data. If set, PointData specifies absolute
    /// locations in the coordinate space with 16-bit signed integer
    /// coordinates. If clear, PointData specifies absolute locations
    /// in the coordinate space with 32-bit floating-point coordinates.
    ///
    /// Note: If the P flag (below) is set, this flag is undefined and
    /// MUST be ignored.
    ///
    /// P (1 bit): This bit indicates whether the PointData field
    /// specifies relative or absolute locations. If set, each element
    /// in PointData specifies a location in the coordinate space that
    /// is relative to the location specified by the previous element
    /// in the array. In the case of the first element in PointData, a
    /// previous location at coordinates (0,0) is assumed. If clear,
    /// PointData specifies absolute locations according to the C flag.
    ///
    /// Note: If this flag is set, the C flag (above) is undefined and
    /// MUST be ignored.
    pub flags: u16,
    /// ObjectID (1 byte): The index of an EmfPlusPen object in the
    /// EMF+ Object Table to draw the Bezier curves. The value MUST be
    /// zero to 63, inclusive.
    pub object_id: u8,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record. At least 4
    /// points MUST be specified.
    ///
    /// If the P bit is set in the Flags field, the value MUST be
    /// 0x00000018 or greater; the minimum Size is computed as follows:
    ///
    /// Size = (Count * 0x00000002) + 0x00000010
    ///
    /// If the P bit is clear and the C bit is set in the Flags field,
    /// the value MUST be 0x00000020 or greater; Size is computed as
    /// follows:
    ///
    /// Size = (Count * 0x00000004) + 0x00000010
    ///
    /// If the P bit is clear and the C bit is clear in the Flags
    /// field, the value MUST be 0x00000030 or greater; Size is
    /// computed as follows:
    ///
    /// Size = (Count * 0x00000008) + 0x00000010
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record. At least 4
    /// points MUST be specified.
    ///
    /// If the P bit is set in the Flags field, the value MUST be
    /// 0x0000000C or greater; the minimum DataSize is computed as
    /// follows:
    ///
    /// DataSize = (Count * 0x00000002) + 0x00000004
    ///
    /// If the P bit is clear and the C bit is set in the Flags field,
    /// the value MUST be 0x00000014 or greater; DataSize is computed
    /// as follows:
    ///
    /// DataSize = (Count * 0x00000004) + 0x00000004
    ///
    /// If the P bit is clear and the C bit is clear in the Flags
    /// field, the value MUST be 0x00000024 or greater; DataSize is
    /// computed as follows:
    ///
    /// DataSize = (Count * 0x00000008) + 0x00000004
    pub data_size: crate::parser::Size,
    /// Count (4 bytes): An unsigned integer that specifies the number
    /// of points in the PointData array. At least 4 points MUST be
    /// specified.
    pub count: u32,
    /// PointData (variable): An array of Count points that specify the
    /// starting, ending, and control points of the Bezier curves. The
    /// ending coordinate of one Bezier curve is the starting
    /// coordinate of the next. The control points are used for
    /// producing the Bezier effect.
    ///
    /// The type of data in this array is specified by the Flags field,
    /// as follows:
    ///
    /// - EmfPlusPointR object: If the P flag is set in the Flags, the points
    ///   specify relative locations.
    /// - EmfPlusPointF object: If the P and C bits are clear in the Flags
    ///   field, the points specify absolute locations.
    /// - EmfPlusPoint object: If the P bit is clear and the C bit is set in
    ///   the Flags field, the points specify relative locations.
    ///
    /// A Bezier curve does not pass through its control points. The
    /// control points act as magnets, pulling the curve in certain
    /// directions to influence the way the lines bend.
    pub point_data: crate::parser::emf_plus::objects::EmfPlusPoints,
}

impl EmfPlusDrawBeziers {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = ?record_type),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::emf_plus::RecordType,
        flags: u16,
        size: u32,
        mut data_size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::read_field;

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u16,
            crate::parser::emf_plus::RecordType::EmfPlusDrawBeziers as u16,
        )?;

        let count = read_field(buf, &mut data_size)?;
        let point_data =
            crate::parser::emf_plus::objects::EmfPlusPoints::parse(
                buf,
                &mut data_size,
                count,
                flag(flags, FLAG_P),
                flag(flags, FLAG_C),
            )?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self {
            record_type,
            flags,
            object_id: object_id(flags),
            size,
            data_size,
            count,
            point_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::objects::{EmfPlusPoint, EmfPlusPoints};

    #[test]
    fn parses_compressed_points_and_stores_the_header() {
        let mut data = vec![];
        data.extend(4_u32.to_le_bytes());
        for v in [0_i16, 0, 1, 2, 3, 4, 5, 6] {
            data.extend(v.to_le_bytes());
        }

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusDrawBeziers::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusDrawBeziers,
            0x4003,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(record.record_type as u16, 0x4019);
        assert_eq!(record.flags, 0x4003);
        assert_eq!(record.object_id, 3);
        assert_eq!(record.size, 0x0000_0020);
        assert_eq!(record.count, 4);
        assert_eq!(
            record.point_data,
            EmfPlusPoints::Absolute(vec![
                EmfPlusPoint { x: 0, y: 0 },
                EmfPlusPoint { x: 1, y: 2 },
                EmfPlusPoint { x: 3, y: 4 },
                EmfPlusPoint { x: 5, y: 6 },
            ]),
        );
    }
}
