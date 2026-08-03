/// The CompositingMode enumeration defines modes for combining source
/// colors with background colors (MS-EMFPLUS 2.1.1.5). The compositing
/// mode represents the enable state of alpha blending.
///
/// Graphics colors are specified by EmfPlusARGB objects.
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
pub enum CompositingMode {
    /// Enables alpha blending, which specifies that when a color is
    /// rendered, it is blended with the background color. The extent of
    /// blending is determined by the value of the alpha component of the
    /// color being rendered.
    CompositingModeSourceOver = 0x00000000,
    /// Disables alpha blending, which means that when a source color is
    /// rendered, it overwrites the background color.
    CompositingModeSourceCopy = 0x00000001,
}

crate::parser::enums::impl_parser!(CompositingMode, u32);
