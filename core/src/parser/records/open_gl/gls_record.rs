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
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_GLSRECORD {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_GLSRECORD as u32,
                    record_type as u32
                ),
            });
        }

        let (cb_data, cb_data_bytes) =
            crate::parser::read_u32_from_le_bytes(buf)?;

        let (data, data_bytes) =
            crate::parser::read_variable(buf, cb_data as usize)?;

        size.consume(cb_data_bytes + data_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, cb_data, data })
    }
}
