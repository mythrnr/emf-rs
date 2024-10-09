/// The FloodFill enumeration defines values that specify how to determine the
/// area for a flood fill operation.
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
pub enum FloodFill {
    /// The fill area is bounded by a specific color.
    FLOODFILLBORDER = 0x00000000,
    /// The fill area is defined by a specific color. Filling continues outward
    /// in all directions as long as the color is encountered. This style is
    /// useful for filling areas with multicolored boundaries.
    FLOODFILLSURFACE = 0x00000001,
}

crate::parser::enums::impl_parser!(FloodFill, u32);
