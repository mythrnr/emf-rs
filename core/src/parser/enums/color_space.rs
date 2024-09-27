/// The ColorSpace enumeration is used to specify when to turn color proofing on
/// and off, and when to delete transforms.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u32)]
pub enum ColorSpace {
    /// Maps colors to the target device's color gamut. This enables color
    /// proofing. All subsequent draw commands render colors as they would
    /// appear on the target device.
    CS_ENABLE = 0x00000001,
    /// Disables color proofing.
    CS_DISABLE = 0x00000002,
    /// If color management is enabled for the target profile, disables it and
    /// deletes the current color transform.
    CS_DELETE_TRANSFORM = 0x00000003,
}

crate::parser::constants::impl_parser!(ColorSpace, u32);
