/// The StringAlignment enumeration defines ways to align strings with
/// respect to a text layout rectangle (MS-EMFPLUS 2.1.1.29).
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
pub enum StringAlignment {
    /// String alignment is toward the origin of the layout rectangle.
    /// This can be used to align characters along a line or to align
    /// text within a rectangle. For a right-to-left layout rectangle,
    /// the origin SHOULD be at the upper right.
    StringAlignmentNear = 0x00000000,
    /// The alignment is centered between the origin and extent of the
    /// layout rectangle.
    StringAlignmentCenter = 0x00000001,
    /// The alignment is to the right side of the layout rectangle.
    StringAlignmentFar = 0x00000002,
}

crate::parser::enums::impl_parser!(StringAlignment, u32);
