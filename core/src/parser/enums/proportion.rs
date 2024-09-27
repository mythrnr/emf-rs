/// The Proportion enumeration defines values for one of the characteristics in
/// the PANOSE system for classifying typefaces.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u8)]
pub enum Proportion {
    /// Any.
    PAN_ANY = 0x00,
    /// No fit.
    PAN_NO_FIT = 0x01,
    /// Old style.
    PAN_PROP_OLD_STYLE = 0x02,
    /// Modern.
    PAN_PROP_MODERN = 0x03,
    /// Even width.
    PAN_PROP_EVEN_WIDTH = 0x04,
    /// Expanded.
    PAN_PROP_EXPANDED = 0x05,
    /// Condensed.
    PAN_PROP_CONDENSED = 0x06,
    /// Very expanded.
    PAN_PROP_VERY_EXPANDED = 0x07,
    /// Very condensed.
    PAN_PROP_VERY_CONDENSED = 0x08,
    /// Monospaced.
    PAN_PROP_MONOSPACED = 0x09,
}

crate::parser::constants::impl_parser!(Proportion, u8);
