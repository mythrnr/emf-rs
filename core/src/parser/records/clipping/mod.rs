//! Implementation of the definitions in Section 2.3.2 of the EMF
//! specifications.

mod exclude_clip_rect;
mod ext_select_clip_rgn;
mod intersect_clip_rect;
mod offset_clip_rgn;
mod select_clip_path;
mod set_meta_rgn;

pub use self::{
    exclude_clip_rect::*, ext_select_clip_rgn::*, intersect_clip_rect::*,
    offset_clip_rgn::*, select_clip_path::*, set_meta_rgn::*,
};
