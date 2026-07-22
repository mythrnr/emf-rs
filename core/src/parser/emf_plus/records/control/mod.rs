//! Implementation of the definitions in Section 2.3.3 of the
//! MS-EMFPLUS specifications (control records).

mod end_of_file;
mod get_dc;
mod header;

pub use self::{end_of_file::*, get_dc::*, header::*};
