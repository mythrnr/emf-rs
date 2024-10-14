//! Implementation of the definitions in Section 2.3.11 of the EMF
//! specifications.

mod color_match_to_target_w;
mod force_ufi_mapping;
mod invert_rgn;
mod move_to_ex;
mod pixel_format;
mod realize_palette;
mod restore_dc;
mod save_dc;
mod scale_viewport_ext_ex;
mod scale_window_ext_ex;
mod set_arc_direction;
mod set_bk_color;
mod set_bk_mode;
mod set_brush_org_ex;
mod set_color_adjustment;
mod set_icm_mode;
mod set_icm_profile_a;
mod set_icm_profile_w;
mod set_layout;
mod set_linked_ufis;
mod set_map_mode;
mod set_mapper_flags;
mod set_miter_limit;
mod set_polyfill_mode;
mod set_rop2;
mod set_stretch_blt_mode;
mod set_text_align;
mod set_text_color;
mod set_text_justification;
mod set_viewport_ext_ex;
mod set_viewport_org_ex;
mod set_window_ext_ex;
mod set_window_org_ex;

pub use self::{
    color_match_to_target_w::*, force_ufi_mapping::*, invert_rgn::*,
    move_to_ex::*, pixel_format::*, realize_palette::*, restore_dc::*,
    save_dc::*, scale_viewport_ext_ex::*, scale_window_ext_ex::*,
    set_arc_direction::*, set_bk_color::*, set_bk_mode::*, set_brush_org_ex::*,
    set_color_adjustment::*, set_icm_mode::*, set_icm_profile_a::*,
    set_icm_profile_w::*, set_layout::*, set_linked_ufis::*, set_map_mode::*,
    set_mapper_flags::*, set_miter_limit::*, set_polyfill_mode::*, set_rop2::*,
    set_stretch_blt_mode::*, set_text_align::*, set_text_color::*,
    set_text_justification::*, set_viewport_ext_ex::*, set_viewport_org_ex::*,
    set_window_ext_ex::*, set_window_org_ex::*,
};
