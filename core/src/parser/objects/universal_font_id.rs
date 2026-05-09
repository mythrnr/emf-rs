/// The UniversalFontId object defines a mechanism for identifying fonts in EMF
/// metafiles.
#[derive(Clone, Debug)]
pub struct UniversalFontId {
    /// Checksum (4 bytes): An unsigned integer that is the checksum of the
    /// font. The checksum value has the following meanings.
    ///
    /// | Value | Meaning |
    /// |:-|:-|
    /// | `0x00000000` | The object is a device font. |
    /// | `0x00000001` | The object is a Type 1 font that has been installed on the client machine and is enumerated by the PostScript printer driver as a device font. |
    /// | `0x00000002` | The object is not a font but is a Type 1 rasterizer. |
    /// | `3 ≤ value` | The object is a bitmap, vector, or TrueType font, or a Type 1 rasterized font that was created by a Type 1 rasterizer. A checksum value SHOULD be computed for the font and compared to the value in this field. If it matches, it is considered to be the same as the font referenced by this metafile record. If it does not match, the system font mapper MAY use a default mechanism to select a back-up font. In this case, Windows uses the logical font that is currently selected in the playback device context.|
    pub checksum: u32,
    /// Index (4 bytes): An unsigned integer that is an index associated with
    /// the font object. The meaning of this field is determined by the type of
    /// font.
    pub index: i32,
}

impl UniversalFontId {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::read_field;

        let mut consumed_bytes: usize = 0;
        let checksum = read_field(buf, &mut consumed_bytes)?;
        let index = read_field(buf, &mut consumed_bytes)?;

        Ok((Self { checksum, index }, consumed_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_consumes_eight_bytes() {
        // checksum = 3 (TrueType-like), index = -1
        let bytes: [u8; 8] = [0x03, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF];
        let mut buf: &[u8] = &bytes;
        let (font_id, consumed) = UniversalFontId::parse(&mut buf).unwrap();
        assert_eq!(font_id.checksum, 3);
        assert_eq!(font_id.index, -1);
        assert_eq!(consumed, 8);
    }

    #[test]
    fn parse_short_buffer_errors() {
        let bytes: [u8; 4] = [0x00; 4];
        let mut buf: &[u8] = &bytes;
        assert!(UniversalFontId::parse(&mut buf).is_err());
    }
}
