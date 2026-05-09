use crate::imports::*;

/// The EMR_COMMENT record contains arbitrary private data.
#[derive(Clone, Debug)]
pub struct EMR_COMMENT {
    /// Type (4 bytes): An unsigned integer from the RecordType enumeration
    /// that identifies this record as a comment record. This value is
    /// 0x00000046.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
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
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = ?record_type),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{
            consume_remaining_bytes, read_bytes_field, read_field,
        };

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_COMMENT as u32,
        )?;

        let data_size: u32 = read_field(buf, &mut size)?;
        let private_data =
            read_bytes_field(buf, &mut size, data_size as usize)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, data_size, private_data })
    }
}
