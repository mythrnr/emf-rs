/// The EMR_SETICMPROFILEW record specifies a color profile in a file with a
/// name consisting of Unicode characters, for graphics output.
///
/// Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, and Windows NT 4.0 do not
/// support EMR_SETICMPROFILEW.
#[derive(Clone, Debug)]
pub struct EMR_SETICMPROFILEW {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETICMPROFILEW. This value is 0x00000071.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// dwFlags (4 bytes): An unsigned integer that contains color profile
    /// flags.
    pub dw_flags: u32,
    /// cbName (4 bytes): An unsigned integer that specifies the number of
    /// bytes in the Unicode UTF16-LE name of the desired color profile.
    pub cb_name: u32,
    /// cbData (4 bytes): An unsigned integer that specifies the size of color
    /// profile data, if attached.
    pub cb_data: u32,
    /// Data (variable): An array of size (cbName + cbData) in bytes, which
    /// specifies the UTF16-LE name and raw data of the desired color profile.
    pub data: Vec<u8>,
}

impl EMR_SETICMPROFILEW {
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
        if record_type != crate::parser::RecordType::EMR_SETICMPROFILEW {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETICMPROFILEW as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (dw_flags, dw_flags_bytes),
            (cb_name, cb_name_bytes),
            (cb_data, cb_data_bytes),
        ) = (
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        size.consume(dw_flags_bytes + cb_name_bytes + cb_data_bytes);

        let (data, data_bytes) =
            crate::parser::read_variable(buf, (cb_name + cb_data) as usize)?;

        size.consume(data_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, dw_flags, cb_name, cb_data, data })
    }
}
