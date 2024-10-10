//! Implementation of the definitions in Section 2.2 of the EMF specifications.

mod bit_fix28_4;
mod color_adjustment;
mod design_vector;
mod emr_format;
mod emr_text;
mod eps_data;
mod gradient_rectangle;
mod gradient_triangle;
mod header;
mod header_extension_1;
mod header_extension_2;
mod log_brush_ex;
mod log_font;
mod log_font_ex;
mod log_font_ex_dv;
mod log_font_panose;
mod log_palette;
mod log_palette_entry;
mod log_pen;
mod log_pen_ex;
mod panose;
mod pixel_format_descriptor;
mod point28_4;
mod region_data;
mod region_data_header;
mod tri_vertex;
mod universal_font_id;
mod x_form;

pub use self::{
    bit_fix28_4::*, color_adjustment::*, design_vector::*, emr_format::*,
    emr_text::*, eps_data::*, gradient_rectangle::*, gradient_triangle::*,
    header::*, header_extension_1::*, header_extension_2::*, log_brush_ex::*,
    log_font::*, log_font_ex::*, log_font_ex_dv::*, log_font_panose::*,
    log_palette::*, log_palette_entry::*, log_pen::*, log_pen_ex::*, panose::*,
    pixel_format_descriptor::*, point28_4::*, region_data::*,
    region_data_header::*, tri_vertex::*, universal_font_id::*, x_form::*,
};
