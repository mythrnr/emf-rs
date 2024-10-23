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
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let ((checksum, checksum_bytes), (index, index_bytes)) = (
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
        );

        Ok((Self { checksum, index }, checksum_bytes + index_bytes))
    }
}
