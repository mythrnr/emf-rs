//! Implementation of the definitions in Section 2.3.6 of the
//! MS-EMFPLUS specifications (property records).
//!
//! Property records carry their value inside the 16-bit Flags field
//! and have no record-specific data, with one exception:
//! EmfPlusSetRenderingOrigin stores its origin coordinates in 8 bytes
//! of record data.

mod set_anti_alias_mode;
mod set_compositing_mode;
mod set_compositing_quality;
mod set_interpolation_mode;
mod set_pixel_offset_mode;
mod set_rendering_origin;
mod set_text_contrast;
mod set_text_rendering_hint;

pub use self::{
    set_anti_alias_mode::*, set_compositing_mode::*,
    set_compositing_quality::*, set_interpolation_mode::*,
    set_pixel_offset_mode::*, set_rendering_origin::*, set_text_contrast::*,
    set_text_rendering_hint::*,
};

/// Decodes an enumeration value carried in the low byte of the record
/// flags, the layout shared by most property records.
fn enum_from_low_byte<T>(
    flags: u16,
    from_repr: impl FnOnce(u32) -> Option<T>,
    name: &'static str,
) -> Result<T, crate::parser::ParseError> {
    let raw = u32::from(flags & 0x00FF);

    from_repr(raw).ok_or_else(|| {
        crate::parser::ParseError::UnexpectedEnumValue {
            cause: alloc::format!(
                "unexpected value as {name}: {raw:#04X}, flags: {flags:#06X}"
            )
            .into(),
        }
    })
}
