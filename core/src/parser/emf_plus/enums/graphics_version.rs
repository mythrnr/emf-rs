/// The GraphicsVersion enumeration defines versions of operating system
/// graphics that are used to create EMF+ metafiles
/// (MS-EMFPLUS 2.1.1.12).
///
/// Graphics versions are specified in EmfPlusGraphicsVersion objects.
///
/// The value occupies the low 12 bits of an EmfPlusGraphicsVersion
/// object; it is never read directly from the stream, so no `parse` is
/// generated for it.
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
#[repr(u16)]
pub enum GraphicsVersion {
    /// GDI+ version 1.0.
    GraphicsVersion1 = 0x0001,
    /// GDI+ version 1.1.
    GraphicsVersion1_1 = 0x0002,
}
