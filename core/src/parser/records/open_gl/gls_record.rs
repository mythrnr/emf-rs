use crate::imports::*;

/// The EMR_GLSRECORD record specifies an OpenGL function.
#[derive(Clone, Debug)]
pub struct EMR_GLSRECORD {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_GLSRECORD. This value is 0x00000066.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// cbData (4 bytes): An unsigned integer that specifies the size in bytes,
    /// of the Data field. If this value is zero, no data is attached to this
    /// record.
    pub cb_data: u32,
    /// Data (variable, optional): An array of bytes that specifies data for
    /// the OpenGL function.
    pub data: Vec<u8>,
}

impl EMR_GLSRECORD {
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
        use crate::parser::records::{
            consume_remaining_bytes, read_bytes_field, read_field,
        };

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_GLSRECORD as u32,
        )?;

        let cb_data: u32 = read_field(buf, &mut size)?;
        let data = read_bytes_field(buf, &mut size, cb_data as usize)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, cb_data, data })
    }
}
