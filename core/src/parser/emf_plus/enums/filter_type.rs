/// The FilterType enumeration defines types of filtering algorithms
/// that can be used for text and graphics quality enhancement and image
/// rendering (MS-EMFPLUS 2.1.1.11).
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
pub enum FilterType {
    /// Filtering is not performed.
    FilterTypeNone = 0x00000000,
    /// Each destination pixel is computed by sampling the nearest pixel
    /// from the source image.
    FilterTypePoint = 0x00000001,
    /// Linear interpolation is performed using the weighted average of a
    /// 2x2 area of pixels surrounding the source pixel.
    FilterTypeLinear = 0x00000002,
    /// Each pixel in the source image contributes equally to the
    /// destination image. This is the slowest of filtering algorithms.
    FilterTypeTriangle = 0x00000003,
    /// A box filter algorithm is used, in which each destination pixel
    /// is computed by averaging a rectangle of source pixels. This
    /// algorithm is useful only when reducing the size of an image.
    FilterTypeBox = 0x00000004,
    /// A 4-sample tent filter is used.
    FilterTypePyramidalQuad = 0x00000006,
    /// A 4-sample Gaussian filter is used, which creates a blur effect
    /// on an image.
    FilterTypeGaussianQuad = 0x00000007,
}

crate::parser::enums::impl_parser!(FilterType, u32);
