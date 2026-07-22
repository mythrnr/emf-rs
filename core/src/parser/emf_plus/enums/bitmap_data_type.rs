/// The BitmapDataType enumeration defines types of bitmap data formats
/// (MS-EMFPLUS 2.1.1.2).
///
/// Bitmap data is specified by EmfPlusBitmap objects.
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
pub enum BitmapDataType {
    /// A bitmap image with pixel data.
    BitmapDataTypePixel = 0x00000000,
    /// An image with compressed data.
    BitmapDataTypeCompressed = 0x00000001,
}

crate::parser::enums::impl_parser!(BitmapDataType, u32);
