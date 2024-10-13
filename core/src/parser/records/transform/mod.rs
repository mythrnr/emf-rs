//! Implementation of the definitions in Section 2.3.12 of the EMF
//! specifications.

mod modify_world_transform;
mod set_world_transform;

pub use self::{modify_world_transform::*, set_world_transform::*};
