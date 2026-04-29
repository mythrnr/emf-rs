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

impl ColorAdjustmentEnumFlags {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn from_raw(raw: u16) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u16 {
        self.0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub const fn contains(self, flag: ColorAdjustmentEnum) -> bool {
        let bit = flag as u16;
        (self.0 & bit) == bit
    }

    pub fn iter(self) -> impl Iterator<Item = ColorAdjustmentEnum> {
        use strum::IntoEnumIterator;

        let raw = self.0;
        ColorAdjustmentEnum::iter().filter(move |v| {
            let bit = *v as u16;
            (raw & bit) == bit
        })
    }
}

impl core::fmt::Debug for ColorAdjustmentEnumFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}
