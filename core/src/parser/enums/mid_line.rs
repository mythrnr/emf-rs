/// The MidLine enumeration defines values for one of the characteristics in the
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
pub enum MidLine {
    /// Any.
    PAN_ANY = 0x00,
    /// No fit.
    PAN_NO_FIT = 0x01,
    /// Standard/trimmed.
    PAN_MIDLINE_STANDARD_TRIMMED = 0x02,
    /// Standard/pointed.
    PAN_MIDLINE_STANDARD_POINTED = 0x03,
    /// Standard/serifed.
    PAN_MIDLINE_STANDARD_SERIFED = 0x04,
    /// High/trimmed.
    PAN_MIDLINE_HIGH_TRIMMED = 0x05,
    /// High/pointed.
    PAN_MIDLINE_HIGH_POINTED = 0x06,
    /// High/serifed.
    PAN_MIDLINE_HIGH_SERIFED = 0x07,
    /// Constant/trimmed.
    PAN_MIDLINE_CONSTANT_TRIMMED = 0x08,
    /// Constant/pointed.
    PAN_MIDLINE_CONSTANT_POINTED = 0x09,
    /// Constant/serifed.
    PAN_MIDLINE_CONSTANT_SERIFED = 0x0A,
    /// Low/trimmed.
    PAN_MIDLINE_LOW_TRIMMED = 0x0B,
    /// Low/pointed.
    PAN_MIDLINE_LOW_POINTED = 0x0C,
    /// Low/serifed.
    PAN_MIDLINE_LOW_SERIFED = 0x0D,
}

crate::parser::enums::impl_parser!(MidLine, u8);
