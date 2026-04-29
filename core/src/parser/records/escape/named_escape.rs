use crate::imports::*;

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
            consume_remaining_bytes, read_bytes_field, read_field, read_with,
        };

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_NAMEDESCAPE as u32,
        )?;

        let i_escape = read_with(
            buf,
            &mut size,
            wmf_core::parser::MetafileEscapes::parse,
        )?;
        let cj_driver = read_field(buf, &mut size)?;
        let cj_in: u32 = read_field(buf, &mut size)?;

        let driver_name = {
            let mut bytes = vec![];

            loop {
                let v: u8 = read_field(buf, &mut size)?;

                if v == 0 {
                    break;
                }

                bytes.push(v);
            }

            crate::parser::utf16le_bytes_to_string(&bytes)?
        };

        let data = read_bytes_field(buf, &mut size, cj_in as usize)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

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
