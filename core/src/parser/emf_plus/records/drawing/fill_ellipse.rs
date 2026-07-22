use crate::parser::emf_plus::records::{EmfPlusBrushIdOrColor, FLAG_C, flag};

/// The EmfPlusFillEllipse record specifies filling the interior of an
/// ellipse.
#[derive(Clone, Debug)]
pub struct EmfPlusFillEllipse {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusFillEllipse from the RecordType enumeration. The
    /// value MUST be 0x400E.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    ///
    /// S (1 bit): This bit specifies the type of data in the BrushId
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
    /// | `0x00000018` | If the C bit is set in the Flags field. |
    /// | `0x00000020` | If the C bit is clear in the Flags field. |
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be one of the
    /// following:
    ///
    /// | Value | Meaning |
    /// |:-|:-|
    /// | `0x0000000C` | If the C bit is set in the Flags field. |
    /// | `0x00000014` | If the C bit is clear in the Flags field. |
    pub data_size: crate::parser::Size,
    /// BrushId (4 bytes): An unsigned integer that specifies the
    /// brush, the content of which is determined by the S bit in the
    /// Flags field. This definition is used to fill the interior of
    /// the ellipse.
    pub brush_id: EmfPlusBrushIdOrColor,
    /// RectData (variable): Either an EmfPlusRect or EmfPlusRectF
    /// object that defines the bounding box of the ellipse.
    pub rect_data: crate::parser::emf_plus::objects::EmfPlusRectData,
}

impl EmfPlusFillEllipse {
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
        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u16,
            crate::parser::emf_plus::RecordType::EmfPlusFillEllipse as u16,
        )?;

        let brush_id =
            EmfPlusBrushIdOrColor::parse(buf, &mut data_size, flags)?;
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

        Ok(Self { record_type, flags, size, data_size, brush_id, rect_data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::objects::EmfPlusRectF;

    #[test]
    fn parses_brush_id_fill_with_compressed_rect() {
        let mut data = vec![];
        data.extend(5_u32.to_le_bytes());
        for v in [1_i16, 2, 3, 4] {
            data.extend(v.to_le_bytes());
        }

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusFillEllipse::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusFillEllipse,
            0x4000,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusFillEllipse,
        );
        assert_eq!(record.flags, 0x4000);
        assert_eq!(record.size, 0x0000_0018);
        assert_eq!(record.brush_id, EmfPlusBrushIdOrColor::BrushId(5));
        assert_eq!(record.rect_data.as_rect_f(), EmfPlusRectF {
            x: 1.0,
            y: 2.0,
            width: 3.0,
            height: 4.0
        });
    }
}
