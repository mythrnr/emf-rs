//! Implementation of the definitions in Section 2.2 of the EMF specifications.

mod bit_fix28_4;
mod color_adjustment;
mod design_vector;
mod emr_format;
mod emr_text;

pub use self::{
    bit_fix28_4::*, color_adjustment::*, design_vector::*, emr_format::*,
    emr_text::*,
};
