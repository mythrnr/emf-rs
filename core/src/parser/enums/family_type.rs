/// The FamilyType enumeration defines values for one of the characteristics in
/// the PANOSE system for classifying typefaces.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u8)]
pub enum FamilyType {
    /// Any.
    PAN_ANY = 0x00,
    /// No fit.
    PAN_NO_FIT = 0x01,
    /// Text and display.
    PAN_FAMILY_TEXT_DISPLAY = 0x02,
    /// Script.
    PAN_FAMILY_SCRIPT = 0x03,
    /// Decorative.
    PAN_FAMILY_DECORATIVE = 0x04,
    /// Pictorial.
    PAN_FAMILY_PICTORIAL = 0x05,
}

crate::parser::constants::impl_parser!(FamilyType, u8);
