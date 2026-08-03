/// The PenAlignment enumeration defines the distribution of the width of
/// the pen with respect to the line being drawn (MS-EMFPLUS 2.1.1.24).
///
/// Graphics pens are specified by EmfPlusPen objects. Pen alignment can
/// be visualized by considering a theoretical one-dimensional line drawn
/// between two specified points. The pen alignment determines the
/// proportion of pen width that is orthogonal to the theoretical line.
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
pub enum PenAlignment {
    /// The EmfPlusPen object is centered over the theoretical line.
    PenAlignmentCenter = 0x00000000,
    /// The pen is positioned on the inside of the theoretical line.
    PenAlignmentInset = 0x00000001,
    /// The pen is positioned to the left of the theoretical line.
    PenAlignmentLeft = 0x00000002,
    /// The pen is positioned on the outside of the theoretical line.
    PenAlignmentOutset = 0x00000003,
    /// The pen is positioned to the right of the theoretical line.
    PenAlignmentRight = 0x00000004,
}

crate::parser::enums::impl_parser!(PenAlignment, u32);
