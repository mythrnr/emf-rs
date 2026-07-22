/// The TextRenderingHint enumeration defines types of text hinting and
/// anti-aliasing, which affects the quality of text rendering
/// (MS-EMFPLUS 2.1.1.32).
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
pub enum TextRenderingHint {
    /// Each text character SHOULD be drawn using whatever font-smoothing
    /// settings have been configured on the operating system.
    TextRenderingHintSystemDefault = 0x00000000,
    /// Each text character SHOULD be drawn using its glyph bitmap.
    /// Smoothing MAY be used to improve the appearance of character
    /// glyph stems and curvature.
    TextRenderingHintSingleBitPerPixelGridFit = 0x00000001,
    /// Each text character SHOULD be drawn using its glyph bitmap.
    /// Smoothing is not used.
    TextRenderingHintSingleBitPerPixel = 0x00000002,
    /// Each text character SHOULD be drawn using its anti-aliased glyph
    /// bitmap with smoothing. The rendering is high quality because of
    /// anti-aliasing, but at a higher performance cost.
    TextRenderingHintAntialiasGridFit = 0x00000003,
    /// Each text character is drawn using its anti-aliased glyph bitmap
    /// without hinting. Better quality results from anti-aliasing, but
    /// stem width differences MAY be noticeable because hinting is
    /// turned off.
    TextRenderingHintAntialias = 0x00000004,
    /// Each text character SHOULD be drawn using its ClearType glyph
    /// bitmap with smoothing. This is the highest-quality text hinting
    /// setting, which is used to take advantage of ClearType font
    /// features.
    TextRenderingHintClearTypeGridFit = 0x00000005,
}

crate::parser::enums::impl_parser!(TextRenderingHint, u32);
