/// The EMR_CREATEPALETTE record defines a logical palette for graphics
/// operations.
///
/// The logical palette defined by this record can be selected into the playback
/// device context by an EMR_SELECTPALETTE record, which specifies the logical
/// palette to use in subsequent graphics operations.
#[derive(Clone, Debug)]
pub struct EMR_CREATEPALETTE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_CREATEPALETTE. This value is 0x00000031.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// ihPal (4 bytes): An unsigned integer that specifies the index of the
    /// logical palette object in the EMF object table. This index MUST be
    /// saved so that this object can be reused or modified.
    pub ih_pal: u32,
    /// LogPalette (variable): A LogPalette object. The Version field of this
    /// object MUST be set to 0x0300. If the NumberOfEntries value in this
    /// object is zero, processing of this record MUST fail.
    pub log_palette: crate::parser::LogPalette,
}

impl EMR_CREATEPALETTE {
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
            consume_remaining_bytes, read_field, read_with,
        };

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_CREATEPALETTE as u32,
        )?;

        let ih_pal: u32 = read_field(buf, &mut size)?;
        let log_palette =
            read_with(buf, &mut size, crate::parser::LogPalette::parse)?;

        crate::parser::ParseError::expect_ne(
            "log_palette.number_of_entries",
            log_palette.number_of_entries,
            0_u16,
        )?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, ih_pal, log_palette })
    }
}
