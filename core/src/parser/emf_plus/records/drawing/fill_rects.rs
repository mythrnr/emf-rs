use crate::{
    imports::*,
    parser::emf_plus::records::{EmfPlusBrushIdOrColor, FLAG_C, flag},
};

/// The EmfPlusFillRects record specifies filling the interiors of a
/// series of rectangles.
#[derive(Clone, Debug)]
pub struct EmfPlusFillRects {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusFillRects from the RecordType enumeration. The
    /// value MUST be set to 0x400A.
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
    /// the 12-byte record header and record-specific data. At least 1
    /// RectData array element MUST be specified in this record.
    ///
    /// 0x0000001C <= value: If the C bit is set in the Flags field,
    /// Size MUST be computed as follows:
    ///
    /// Size = (Count * 0x00000008) + 0x00000014
    ///
    /// 0x00000024 <= value: If the C bit is clear in the Flags field,
    /// Size MUST be computed as follows:
    ///
    /// Size = (Count * 0x00000010) + 0x00000014
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. At least 1 RectData array element MUST be specified in
    /// this record.
    ///
    /// 0x00000010 <= value: If the C bit is set in the Flags field,
    /// DataSize MUST be computed as follows:
    ///
    /// DataSize = (Count * 0x00000008) + 0x00000008
    ///
    /// 0x00000018 <= value: If the C bit is clear in the Flags field,
    /// DataSize MUST be computed as follows:
    ///
    /// DataSize = (Count * 0x00000010) + 0x00000008
    pub data_size: crate::parser::Size,
    /// BrushId (4 bytes): An unsigned integer that defines the brush,
    /// the content of which is determined by the S bit in the Flags
    /// field.
    pub brush_id: EmfPlusBrushIdOrColor,
    /// Count (4 bytes): An unsigned integer that specifies the number
    /// of rectangles in the RectData field.
    pub count: u32,
    /// RectData (variable): An array of either an EmfPlusRect or
    /// EmfPlusRectF objects of Count length that defines the rectangle
    /// data.
    pub rect_data: Vec<crate::parser::emf_plus::objects::EmfPlusRectData>,
}

impl EmfPlusFillRects {
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
            crate::parser::emf_plus::RecordType::EmfPlusFillRects as u16,
        )?;

        let brush_id =
            EmfPlusBrushIdOrColor::parse(buf, &mut data_size, flags)?;
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
            size,
            data_size,
            brush_id,
            count,
            rect_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::objects::{EmfPlusARGB, EmfPlusRectF};

    #[test]
    fn parses_color_fill_with_compressed_rects() {
        let mut data = vec![];
        data.extend([0x01, 0x02, 0x03, 0xFF]);
        data.extend(1_u32.to_le_bytes());
        for v in [10_i16, 20, 30, 40] {
            data.extend(v.to_le_bytes());
        }

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusFillRects::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusFillRects,
            0xC000,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusFillRects,
        );
        assert_eq!(record.flags, 0xC000);
        assert_eq!(record.size, 0x0000_001C);
        assert_eq!(
            record.brush_id,
            EmfPlusBrushIdOrColor::Color(EmfPlusARGB {
                blue: 0x01,
                green: 0x02,
                red: 0x03,
                alpha: 0xFF,
            }),
        );
        assert_eq!(record.count, 1);
        assert_eq!(record.rect_data.len(), 1);
        assert_eq!(record.rect_data[0].as_rect_f(), EmfPlusRectF {
            x: 10.0,
            y: 20.0,
            width: 30.0,
            height: 40.0
        },);
    }

    #[test]
    fn parses_brush_id_fill() {
        let mut data = vec![];
        data.extend(7_u32.to_le_bytes());
        data.extend(0_u32.to_le_bytes());

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusFillRects::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusFillRects,
            0,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(record.brush_id, EmfPlusBrushIdOrColor::BrushId(7));
        assert_eq!(record.count, 0);
        assert!(record.rect_data.is_empty());
    }
}
