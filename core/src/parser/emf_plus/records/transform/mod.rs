//! Implementation of the definitions in Section 2.3.9 of the
//! MS-EMFPLUS specifications (transform records).

mod multiply_world_transform;
mod reset_world_transform;
mod rotate_world_transform;
mod scale_world_transform;
mod set_page_transform;
mod set_world_transform;
mod translate_world_transform;

pub use self::{
    multiply_world_transform::*, reset_world_transform::*,
    rotate_world_transform::*, scale_world_transform::*, set_page_transform::*,
    set_world_transform::*, translate_world_transform::*,
};

/// The A bit (0x2000) of transform record flags: the transform is
/// post-multiplied (appended) to the current world transform instead
/// of pre-multiplied (prepended).
pub(in crate::parser::emf_plus) const FLAG_POST_MULTIPLY: u16 = 0x2000;
