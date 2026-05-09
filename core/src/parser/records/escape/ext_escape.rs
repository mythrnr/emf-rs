use crate::imports::*;

/// The EMR_EXTESCAPE record passes arbitrary information to a printer driver.
/// The intent is that the information does not result in drawing being done.
#[derive(Clone, Debug)]
pub struct EMR_EXTESCAPE {
    /// Type (4 bytes): An unsigned integer that identifies this record type
    /// from the RecordType enumeration. This value is 0x0000006A.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// iEscape (4 bytes): An unsigned integer that specifies the printer
    /// driver escape to execute. This MUST be one of the values in the
    /// MetafileEscapes enumeration ([MS-WMF] section 2.1.1.17).
    pub i_escape: wmf_core::parser::MetafileEscapes,
    /// cjIn (4 bytes): An unsigned integer specifying the number of bytes to
    /// pass to the printer driver.
    pub cj_in: u32,
    /// Data (variable): The data to pass to the printer driver. There MUST be
    /// cjIn bytes available.
    pub data: Vec<u8>,
}

impl EMR_EXTESCAPE {
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
            crate::parser::RecordType::EMR_EXTESCAPE as u32,
        )?;

        let i_escape = read_with(
            buf,
            &mut size,
            wmf_core::parser::MetafileEscapes::parse,
        )?;
        let cj_in: u32 = read_field(buf, &mut size)?;

        let data = read_bytes_field(buf, &mut size, cj_in as usize)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, i_escape, cj_in, data })
    }
}
