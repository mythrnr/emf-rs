use crate::{
    imports::*,
    parser::emf_plus::records::{EmfPlusBrushIdOrColor, object_id},
};

/// The EmfPlusDrawString record specifies text output with string
/// formatting.
#[derive(Clone, Debug)]
pub struct EmfPlusDrawString {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusDrawString from the RecordType enumeration. The
    /// value MUST be 0x401C.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    ///
    /// S (1 bit): This bit indicates the type of data in the BrushId
    /// field. If set, BrushId specifies a color as an EmfPlusARGB
    /// object. If clear, BrushId contains the index of an EmfPlusBrush
    /// object in the EMF+ Object Table.
    pub flags: u16,
    /// ObjectID (1 byte): The index of an EmfPlusFont object in the
    /// EMF+ Object Table to render the text. The value MUST be zero to
    /// 63, inclusive.
    pub object_id: u8,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header, record-specific data, and any extra
    /// alignment padding. For this record type, the value MUST be
    /// 0x0000002A or greater; the size of the record is computed as
    /// follows:
    ///
    /// Size = (Length * 0x00000002) + 0x00000028
    /// (+ AlignmentPaddingSize where AlignmentPaddingSize is the
    /// number of bytes in AlignmentPadding)
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data and any
    /// extra alignment padding that follows. For this record type, the
    /// value MUST be 0x0000001E or greater; the size of the data is
    /// computed as follows:
    ///
    /// DataSize = (Length * 0x00000002) + 0x0000001C
    /// (+ AlignmentPaddingSize where AlignmentPaddingSize is the
    /// number of bytes in AlignmentPadding)
    pub data_size: crate::parser::Size,
    /// BrushId (4 bytes): An unsigned integer that specifies the
    /// brush, the content of which is determined by the S bit in the
    /// Flags field. This definition is used to paint the foreground
    /// text color; that is, just the glyphs themselves.
    pub brush_id: EmfPlusBrushIdOrColor,
    /// FormatID (4 bytes): An unsigned integer that specifies the
    /// index of an optional EmfPlusStringFormat object in the EMF+
    /// Object Table. This object specifies text layout information and
    /// display manipulations to be applied to a string.
    pub format_id: u32,
    /// Length (4 bytes): An unsigned integer that specifies the number
    /// of characters in the string.
    pub length: u32,
    /// LayoutRect (16 bytes): An EmfPlusRectF object that defines the
    /// bounding area of the destination that will receive the string.
    pub layout_rect: crate::parser::emf_plus::objects::EmfPlusRectF,
    /// StringData (variable): An array of 16-bit Unicode characters
    /// that specifies the string to be drawn, decoded from UTF-16LE.
    pub string_data: String,
}

impl EmfPlusDrawString {
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
            crate::parser::emf_plus::RecordType::EmfPlusDrawString as u16,
        )?;

        let brush_id =
            EmfPlusBrushIdOrColor::parse(buf, &mut data_size, flags)?;
        let format_id: u32 = read_field(buf, &mut data_size)?;
        let length: u32 = read_field(buf, &mut data_size)?;
        let layout_rect = read_with(
            buf,
            &mut data_size,
            crate::parser::emf_plus::objects::EmfPlusRectF::parse,
        )?;

        // The declared character count must fit in the record data
        // that actually follows; checking before the read keeps a
        // crafted length field from sizing the string buffer.
        crate::parser::ParseError::expect_le(
            "string length (bytes)",
            u64::from(length) * 2,
            data_size.remaining_bytes() as u64,
        )?;

        let string_data = crate::parser::emf_plus::read_utf16_field(
            buf,
            &mut data_size,
            length,
            "string length",
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
            brush_id,
            format_id,
            length,
            layout_rect,
            string_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::objects::EmfPlusARGB;

    #[test]
    fn parses_color_brush_and_utf16_string() {
        let mut data = vec![];
        data.extend([0xFF, 0x00, 0x00, 0xFF]); // blue=255, alpha=255
        data.extend(2_u32.to_le_bytes());
        data.extend(3_u32.to_le_bytes());
        for v in [0.0_f32, 0.0, 100.0, 20.0] {
            data.extend(v.to_le_bytes());
        }
        for c in "abc".encode_utf16() {
            data.extend(c.to_le_bytes());
        }
        data.extend([0x00, 0x00]); // alignment padding

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusDrawString::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusDrawString,
            0x8001,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(record.object_id, 1);
        assert_eq!(
            record.brush_id,
            EmfPlusBrushIdOrColor::Color(EmfPlusARGB {
                blue: 0xFF,
                green: 0x00,
                red: 0x00,
                alpha: 0xFF,
            }),
        );
        assert_eq!(record.format_id, 2);
        assert_eq!(record.length, 3);
        assert_eq!(record.string_data, "abc");
    }
}
