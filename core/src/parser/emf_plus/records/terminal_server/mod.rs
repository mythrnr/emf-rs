//! Implementation of the definitions in Section 2.3.8 of the
//! MS-EMFPLUS specifications (terminal server records).
//!
//! Windows never generates these records; they are parsed for
//! completeness.

mod set_ts_clip;
mod set_ts_graphics;

pub use self::{set_ts_clip::*, set_ts_graphics::*};
