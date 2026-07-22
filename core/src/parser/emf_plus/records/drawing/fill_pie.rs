use crate::parser::emf_plus::records::{EmfPlusBrushIdOrColor, FLAG_C, flag};

/// The EmfPlusFillPie record specifies filling a section of the
/// interior of an ellipse.
#[derive(Clone, Debug)]
pub struct EmfPlusFillPie {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusFillPie from the RecordType enumeration. The
    /// value MUST be 0x4010.
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
    /// C (1 bit): This bit indicates whether the data in the RectData
    /// field is compressed. If set, RectData contains an EmfPlusRect
    /// object. If clear, RectData contains an EmfPlusRectF object.
    pub flags: u16,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be one of the following:
    ///
    /// | Value | Meaning |
    /// |:-|:-|
    /// | `0x00000020` | If the C bit is set in the Flags field. |
    /// | `0x00000028` | If the C bit is clear in the Flags field. |
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be one of the
    /// following:
    ///
    /// | Value | Meaning |
    /// |:-|:-|
    /// | `0x00000014` | If the C bit is set in the Flags field. |
    /// | `0x0000001C` | If the C bit is clear in the Flags field. |
    pub data_size: crate::parser::Size,
    /// BrushId (4 bytes): An unsigned integer that defines the brush,
    /// the content of which is determined by the S bit in the Flags
    /// field.
    pub brush_id: EmfPlusBrushIdOrColor,
    /// StartAngle (4 bytes): A non-negative floating-point value that
    /// specifies the angle between the x-axis and the starting point
    /// of the pie wedge. Any value is acceptable, but it MUST be
    /// interpreted modulo 360, with the result that is used being in
    /// the range 0.0 inclusive to 360.0 exclusive.
    pub start_angle: f32,
    /// SweepAngle (4 bytes): A floating-point value that specifies the
    /// extent of the arc that defines the pie wedge to fill, as an
    /// angle in degrees measured from the starting point defined by
    /// the StartAngle value. Any value is acceptable, but it MUST be
    /// clamped to -360.0 to 360.0 inclusive. A positive value
    /// indicates that the sweep is defined in a clockwise direction,
    /// and a negative value indicates that the sweep is defined in a
    /// counter-clockwise direction.
    pub sweep_angle: f32,
    /// RectData (variable): Either an EmfPlusRect or EmfPlusRectF
    /// object that defines the bounding box of the ellipse that
    /// contains the pie wedge. This rectangle defines the position,
    /// size, and shape of the pie. The type of object in this field is
    /// specified by the value of the Flags field.
    pub rect_data: crate::parser::emf_plus::objects::EmfPlusRectData,
}

impl EmfPlusFillPie {
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
            crate::parser::emf_plus::RecordType::EmfPlusFillPie as u16,
        )?;

        let brush_id =
            EmfPlusBrushIdOrColor::parse(buf, &mut data_size, flags)?;
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
            size,
            data_size,
            brush_id,
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
    fn parses_angles_and_compressed_rect() {
        let mut data = vec![];
        data.extend(1_u32.to_le_bytes());
        data.extend(0.0_f32.to_le_bytes());
        data.extend(90.0_f32.to_le_bytes());
        for v in [1_i16, 2, 3, 4] {
            data.extend(v.to_le_bytes());
        }

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusFillPie::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusFillPie,
            0x4000,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusFillPie,
        );
        assert_eq!(record.flags, 0x4000);
        assert_eq!(record.size, 0x0000_0020);
        assert_eq!(record.brush_id, EmfPlusBrushIdOrColor::BrushId(1));
        assert_eq!(record.start_angle.to_bits(), 0.0_f32.to_bits());
        assert_eq!(record.sweep_angle.to_bits(), 90.0_f32.to_bits());
        assert_eq!(record.rect_data.as_rect_f(), EmfPlusRectF {
            x: 1.0,
            y: 2.0,
            width: 3.0,
            height: 4.0
        });
    }
}
