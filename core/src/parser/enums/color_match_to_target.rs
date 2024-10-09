/// The ColorMatchToTarget enumeration is used to determine whether a color
/// profile has been embedded in the metafile.
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
pub enum ColorMatchToTarget {
    /// Indicates that a color profile has not been embedded in the metafile.
    COLORMATCHTOTARGET_NOTEMBEDDED = 0x00000000,
    /// Indicates that a color profile has been embedded in the metafile.
    COLORMATCHTOTARGET_EMBEDDED = 0x00000001,
}

crate::parser::enums::impl_parser!(ColorMatchToTarget, u32);
