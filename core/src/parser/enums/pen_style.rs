/// The PenStyle enumeration defines the attributes of pens that can be used in
/// graphics operations. A pen style is a combination of pen type, line style,
/// line cap, and line join.
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
pub enum PenStyle {
    /// A pen type that specifies a line with a width of one logical unit and a
    /// style that is a solid color.
    // PS_COSMETIC = 0x00000000,
    /// A line cap that specifies round ends.
    // PS_ENDCAP_ROUND = 0x00000000,
    /// A line join that specifies round joins.
    // PS_JOIN_ROUND = 0x00000000,
    /// A line style that is a solid color.
    PS_SOLID = 0x00000000,
    /// A line style that is dashed.
    PS_DASH = 0x00000001,
    /// A line style that is dotted.
    PS_DOT = 0x00000002,
    /// A line style that consists of alternating dashes and dots.
    PS_DASHDOT = 0x00000003,
    /// A line style that consists of dashes and double dots.
    PS_DASHDOTDOT = 0x00000004,
    /// A line style that is invisible.
    PS_NULL = 0x00000005,
    /// A line style that is a solid color. When this style is specified in a
    /// drawing record that takes a bounding rectangle, the dimensions of the
    /// figure are shrunk so that it fits entirely in the bounding rectangle,
    /// considering the width of the pen.
    PS_INSIDEFRAME = 0x00000006,
    /// A line style that is defined by a styling array, which specifies the
    /// lengths of dashes and gaps in the line.
    PS_USERSTYLE = 0x00000007,
    /// A line style in which every other pixel is set. This style is
    /// applicable only to a pen type of PS_COSMETIC.
    PS_ALTERNATE = 0x00000008,
    /// A line cap that specifies square ends.
    PS_ENDCAP_SQUARE = 0x00000100,
    /// A line cap that specifies flat ends.
    PS_ENDCAP_FLAT = 0x00000200,
    /// A line join that specifies beveled joins.
    PS_JOIN_BEVEL = 0x00001000,
    /// A line join that specifies mitered joins when the lengths of the joins
    /// are within the current miter length limit. If the lengths of the joins
    /// exceed the miter limit, beveled joins are specified.
    ///
    /// The miter length limit is a metafile state property that is set by the
    /// EMR_SETMITERLIMIT record.
    PS_JOIN_MITER = 0x00002000,
    /// A pen type that specifies a line with a width that is measured in
    /// logical units and a style that can contain any of the attributes of a
    /// brush.
    PS_GEOMETRIC = 0x00010000,
}

crate::parser::enums::impl_parser!(PenStyle, u32);

/// Bitmask of `PenStyle` variants packed into a single u32.
///
/// Replaces the prior `BTreeSet<PenStyle>` representation: pen-style
/// fields combine line-style, end-cap, line-join, and pen-type bits
/// drawn from disjoint categories, so the EMF stream stores them as
/// a single ORed integer. Keeping that integer verbatim avoids the
/// per-record B-tree allocation that the BTreeSet form required and
/// shrinks the wasm binary footprint contributed by `Ord`/`Debug`/
/// `Clone` for the set type.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct PenStyleFlags(u32);

impl PenStyleFlags {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns true when every bit of `flag` is set in `self`.
    /// Zero-valued variants (e.g. `PS_SOLID`, `PS_ENDCAP_ROUND`,
    /// `PS_JOIN_ROUND`) always evaluate as present, matching the
    /// previous `BTreeSet` form's semantics.
    pub const fn contains(self, flag: PenStyle) -> bool {
        let bit = flag as u32;
        (self.0 & bit) == bit
    }

    /// Convenience constructor for the single-style pen presets used
    /// by `LogPenEx::black_pen` / `white_pen` / `null_pen` and the
    /// `From<LogPen>` adapter.
    pub const fn single(flag: PenStyle) -> Self {
        Self(flag as u32)
    }

    /// Iterates over the variants whose bits are set in `self`. The
    /// iteration order follows `strum::EnumIter`, which mirrors
    /// declaration order.
    pub fn iter(self) -> impl Iterator<Item = PenStyle> {
        use strum::IntoEnumIterator;

        let raw = self.0;
        PenStyle::iter().filter(move |v| {
            let bit = *v as u32;
            (raw & bit) == bit
        })
    }
}

impl core::fmt::Debug for PenStyleFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}
