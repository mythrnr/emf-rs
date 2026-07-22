use crate::parser::emf_plus::records::{
    EmfPlusBrushIdOrColor, FLAG_C, FLAG_P, flag,
};

/// The EmfPlusFillPolygon record specifies filling the interior of a
/// polygon.
#[derive(Clone, Debug)]
pub struct EmfPlusFillPolygon {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusFillPolygon from the RecordType enumeration. The
    /// value MUST be 0x400C.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    ///
    /// S (1 bit): This bit indicates the type of data in the BrushId
    /// field. If set, BrushId specifies a color as an EmfPlusARGB
    /// object. If clear, BrushId contains the index of an EmfPlusBrush
    /// object in the EMF+ Object Table.
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
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record. At least 3
    /// points MUST be specified.
    ///
    /// 0x0000001C <= value: If the P bit is set in the Flags field,
    /// the minimum Size is computed as follows:
    ///
    /// Size = ((((Count * 0x00000002) + 0x00000014 + 0x00000003) / 4) * 4)
    ///
    /// 0x00000020 <= value: If the P bit is clear and the C bit is set
    /// in the Flags field, Size is computed as follows:
    ///
    /// Size = (Count * 0x00000004) + 0x00000014
    ///
    /// 0x0000002C <= value: If the P bit is clear and the C bit is
    /// clear in the Flags field, Size is computed as follows:
    ///
    /// Size = (Count * 0x00000008) + 0x00000014
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data in the
    /// record. At least 3 points MUST be specified.
    ///
    /// 0x00000010 <= value: If the P bit is set in the Flags field,
    /// the minimum DataSize is computed as follows:
    ///
    /// DataSize = ((((Count * 0x00000002) + 0x0000008 + 0x00000003) / 4) * 4)
    ///
    /// 0x00000014 <= value: If the P bit is clear and the C bit is set
    /// in the Flags field, DataSize is computed as follows:
    ///
    /// DataSize = (Count * 0x00000004) + 0x00000008
    ///
    /// 0x00000020 <= value: If the P bit is clear and the C bit is
    /// clear in the Flags field, DataSize is computed as follows:
    ///
    /// DataSize = (Count * 0x00000008) + 0x00000008
    pub data_size: crate::parser::Size,
    /// BrushId (4 bytes): An unsigned integer that defines the brush,
    /// the content of which is determined by the S bit in the Flags
    /// field.
    pub brush_id: EmfPlusBrushIdOrColor,
    /// Count (4 bytes): An unsigned integer that specifies the number
    /// of points in the PointData field. At least 3 points MUST be
    /// specified.
    pub count: u32,
    /// PointData (variable): An array of Count points that define the
    /// vertices of the polygon. The first two points in the array
    /// specify the first side of the polygon. Each additional point
    /// specifies a new side, the vertices of which include the point
    /// and the previous point. If the last point and the first point
    /// do not coincide, they specify the last side of the polygon.
    ///
    /// The type of data in this array is specified by the Flags field,
    /// as follows:
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

impl EmfPlusFillPolygon {
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
            crate::parser::emf_plus::RecordType::EmfPlusFillPolygon as u16,
        )?;

        let brush_id =
            EmfPlusBrushIdOrColor::parse(buf, &mut data_size, flags)?;
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
            size,
            data_size,
            brush_id,
            count,
            point_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::objects::{
        EmfPlusARGB, EmfPlusPoint, EmfPlusPoints,
    };

    #[test]
    fn parses_color_fill_with_compressed_points() {
        let mut data = vec![];
        data.extend([0x01, 0x02, 0x03, 0xFF]); // blue, green, red, alpha
        data.extend(3_u32.to_le_bytes());
        for v in [0_i16, 0, 10, 0, 10, 20] {
            data.extend(v.to_le_bytes());
        }

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusFillPolygon::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusFillPolygon,
            0xC000,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusFillPolygon,
        );
        assert_eq!(record.flags, 0xC000);
        assert_eq!(record.size, 0x0000_0020);
        assert_eq!(
            record.brush_id,
            EmfPlusBrushIdOrColor::Color(EmfPlusARGB {
                blue: 0x01,
                green: 0x02,
                red: 0x03,
                alpha: 0xFF,
            }),
        );
        assert_eq!(record.count, 3);
        assert_eq!(
            record.point_data,
            EmfPlusPoints::Absolute(vec![
                EmfPlusPoint { x: 0, y: 0 },
                EmfPlusPoint { x: 10, y: 0 },
                EmfPlusPoint { x: 10, y: 20 },
            ]),
        );
    }
}
