/// The Weight enumeration defines values for one of the characteristics in the
/// PANOSE system for classifying typefaces.
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
#[repr(u8)]
pub enum Weight {
    /// Any.
    PAN_ANY = 0x00,
    /// No fit.
    PAN_NO_FIT = 0x01,
    /// Very light.
    PAN_WEIGHT_VERY_LIGHT = 0x02,
    /// Light.
    PAN_WEIGHT_LIGHT = 0x03,
    /// Thin.
    PAN_WEIGHT_THIN = 0x04,
    /// Book.
    PAN_WEIGHT_BOOK = 0x05,
    /// Medium.
    PAN_WEIGHT_MEDIUM = 0x06,
    /// Demi.
    PAN_WEIGHT_DEMI = 0x07,
    /// Bold.
    PAN_WEIGHT_BOLD = 0x08,
    /// Heavy.
    PAN_WEIGHT_HEAVY = 0x09,
    /// Black.
    PAN_WEIGHT_BLACK = 0x0A,
    /// Nord.
    PAN_WEIGHT_NORD = 0x0B,
}

crate::parser::enums::impl_parser!(Weight, u8);
