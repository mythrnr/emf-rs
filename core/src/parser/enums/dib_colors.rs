/// The DIBColors enumeration defines how to interpret the values in the color
/// table of a DIB.
///
/// DIBs are specified by DeviceIndependentBitmap objects ([MS-WMF] section
/// 2.2.2.9).
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
pub enum DIBColors {
    /// The color table contains literal RGB values.
    DIB_RGB_COLORS = 0x00,
    /// The color table consists of an array of 16-bit indexes into the
    /// LogPalette object that is currently defined in the playback device
    /// context.
    DIB_PAL_COLORS = 0x01,
    /// No color table exists. The pixels in the DIB are indices into the
    /// current logical palette in the playback device context.
    DIB_PAL_INDICES = 0x02,
}

crate::parser::enums::impl_parser!(DIBColors, u32);

impl From<DIBColors> for wmf_core::parser::ColorUsage {
    fn from(v: DIBColors) -> Self {
        match v {
            DIBColors::DIB_RGB_COLORS => Self::DIB_RGB_COLORS,
            DIBColors::DIB_PAL_COLORS => Self::DIB_PAL_COLORS,
            DIBColors::DIB_PAL_INDICES => Self::DIB_PAL_INDICES,
        }
    }
}
