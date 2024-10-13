/// The EMR_NAMEDESCAPE record passes arbitrary information to a named printer
/// driver.
#[derive(Clone, Debug)]
pub struct EMR_NAMEDESCAPE {
    /// Type (4 bytes): An unsigned integer that identifies this record type
    /// from the RecordType enumeration. It MUST be EMR_NAMEDESCAPE, which is
    /// 0x0000006E.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// iEscape (4 bytes): An unsigned integer that specifies the printer
    /// driver escape to execute. This MUST be one of the values in the
    /// MetafileEscapes enumeration ([MS-WMF] section 2.1.1.17).
    pub i_escape: wmf_core::parser::MetafileEscapes,
    /// cjDriver (4 bytes): An unsigned integer that specifies the number of
    /// bytes in the DriverName field. This value MUST be an even number.
    pub cj_driver: u32,
    /// cjIn (4 bytes): An unsigned integer specifying the number of bytes in
    /// the Data field to pass to the printer driver.
    pub cj_in: u32,
    /// DriverName (variable): A null-terminated string of Unicode characters
    /// that specifies the name of the printer driver to receive data.
    pub driver_name: String,
    /// Data (variable): The data to pass to the printer driver.
    pub data: Vec<u8>,
}

impl EMR_NAMEDESCAPE {
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
        if record_type != crate::parser::RecordType::EMR_NAMEDESCAPE {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_NAMEDESCAPE as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (i_escape, i_escape_bytes),
            (cj_driver, cj_driver_bytes),
            (cj_in, cj_in_bytes),
        ) = (
            wmf_core::parser::MetafileEscapes::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        let (driver_name, driver_name_bytes) = {
            let mut bytes = vec![];
            let mut driver_name_bytes = 0;

            loop {
                let (v, b) = crate::parser::read::<_, 1>(buf)?;

                driver_name_bytes += b;

                if v[0] == 0 {
                    break;
                }

                bytes.push(v[0]);
            }

            let driver_name = crate::parser::utf16le_bytes_to_string(&bytes)?;

            (driver_name, driver_name_bytes)
        };

        let (data, data_bytes) =
            crate::parser::read_variable(buf, cj_in as usize)?;

        size.consume(
            i_escape_bytes
                + cj_driver_bytes
                + cj_in_bytes
                + data_bytes
                + driver_name_bytes,
        );

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self {
            record_type,
            size,
            i_escape,
            cj_driver,
            cj_in,
            driver_name,
            data,
        })
    }
}
