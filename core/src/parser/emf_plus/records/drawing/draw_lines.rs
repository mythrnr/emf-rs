use crate::parser::emf_plus::records::{FLAG_C, FLAG_P, flag, object_id};

/// The EmfPlusDrawLines record specifies drawing a series of connected
/// lines.
#[derive(Clone, Debug)]
pub struct EmfPlusDrawLines {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusDrawLines from the RecordType enumeration. The
    /// value MUST be 0x400D.
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
    /// Note: If this flag is set, the C flag (above) is undefined and
    /// MUST be ignored.
    pub flags: u16,
    /// L (1 bit): This bit indicates whether to draw an extra line
    /// between the last point and the first point, to close the shape.
    ///
    /// This is the 0x2000 bit of the record flags.
    pub closed: bool,
    /// ObjectID (1 byte): The index of an EmfPlusPen object in the
    /// EMF+ Object Table to draw the lines. The value MUST be zero to
    /// 63, inclusive.
    pub object_id: u8,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record. For this
    /// record type, the value MUST be one of the following.
    ///
    /// 0x00000014 <= value: If the P bit is set in the Flags field,
    /// the minimum Size is computed as follows:
    ///
    /// Size = (Count * 0x00000002) + 0x00000010
    ///
    /// 0x00000018 <= value: If the P bit is clear and the C bit is set
    /// in the Flags field, Size is computed as follows:
    ///
    /// Size = (Count * 0x00000004) + 0x00000010
    ///
    /// 0x00000020 <= value: If the P bit is clear and the C bit is
    /// clear in the Flags field, Size is computed as follows:
    ///
    /// Size = (Count * 0x00000008) + 0x00000010
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be one of the
    /// following.
    ///
    /// 0x00000008 <= value: If the P bit is set in the Flags field,
    /// the minimum DataSize is computed as follows:
    ///
    /// DataSize = (Count * 0x00000002) + 0x00000004
    ///
    /// 0x0000000C <= value: If the P bit is clear and the C bit is set
    /// in the Flags field, DataSize is computed as follows:
    ///
    /// DataSize = (Count * 0x00000004) + 0x00000004
    ///
    /// 0x00000014 <= value: If the P bit is clear and the C bit is
    /// clear in the Flags field, DataSize is computed as follows:
    ///
    /// DataSize = (Count * 0x00000008) + 0x00000004
    pub data_size: crate::parser::Size,
    /// Count (4 bytes): An unsigned integer that specifies the number
    /// of points in the PointData array. At least 2 points MUST be
    /// specified.
    pub count: u32,
    /// PointData (variable): An array of Count points that specify the
    /// starting and ending points of the lines to be drawn.
    ///
    /// The type of data in this array is specified by the Flags field,
    /// as follows.
    ///
    /// - EmfPlusPointR object: If the P flag is set in the Flags, the points
    ///   specify relative locations.
    /// - EmfPlusPoint object: If the P bit is clear and the C bit is set in
    ///   the Flags field, the points specify absolute locations with 16-bit
    ///   signed integer values.
    /// - EmfPlusPointF object: If the P bit is clear and the C bit is clear in
    ///   the Flags field, the points specify absolute locations with 32-bit
    ///   floating-point values.
    pub point_data: crate::parser::emf_plus::objects::EmfPlusPoints,
}

impl EmfPlusDrawLines {
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
            crate::parser::emf_plus::RecordType::EmfPlusDrawLines as u16,
        )?;

        let count: u32 = read_field(buf, &mut data_size)?;
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
            closed: flags & 0x2000 != 0,
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
    fn parses_compressed_points_and_flags() {
        let mut data = vec![];
        data.extend(2_u32.to_le_bytes());
        for v in [1_i16, 2, 3, 4] {
            data.extend(v.to_le_bytes());
        }

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusDrawLines::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusDrawLines,
            0x6005,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusDrawLines,
        );
        assert_eq!(record.flags, 0x6005);
        assert_eq!(record.size, 0x0000_0018);
        assert!(record.closed);
        assert_eq!(record.object_id, 5);
        assert_eq!(record.count, 2);
        assert_eq!(
            record.point_data,
            EmfPlusPoints::Absolute(vec![
                EmfPlusPoint { x: 1, y: 2 },
                EmfPlusPoint { x: 3, y: 4 },
            ]),
        );
    }
}
