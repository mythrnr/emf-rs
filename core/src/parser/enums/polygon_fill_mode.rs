/// The PolygonFillMode enumeration defines values that specify how to calculate
/// the region of a polygon that is to be filled.
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
pub enum PolygonFillMode {
    /// Selects alternate mode (fills the area between odd-numbered and
    /// even-numbered polygon sides on each scan line).
    ALTERNATE = 0x01,
    /// Selects winding mode (fills any region with a nonzero winding value).
    WINDING = 0x02,
}

crate::parser::enums::impl_parser!(PolygonFillMode, u8);
