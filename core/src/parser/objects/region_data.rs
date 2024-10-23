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
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (region_data_header, region_data_header_bytes) =
            crate::parser::RegionDataHeader::parse(buf)?;
        let (data, data_bytes) = {
            let mut entries = vec![];
            let mut bytes = 0;

            for _ in 0..region_data_header.count_rects {
                let (v, b) = wmf_core::parser::RectL::parse(buf)?;

                entries.push(v);
                bytes += b;
            }

            (entries, bytes)
        };

        Ok((
            Self { region_data_header, data },
            region_data_header_bytes + data_bytes,
        ))
    }
}
