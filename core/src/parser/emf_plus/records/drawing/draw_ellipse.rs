use crate::parser::emf_plus::records::{FLAG_C, flag, object_id};

/// The EmfPlusDrawEllipse record specifies drawing an ellipse.
#[derive(Clone, Debug)]
pub struct EmfPlusDrawEllipse {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusDrawEllipse from the RecordType enumeration. The
    /// value MUST be 0x400F.
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
    /// EMF+ Object Table to draw the ellipse. The value MUST be zero
    /// to 63, inclusive.
    pub object_id: u8,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be one of the following:
    ///
    /// | Value | Meaning |
    /// |:-|:-|
    /// | `0x00000014` | If the C bit is set in the Flags field. |
    /// | `0x0000001C` | If the C bit is clear in the Flags field. |
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be one of the
    /// following:
    ///
    /// | Value | Meaning |
    /// |:-|:-|
    /// | `0x00000008` | If the C bit is set in the Flags field. |
    /// | `0x00000010` | If the C bit is clear in the Flags field. |
    pub data_size: crate::parser::Size,
    /// RectData (variable): Either an EmfPlusRect or EmfPlusRectF
    /// object that defines the bounding box of the ellipse.
    pub rect_data: crate::parser::emf_plus::objects::EmfPlusRectData,
}

impl EmfPlusDrawEllipse {
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
            crate::parser::emf_plus::RecordType::EmfPlusDrawEllipse as u16,
        )?;

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
        for v in [5_i16, 6, 7, 8] {
            data.extend(v.to_le_bytes());
        }

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusDrawEllipse::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusDrawEllipse,
            0x4009,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(record.record_type as u16, 0x400F);
        assert_eq!(record.flags, 0x4009);
        assert_eq!(record.object_id, 9);
        assert_eq!(record.size, 0x0000_0014);
        assert_eq!(record.rect_data.as_rect_f(), EmfPlusRectF {
            x: 5.0,
            y: 6.0,
            width: 7.0,
            height: 8.0,
        });
    }
}
