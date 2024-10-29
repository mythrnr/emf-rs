/// The HatchStyle enumeration is an extension to the WMF HatchStyle
/// enumeration.
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
pub enum HatchStyle {
    /// A horizontal hatch.
    HS_HORIZONTAL = 0x0000,
    /// A vertical hatch.
    HS_VERTICAL = 0x0001,
    /// A 45-degree downward, left-to-right hatch.
    HS_FDIAGONAL = 0x0002,
    /// A 45-degree upward, left-to-right hatch.
    HS_BDIAGONAL = 0x0003,
    /// A horizontal and vertical cross-hatch.
    HS_CROSS = 0x0004,
    /// A 45-degree crosshatch.
    HS_DIAGCROSS = 0x0005,
    /// The hatch is not a pattern, but is a solid color.
    HS_SOLIDCLR = 0x0006,
    /// The hatch is not a pattern, but is a dithered color.
    HS_DITHEREDCLR = 0x0007,
    /// The hatch is not a pattern, but is a solid color, defined by the
    /// current text (foreground) color.
    HS_SOLIDTEXTCLR = 0x0008,
    /// The hatch is not a pattern, but is a dithered color, defined by the
    /// current text (foreground) color.
    HS_DITHEREDTEXTCLR = 0x0009,
    /// The hatch is not a pattern, but is a solid color, defined by the
    /// current background color.
    HS_SOLIDBKCLR = 0x000A,
    /// The hatch is not a pattern, but is a dithered color, defined by the
    /// current background color.
    HS_DITHEREDBKCLR = 0x000B,
}

crate::parser::enums::impl_parser!(HatchStyle, u32);
