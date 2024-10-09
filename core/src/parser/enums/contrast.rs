/// The Contrast enumeration defines values for one of the characteristics in
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
pub enum Contrast {
    /// Any.
    PAN_ANY = 0x00,
    /// No fit.
    PAN_NO_FIT = 0x01,
    /// None.
    PAN_CONTRAST_NONE = 0x02,
    /// Very low.
    PAN_CONTRAST_VERY_LOW = 0x03,
    /// Low.
    PAN_CONTRAST_LOW = 0x04,
    /// Medium low.
    PAN_CONTRAST_MEDIUM_LOW = 0x05,
    /// Medium.
    PAN_CONTRAST_MEDIUM = 0x06,
    /// Medium high.
    PAN_CONTRAST_MEDIUM_HIGH = 0x07,
    /// High.
    PAN_CONTRAST_HIGH = 0x08,
    /// Very high.
    PAN_CONTRAST_VERY_HIGH = 0x09,
}

crate::parser::enums::impl_parser!(Contrast, u8);
