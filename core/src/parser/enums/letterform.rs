/// The Letterform enumeration defines values for one of the characteristics in
/// the PANOSE system for classifying typefaces.
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
pub enum Letterform {
    /// Any.
    PAN_ANY = 0x00,
    /// No fit.
    PAN_NO_FIT = 0x01,
    /// Normal/contact.
    PAN_LETT_NORMAL_CONTACT = 0x02,
    /// Normal/weighted.
    PAN_LETT_NORMAL_WEIGHTED = 0x03,
    /// Normal/boxed.
    PAN_LETT_NORMAL_BOXED = 0x04,
    /// Normal/flattened.
    PAN_LETT_NORMAL_FLATTENED = 0x05,
    /// Normal/rounded.
    PAN_LETT_NORMAL_ROUNDED = 0x06,
    /// Normal/off center.
    PAN_LETT_NORMAL_OFF_CENTER = 0x07,
    /// Normal/square
    PAN_LETT_NORMAL_SQUARE = 0x08,
    /// Oblique/contact.
    PAN_LETT_OBLIQUE_CONTACT = 0x09,
    /// Oblique/weighted.
    PAN_LETT_OBLIQUE_WEIGHTED = 0x0A,
    /// Oblique/boxed.
    PAN_LETT_OBLIQUE_BOXED = 0x0B,
    /// Oblique/flattened.
    PAN_LETT_OBLIQUE_FLATTENED = 0x0C,
    /// Oblique/rounded.
    PAN_LETT_OBLIQUE_ROUNDED = 0x0D,
    /// Oblique/off center.
    PAN_LETT_OBLIQUE_OFF_CENTER = 0x0E,
    /// Oblique/square.
    PAN_LETT_OBLIQUE_SQUARE = 0x0F,
}

crate::parser::enums::impl_parser!(Letterform, u8);
