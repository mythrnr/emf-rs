//! Implementation of the definitions in Section 2.3.6 of the EMF
//! specifications.

mod draw_escape;
mod ext_escape;
mod named_escape;

pub use self::{draw_escape::*, ext_escape::*, named_escape::*};
