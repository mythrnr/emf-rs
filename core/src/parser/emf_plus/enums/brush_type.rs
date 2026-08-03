/// The BrushType enumeration defines types of graphics brushes, which
/// are used to fill graphics regions (MS-EMFPLUS 2.1.1.3).
///
/// Graphics brushes are specified by EmfPlusBrush objects.
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
pub enum BrushType {
    /// A solid-color brush, which is characterized by an EmfPlusARGB
    /// value.
    BrushTypeSolidColor = 0x00000000,
    /// A hatch brush, which is characterized by a predefined pattern.
    BrushTypeHatchFill = 0x00000001,
    /// A texture brush, which is characterized by an image.
    BrushTypeTextureFill = 0x00000002,
    /// A path gradient brush, which is characterized by a color gradient
    /// path gradient brush data.
    BrushTypePathGradient = 0x00000003,
    /// BrushData contains linear gradient brush data.
    BrushTypeLinearGradient = 0x00000004,
}

crate::parser::enums::impl_parser!(BrushType, u32);
