/// The EMR_RESTOREDC record restores the playback device context to the
/// specified state. The playback device context is restored by popping state
/// information off a stack that was created by a prior EMR_SAVEDC record.
#[derive(Clone, Debug)]
pub struct EMR_RESTOREDC {
    /// Type (4 bytes): An unsigned integer that identifies the record type as
    /// EMR_RESTOREDC. This value is 0x00000022.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of the
    /// record in bytes. This value is 0x0000000C.
    pub size: crate::parser::Size,
    /// SavedDC (4 bytes): A signed integer that specifies the saved state to
    /// restore relative to the current state. This value MUST be negative; -1
    /// represents the state that was most recently saved on the stack, -2 the
    /// one before that, etc.
    pub saved_dc: i32,
}

impl EMR_RESTOREDC {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_RESTOREDC {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_RESTOREDC as u32,
                    record_type as u32
                ),
            });
        }

        if size.byte_count() != 0x0000000C {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "size field must be `0x0000000C`, but parsed value is \
                     {:#010X}",
                    size.byte_count(),
                ),
            });
        }

        let (saved_dc, saved_dc_bytes) =
            crate::parser::read_i32_from_le_bytes(buf)?;

        size.consume(saved_dc_bytes);

        if saved_dc >= 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "saved_dc field must be negative value, but parsed value \
                     is {:#010X}",
                    size.byte_count(),
                ),
            });
        }

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, saved_dc })
    }
}
