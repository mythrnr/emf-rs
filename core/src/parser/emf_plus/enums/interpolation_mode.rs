/// The InterpolationMode enumeration defines ways to perform scaling,
/// including stretching and shrinking (MS-EMFPLUS 2.1.1.16).
///
/// To stretch an image, each pixel in the original image SHOULD be
/// mapped to a group of pixels in the larger image. To shrink an image,
/// groups of pixels in the original image SHOULD be mapped to single
/// pixels in the smaller image. The effectiveness of the algorithm that
/// performs these mappings determines the quality of a scaled image.
/// Higher-quality interpolation generally uses more data points and
/// requires more processing time than lower-quality interpolation.
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
pub enum InterpolationMode {
    /// The default interpolation mode, which is defined as
    /// InterpolationModeBilinear.
    InterpolationModeDefault = 0x00000000,
    /// A low-quality interpolation mode, which is defined as
    /// InterpolationModeNearestNeighbor.
    InterpolationModeLowQuality = 0x00000001,
    /// A high-quality interpolation mode, which is defined as
    /// InterpolationModeHighQualityBicubic.
    InterpolationModeHighQuality = 0x00000002,
    /// Bilinear interpolation, which uses the closest 2x2 neighborhood
    /// of known pixels surrounding the interpolated pixel. The weighted
    /// average of these 4 known pixel values determines the value to
    /// assign to the interpolated pixel. The result is smoother looking
    /// than InterpolationModeNearestNeighbor.
    InterpolationModeBilinear = 0x00000003,
    /// Bicubic interpolation, which uses the closest 4x4 neighborhood of
    /// known pixels surrounding the interpolated pixel. The weighted
    /// average of these 16 known pixel values determines the value to
    /// assign to the interpolated pixel. Because the known pixels are
    /// likely to be at varying distances from the interpolated pixel,
    /// closer pixels are given a higher weight in the calculation. The
    /// result is smoother looking than InterpolationModeBilinear.
    InterpolationModeBicubic = 0x00000004,
    /// Nearest-neighbor interpolation, which uses only the value of the
    /// pixel that is closest to the interpolated pixel. This mode simply
    /// duplicates or removes pixels, producing the lowest-quality result
    /// among these options.
    InterpolationModeNearestNeighbor = 0x00000005,
    /// Bilinear interpolation with prefiltering.
    InterpolationModeHighQualityBilinear = 0x00000006,
    /// Bicubic interpolation with prefiltering, which produces the
    /// highest-quality result among these options.
    InterpolationModeHighQualityBicubic = 0x00000007,
}

crate::parser::enums::impl_parser!(InterpolationMode, u32);
