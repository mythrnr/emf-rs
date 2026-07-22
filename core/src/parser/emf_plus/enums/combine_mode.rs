/// The CombineMode enumeration defines modes for combining two graphics
/// regions (MS-EMFPLUS 2.1.1.4). In the following descriptions, the
/// regions to be combined are referred to as the "existing" and "new"
/// regions.
///
/// Graphics regions are specified by EmfPlusRegion objects.
///
/// In clipping records the value travels in bits 8-11 of the record
/// flags.
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
pub enum CombineMode {
    /// Replace the existing region with the new region.
    CombineModeReplace = 0x00000000,
    /// Replace the existing region with the intersection of the existing
    /// region and the new region.
    CombineModeIntersect = 0x00000001,
    /// Replace the existing region with the union of the existing and
    /// new regions.
    CombineModeUnion = 0x00000002,
    /// Replace the existing region with the XOR of the existing and new
    /// regions.
    CombineModeXOR = 0x00000003,
    /// Replace the existing region with the part of itself that is not
    /// in the new region.
    CombineModeExclude = 0x00000004,
    /// Replace the existing region with the part of the new region that
    /// is not in the existing region.
    CombineModeComplement = 0x00000005,
}

crate::parser::enums::impl_parser!(CombineMode, u32);
