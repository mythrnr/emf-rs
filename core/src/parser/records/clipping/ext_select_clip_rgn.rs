/// The EMR_EXTSELECTCLIPRGN record combines the specified region with the
/// current clipping region using the specified mode.
#[derive(Clone, Debug)]
pub struct EMR_EXTSELECTCLIPRGN {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_EXTSELECTCLIPRGN. This value is 0x0000004B.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: u32,
    /// RgnDataSize (4 bytes): An unsigned integer that specifies the size of
    /// the RgnData field in bytes.
    pub rgn_data_size: u32,
    /// RegionMode (4 bytes): An unsigned integer that specifies the way to use
    /// the region. This value is in the RegionMode enumeration.
    pub region_mode: crate::parser::RegionMode,
    /// RgnData (variable): An array of bytes that specifies a RegionData
    /// object in logical units. If RegionMode is RGN_COPY, this data can be
    /// omitted and the clipping region SHOULD be set to the default clipping
    /// region.
    pub rgn_data: Vec<crate::parser::RegionData>,
}

impl EMR_EXTSELECTCLIPRGN {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_EXTSELECTCLIPRGN {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_EXTSELECTCLIPRGN as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (size, size_bytes),
            (rgn_data_size, rgn_data_size_bytes),
            (region_mode, region_mode_bytes),
        ) = (
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::RegionMode::parse(buf)?,
        );

        let mut consumed_bytes =
            size_bytes + rgn_data_size_bytes + region_mode_bytes;

        let (rgn_data, rgn_data_bytes) = {
            let mut entries = vec![];
            let mut bytes = 0;

            for _ in 0..rgn_data_size {
                let (v, b) = crate::parser::RegionData::parse(buf)?;

                entries.push(v);
                bytes += b;
            }

            (entries, bytes)
        };

        consumed_bytes += rgn_data_bytes;

        crate::parser::records::consume_remaining_bytes(
            buf,
            size as usize - consumed_bytes,
        )?;

        Ok(Self { record_type, size, rgn_data_size, region_mode, rgn_data })
    }
}
