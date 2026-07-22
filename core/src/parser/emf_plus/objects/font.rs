use crate::{imports::*, parser::emf_plus::objects::EmfPlusGraphicsVersion};

/// The EmfPlusFont object specifies properties that determine the
/// appearance of text, including typeface, size, and style
/// (MS-EMFPLUS 2.2.1.3).
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusFont {
    /// Version (4 bytes): An EmfPlusGraphicsVersion object (section
    /// 2.2.2.19) that specifies the version of operating system
    /// graphics that was used to create this object.
    pub version: EmfPlusGraphicsVersion,
    /// EmSize (4 bytes): A floating-point value that specifies the em
    /// size of the font in units specified by the SizeUnit field.
    pub em_size: f32,
    /// SizeUnit (4 bytes): An unsigned integer that specifies the
    /// units used for the EmSize field. These are typically the units
    /// that were employed when designing the font. The value is in the
    /// UnitType enumeration (section 2.1.1.32).
    ///
    /// UnitTypeDisplay and UnitTypeWorld are disallowed for fonts by
    /// the specification, but the value is kept as parsed.
    pub size_unit: crate::parser::emf_plus::UnitType,
    /// FontStyleFlags (4 bytes): A signed integer that specifies
    /// attributes of the character glyphs that affect the appearance
    /// of the font, such as bold and italic. This value is composed of
    /// FontStyle flags (section 2.1.2.4).
    pub font_style_flags: crate::parser::emf_plus::FontStyleFlags,
    /// Reserved (4 bytes): An unsigned integer that is reserved and
    /// MUST be ignored.
    pub reserved: u32,
    /// FamilyName (variable): A string of Length Unicode characters
    /// that contains the name of the font family.
    ///
    /// Decoded from UTF-16LE; the Length field of the wire format is
    /// consumed at parse time and not stored.
    pub family_name: String,
}

impl EmfPlusFont {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let version =
            read_with(buf, &mut consumed_bytes, EmfPlusGraphicsVersion::parse)?;
        let em_size = read_field(buf, &mut consumed_bytes)?;
        let size_unit = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::UnitType::parse,
        )?;
        let font_style_flags = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::FontStyleFlags::parse,
        )?;
        let reserved: u32 = read_field(buf, &mut consumed_bytes)?;
        let length: u32 = read_field(buf, &mut consumed_bytes)?;
        let family_name = crate::parser::emf_plus::read_utf16_field(
            buf,
            &mut consumed_bytes,
            length,
            "font family name length",
        )?;

        Ok((
            Self {
                version,
                em_size,
                size_unit,
                font_style_flags,
                reserved,
                family_name,
            },
            consumed_bytes,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_family_name_as_utf16() {
        let mut data = vec![];
        data.extend(0xDBC0_1002_u32.to_le_bytes());
        data.extend(12.0_f32.to_le_bytes());
        data.extend(3_u32.to_le_bytes()); // UnitTypePoint
        data.extend(1_u32.to_le_bytes()); // Bold
        data.extend(0_u32.to_le_bytes());
        data.extend(5_u32.to_le_bytes());
        for c in "Arial".encode_utf16() {
            data.extend(c.to_le_bytes());
        }

        let mut buf: &[u8] = &data;
        let (font, consumed) = EmfPlusFont::parse(&mut buf).unwrap();

        assert_eq!(consumed, data.len());
        assert_eq!(font.family_name, "Arial");
        assert!(
            font.font_style_flags
                .contains(crate::parser::emf_plus::FontStyleFlags::BOLD)
        );
    }
}
