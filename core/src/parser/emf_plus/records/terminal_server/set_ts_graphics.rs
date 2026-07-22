use crate::parser::emf_plus::objects::{
    EmfPlusPalette, EmfPlusTransformMatrix,
};

/// The EmfPlusSetTSGraphics record specifies the state of a graphics
/// device context for a terminal server.
#[derive(Clone, Debug)]
pub struct EmfPlusSetTSGraphics {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusSetTSGraphics from the RecordType enumeration.
    /// The value MUST be 0x4039.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    ///
    /// T (1 bit): If set, this record contains an EmfPlusPalette
    /// object in the Palette field following the graphics state data.
    /// The T bit is the 0x0001 bit of the record flags.
    pub flags: u16,
    /// V (1 bit): If set, the palette contains only the basic VGA
    /// colors.
    ///
    /// The V bit is the 0x0002 bit of the record flags.
    pub vga_palette: bool,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned size of the entire record in bytes, including
    /// the 12-byte record header and record-specific data. This value
    /// MUST be 0x00000030 plus the size of the Palette field.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. This value MUST be 0x00000024 plus the size of the
    /// Palette field.
    pub data_size: crate::parser::Size,
    /// AntiAliasMode (1 byte): An unsigned integer that specifies the
    /// quality of line rendering, including the type of line
    /// anti-aliasing. It is defined in the SmoothingMode enumeration.
    pub anti_alias_mode: crate::parser::emf_plus::SmoothingMode,
    /// TextRenderHint (1 byte): An unsigned integer that specifies the
    /// quality of text rendering, including the type of text
    /// anti-aliasing. It is defined in the TextRenderingHint
    /// enumeration.
    pub text_render_hint: crate::parser::emf_plus::TextRenderingHint,
    /// CompositingMode (1 byte): An unsigned integer that specifies
    /// how source colors are combined with background colors. It MUST
    /// be a value in the CompositingMode enumeration.
    pub compositing_mode: crate::parser::emf_plus::CompositingMode,
    /// CompositingQuality (1 byte): An unsigned integer that specifies
    /// the degree of smoothing to apply to lines, curves and the edges
    /// of filled areas to make them appear more continuous or sharply
    /// defined. It MUST be a value in the CompositingQuality
    /// enumeration.
    pub compositing_quality: crate::parser::emf_plus::CompositingQuality,
    /// RenderOriginX (2 bytes): A signed integer, which is the
    /// horizontal coordinate of the origin for rendering halftoning
    /// and dithering matrixes.
    pub render_origin_x: i16,
    /// RenderOriginY (2 bytes): A signed integer, which is the
    /// vertical coordinate of the origin for rendering halftoning and
    /// dithering matrixes.
    pub render_origin_y: i16,
    /// TextContrast (2 bytes): An unsigned integer that specifies the
    /// gamma correction value used for rendering anti-aliased and
    /// ClearType text. This value MUST be in the range 0 to 12,
    /// inclusive.
    pub text_contrast: u16,
    /// FilterType (1 byte): An unsigned integer that specifies how
    /// scaling, including stretching and shrinking, is performed. It
    /// MUST be a value in the FilterType enumeration.
    pub filter_type: crate::parser::emf_plus::FilterType,
    /// PixelOffset (1 byte): An unsigned integer that specifies the
    /// overall quality of the image and text-rendering process. It
    /// MUST be a value in the PixelOffsetMode enumeration.
    pub pixel_offset: crate::parser::emf_plus::PixelOffsetMode,
    /// WorldToDevice (24 bytes): An 192-bit EmfPlusTransformMatrix
    /// object that specifies the world space to device space
    /// transforms.
    pub world_to_device: EmfPlusTransformMatrix,
    /// Palette (variable): An optional EmfPlusPalette object.
    ///
    /// Present when the T bit (0x0001) of the record flags is set.
    pub palette: Option<EmfPlusPalette>,
}

