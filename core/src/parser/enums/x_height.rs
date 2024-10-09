/// The XHeight enumeration defines values for one of the characteristics in the
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
pub enum XHeight {
    /// Any.
    PAN_ANY = 0x00,
    /// No fit.
    PAN_NO_FIT = 0x01,
    /// Constant/small.
    PAN_XHEIGHT_CONSTANT_SMALL = 0x02,
    /// Constant/standard.
    PAN_XHEIGHT_CONSTANT_STD = 0x03,
    /// Constant/large.
    PAN_XHEIGHT_CONSTANT_LARGE = 0x04,
    /// Ducking/small
    PAN_XHEIGHT_DUCKING_SMALL = 0x05,
    /// Ducking/standard.
    PAN_XHEIGHT_DUCKING_STD = 0x06,
    /// Ducking/large.
    PAN_XHEIGHT_DUCKING_LARGE = 0x07,
}

crate::parser::enums::impl_parser!(XHeight, u8);
