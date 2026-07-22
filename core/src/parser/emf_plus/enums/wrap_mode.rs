/// The WrapMode enumeration defines how the pattern from a texture or
/// gradient brush is tiled across a shape or at shape boundaries, when
/// it is smaller than the area being filled (MS-EMFPLUS 2.1.1.34).
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
pub enum WrapMode {
    /// Tiles the gradient or texture.
    WrapModeTile = 0x00000000,
    /// Reverses the texture or gradient horizontally, and then tiles
    /// the texture or gradient.
    WrapModeTileFlipX = 0x00000001,
    /// Reverses the texture or gradient vertically, and then tiles the
    /// texture or gradient.
    WrapModeTileFlipY = 0x00000002,
    /// Reverses the texture or gradient horizontally and vertically,
    /// and then tiles the texture or gradient.
    WrapModeTileFlipXY = 0x00000003,
    /// Fixes the texture or gradient to the object boundary.
    WrapModeClamp = 0x00000004,
}

crate::parser::enums::impl_parser!(WrapMode, u32);
