use crate::imports::*;

/// The EMR_CREATECOLORSPACEW record creates a logical color space object from a
/// color profile with a name consisting of Unicode characters.
///
/// Windows NT 3.1, Windows NT 3.5, and Windows NT 3.51 do not support
/// EMR_CREATECOLORSPACEW.
///
/// The logical color space object defined by this record can be selected into
/// the playback device context by an EMR_SETCOLORSPACE record, which defines
/// the logical color spaceto use in subsequent graphics operations.
#[derive(Clone, Debug)]
pub struct EMR_CREATECOLORSPACEW {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_CREATECOLORSPACEW. This value is 0x0000007A.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// ihCS (4 bytes): An unsigned integer that specifies the index of the
    /// logical color space object in the EMF object table. This index MUST be
    /// saved so that this object can be reused or modified.
    pub ih_cs: u32,
    /// lcs (variable): A LogColorSpaceW object ([MS-WMF] section 2.2.2.12)
    /// that can specify the name of a color profile in Unicode UTF16-LE
    /// characters.
    pub lcs: wmf_core::parser::LogColorSpaceW,
    /// dwFlags (4 bytes): An unsigned integer that provides information about
    /// the data in this record.
    ///
    /// C (1 bit): If set, the Data field contains color profile data.
    pub dw_flags: u32,
    /// cbData (4 bytes): An unsigned integer that specifies the size in bytes,
    /// of the Data field.
    pub cb_data: u32,
    /// Data (variable, optional): An array of bytes that specifies color
    /// profile data. When cbData is zero, this field is optional and is
    /// ignored.
    pub data: Vec<u8>,
}

impl EMR_CREATECOLORSPACEW {
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
            crate::parser::RecordType::EMR_CREATECOLORSPACEW as u32,
        )?;

        let ih_cs: u32 = read_field(buf, &mut size)?;
        let lcs =
            read_with(buf, &mut size, wmf_core::parser::LogColorSpaceW::parse)?;
        let dw_flags: u32 = read_field(buf, &mut size)?;
        let cb_data: u32 = read_field(buf, &mut size)?;
        let data = read_bytes_field(buf, &mut size, cb_data as usize)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, ih_cs, lcs, dw_flags, cb_data, data })
    }
}
