/// The CurveChannel enumeration defines color channels that can be
/// affected by a color curve effect adjustment to an image
/// (MS-EMFPLUS 2.1.1.8).
///
/// Bitmap images are specified by EmfPlusBitmap objects.
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
pub enum CurveChannel {
    /// A color curve adjustment applies to all color channels.
    CurveChannelAll = 0x00000000,
    /// A color curve adjustment applies only to the red color channel.
    CurveChannelRed = 0x00000001,
    /// A color curve adjustment applies only to the green color channel.
    CurveChannelGreen = 0x00000002,
    /// A color curve adjustment applies only to the blue color channel.
    CurveChannelBlue = 0x00000003,
}

crate::parser::enums::impl_parser!(CurveChannel, u32);
