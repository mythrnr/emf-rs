use crate::{
    imports::*,
    parser::emf_plus::records::{FLAG_C, flag, object_id},
};

/// The EmfPlusDrawRects record specifies drawing a series of
/// rectangles.
#[derive(Clone, Debug)]
pub struct EmfPlusDrawRects {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusDrawRects from the RecordType enumeration. The
    /// value MUST be 0x400B.
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
    /// EMF+ Object Table to draw the rectangles. The value MUST be
    /// zero to 63, inclusive.
    pub object_id: u8,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data.
    ///
    /// At least 1 RectData array element MUST be specified in this
    /// record.
    ///
    /// 0x00000018 <= value: If the C bit is set in the Flags field,
    /// Size MUST be computed as follows:
    ///
    /// Size = (Count * 0x00000008) + 0x00000010
    ///
    /// 0x00000020 <= value: If the C bit is clear in the Flags field,
    /// Size MUST be computed as follows:
    ///
    /// Size = (Count * 0x00000010) + 0x00000010
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows.
    ///
    /// At least 1 RectData array element MUST be specified in this
    /// record.
    ///
    /// 0x0000000C <= value: If the C bit is set in the Flags field,
    /// DataSize MUST be computed as follows:
    ///
    /// DataSize = (Count * 0x00000008) + 0x00000004
    ///
    /// 0x00000014 <= value: If the C bit is clear in the Flags field,
    /// DataSize MUST be computed as follows:
    ///
    /// DataSize = (Count * 0x00000010) + 0x00000004
    pub data_size: crate::parser::Size,
    /// Count (4 bytes): An unsigned integer that specifies the number
    /// of rectangles in the RectData member.
    pub count: u32,
    /// RectData (variable): An array of either an EmfPlusRect or
    /// EmfPlusRectF objects of Count length that defines the rectangle
    /// data.
    pub rect_data: Vec<crate::parser::emf_plus::objects::EmfPlusRectData>,
}

impl EmfPlusDrawRects {
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
            crate::parser::emf_plus::RecordType::EmfPlusDrawRects as u16,
        )?;

        let count: u32 = read_field(buf, &mut data_size)?;
        let rect_data =
            crate::parser::emf_plus::objects::EmfPlusRectData::parse_list(
                buf,
                &mut data_size,
                count,
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
            rect_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::objects::{EmfPlusRect, EmfPlusRectData};

    #[test]
    fn parses_compressed_rectangles() {
        let mut data = vec![];
        data.extend(2_u32.to_le_bytes());
        for v in [1_i16, 2, 3, 4, 5, 6, 7, 8] {
            data.extend(v.to_le_bytes());
        }

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusDrawRects::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusDrawRects,
            0x4004,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusDrawRects,
        );
        assert_eq!(record.flags, 0x4004);
        assert_eq!(record.size, 0x0000_0020);
        assert_eq!(record.object_id, 4);
        assert_eq!(record.count, 2);
        assert_eq!(record.rect_data, vec![
            EmfPlusRectData::Compressed(EmfPlusRect {
                x: 1,
                y: 2,
                width: 3,
                height: 4,
            }),
            EmfPlusRectData::Compressed(EmfPlusRect {
                x: 5,
                y: 6,
                width: 7,
                height: 8,
            }),
        ]);
    }
}
