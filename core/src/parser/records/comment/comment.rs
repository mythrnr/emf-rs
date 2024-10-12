/// The EMR_COMMENT record contains arbitrary private data.
#[derive(Clone, Debug)]
pub struct EMR_COMMENT {
    /// Type (4 bytes): An unsigned integer from the RecordType enumeration
    /// that identifies this record as a comment record. This value is
    /// 0x00000046.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the size in
    /// bytes, of the CommentIdentifier and CommentRecordParm fields in the
    /// RecordBuffer field that follows. It MUST NOT include the size of itself
    /// or the size of the AlignmentPadding field, if present.
    pub data_size: u32,
    /// PrivateData (variable, optional): An array of bytes that specifies the
    /// private data. The first 32-bit field of this data MUST NOT be one of
    /// the predefined comment identifier values specified in section 2.3.3.
    ///
    /// Private data is unknown to EMF; it is meaningful only to applications
    /// that know the format of the data and how to use it. EMR_COMMENT private
    /// data records MAY(Windows NT 3.1, Windows NT 3.51, and Windows NT 4.0
    /// ignore EMR_COMMENT records.) be ignored.
    pub private_data: Vec<u8>,
}

impl EMR_COMMENT {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_COMMENT {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_COMMENT as u32,
                    record_type as u32
                ),
            });
        }

        let ((size, size_bytes), (data_size, data_size_bytes)) = (
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );
        let (private_data, private_data_bytes) =
            crate::parser::read_variable(buf, data_size as usize)?;

        crate::parser::records::consume_remaining_bytes(
            buf,
            size as usize - (size_bytes + data_size_bytes + private_data_bytes),
        )?;

        Ok(Self { record_type, size, data_size, private_data })
    }
}
