//! Implementation of the definitions in Section 2.3.7 of the
//! MS-EMFPLUS specifications (state records).

mod begin_container;
mod begin_container_no_params;
mod end_container;
mod restore;
mod save;

pub use self::{
    begin_container::*, begin_container_no_params::*, end_container::*,
    restore::*, save::*,
};
