/// The RegionMode enumeration defines values that are used with
/// EMR_SELECTCLIPPATH and EMR_EXTSELECTCLIPRGN, specifying the current path
/// bracket or a new region that is being combined with the current clipping
/// region.
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
pub enum RegionMode {
    /// The new clipping region includes the intersection (overlapping areas)
    /// of the current clipping region and the current path bracket (or new
    /// region).
    RGN_AND = 0x01,
    /// The new clipping region includes the union (combined areas) of the
    /// current clipping region and the current path bracket (or new region).
    RGN_OR = 0x02,
    /// The new clipping region includes the union of the current clipping
    /// region and the current path bracket (or new region) but without the
    /// overlapping areas.
    RGN_XOR = 0x03,
    /// The new clipping region includes the areas of the current clipping
    /// region with those of the current path bracket (or new region) excluded.
    RGN_DIFF = 0x04,
    /// The new clipping region is the current path bracket (or the new
    /// region).
    RGN_COPY = 0x05,
}

crate::parser::enums::impl_parser!(RegionMode, u32);
