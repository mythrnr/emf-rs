use crate::imports::*;

/// The RegionData object specifies data that defines a region, which is made of
/// non-overlapping rectangles.
#[derive(Clone, Debug)]
pub struct RegionData {
    /// RegionDataHeader (32 bytes): A 256-bit RegionDataHeader object that
    /// defines the contents of the Data field.
    pub region_data_header: crate::parser::RegionDataHeader,
    /// Data (variable): An array of RectL objects ([MS-WMF] section 2.2.2.19);
    /// the objects are merged to create the region.
    pub data: Vec<wmf_core::parser::RectL>,
}

impl RegionData {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{check_total_points, read_with};

        let mut consumed_bytes: usize = 0;
        let region_data_header = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::RegionDataHeader::parse,
        )?;

        // `count_rects` is unbounded in the spec, so reject crafted
        // values up front before they drive `Vec::with_capacity`.
        // Reuses the polygon point-count cap (16 Mi); a single RectL
        // is 16 bytes, so this bound also stays well below
        // `MAX_RECORD_BYTES`.
        check_total_points(region_data_header.count_rects)?;

        let data = {
            let mut entries =
                Vec::with_capacity(region_data_header.count_rects as usize);

            for _ in 0..region_data_header.count_rects {
                entries.push(read_with(
                    buf,
                    &mut consumed_bytes,
                    wmf_core::parser::RectL::parse,
                )?);
            }

            entries
        };

        Ok((Self { region_data_header, data }, consumed_bytes))
    }
}
