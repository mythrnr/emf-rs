use crate::parser::emf_plus::records::{FLAG_C, FLAG_P, flag, object_id};

/// The EmfPlusDrawImagePoints record specifies drawing a scaled image
/// inside a parallelogram.
#[derive(Clone, Debug)]
pub struct EmfPlusDrawImagePoints {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusDrawImagePoints from the RecordType enumeration.
    /// The value MUST be 0x401B.
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
    /// E (1 bit): This bit indicates that the rendering of the image
    /// includes applying an effect. If set, an object of the Effect
    /// class MUST have been specified in an earlier
    /// EmfPlusSerializableObject record.
    ///
    /// This is the 0x2000 bit of the record flags.
    pub apply_effect: bool,
    /// ObjectID (1 byte): The index of an EmfPlusImage object in the
    /// EMF+ Object Table, which specifies the image to render. The
    /// value MUST be zero to 63, inclusive.
    pub object_id: u8,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record. For this
    /// record type, the value MUST be one of the following.
    ///
    /// - 0x00000030: If the P bit is set in the Flags field.
    /// - 0x00000034: If the P bit is clear and the C bit is set in the Flags
    ///   field.
    /// - 0x00000040: If the P bit is clear and the C bit is clear in the Flags
    ///   field.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be one of the
    /// following.
    ///
    /// - 0x00000024: If the P bit is set in the Flags field.
    /// - 0x00000028: If the P bit is clear and the C bit is set in the Flags
    ///   field.
    /// - 0x00000034: If the P bit is clear and the C bit is clear in the Flags
    ///   field.
    pub data_size: crate::parser::Size,
    /// ImageAttributesID (4 bytes): An unsigned integer that contains
    /// the index of the optional EmfPlusImageAttributes object in the
    /// EMF+ Object Table.
    pub image_attributes_id: u32,
    /// SrcUnit (4 bytes): A signed integer that defines the units of
    /// the SrcRect field. It MUST be the UnitPixel value of the
    /// UnitType enumeration.
    pub src_unit: crate::parser::emf_plus::UnitType,
    /// SrcRect (16 bytes): An EmfPlusRectF object that defines a
    /// portion of the image to be rendered.
    pub src_rect: crate::parser::emf_plus::objects::EmfPlusRectF,
    /// Count (4 bytes): An unsigned integer that specifies the number
    /// of points in the PointData array. Exactly 3 points MUST be
    /// specified.
    pub count: u32,
    /// PointData (variable): An array of Count points that specify
    /// three points of a parallelogram. The three points represent the
    /// upper-left, upper-right, and lower-left corners of the
    /// parallelogram. The fourth point of the parallelogram is
    /// extrapolated from the first three. The portion of the image
    /// specified by the SrcRect field SHOULD have scaling and shearing
    /// transforms applied if necessary to fit inside the
    /// parallelogram.
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

impl EmfPlusDrawImagePoints {
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
        use crate::parser::records::{read_field, read_with};

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u16,
            crate::parser::emf_plus::RecordType::EmfPlusDrawImagePoints as u16,
        )?;

        let image_attributes_id: u32 = read_field(buf, &mut data_size)?;
        let src_unit = read_with(
            buf,
            &mut data_size,
            crate::parser::emf_plus::UnitType::parse,
        )?;
        let src_rect = read_with(
            buf,
            &mut data_size,
            crate::parser::emf_plus::objects::EmfPlusRectF::parse,
        )?;
        let count: u32 = read_field(buf, &mut data_size)?;

        // Count MUST be 3: the fourth corner of the parallelogram is
        // extrapolated from the first three.
        crate::parser::ParseError::expect_eq("Count", count, 3)?;

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
            apply_effect: flags & 0x2000 != 0,
            object_id: object_id(flags),
            size,
            data_size,
            image_attributes_id,
            src_unit,
            src_rect,
            count,
            point_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        imports::*,
        parser::emf_plus::objects::{
            EmfPlusPoint, EmfPlusPoints, EmfPlusRectF,
        },
    };

    fn record_data() -> Vec<u8> {
        let mut data = vec![];
        data.extend(7_u32.to_le_bytes());
        data.extend(2_u32.to_le_bytes()); // UnitTypePixel
        for v in [0.0_f32, 0.0, 64.0, 32.0] {
            data.extend(v.to_le_bytes());
        }
        data.extend(3_u32.to_le_bytes());
        for v in [1_i16, 2, 3, 4, 5, 6] {
            data.extend(v.to_le_bytes());
        }
        data
    }

    #[test]
    fn parses_compressed_points_and_flags() {
        let data = record_data();
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusDrawImagePoints::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusDrawImagePoints,
            0x6002,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusDrawImagePoints,
        );
        assert_eq!(record.flags, 0x6002);
        assert_eq!(record.size, 0x0000_0034);
        assert!(record.apply_effect);
        assert_eq!(record.object_id, 2);
        assert_eq!(record.image_attributes_id, 7);
        assert_eq!(
            record.src_unit,
            crate::parser::emf_plus::UnitType::UnitTypePixel,
        );
        assert_eq!(record.src_rect, EmfPlusRectF {
            x: 0.0,
            y: 0.0,
            width: 64.0,
            height: 32.0,
        });
        assert_eq!(record.count, 3);
        assert_eq!(
            record.point_data,
            EmfPlusPoints::Absolute(vec![
                EmfPlusPoint { x: 1, y: 2 },
                EmfPlusPoint { x: 3, y: 4 },
                EmfPlusPoint { x: 5, y: 6 },
            ]),
        );
    }

    #[test]
    fn rejects_a_count_other_than_three() {
        let mut data = record_data();
        // Overwrite the Count field (bytes 24..28) with 2.
        data[24..28].copy_from_slice(&2_u32.to_le_bytes());
        // Drop one point so the record stays self-consistent.
        data.truncate(data.len() - 4);

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);

        assert!(
            EmfPlusDrawImagePoints::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusDrawImagePoints,
                0x4000,
                u32::try_from(data.len()).unwrap() + 12,
                data_size,
            )
            .is_err()
        );
    }
}
