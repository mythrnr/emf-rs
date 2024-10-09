/// The ArmStyle enumeration defines values for one of the characteristics in
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
pub enum ArmStyle {
    /// Any.
    PAN_ANY = 0x00,
    /// No fit.
    PAN_NO_FIT = 0x01,
    /// Straight arms/horizontal.
    PAN_STRAIGHT_ARMS_HORZ = 0x02,
    /// Straight arms/wedge.
    PAN_STRAIGHT_ARMS_WEDGE = 0x03,
    /// Straight arms/vertical.
    PAN_STRAIGHT_ARMS_VERT = 0x04,
    /// Straight arms/single-serif.
    PAN_STRAIGHT_ARMS_SINGLE_SERIF = 0x05,
    /// Straight arms/double-serif.
    PAN_STRAIGHT_ARMS_DOUBLE_SERIF = 0x06,
    /// Nonstraight arms/horizontal.
    PAN_BENT_ARMS_HORZ = 0x07,
    /// Nonstraight arms/wedge.
    PAN_BENT_ARMS_WEDGE = 0x08,
    /// Nonstraight arms/vertical.
    PAN_BENT_ARMS_VERT = 0x09,
    /// Nonstraight arms/single-serif.
    PAN_BENT_ARMS_SINGLE_SERIF = 0x0A,
    /// Nonstraight arms/double-serif.
    PAN_BENT_ARMS_DOUBLE_SERIF = 0x0B,
}

crate::parser::enums::impl_parser!(ArmStyle, u8);
