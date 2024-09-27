/// The SerifType enumeration defines values for one of the characteristics in
/// the PANOSE system for classifying typefaces.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u8)]
pub enum SerifType {
    /// Any.
    PAN_ANY = 0x00,
    /// No fit.
    PAN_NO_FIT = 0x01,
    /// Cove.
    PAN_SERIF_COVE = 0x02,
    /// Obtuse cove.
    PAN_SERIF_OBTUSE_COVE = 0x03,
    /// Square cove.
    PAN_SERIF_SQUARE_COVE = 0x04,
    /// Obtuse square cove.
    PAN_SERIF_OBTUSE_SQUARE_COVE = 0x05,
    /// Square.
    PAN_SERIF_SQUARE = 0x06,
    /// Thin.
    PAN_SERIF_THIN = 0x07,
    /// Bone.
    PAN_SERIF_BONE = 0x08,
    /// Exaggerated.
    PAN_SERIF_EXAGGERATED = 0x09,
    /// Triangle.
    PAN_SERIF_TRIANGLE = 0x0A,
    /// Normal sans.
    PAN_SERIF_NORMAL_SANS = 0x0B,
    /// Obtuse sans.
    PAN_SERIF_OBTUSE_SANS = 0x0C,
    /// Perp sans.
    PAN_SERIF_PERP_SANS = 0x0D,
    /// Flared.
    PAN_SERIF_FLARED = 0x0E,
    /// Rounded.
    PAN_SERIF_ROUNDED = 0x0F,
}

crate::parser::constants::impl_parser!(SerifType, u8);
