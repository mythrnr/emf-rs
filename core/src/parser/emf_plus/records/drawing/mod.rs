//! Implementation of the definitions in Section 2.3.4 of the
//! MS-EMFPLUS specifications (drawing records).

mod clear;
mod draw_arc;
mod draw_beziers;
mod draw_closed_curve;
mod draw_curve;
mod draw_driver_string;
mod draw_ellipse;
mod draw_image;
mod draw_image_points;
mod draw_lines;
mod draw_path;
mod draw_pie;
mod draw_rects;
mod draw_string;
mod fill_closed_curve;
mod fill_ellipse;
mod fill_path;
mod fill_pie;
mod fill_polygon;
mod fill_rects;
mod fill_region;

pub use self::{
    clear::*, draw_arc::*, draw_beziers::*, draw_closed_curve::*,
    draw_curve::*, draw_driver_string::*, draw_ellipse::*, draw_image::*,
    draw_image_points::*, draw_lines::*, draw_path::*, draw_pie::*,
    draw_rects::*, draw_string::*, fill_closed_curve::*, fill_ellipse::*,
    fill_path::*, fill_pie::*, fill_polygon::*, fill_rects::*, fill_region::*,
};
