/// The SmoothingMode enumeration defines smoothing modes to apply to
/// lines, curves, and the edges of filled areas to make them appear more
/// continuous or sharply defined (MS-EMFPLUS 2.1.1.28).
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
pub enum SmoothingMode {
    /// Default curve smoothing with no anti-aliasing.
    SmoothingModeDefault = 0x00000000,
    /// Best performance with no anti-aliasing.
    SmoothingModeHighSpeed = 0x00000001,
    /// Best quality with anti-aliasing.
    SmoothingModeHighQuality = 0x00000002,
    /// No curve smoothing and no anti-aliasing.
    SmoothingModeNone = 0x00000003,
    /// Anti-aliasing using an 8x4 box filter.
    SmoothingModeAntiAlias8x4 = 0x00000004,
    /// Anti-aliasing using an 8x8 box filter.
    SmoothingModeAntiAlias8x8 = 0x00000005,
}

crate::parser::enums::impl_parser!(SmoothingMode, u32);
