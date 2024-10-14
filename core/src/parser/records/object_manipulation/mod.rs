//! Implementation of the definitions in Section 2.3.8 of the EMF
//! specifications.

mod color_correct_palette;
mod delete_color_space;
mod delete_object;
mod resize_palette;
mod select_object;
mod select_palette;
mod set_color_space;
mod set_palette_entries;

pub use self::{
    color_correct_palette::*, delete_color_space::*, delete_object::*,
    resize_palette::*, select_object::*, select_palette::*, set_color_space::*,
    set_palette_entries::*,
};
