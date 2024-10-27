/// This record opens path bracket construction.
///
/// Once path bracket construction is open, an application can begin specifying
/// records to define the points that lie in the path. Path bracket construction
/// MUST be closed by an EMR_ABORTPATH or EMR_ENDPATH record.
///
/// When an application processes an EMR_BEGINPATH record, path bracket
/// construction MUST NOT be open.
#[derive(Clone, Debug)]
pub struct EMR_BEGINPATH {
    /// Type (4 bytes): An unsigned integer that identifies this record type
    /// from the RecordType enumeration. It MUST be EMR_BEGINPATH, which is
    /// 0x0000003B.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. For path bracket records, this value is
    /// 0x00000008.
    pub size: crate::parser::Size,
}

impl EMR_BEGINPATH {
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
        if record_type != crate::parser::RecordType::EMR_BEGINPATH {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_BEGINPATH as u32,
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
