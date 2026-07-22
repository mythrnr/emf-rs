/// The CompositingQuality enumeration defines levels of quality for
/// creating composite images (MS-EMFPLUS 2.1.1.6).
///
/// Graphics colors are specified by EmfPlusARGB objects.
///
/// Compositing is done during rendering when source pixels are combined
/// with destination pixels. The compositing quality directly relates to
/// the visual quality of the output and is inversely proportional to the
/// time required for rendering. The higher the quality, the more
/// surrounding pixels need to be taken into account during the
/// compositing operation; hence, the slower the render time.
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
pub enum CompositingQuality {
    /// No gamma correction is performed. Gamma correction controls the
    /// overall brightness and contrast of an image. Without gamma
    /// correction, composited images can appear too light or too dark.
    CompositingQualityDefault = 0x00000001,
    /// No gamma correction is performed. Compositing speed is favored at
    /// the expense of quality. In terms of the result, there is no
    /// difference between this value and CompositingQualityDefault.
    CompositingQualityHighSpeed = 0x00000002,
    /// Gamma correction is performed. Compositing quality is favored at
    /// the expense of speed.
    CompositingQualityHighQuality = 0x00000003,
    /// Enable gamma correction for higher-quality compositing with lower
    /// speed. In terms of the result, there is no difference between
    /// this value and CompositingQualityHighQuality.
    CompositingQualityGammaCorrected = 0x00000004,
    /// No gamma correction is performed; however, using linear values
    /// results in better quality than the default at a slightly lower
    /// speed.
    CompositingQualityAssumeLinear = 0x00000005,
}

crate::parser::enums::impl_parser!(CompositingQuality, u32);
