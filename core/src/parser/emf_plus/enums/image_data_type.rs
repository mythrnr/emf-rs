/// The ImageDataType enumeration defines types of image data formats
/// (MS-EMFPLUS 2.1.1.15).
///
/// Graphics images are specified by EmfPlusImage objects.
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
pub enum ImageDataType {
    /// The type of image is not known.
    ImageDataTypeUnknown = 0x00000000,
    /// A bitmap image.
    ImageDataTypeBitmap = 0x00000001,
    /// A metafile image.
    ImageDataTypeMetafile = 0x00000002,
}

crate::parser::enums::impl_parser!(ImageDataType, u32);
