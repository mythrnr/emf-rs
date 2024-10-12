/// The LogFontEx object specifies the extended attributes of a logical font.
#[derive(Clone, Debug)]
pub struct LogFontEx {
    /// LogFont (92 bytes): A LogFont object that specifies the basic
    /// attributes of the logical font.
    pub log_font: crate::parser::LogFont,
    /// FullName (128 bytes): A string of 64 Unicode characters that contains
    /// the font's full name. If the length of this string is less than 64
    /// characters, a terminating NULL MUST be present, after which the
    /// remainder of this field MUST be ignored.
    pub full_name: String,
    /// Style (64 bytes): A string of 32 Unicode characters that defines the
    /// font's style. If the length of this string is less than 32 characters,
    /// a terminating NULL MUST be present, after which the remainder of this
    /// field MUST be ignored.
    pub style: String,
    /// Script (64 bytes): A string of 32 Unicode characters that defines the
    /// character set of the font. If the length of this string is less than 32
    /// characters, a terminating NULL MUST be present, after which the
    /// remainder of this field MUST be ignored.
    pub script: String,
}

impl LogFontEx {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (log_font, log_font_bytes),
            (full_name, full_name_bytes),
            (style, style_bytes),
            (script, script_bytes),
        ) = (
            crate::parser::LogFont::parse(buf)?,
            crate::parser::read::<_, 128>(buf)?,
            crate::parser::read::<_, 64>(buf)?,
            crate::parser::read::<_, 64>(buf)?,
        );

        let (full_name, style, script) = (
            crate::parser::utf16le_bytes_to_string(&full_name)?,
            crate::parser::utf16le_bytes_to_string(&style)?,
            crate::parser::utf16le_bytes_to_string(&script)?,
        );

        Ok((
            Self { log_font, full_name, style, script },
            log_font_bytes + full_name_bytes + style_bytes + script_bytes,
        ))
    }
}