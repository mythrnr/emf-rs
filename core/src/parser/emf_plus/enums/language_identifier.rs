/// The EmfPlusLanguageIdentifier object specifies a language code
/// identifier (LCID) that corresponds to the natural language in a
/// locale, including countries, geographical regions, and administrative
/// districts (MS-EMFPLUS 2.2.2.23). Each object is an encoding of a
/// primary language and sublanguage identifier.
///
/// SubLanguageId (6 bits): The country, geographic region or
/// administrative district for the natural language specified in the
/// PrimaryLanguageId field. Sublanguage identifiers are vendor
/// extensible. Vendor-defined sublanguage identifiers MUST be in the
/// range 0x20 to 0x3F, inclusive.
///
/// PrimaryLanguageId (10 bits): The natural language. Primary language
/// identifiers are vendor extensible. Vendor-defined primary language
/// identifiers MUST be in the range 0x0200 to 0x03FF, inclusive.
///
/// The encoded LCID values are defined in [MS-LCID] section 2.2.
///
/// The full enumeration defines several hundred locale values, and the
/// field is informational for rendering, so the raw value is kept
/// instead of enumerating every variant. The primary language and
/// sublanguage accessors decompose the value as specified in
/// [MS-LCID].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LanguageIdentifier(u32);

impl LanguageIdentifier {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (value, consumed_bytes) =
            <u32 as crate::parser::ReadLeField>::read_le(buf)?;

        Ok((Self(value), consumed_bytes))
    }

    /// The raw 32-bit value as read from the stream.
    pub fn raw(self) -> u32 {
        self.0
    }

    /// The primary language identifier (low 10 bits).
    pub fn primary_language_id(self) -> u16 {
        (self.0 & 0x03FF) as u16
    }

    /// The sublanguage identifier (bits 10-15).
    pub fn sublanguage_id(self) -> u16 {
        ((self.0 >> 10) & 0x003F) as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decomposes_lang_id() {
        // ja-JP: primary 0x11, sublanguage 0x01.
        let v = LanguageIdentifier(0x0411);
        assert_eq!(v.primary_language_id(), 0x11);
        assert_eq!(v.sublanguage_id(), 0x01);
    }
}
