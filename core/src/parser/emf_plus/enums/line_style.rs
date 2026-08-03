/// The LineStyle enumeration defines styles of lines that are drawn with
/// graphics pens (MS-EMFPLUS 2.1.1.20).
///
/// Graphics lines are specified by EmfPlusPen objects.
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
pub enum LineStyle {
    /// A solid line.
    LineStyleSolid = 0x00000000,
    /// A dashed line.
    LineStyleDash = 0x00000001,
    /// A dotted line.
    LineStyleDot = 0x00000002,
    /// An alternating dash-dot line.
    LineStyleDashDot = 0x00000003,
    /// An alternating dash-dot-dot line.
    LineStyleDashDotDot = 0x00000004,
    /// A user-defined, custom dashed line.
    LineStyleCustom = 0x00000005,
}

crate::parser::enums::impl_parser!(LineStyle, u32);
