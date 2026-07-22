/// The CurveAdjustments enumeration defines adjustments that can be
/// applied to the color curve of an image (MS-EMFPLUS 2.1.1.7).
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
pub enum CurveAdjustments {
    /// The simulation of increasing or decreasing the exposure of an
    /// image.
    AdjustExposure = 0x00000000,
    /// The simulation of increasing or decreasing the density of an
    /// image.
    AdjustDensity = 0x00000001,
    /// An increase or decrease of the contrast of an image.
    AdjustContrast = 0x00000002,
    /// An increase or decrease of the value of a color channel of an
    /// image, if that channel already has a value that is above half
    /// intensity. This adjustment can be used to increase definition in
    /// the light areas of an image without affecting the dark areas.
    AdjustHighlight = 0x00000003,
    /// An increase or decrease of the value of a color channel of an
    /// image, if that channel already has a value that is below half
    /// intensity. This adjustment can be used to increase definition in
    /// the dark areas of an image without affecting the light areas.
    AdjustShadow = 0x00000004,
    /// An adjustment that lightens or darkens an image. Color channel
    /// values in the middle of the intensity range are altered more than
    /// color channel values near the minimum or maximum extremes of
    /// intensity. This adjustment can be used to lighten or darken an
    /// image without losing the contrast between the darkest and
    /// lightest parts of the image.
    AdjustMidtone = 0x00000005,
    /// An adjustment to the white saturation of an image, defined as the
    /// maximum value in the range of intensities for a given color
    /// channel, whose range is typically 0 to 255.
    ///
    /// For example, a white saturation adjustment value of 240 specifies
    /// that color channel values in the range 0 to 240 are adjusted so
    /// that they spread out over the range 0 to 255, with color channel
    /// values greater than 240 set to 255.
    AdjustWhiteSaturation = 0x00000006,
    /// An adjustment to the black saturation of an image, which is the
    /// minimum value in the range of intensities for a given color
    /// channel, which is typically 0 to 255.
    ///
    /// For example, a black saturation adjustment value of 15 specifies
    /// that color channel values in the range 15 to 255 are adjusted so
    /// that they spread out over the range 0 to 255, with color channel
    /// values less than 15 set to 0.
    AdjustBlackSaturation = 0x00000007,
}

crate::parser::enums::impl_parser!(CurveAdjustments, u32);