impl EmfPlusSetTSGraphics {
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
            crate::parser::emf_plus::RecordType::EmfPlusSetTSGraphics as u16,
        )?;

        let anti_alias_mode = enum_from_byte(
            buf,
            &mut data_size,
            crate::parser::emf_plus::SmoothingMode::from_repr,
            "SmoothingMode",
        )?;
        let text_render_hint = enum_from_byte(
            buf,
            &mut data_size,
            crate::parser::emf_plus::TextRenderingHint::from_repr,
            "TextRenderingHint",
        )?;
        let compositing_mode = enum_from_byte(
            buf,
            &mut data_size,
            crate::parser::emf_plus::CompositingMode::from_repr,
            "CompositingMode",
        )?;
        let compositing_quality = enum_from_byte(
            buf,
            &mut data_size,
            crate::parser::emf_plus::CompositingQuality::from_repr,
            "CompositingQuality",
        )?;
        let render_origin_x: i16 = read_field(buf, &mut data_size)?;
        let render_origin_y: i16 = read_field(buf, &mut data_size)?;
        let text_contrast: u16 = read_field(buf, &mut data_size)?;

        crate::parser::ParseError::expect_le(
            "TextContrast",
            text_contrast,
            12,
        )?;

        let filter_type = enum_from_byte(
            buf,
            &mut data_size,
            crate::parser::emf_plus::FilterType::from_repr,
            "FilterType",
        )?;
        let pixel_offset = enum_from_byte(
            buf,
            &mut data_size,
            crate::parser::emf_plus::PixelOffsetMode::from_repr,
            "PixelOffsetMode",
        )?;
        let world_to_device =
            read_with(buf, &mut data_size, EmfPlusTransformMatrix::parse)?;

        let palette = if flags & 0x0001 != 0 {
            Some(read_with(buf, &mut data_size, EmfPlusPalette::parse)?)
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
            vga_palette: flags & 0x0002 != 0,
            size,
            data_size,
            anti_alias_mode,
            text_render_hint,
            compositing_mode,
            compositing_quality,
            render_origin_x,
            render_origin_y,
            text_contrast,
            filter_type,
            pixel_offset,
            world_to_device,
            palette,
        })
    }
}

/// Reads a 1-byte field that carries a 32-bit enumeration value.
fn enum_from_byte<R: crate::Read, T>(
    buf: &mut R,
    data_size: &mut crate::parser::Size,
    from_repr: impl FnOnce(u32) -> Option<T>,
    name: &'static str,
) -> Result<T, crate::parser::ParseError> {
    use crate::parser::records::read_field;

    let raw: u8 = read_field(buf, data_size)?;

    from_repr(u32::from(raw)).ok_or_else(|| {
        crate::parser::ParseError::UnexpectedEnumValue {
            cause: alloc::format!("unexpected value as {name}: {raw:#04X}")
                .into(),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imports::*;

    fn graphics_state_bytes() -> Vec<u8> {
        // SmoothingModeHighQuality, TextRenderingHintAntialias,
        // CompositingModeSourceOver, CompositingQualityHighSpeed.
        let mut data = vec![0x02, 0x04, 0x00, 0x02];
        data.extend(16_i16.to_le_bytes());
        data.extend(32_i16.to_le_bytes());
        data.extend(4_u16.to_le_bytes());
        data.push(0x01); // FilterTypePoint
        data.push(0x03); // PixelOffsetModeNone
        for v in [1.0_f32, 0.0, 0.0, 1.0, 0.0, 0.0] {
            data.extend(v.to_le_bytes());
        }
        data
    }

    #[test]
    fn parses_graphics_state_without_palette() {
        let data = graphics_state_bytes();
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusSetTSGraphics::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusSetTSGraphics,
            0x0002,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusSetTSGraphics,
        );
        assert_eq!(record.flags, 0x0002);
        assert_eq!(record.size, 0x0000_0030);
        assert!(record.vga_palette);
        assert_eq!(
            record.anti_alias_mode,
            crate::parser::emf_plus::SmoothingMode::SmoothingModeHighQuality,
        );
        assert_eq!(record.render_origin_x, 16);
        assert_eq!(record.render_origin_y, 32);
        assert_eq!(record.text_contrast, 4);
        assert_eq!(record.world_to_device.m11.to_bits(), 1.0_f32.to_bits());
        assert!(record.palette.is_none());
    }

    #[test]
    fn parses_the_palette_when_the_t_bit_is_set() {
        let mut data = graphics_state_bytes();
        data.extend(0_u32.to_le_bytes()); // PaletteStyleFlags
        data.extend(1_u32.to_le_bytes()); // PaletteCount
        data.extend([0x11, 0x22, 0x33, 0x44]); // one ARGB entry

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusSetTSGraphics::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusSetTSGraphics,
            0x0001,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert!(!record.vga_palette);
        assert_eq!(record.palette.unwrap().palette_entries.len(), 1);
    }

    #[test]
    fn rejects_text_contrast_out_of_range() {
        let mut data = graphics_state_bytes();
        // TextContrast sits after 4 enum bytes and 2 i16 fields.
        data[8..10].copy_from_slice(&13_u16.to_le_bytes());

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);

        assert!(
            EmfPlusSetTSGraphics::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusSetTSGraphics,
                0x0000,
                u32::try_from(data.len()).unwrap() + 12,
                data_size,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_wrong_record_type() {
        let data = graphics_state_bytes();
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);

        assert!(
            EmfPlusSetTSGraphics::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusSetTSClip,
                0x0000,
                u32::try_from(data.len()).unwrap() + 12,
                data_size,
            )
            .is_err()
        );
    }
}
