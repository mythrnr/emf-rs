/// The StrokeVariation enumeration defines values for one of the
/// characteristics in the PANOSE system for classifying typefaces.
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
pub enum StrokeVariation {
    /// Any.
    PAN_ANY = 0x00,
    /// No fit.
    PAN_NO_FIT = 0x01,
    /// Gradual/diagonal.
    PAN_STROKE_GRADUAL_DIAG = 0x02,
    /// Gradual/transitional.
    PAN_STROKE_GRADUAL_TRAN = 0x03,
    /// Gradual/vertical.
    PAN_STROKE_GRADUAL_VERT = 0x04,
    /// Gradual/horizontal.
    PAN_STROKE_GRADUAL_HORZ = 0x05,
    /// Rapid/vertical.
    PAN_STROKE_RAPID_VERT = 0x06,
    /// Rapid/horizontal.
    PAN_STROKE_RAPID_HORZ = 0x07,
    /// Instant/vertical.
    PAN_STROKE_INSTANT_VERT = 0x08,
}

crate::parser::enums::impl_parser!(StrokeVariation, u8);
