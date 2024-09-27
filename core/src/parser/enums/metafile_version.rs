/// The MetafileVersion enumeration defines the interoperability version for EMF
/// metafile.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u32)]
pub enum MetafileVersion {
    /// Specifies EMF metafile interoperability.
    META_FORMAT_ENHANCED = 0x00010000,
}

crate::parser::constants::impl_parser!(MetafileVersion, u32);
