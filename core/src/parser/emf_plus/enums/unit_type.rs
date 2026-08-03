/// The UnitType enumeration defines units of measurement in different
/// coordinate systems (MS-EMFPLUS 2.1.1.33).
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
pub enum UnitType {
    /// A unit of logical distance within the world space.
    UnitTypeWorld = 0x00000000,
    /// A unit of distance based on the characteristics of the physical
    /// display.
    UnitTypeDisplay = 0x00000001,
    /// A unit of 1 pixel.
    UnitTypePixel = 0x00000002,
    /// A unit of 1 printer's point, or 1/72 inch.
    UnitTypePoint = 0x00000003,
    /// A unit of 1 inch.
    UnitTypeInch = 0x00000004,
    /// A unit of 1/300 inch.
    UnitTypeDocument = 0x00000005,
    /// A unit of 1 millimeter.
    UnitTypeMillimeter = 0x00000006,
}

crate::parser::enums::impl_parser!(UnitType, u32);
