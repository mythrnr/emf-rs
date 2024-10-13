//! Implementation of the definitions in Section 2.3 of the EMF specifications.

mod bitmap;
mod clipping;
mod comment;
mod control;
mod drawing;
mod escape;
mod object_creation;
mod object_manipulation;
mod open_gl;
mod path_bracket;
mod state;
mod transform;

pub use self::{
    bitmap::*, clipping::*, comment::*, control::*, drawing::*, escape::*,
    object_creation::*, object_manipulation::*, open_gl::*, path_bracket::*,
    state::*, transform::*,
};

fn consume_remaining_bytes<R: std::io::Read>(
    buf: &mut R,
    remaining_bytes: usize,
) -> Result<(), crate::parser::ParseError> {
    let _ = crate::parser::read_variable(buf, remaining_bytes)?;

    Ok(())
}
