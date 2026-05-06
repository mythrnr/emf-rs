//! Implementation of the definitions in Section 2.1 of the EMF
//! specifications.

mod arc_direction;
mod arm_style;
mod background_mode;
mod color_adjustment;
mod color_match_to_target;
mod color_space;
mod contrast;
mod dib_colors;
mod emr_comment;
mod ext_text_out_options;
mod family_type;
mod flood_fill;
mod format_signature;
mod gradient_fill;
mod graphics_mode;
mod hatch_style;
mod icm_mode;
mod illuminant;
mod layout_mode;
mod letterform;
mod map_mode;
mod metafile_version;
mod mid_line;
mod modify_world_transform_mode;
mod pen_style;
mod point;
mod polygon_fill_mode;
mod proportion;
mod record_type;
mod region_mode;
mod serif_type;
mod stock_object;
mod stretch_mode;
mod stroke_variation;
mod weight;
mod x_height;

pub use self::{
    arc_direction::*, arm_style::*, background_mode::*, color_adjustment::*,
    color_match_to_target::*, color_space::*, contrast::*, dib_colors::*,
    emr_comment::*, ext_text_out_options::*, family_type::*, flood_fill::*,
    format_signature::*, gradient_fill::*, graphics_mode::*, hatch_style::*,
    icm_mode::*, illuminant::*, layout_mode::*, letterform::*, map_mode::*,
    metafile_version::*, mid_line::*, modify_world_transform_mode::*,
    pen_style::*, point::*, polygon_fill_mode::*, proportion::*,
    record_type::*, region_mode::*, serif_type::*, stock_object::*,
    stretch_mode::*, stroke_variation::*, weight::*, x_height::*,
};

/// Generates `impl $T { pub fn parse<R: Read>(buf: &mut R) ->
/// Result<(Self, usize), ParseError> }` for an enum that derives
/// `strum::FromRepr` over a little-endian integer of the given type.
///
/// The body reads a `$raw`-typed value via `ReadLeField::read_le`,
/// looks it up with `Self::from_repr`, and returns
/// `ParseError::UnexpectedEnumValue` when the value is not a known
/// variant. The selector arm (`u8` / `u16` / `u32` / `i32`) sets the
/// hex width used in the error message so diagnostics match the
/// wire-format byte size: `u8` -> `{:#04X}`, `u16` -> `{:#06X}`,
/// `u32` and `i32` -> `{:#010X}`.
///
/// When the `tracing` feature is enabled, the generated `parse` is
/// wrapped with `#[tracing::instrument]` at the TRACE level so per-
/// record decode failures surface in the log stream.
///
/// Invoke as `impl_parser!(EnumType, u32);`. The `(_, ...)` arm is an
/// internal dispatch detail and is not part of the public surface.
#[rustfmt::skip]
macro_rules! impl_parser {
    ($t:ident,u8) => {
        $crate::parser::enums::impl_parser!(_, $t, u8, 4);
    };
    ($t:ident,u16) => {
        $crate::parser::enums::impl_parser!(_, $t, u16, 6);
    };
    ($t:ident,u32) => {
        $crate::parser::enums::impl_parser!(_, $t, u32, 10);
    };
    ($t:ident,i32) => {
        $crate::parser::enums::impl_parser!(_, $t, i32, 10);
    };
    (_, $t:ident, $raw:ty, $digits:expr) => {
        impl $t {
            #[cfg_attr(feature = "tracing", ::tracing::instrument(
                level = tracing::Level::TRACE,
                skip_all,
                err(level = tracing::Level::ERROR, Display),
            ))]
            pub fn parse<R: $crate::Read>(
                buf: &mut R,
            ) -> Result<(Self, usize), $crate::parser::ParseError> {
                let (value, consumed_bytes) =
                    <$raw as $crate::parser::ReadLeField>::read_le(buf)?;
                let Some(v) = Self::from_repr(value) else {
                    return Err(
                        $crate::parser::ParseError::UnexpectedEnumValue {
                            cause: ::alloc::format!(
                                ::core::concat!(
                                    "unexpected value as ",
                                    ::core::stringify!($t),
                                    ": {:#0", $digits, "X}",
                                ),
                                value
                            )
                            .into(),
                        },
                    );
                };

                Ok((v, consumed_bytes))
            }
        }
    };
}

use impl_parser;

/// Generates the standard inherent API and `Debug` impl for a
/// `#[repr(transparent)]` flags wrapper struct (`$flags($raw)`) that
/// stores an OR-combination of variants drawn from `$enum`.
///
/// The generated impl provides `empty` / `from_raw` / `raw` /
/// `is_empty` / `contains` / `single` / `iter` plus a `Debug` impl
/// that prints the set of variants whose bits are present. `contains`
/// uses the `(raw & bit) == bit` check, so any variant whose
/// discriminant is `0` is reported as present regardless of the
/// stored bits; document this caveat on the wrapper struct when it
/// applies (see `PenStyleFlags`).
///
/// The wrapped enum must derive `strum::FromRepr` and
/// `strum::EnumIter`, and every variant's discriminant must fit in
/// `$raw` (the macro casts via `as $raw`).
///
/// Invoke as `impl_flags!(FlagsType, EnumType, u32);`.
macro_rules! impl_flags {
    ($flags:ident, $enum:ident, $raw:ty) => {
        impl $flags {
            pub const fn empty() -> Self {
                Self(0)
            }

            pub const fn from_raw(raw: $raw) -> Self {
                Self(raw)
            }

            pub const fn raw(self) -> $raw {
                self.0
            }

            pub const fn is_empty(self) -> bool {
                self.0 == 0
            }

            /// Returns true when every bit of `flag` is set in `self`.
            pub const fn contains(self, flag: $enum) -> bool {
                let bit = flag as $raw;
                (self.0 & bit) == bit
            }

            /// Constructs a flags value with only `flag`'s bits set.
            pub const fn single(flag: $enum) -> Self {
                Self(flag as $raw)
            }

            /// Iterates over the variants whose bits are set in `self`.
            /// Iteration order follows `strum::EnumIter`, which mirrors
            /// declaration order on the enum.
            pub fn iter(self) -> impl Iterator<Item = $enum> {
                use strum::IntoEnumIterator;

                let raw = self.0;
                $enum::iter().filter(move |v| {
                    let bit = *v as $raw;
                    (raw & bit) == bit
                })
            }
        }

        impl ::core::fmt::Debug for $flags {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                f.debug_set().entries(self.iter()).finish()
            }
        }
    };
}

use impl_flags;
