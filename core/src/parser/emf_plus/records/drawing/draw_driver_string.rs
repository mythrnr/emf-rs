use crate::{
    imports::*,
    parser::emf_plus::records::{EmfPlusBrushIdOrColor, object_id},
};

/// The EmfPlusDrawDriverString record specifies text output with
/// character positions.
#[derive(Clone, Debug)]
pub struct EmfPlusDrawDriverString {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusDrawDriverString from the RecordType
    /// enumeration. The value MUST be 0x4036.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    ///
    /// S (1 bit): This bit indicates the type of data in the BrushId
    /// field. If set, BrushId specifies the color value in an
    /// EmfPlusARGB object. If clear, BrushId contains the EMF+ Object
    /// Table index of an EmfPlusBrush object.
    pub flags: u16,
    /// ObjectID (1 byte): The EMF+ Object Table index of an
    /// EmfPlusFont object to render the text. The value MUST be zero
    /// to 63, inclusive.
    pub object_id: u8,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data.
    ///
    /// When glyphs are provided, but no transform matrix is specified
    /// in the TransformMatrix field, the value MUST be 0x0000001C or
    /// greater; the size of the record is computed as follows:
    ///
    /// Size = (GlyphCount * 0x0000000A) + 0x0000001C
    ///
    /// When glyphs are provided, and a transform matrix is specified
    /// in the TransformMatrix field, the value MUST be 0x00000034 or
    /// greater; the size of the record is computed as follows:
    ///
    /// Size = (GlyphCount * 0x0000000A) + 0x00000034
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows.
    ///
    /// When glyphs are provided, but no transform matrix is specified
    /// in the TransformMatrix field, the value MUST be 0x00000010 or
    /// greater; the size of the data is computed as follows:
    ///
    /// DataSize = (GlyphCount * 0x0000000A) + 0x00000010
    ///
    /// When glyphs are provided, and a transform matrix is specified
    /// in the TransformMatrix field, the value MUST be 0x00000028 or
    /// greater; the size of the data is computed as follows:
    ///
    /// DataSize = (GlyphCount * 0x0000000A) + 0x00000028
    pub data_size: crate::parser::Size,
    /// BrushId (4 bytes): An unsigned integer that specifies either
    /// the foreground color of the text or a graphics brush, depending
    /// on the value of the S flag in the Flags.
    pub brush_id: EmfPlusBrushIdOrColor,
    /// DriverStringOptionsFlags (4 bytes): An unsigned integer that
    /// specifies the spacing, orientation, and quality of rendering
    /// for the string. This value MUST be composed of
    /// DriverStringOptions flags.
    pub driver_string_options_flags:
        crate::parser::emf_plus::DriverStringOptionsFlags,
    /// MatrixPresent (4 bytes): An unsigned integer that specifies
    /// whether a transform matrix is present in the TransformMatrix
    /// field.
    ///
    /// | Value | Meaning |
    /// |:-|:-|
    /// | `0x00000000` | The transform matrix is not present in the record. |
    /// | `0x00000001` | The transform matrix is present in the record. |
    pub matrix_present: bool,
    /// GlyphCount (4 bytes): An unsigned integer that specifies number
    /// of glyphs in the string.
    pub glyph_count: u32,
    /// Glyphs (variable): An array of 16-bit values that define the
    /// text string to draw.
    ///
    /// If the DriverStringOptionsCmapLookup flag in the
    /// DriverStringOptionsFlags field is set, each value in this array
    /// specifies a Unicode character. Otherwise, each value specifies
    /// an index to a character glyph in the EmfPlusFont object
    /// specified by the ObjectId value in Flags field.
    pub glyphs: Vec<u16>,
    /// GlyphPos (variable): An array of EmfPlusPointF objects that
    /// specify the output position of each character glyph. There MUST
    /// be GlyphCount elements, which have a one-to-one correspondence
    /// with the elements in the Glyphs array.
    ///
    /// Glyph positions are calculated from the position of the first
    /// glyph if the DriverStringOptionsRealizedAdvance flag in
    /// DriverStringOptions flags is set. In this case, GlyphPos
    /// specifies the position of the first glyph only.
    pub glyph_pos: Vec<crate::parser::emf_plus::objects::EmfPlusPointF>,
    /// TransformMatrix (24 bytes): An optional EmfPlusTransformMatrix
    /// object that specifies the transformation to apply to each value
    /// in the text array. The presence of this data is determined from
    /// the MatrixPresent field.
    pub transform_matrix:
        Option<crate::parser::emf_plus::objects::EmfPlusTransformMatrix>,
}

