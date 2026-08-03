/// The CustomLineCapDataType enumeration defines types of custom line
/// cap data, which specify styles and shapes for the ends of graphics
/// lines (MS-EMFPLUS 2.1.1.9).
///
/// Custom line cap data is specified by EmfPlusCustomLineCap objects.
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
pub enum CustomLineCapDataType {
    /// A default custom line cap.
    CustomLineCapDataTypeDefault = 0x00000000,
    /// An adjustable arrow custom line cap.
    CustomLineCapDataTypeAdjustableArrow = 0x00000001,
}

crate::parser::enums::impl_parser!(CustomLineCapDataType, u32);
