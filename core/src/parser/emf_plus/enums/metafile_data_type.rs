/// The MetafileDataType enumeration defines types of metafiles data that
/// can be embedded in an EMF+ metafile (MS-EMFPLUS 2.1.1.21).
///
/// Embedded metafile data is specified by EmfPlusMetafileData objects.
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
pub enum MetafileDataType {
    /// The metafile is a WMF metafile [MS-WMF] that specifies graphics
    /// operations with WMF records.
    MetafileDataTypeWmf = 0x00000001,
    /// The metafile is a WMF metafile that specifies graphics operations
    /// with WMF records, and which contains additional header
    /// information that makes the WMF metafile device-independent.
    MetafileDataTypeWmfPlaceable = 0x00000002,
    /// The metafile is an EMF metafile that specifies graphics
    /// operations with EMF records ([MS-EMF] section 2.3).
    MetafileDataTypeEmf = 0x00000003,
    /// The metafile is an EMF+ metafile that specifies graphics
    /// operations with EMF+ records only.
    MetafileDataTypeEmfPlusOnly = 0x00000004,
    /// The metafile is an EMF+ metafile that specifies graphics
    /// operations with both EMF and EMF+ records.
    MetafileDataTypeEmfPlusDual = 0x00000005,
}

crate::parser::enums::impl_parser!(MetafileDataType, u32);