impl EmfPlusDrawDriverString {
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
            crate::parser::emf_plus::RecordType::EmfPlusDrawDriverString as u16,
        )?;

        let brush_id =
            EmfPlusBrushIdOrColor::parse(buf, &mut data_size, flags)?;
        let driver_string_options_flags = read_with(
            buf,
            &mut data_size,
            crate::parser::emf_plus::DriverStringOptionsFlags::parse,
        )?;
        let matrix_present: u32 = read_field(buf, &mut data_size)?;
        let glyph_count = read_field(buf, &mut data_size)?;

        crate::parser::emf_plus::check_element_count(
            "GlyphCount",
            glyph_count,
        )?;

        // The declared glyph count must fit in the record data that
        // actually follows (2 bytes per glyph plus 8 bytes per glyph
        // position); checking before the reads keeps a crafted count
        // field from consuming bytes past the record data, mirroring
        // the string length check of EmfPlusDrawString.
        crate::parser::ParseError::expect_le(
            "glyph data (bytes)",
            u64::from(glyph_count) * 10,
            data_size.remaining_bytes() as u64,
        )?;

        let mut glyphs = vec![];
        for _ in 0..glyph_count {
            glyphs.push(read_field(buf, &mut data_size)?);
        }

        let mut glyph_pos = vec![];
        for _ in 0..glyph_count {
            glyph_pos.push(read_with(
                buf,
                &mut data_size,
                crate::parser::emf_plus::objects::EmfPlusPointF::parse,
            )?);
        }

        let transform_matrix = if matrix_present != 0 {
            Some(read_with(
                buf,
                &mut data_size,
                crate::parser::emf_plus::objects::EmfPlusTransformMatrix::parse,
            )?)
        } else {
            None
        };

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
            driver_string_options_flags,
            matrix_present: matrix_present != 0,
            glyph_count,
            glyphs,
            glyph_pos,
            transform_matrix,
        })
    }

    /// Decodes the glyphs as text. Only meaningful when the
    /// CmapLookup option flag is set; otherwise the glyphs are font
    /// glyph indexes, not characters.
    pub fn text(&self) -> String {
        String::from_utf16_lossy(&self.glyphs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::{
        DriverStringOptionsFlags,
        objects::{EmfPlusARGB, EmfPlusPointF},
    };

    #[test]
    fn parses_glyphs_and_positions_and_stores_the_header() {
        let mut data = vec![];
        data.extend([0x11, 0x22, 0x33, 0x44]); // BrushId as a color
        data.extend(1_u32.to_le_bytes()); // CmapLookup
        data.extend(0_u32.to_le_bytes()); // MatrixPresent
        data.extend(2_u32.to_le_bytes()); // GlyphCount
        for c in "Hi".encode_utf16() {
            data.extend(c.to_le_bytes());
        }
        for v in [1.0_f32, 2.0, 3.0, 4.0] {
            data.extend(v.to_le_bytes());
        }

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusDrawDriverString::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusDrawDriverString,
            0x8007,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(record.record_type as u16, 0x4036);
        assert_eq!(record.flags, 0x8007);
        assert_eq!(record.object_id, 7);
        assert_eq!(record.size, 0x0000_0030);
        assert_eq!(
            record.brush_id,
            EmfPlusBrushIdOrColor::Color(EmfPlusARGB {
                blue: 0x11,
                green: 0x22,
                red: 0x33,
                alpha: 0x44,
            }),
        );
        assert!(
            record
                .driver_string_options_flags
                .contains(DriverStringOptionsFlags::CMAP_LOOKUP)
        );
        assert!(!record.matrix_present);
        assert_eq!(record.glyph_count, 2);
        assert_eq!(record.glyphs, vec![0x48, 0x69]);
        assert_eq!(record.glyph_pos, vec![
            EmfPlusPointF { x: 1.0, y: 2.0 },
            EmfPlusPointF { x: 3.0, y: 4.0 },
        ]);
        assert!(record.transform_matrix.is_none());
        assert_eq!(record.text(), "Hi");
    }
}
