use crate::imports::*;

/// The EMR_EXTSELECTCLIPRGN record combines the specified region with the
/// current clipping region using the specified mode.
#[derive(Clone, Debug)]
pub struct EMR_EXTSELECTCLIPRGN {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_EXTSELECTCLIPRGN. This value is 0x0000004B.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
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
            check_total_points, consume_remaining_bytes, read_field, read_with,
        };

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_EXTSELECTCLIPRGN as u32,
        )?;

        let rgn_data_size: u32 = read_field(buf, &mut size)?;
        let region_mode =
            read_with(buf, &mut size, crate::parser::RegionMode::parse)?;

        // Cap `rgn_data_size` so a crafted u32::MAX cannot drive the
        // loop to exhaustion. Pre-allocating is intentionally skipped
        // because a single `RegionData` owns a nested `Vec<RectL>`.
        check_total_points(rgn_data_size)?;

        let rgn_data = {
            let mut entries = vec![];

            for _ in 0..rgn_data_size {
                entries.push(read_with(
                    buf,
                    &mut size,
                    crate::parser::RegionData::parse,
                )?);
            }

            entries
        };

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, rgn_data_size, region_mode, rgn_data })
    }
}
