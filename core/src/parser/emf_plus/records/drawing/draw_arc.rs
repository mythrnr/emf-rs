use crate::parser::emf_plus::records::{FLAG_C, flag, object_id};

/// The EmfPlusDrawArc record specifies drawing the arc of an ellipse.
#[derive(Clone, Debug)]
pub struct EmfPlusDrawArc {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusDrawArc from the RecordType enumeration. The
    /// value MUST be 0x4012.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    ///
    /// C (1 bit): This bit indicates whether the data in the RectData
    /// field is compressed. If set, RectData contains an EmfPlusRect
    /// object. If clear, RectData contains an EmfPlusRectF object.
    pub flags: u16,
    /// ObjectID (1 byte): The index of an EmfPlusPen object in the
    /// EMF+ Object Table to draw the arc. The value MUST be zero to
    /// 63, inclusive.
    pub object_id: u8,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be one of the following:
    ///
    /// | Value | Meaning |
    /// |:-|:-|
    /// | `0x0000001C` | If the C bit is set in the Flags field. |
    /// | `0x00000024` | If the C bit is clear in the Flags field. |
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be one of the
    /// following:
    ///
    /// | Value | Meaning |
    /// |:-|:-|
    /// | `0x00000010` | If the C bit is set in the Flags field. |
    /// | `0x00000018` | If the C bit is clear in the Flags field. |
    pub data_size: crate::parser::Size,
    /// StartAngle (4 bytes): A non-negative floating-point value that
    /// specifies the angle between the x-axis and the starting point
    /// of the arc. Any value is acceptable, but it MUST be interpreted
    /// modulo 360, with the result that is used being in the range 0.0
    /// inclusive to 360.0 exclusive.
    pub start_angle: f32,
    /// SweepAngle (4 bytes): A floating-point value that specifies the
    /// extent of the arc to draw, as an angle in degrees measured from
    /// the starting point defined by the StartAngle value. Any value
    /// is acceptable, but it MUST be clamped to -360.0 to 360.0
    /// inclusive. A positive value indicates that the sweep is defined
    /// in a clockwise direction, and a negative value indicates that
    /// the sweep is defined in a counter-clockwise direction.
    pub sweep_angle: f32,
    /// RectData (variable): Either an EmfPlusRect or EmfPlusRectF
    /// object that defines the bounding box of the ellipse that is
    /// collinear with the arc. This rectangle defines the position,
    /// size, and shape of the arc. The type of object in this field is
    /// specified by the value of the Flags field.
    pub rect_data: crate::parser::emf_plus::objects::EmfPlusRectData,
}

impl EmfPlusDrawArc {
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
            crate::parser::emf_plus::RecordType::EmfPlusDrawArc as u16,
        )?;

        let start_angle = read_field(buf, &mut data_size)?;
        let sweep_angle = read_field(buf, &mut data_size)?;
        let rect_data =
            crate::parser::emf_plus::objects::EmfPlusRectData::parse(
                buf,
                &mut data_size,
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
            start_angle,
            sweep_angle,
            rect_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::objects::EmfPlusRectF;

    #[test]
    fn parses_a_compressed_rect_and_stores_the_header() {
        let mut data = vec![];
        data.extend(90.0_f32.to_le_bytes());
        data.extend((-180.0_f32).to_le_bytes());
        for v in [1_i16, 2, 3, 4] {
            data.extend(v.to_le_bytes());
        }

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusDrawArc::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusDrawArc,
            0x4005,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(record.record_type as u16, 0x4012);
        assert_eq!(record.flags, 0x4005);
        assert_eq!(record.object_id, 5);
        assert_eq!(record.size, 0x0000_001C);
        assert_eq!(record.start_angle.to_bits(), 90.0_f32.to_bits());
        assert_eq!(record.sweep_angle.to_bits(), (-180.0_f32).to_bits());
        assert_eq!(record.rect_data.as_rect_f(), EmfPlusRectF {
            x: 1.0,
            y: 2.0,
            width: 3.0,
            height: 4.0,
        });
    }
}
