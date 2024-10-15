//! Implementation of the definitions in Section 2.3.5 of the EMF
//! specifications.

mod angle_arc;
mod arc;
mod arc_to;
mod chord;
mod ellipse;
mod ext_flood_fill;
mod ext_text_out_a;
mod ext_text_out_w;
mod fill_path;
mod fill_rgn;
mod frame_rgn;
mod gradient_fill;
mod line_to;
mod paint_rgn;
mod pie;
mod poly_bezier;
mod poly_bezier_16;
mod poly_bezier_to;
mod poly_bezier_to_16;
mod poly_draw;
mod poly_draw_16;
mod poly_polygon;
mod poly_polygon_16;
mod poly_polyline;
mod poly_polyline_16;
mod poly_text_out_a;
mod poly_text_out_w;
mod polygon;
mod polygon_16;
mod polyline;
mod polyline_16;
mod polyline_to;
mod polyline_to_16;
mod rectangle;
mod round_rect;
mod set_pixel_v;
mod small_text_out;
mod stroke_and_fill_path;
mod stroke_path;

pub use self::{
    angle_arc::*, arc::*, arc_to::*, chord::*, ellipse::*, ext_flood_fill::*,
    ext_text_out_a::*, ext_text_out_w::*, fill_path::*, fill_rgn::*,
    frame_rgn::*, gradient_fill::*, line_to::*, paint_rgn::*, pie::*,
    poly_bezier::*, poly_bezier_16::*, poly_bezier_to::*, poly_bezier_to_16::*,
    poly_draw::*, poly_draw_16::*, poly_polygon::*, poly_polygon_16::*,
    poly_polyline::*, poly_polyline_16::*, poly_text_out_a::*,
    poly_text_out_w::*, polygon::*, polygon_16::*, polyline::*, polyline_16::*,
    polyline_to::*, polyline_to_16::*, rectangle::*, round_rect::*,
    set_pixel_v::*, small_text_out::*, stroke_and_fill_path::*, stroke_path::*,
};
