/// The ColorAdjustment enumeration is used to specify how the output image is
/// prepared when the stretch mode is HALFTONE.
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
#[repr(u16)]
pub enum ColorAdjustmentEnum {
    /// Specifies that the negative of the original image SHOULD be displayed.
    CA_NEGATIVE = 0x0001,
    /// Specifies that a logarithmic process SHOULD be applied to the final
    /// density of the output colors. This will increase the color contrast
    /// when the luminance is low.
    CA_LOG_FILTER = 0x0002,
}

crate::parser::enums::impl_parser!(ColorAdjustmentEnum, u16);

/// Bitmask of `ColorAdjustmentEnum` variants packed into a single
/// u16. Replaces the prior `BTreeSet<ColorAdjustmentEnum>`
/// representation to drop the per-record B-tree allocation.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct ColorAdjustmentEnumFlags(u16);

crate::parser::enums::impl_flags!(
    ColorAdjustmentEnumFlags,
    ColorAdjustmentEnum,
    u16
);
