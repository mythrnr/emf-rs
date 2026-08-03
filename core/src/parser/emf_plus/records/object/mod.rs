//! Implementation of the definitions in Section 2.3.5 of the
//! MS-EMFPLUS specifications (object records).

#[allow(clippy::module_inception)]
mod object;
mod serializable_object;

pub use self::{object::*, serializable_object::*};
