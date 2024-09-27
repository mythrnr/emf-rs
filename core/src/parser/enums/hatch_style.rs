/// The HatchStyle enumeration is an extension to the WMF HatchStyle
/// enumeration.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u16)]
pub enum HatchStyle {
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

crate::parser::constants::impl_parser!(HatchStyle, u16);
