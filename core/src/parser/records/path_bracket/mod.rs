//! Implementation of the definitions in Section 2.3.10 of the EMF
//! specifications.

mod abort_path;
mod begin_path;
mod close_figure;
mod end_path;
mod flatten_path;
mod widen_path;

pub use self::{
    abort_path::*, begin_path::*, close_figure::*, end_path::*,
    flatten_path::*, widen_path::*,
};
