/// The ColorAdjustment enumeration is used to specify how the output image is
/// prepared when the stretch mode is HALFTONE.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u16)]
pub enum ColorAdjustment {
    /// Specifies that the negative of the original image SHOULD be displayed.
    CA_NEGATIVE = 0x0001,
    /// Specifies that a logarithmic process SHOULD be applied to the final
    /// density of the output colors. This will increase the color contrast
    /// when the luminance is low.
    CA_LOG_FILTER = 0x0002,
}

crate::parser::constants::impl_parser!(ColorAdjustment, u16);
