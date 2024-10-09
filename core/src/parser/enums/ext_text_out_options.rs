/// The ExtTextOutOptions enumeration specifies parameters that control various
/// aspects of the output of text by EMR_SMALLTEXTOUT records and in EmrText
/// objects.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    strum::FromRepr,
    strum::EnumIter,
)]
#[repr(u32)]
pub enum ExtTextOutOptions {
    /// This bit indicates that the current background color SHOULD be used to
    /// fill the rectangle.
    ETO_OPAQUE = 0x00000002,
    /// This bit indicates that the text SHOULD be clipped to the rectangle.
    ETO_CLIPPED = 0x00000004,
    /// This bit indicates that the codes for characters in an output text
    /// string are indexes of the character glyphs in a TrueType font. Glyph
    /// indexes are font-specific, so to display the correct characters on
    /// playback, the font that is used MUST be identical to the font used to
    /// generate the indexes.
    ///
    /// Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, Windows 98, and
    /// Windows Millennium Edition: Do not support the ETO_GLYPH_INDEX flag
    /// used for bitmap and vector fonts—in addition to TrueType fonts—to
    /// indicate that no further language processing is necessary and that GDI
    /// processes the string directly. See [MSDN-GDI+] for more information.
    ETO_GLYPH_INDEX = 0x00000010,
    /// This bit indicates that the text MUST be laid out in right-to-left
    /// reading order, instead of the default left-to-right order. This SHOULD
    /// be applied only when the font selected into the playback device context
    /// is either Hebrew or Arabic.
    ///
    /// Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, Windows 98, and
    /// Windows Millennium Edition: Do not support the ETO_RTLREADING flag used
    /// to indicate right-to-left reading order.
    ETO_RTLREADING = 0x00000080,
    /// This bit indicates that the record does not specify a bounding
    /// rectangle for the text output.
    ETO_NO_RECT = 0x00000100,
    /// This bit indicates that the codes for characters in an output text
    /// string are 8 bits, derived from the low bytes of Unicode UTF16-LE
    /// character codes, in which the high byte is assumed to be 0.
    ETO_SMALL_CHARS = 0x00000200,
    /// This bit indicates that to display numbers, digits appropriate to the
    /// locale SHOULD be used.
    ///
    /// Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, Windows 98, and
    /// Windows Millennium Edition: Do not support the ETO_NUMERICSLOCAL flag
    /// used to indicate the display of numeric digits appropriate to the
    /// locale.
    ETO_NUMERICSLOCAL = 0x00000400,
    /// This bit indicates that to display numbers, European digits SHOULD be
    /// used.
    ///
    /// Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, Windows 98, and
    /// Windows Millennium Edition: Do not support the ETO_NUMERICSLATIN flag
    /// used to indicate the display of numeric digits appropriate to Europe.
    ETO_NUMERICSLATIN = 0x00000800,
    /// This bit indicates that no special operating system processing for
    /// glyph placement is performed on right-to-left strings; that is, all
    /// glyph positioning SHOULD be taken care of by drawing and state records
    /// in the metafile.
    ///
    /// Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, Windows 98, and
    /// Windows Millennium Edition: Do not support the ETO_IGNORELANGUAGE flag
    /// used to indicate that international scripting support is not used,
    /// which might cause no text to be output.
    ETO_IGNORELANGUAGE = 0x00001000,
    /// This bit indicates that both horizontal and vertical character
    /// displacement values SHOULD be provided.
    ///
    /// Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, Windows 98, Windows
    /// Millennium Edition, Windows NT 4.0, and Windows 2000: Do not support
    /// the ETO_PDY flag used to indicate that both horizontal and vertical
    /// character displacement values are provided.
    ETO_PDY = 0x00002000,
    /// This bit is reserved and SHOULD NOT be used.
    ///
    /// Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, Windows 98, Windows
    /// Millennium Edition, Windows NT 4.0, and Windows 2000: Do not support
    /// the ETO_REVERSE_INDEX_MAP flag.
    ETO_REVERSE_INDEX_MAP = 0x00010000,
}

crate::parser::enums::impl_parser!(ExtTextOutOptions, u32);
