/// The MetafileVersion enumeration defines the interoperability version for EMF
/// metafile.
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
pub enum MetafileVersion {
    /// Specifies EMF metafile interoperability.
    META_FORMAT_ENHANCED = 0x00010000,
}

crate::parser::enums::impl_parser!(MetafileVersion, u32);
