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
pub enum LayoutMode {
    /// Sets the default horizontal layout to be left-to-right. This is the
    /// default mode for English and European locales.
    LAYOUT_LTR = 0x00000000,
    /// Sets the default horizontal layout to be right-to-left. This mode is
    /// required for some languages, including Arabic and Hebrew.
    LAYOUT_RTL = 0x00000001,
    /// Disables mirroring of bitmaps that are drawn by bitmap records when the
    /// layout mode is right-to-left.
    LAYOUT_BITMAPORIENTATIONPRESERVED = 0x00000008,
}

crate::parser::enums::impl_parser!(LayoutMode, u32);
