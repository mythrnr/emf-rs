/// The EmfPlusEndOfFile record specifies the end of EMF+ data in the
/// metafile.
#[derive(Clone, Debug)]
pub struct EmfPlusEndOfFile {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusEndOfFile from the RecordType enumeration. The
    /// value MUST be 0x4002.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that is not used. This
    /// field SHOULD be set to zero and MUST be ignored upon receipt.
    pub flags: u16,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be 0x0000000C.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of data in the record-specific
    /// data that follows. For this record type, the value MUST be
    /// 0x00000000.
    pub data_size: crate::parser::Size,
}

impl EmfPlusEndOfFile {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = ?record_type),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::emf_plus::RecordType,
        flags: u16,
        size: u32,
        mut data_size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u16,
            crate::parser::emf_plus::RecordType::EmfPlusEndOfFile as u16,
        )?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self { record_type, flags, size, data_size })
    }
}
