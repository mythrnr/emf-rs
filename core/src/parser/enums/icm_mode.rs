/// The ICMMode enumeration defines values that specify when to turn on and off
/// Image Color Management (ICM).
///
/// Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, Windows 98, and Windows
/// Millennium Edition: Do not support Image Color Management (ICM).
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u8)]
pub enum ICMMode {
    /// Turns off ICM; turns on old-style color correction of halftones.
    ICM_OFF = 0x01,
    /// Turns on ICM; turns off old-style color correction of halftones.
    ICM_ON = 0x02,
    /// Queries the current state of color management.
    ICM_QUERY = 0x03,
    /// Turns off both ICM and old-style color correction of halftones.
    ICM_DONE_OUTSIDEDC = 0x04,
}

crate::parser::constants::impl_parser!(ICMMode, u8);
