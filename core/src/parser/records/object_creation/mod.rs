//! Implementation of the definitions in Section 2.3.7 of the EMF
//! specifications.

mod create_brush_indirect;
mod create_color_space;
mod create_color_space_w;
mod create_dib_pattern_brush_pt;
mod create_mono_brush;
mod create_palette;
mod create_pen;
mod ext_create_font_indirect_w;
mod ext_create_pen;

pub use self::{
    create_brush_indirect::*, create_color_space::*, create_color_space_w::*,
    create_dib_pattern_brush_pt::*, create_mono_brush::*, create_palette::*,
    create_pen::*, ext_create_font_indirect_w::*, ext_create_pen::*,
};
