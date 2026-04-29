/// The EMR_COLORCORRECTPALETTE record specifies the correction of entries of a
/// logical palette object using WCS.
///
/// Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, and Windows NT 4.0 do not
/// support EMR_COLORCORRECTPALETTE.
#[derive(Clone, Debug)]
pub struct EMR_COLORCORRECTPALETTE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_COLORCORRECTPALETTE. This value is 0x0000006F.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes. This value is 0x00000018.
    pub size: crate::parser::Size,
    /// ihPalette (4 bytes): An unsigned integer that specifies the index of a
    /// logical palette object in the EMF object table.
    pub ih_palette: u32,
    /// nFirstEntry (4 bytes): An unsigned integer that specifies the index of
    /// the first entry to correct.
    pub n_first_entry: u32,
    /// nPalEntries (4 bytes): An unsigned integer that specifies the number of
    /// palette entries to correct.
    pub n_pal_entries: u32,
    /// nReserved (4 bytes): An unsigned integer that is undefined and unused.
    pub n_reserved: u32,
}

impl EMR_COLORCORRECTPALETTE {
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
        use crate::parser::records::{consume_remaining_bytes, read_field};

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_COLORCORRECTPALETTE as u32,
        )?;
        crate::parser::ParseError::expect_eq(
            "size field",
            size.byte_count() as u32,
            0x00000018_u32,
        )?;

        let ih_palette = read_field(buf, &mut size)?;
        let n_first_entry = read_field(buf, &mut size)?;
        let n_pal_entries = read_field(buf, &mut size)?;
        let n_reserved = read_field(buf, &mut size)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self {
            record_type,
            size,
            ih_palette,
            n_first_entry,
            n_pal_entries,
            n_reserved,
        })
    }
}
