/// The EmfPlusGetDC record specifies that subsequent EMF records
/// ([MS-EMF] section 2.3) encountered in the metafile SHOULD be
/// processed.
///
/// EMF records cease being processed when the next EMF+ record is
/// encountered.
#[derive(Clone, Debug)]
pub struct EmfPlusGetDC {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusGetDC from the RecordType enumeration. The value
    /// MUST be 0x4004.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that is not used. This
    /// field SHOULD be set to zero and MUST be ignored upon receipt.
    pub flags: u16,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned size of the entire record in bytes, including
    /// the 12-byte record header and the record-specific buffer data.
    /// For this record type, the value is 0x0000000C.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of data in the record-specific
    /// data that follows. This number does not include the size of the
    /// invariant part of this record. For this record type, the value
    /// is 0x00000000.
    pub data_size: crate::parser::Size,
}

impl EmfPlusGetDC {
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
            crate::parser::emf_plus::RecordType::EmfPlusGetDC as u16,
        )?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self { record_type, flags, size, data_size })
    }
}
