//! Implementation of the definitions in Section 2.1 of the MS-EMFPLUS
//! specifications (enumerations and bit flag constants).

mod bitmap_data_type;
mod brush_type;
mod combine_mode;
mod compositing_mode;
mod compositing_quality;
mod curve_adjustments;
mod curve_channel;
mod custom_line_cap_data_type;
mod dashed_line_cap_type;
mod filter_type;
mod flags;
mod graphics_version;
mod hatch_style;
mod hotkey_prefix;
mod image_data_type;
mod interpolation_mode;
mod language_identifier;
mod line_cap_type;
mod line_join_type;
mod line_style;
mod metafile_data_type;
mod object_type;
mod path_point_type;
mod pen_alignment;
mod pixel_format;
mod pixel_offset_mode;
mod record_type;
mod region_node_data_type;
mod smoothing_mode;
mod string_alignment;
mod string_digit_substitution;
mod string_trimming;
mod text_rendering_hint;
mod unit_type;
mod wrap_mode;

pub use self::{
    bitmap_data_type::*, brush_type::*, combine_mode::*, compositing_mode::*,
    compositing_quality::*, curve_adjustments::*, curve_channel::*,
    custom_line_cap_data_type::*, dashed_line_cap_type::*, filter_type::*,
    flags::*, graphics_version::*, hatch_style::*, hotkey_prefix::*,
    image_data_type::*, interpolation_mode::*, language_identifier::*,
    line_cap_type::*, line_join_type::*, line_style::*, metafile_data_type::*,
    object_type::*, path_point_type::*, pen_alignment::*, pixel_format::*,
    pixel_offset_mode::*, record_type::*, region_node_data_type::*,
    smoothing_mode::*, string_alignment::*, string_digit_substitution::*,
    string_trimming::*, text_rendering_hint::*, unit_type::*, wrap_mode::*,
};
