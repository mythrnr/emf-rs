/// This record redefines the current path as the area that would be painted if
/// its path were drawn using the current pen.
#[derive(Clone, Debug)]
pub struct EMR_WIDENPATH {
    /// Type (4 bytes): An unsigned integer that identifies this record type
    /// from the RecordType enumeration. It MUST be EMR_WIDENPATH, which is
    /// 0x00000042.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. For path bracket records, this value is
    /// 0x00000008.
    pub size: crate::parser::Size,
}

impl EMR_WIDENPATH {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = ?record_type),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::consume_remaining_bytes;

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_WIDENPATH as u32,
        )?;
        crate::parser::ParseError::expect_eq(
            "size field",
            size.byte_count() as u32,
            0x00000008_u32,
        )?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size })
    }
}
