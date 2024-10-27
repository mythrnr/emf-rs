/// This record closes path bracket construction and selects the path into the
/// playback device context.
#[derive(Clone, Debug)]
pub struct EMR_ENDPATH {
    /// Type (4 bytes): An unsigned integer that identifies this record type
    /// from the RecordType enumeration. It MUST be EMR_ENDPATH, which is
    /// 0x0000003C.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. For path bracket records, this value is
    /// 0x00000008.
    pub size: crate::parser::Size,
}

impl EMR_ENDPATH {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_ENDPATH {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_ENDPATH as u32,
                    record_type as u32
                ),
            });
        }

        if size.byte_count() != 0x00000008 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "size field must be `0x00000008`, but parsed value is \
                     {:#010X}",
                    size.byte_count(),
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
