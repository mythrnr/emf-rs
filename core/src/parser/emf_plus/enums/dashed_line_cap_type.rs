/// The DashedLineCapType enumeration defines types of line caps to use
/// at the ends of dashed lines that are drawn with graphics pens
/// (MS-EMFPLUS 2.1.1.10).
///
/// Dashed lines are specified by EmfPlusDashedLineData objects.
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
#[repr(u32)]
pub enum DashedLineCapType {
    /// A flat dashed line cap.
    DashedLineCapTypeFlat = 0x00000000,
    /// A round dashed line cap. The value 0x00000001 is not defined by
    /// the specification.
    DashedLineCapTypeRound = 0x00000002,
    /// A triangular dashed line cap.
    DashedLineCapTypeTriangle = 0x00000003,
}

crate::parser::enums::impl_parser!(DashedLineCapType, u32);
