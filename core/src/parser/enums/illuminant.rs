/// The Illuminant enumeration defines values that specify the illuminant value
/// of an image, which determines the standard light source under which the
/// image is viewed so that the color can be adjusted appropriately.
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
pub enum Illuminant {
    /// Device's default. Standard used by output devices.
    ILLUMINANT_DEVICE_DEFAULT = 0x00,
    /// Tungsten lamp.
    ILLUMINANT_TUNGSTEN = 0x01,
    /// Noon sunlight.
    ILLUMINANT_B = 0x02,
    /// Daylight.
    ILLUMINANT_DAYLIGHT = 0x03,
    /// Normal print.
    ILLUMINANT_D50 = 0x04,
    /// Bond paper print.
    ILLUMINANT_D55 = 0x05,
    /// Standard daylight. Standard for CRTs and pictures.
    ILLUMINANT_D65 = 0x06,
    /// Northern daylight.
    ILLUMINANT_D75 = 0x07,
    /// Cool white lamp.
    ILLUMINANT_FLUORESCENT = 0x08,
}

crate::parser::enums::impl_parser!(Illuminant, u8);
