/// The LogFontPanose object specifies the PANOSE characteristics of a logical
/// font.
#[derive(Clone, Debug)]
pub struct LogFontPanose {
    /// LogFont (92 bytes): A LogFont object that specifies the basic
    /// attributes of the logical font.
    pub log_font: crate::parser::LogFont,
    /// FullName (128 bytes): A string of 64 Unicode characters that defines
    /// the font's full name. If the length of this string is less than 64
    /// characters, a terminating NULL MUST be present, after which the
    /// remainder of this field MUST be ignored.
    pub full_name: String,
    /// Style (64 bytes): A string of 32 Unicode characters that defines the
    /// font's style. If the length of this string is less than 32 characters,
    /// a terminating NULL MUST be present, after which the remainder of this
    /// field MUST be ignored.
    pub style: String,
    /// Version (4 bytes): This field MUST be ignored.
    pub version: [u8; 4],
    /// StyleSize (4 bytes): An unsigned integer that specifies the point size
    /// at which font hinting is performed. If set to zero, font hinting is
    /// performed at the point size corresponding to the Height field in the
    /// LogFont object in the LogFont field.
    pub style_size: u32,
    /// Match (4 bytes): This field MUST be ignored.
    pub _match: [u8; 4],
    /// Reserved (4 bytes): An unsigned integer that MUST be set to zero and
    /// MUST be ignored.
    pub reserved: u32,
    /// VendorId (4 bytes): This field MUST be ignored.
    pub vendor_id: [u8; 4],
    /// Culture (4 bytes): An unsigned integer that MUST be set to zero and
    /// MUST be ignored.
    pub culture: u32,
    /// Panose (10 bytes): A Panose object that specifies the PANOSE
    /// characteristics of the logical font. If all fields of this object are
    /// zero, it MUST be ignored.
    pub panose: crate::parser::Panose,
    /// Padding (2 bytes): A field that exists only to ensure 32-bit alignment
    /// of this structure. It MUST be ignored.
    pub padding: [u8; 2],
}

impl LogFontPanose {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (log_font, log_font_bytes),
            (full_name, full_name_bytes),
            (style, style_bytes),
            (version, version_bytes),
            (style_size, style_size_bytes),
            (_match, _match_bytes),
            (reserved, reserved_bytes),
            (vendor_id, vendor_id_bytes),
            (culture, culture_bytes),
            (panose, panose_bytes),
            (padding, padding_bytes),
        ) = (
            crate::parser::LogFont::parse(buf)?,
            crate::parser::read::<_, 128>(buf)?,
            crate::parser::read::<_, 64>(buf)?,
            crate::parser::read::<_, 4>(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read::<_, 4>(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read::<_, 4>(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::Panose::parse(buf)?,
            crate::parser::read::<_, 2>(buf)?,
        );

        let (full_name, style) = (
            crate::parser::utf16le_bytes_to_string(&full_name)?,
            crate::parser::utf16le_bytes_to_string(&style)?,
        );

        Ok((
            Self {
                log_font,
                full_name,
                style,
                version,
                style_size,
                _match,
                reserved,
                vendor_id,
                culture,
                panose,
                padding,
            },
            log_font_bytes
                + full_name_bytes
                + style_bytes
                + version_bytes
                + style_size_bytes
                + _match_bytes
                + reserved_bytes
                + vendor_id_bytes
                + culture_bytes
                + panose_bytes
                + padding_bytes,
        ))
    }
}
