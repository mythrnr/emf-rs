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

#[rustfmt::skip]
macro_rules! impl_parser {
    ($t:ident,u8) => {
        $crate::parser::enums::impl_parser!(_, $t, u8, 1, 4);
    };
    ($t:ident,u16) => {
        $crate::parser::enums::impl_parser!(_, $t, u16, 2, 6);
    };
    ($t:ident,u32) => {
        $crate::parser::enums::impl_parser!(_, $t, u32, 4, 10);
    };
    ($t:ident,i32) => {
        $crate::parser::enums::impl_parser!(_, $t, i32, 4, 10);
    };
    (_, $t:ident, $raw:ty, $size:expr, $digits:expr) => {
        paste::paste! {
            impl $t {
                #[::tracing::instrument(
                    level = tracing::Level::TRACE,
                    skip_all,
                    err(level = tracing::Level::ERROR, Display),
                )]
                pub fn parse<R: $crate::Read>(
                    buf: &mut R,
                ) -> Result<(Self, usize), $crate::parser::ParseError> {
                    let (value, consumed_bytes) = crate::parser::[<read_ $raw _from_le_bytes>](buf)?;
                    let Some(v)  = Self::from_repr(value) else {
                        return Err($crate::parser::ParseError::UnexpectedEnumValue {
                            cause: ::alloc::format!(
                                ::core::concat!(
                                    "unexpected value as ",
                                    ::core::stringify!($t),
                                    ": {:#0", $digits, "X}",
                                ),
                                value
                            ),
                        });
                    };

                    Ok((v, consumed_bytes))
                }
            }
        }
    };
}

use impl_parser;
