use crate::imports::*;

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
    _match: [u8; 4],
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
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{
            read_array_field, read_bytes_field, read_field, read_with,
        };

        let mut consumed_bytes: usize = 0;
        let log_font =
            read_with(buf, &mut consumed_bytes, crate::parser::LogFont::parse)?;

        // FullName / Style stay as Vec<u8> because they feed directly
        // into `utf16le_bytes_to_string`, which takes a slice.
        let full_name_bytes = read_bytes_field(buf, &mut consumed_bytes, 128)?;
        let style_bytes = read_bytes_field(buf, &mut consumed_bytes, 64)?;
        let version: [u8; 4] = read_array_field(buf, &mut consumed_bytes)?;
        let style_size = read_field(buf, &mut consumed_bytes)?;
        let _match: [u8; 4] = read_array_field(buf, &mut consumed_bytes)?;
        let reserved = read_field(buf, &mut consumed_bytes)?;
        let vendor_id: [u8; 4] = read_array_field(buf, &mut consumed_bytes)?;
        let culture = read_field(buf, &mut consumed_bytes)?;
        let panose =
            read_with(buf, &mut consumed_bytes, crate::parser::Panose::parse)?;
        let padding: [u8; 2] = read_array_field(buf, &mut consumed_bytes)?;

        let full_name =
            crate::parser::utf16le_bytes_to_string(&full_name_bytes)?;
        let style = crate::parser::utf16le_bytes_to_string(&style_bytes)?;

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
            consumed_bytes,
        ))
    }
}
