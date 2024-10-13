/// The ArcDirection enumeration is used in setting the drawing direction for
/// arcs and rectangles.
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
pub enum ArcDirection {
    /// Figures drawn counterclockwise.
    AD_COUNTERCLOCKWISE = 0x00000001,
    /// Figures drawn clockwise.
    AD_CLOCKWISE = 0x00000002,
}

crate::parser::enums::impl_parser!(ArcDirection, u32);
