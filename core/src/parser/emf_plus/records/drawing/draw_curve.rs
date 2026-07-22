use crate::parser::emf_plus::records::{FLAG_C, flag, object_id};

/// The EmfPlusDrawCurve record specifies drawing a cardinal spline.
#[derive(Clone, Debug)]
pub struct EmfPlusDrawCurve {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusDrawCurve from the RecordType enumeration. The
    /// value MUST be 0x4018.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    ///
    /// C (1 bit): This bit indicates whether the data in the PointData
    /// field is compressed. If set, PointData contains an array of
    /// EmfPlusPoint objects. If clear, PointData contains an array of
    /// EmfPlusPointF objects.
    pub flags: u16,
    /// ObjectID (1 byte): The index of an EmfPlusPen object in the
    /// EMF+ Object Table to draw the curve. The value MUST be zero to
    /// 63, inclusive.
    pub object_id: u8,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. At least 2
    /// PointData elements MUST be specified in this record.
    ///
    /// If the C bit is set in the Flags field, Count points with
    /// 16-bit signed integer values are defined in the PointData
    /// field. In this case, the value MUST be 0x00000024 or greater,
    /// and Size MUST be computed as follows:
    ///
    /// Size = (Count * 0x00000004) + 0x0000001C
    ///
    /// If the C bit is clear in the Flags field, Count points with
    /// 32-bit floating-point values are defined in the PointData
    /// field. In this case, the value MUST be 0x0000002C or greater,
    /// and Size MUST be computed as follows:
    ///
    /// Size = (Count * 0x00000008) + 0x0000001C
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. At least 2 PointData elements MUST be specified in
    /// this record.
    ///
    /// If the C bit is set in the Flags field, Count points with
    /// 16-bit signed integer values are defined in the PointData
    /// field. In this case, the value MUST be 0x00000018 or greater,
    /// and DataSize MUST be computed as follows:
    ///
    /// DataSize = (Count * 0x00000004) + 0x00000010
    ///
    /// If the C bit is clear in the Flags field, Count points with
    /// 32-bit floating-point values are defined in the PointData
    /// field. In this case, the value MUST be 0x00000020 or greater,
    /// and DataSize MUST be computed as follows:
    ///
    /// DataSize = (Count * 0x00000008) + 0x00000010
    pub data_size: crate::parser::Size,
    /// Tension (4 bytes): A floating-point value that specifies how
    /// tightly the spline bends as it passes through the points. A
    /// value of 0 specifies that the spline is a sequence of straight
    /// lines. As the value increases, the curve becomes more rounded.
    /// For more information, see [SPLINE77] and [PETZOLD].
    pub tension: f32,
    /// Offset (4 bytes): An unsigned integer that specifies the
    /// element in the PointData array that defines the starting point
    /// of the spline.
    pub offset: u32,
    /// NumSegments (4 bytes): An unsigned integer that specifies the
    /// number of line segments making up the spline.
    pub num_segments: u32,
    /// Count (4 bytes): An unsigned integer that specifies the number
    /// of points in the PointData array. The minimum number of points
    /// for drawing a curve is 2 - the starting and ending points.
    pub count: u32,
    /// PointData (variable): An array of either 16-bit signed integers
    /// or 32-bit floating-point values of Count length that defines
    /// the coordinates of the endpoints of the lines to be stroked.
    pub point_data: crate::parser::emf_plus::objects::EmfPlusPoints,
}

impl EmfPlusDrawCurve {
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
            crate::parser::emf_plus::RecordType::EmfPlusDrawCurve as u16,
        )?;

        let tension = read_field(buf, &mut data_size)?;
        let offset = read_field(buf, &mut data_size)?;
        let num_segments = read_field(buf, &mut data_size)?;
        let count = read_field(buf, &mut data_size)?;
        // This record type has no P flag: point data is never
        // relative.
        let point_data =
            crate::parser::emf_plus::objects::EmfPlusPoints::parse(
                buf,
                &mut data_size,
                count,
                false,
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
            tension,
            offset,
            num_segments,
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
        data.extend(0.25_f32.to_le_bytes());
        data.extend(0_u32.to_le_bytes());
        data.extend(1_u32.to_le_bytes());
        data.extend(2_u32.to_le_bytes());
        for v in [10_i16, 20, 30, 40] {
            data.extend(v.to_le_bytes());
        }

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusDrawCurve::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusDrawCurve,
            0x4001,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(record.record_type as u16, 0x4018);
        assert_eq!(record.flags, 0x4001);
        assert_eq!(record.object_id, 1);
        assert_eq!(record.size, 0x0000_0024);
        assert_eq!(record.tension.to_bits(), 0.25_f32.to_bits());
        assert_eq!(record.offset, 0);
        assert_eq!(record.num_segments, 1);
        assert_eq!(record.count, 2);
        assert_eq!(
            record.point_data,
            EmfPlusPoints::Absolute(vec![
                EmfPlusPoint { x: 10, y: 20 },
                EmfPlusPoint { x: 30, y: 40 },
            ]),
        );
    }
}
