//! Implementation of the definitions in Section 2.3.1 of the
//! MS-EMFPLUS specifications (clipping records).

mod offset_clip;
mod reset_clip;
mod set_clip_path;
mod set_clip_rect;
mod set_clip_region;

pub use self::{
    offset_clip::*, reset_clip::*, set_clip_path::*, set_clip_rect::*,
    set_clip_region::*,
};
