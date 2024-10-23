/// Saves the current state of the playback device context in an array of states
/// saved by preceding EMR_SAVEDC records if any.
///
/// An EMR_RESTOREDC record is used to restore the state.
///
/// This record specifies no parameters.
#[derive(Clone, Debug)]
pub struct EMR_SAVEDC {
    /// Type (4 bytes): An unsigned integer that identifies the record type as
    /// EMR_SAVEDC. This value is 0x00000021.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of the
    /// record in bytes.
    pub size: crate::parser::Size,
}

impl EMR_SAVEDC {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_SAVEDC {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SAVEDC as u32,
                    record_type as u32
                ),
            });
        }

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size })
    }
}
