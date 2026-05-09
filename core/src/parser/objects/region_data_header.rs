/// The RegionDataHeader object defines the properties of a RegionData object.
#[derive(Clone, Debug)]
pub struct RegionDataHeader {
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// object in bytes. This value is 0x00000020.
    pub size: u32,
    /// Type (4 bytes): An unsigned integer that specifies the region type.
    /// This value is 0x00000001.
    pub typ: u32,
    /// CountRects (4 bytes): An unsigned integer that specifies the number of
    /// rectangles in this region.
    pub count_rects: u32,
    /// RgnSize (4 bytes): An unsigned integer that specifies the size of the
    /// buffer of rectangles in bytes.
    pub rgn_size: u32,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19), which
    /// specifies the bounds of the region.
    pub bounds: wmf_core::parser::RectL,
}

impl RegionDataHeader {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let size = read_field(buf, &mut consumed_bytes)?;
        let typ = read_field(buf, &mut consumed_bytes)?;
        let count_rects = read_field(buf, &mut consumed_bytes)?;
        let rgn_size = read_field(buf, &mut consumed_bytes)?;
        let bounds = read_with(
            buf,
            &mut consumed_bytes,
            wmf_core::parser::RectL::parse,
        )?;

        crate::parser::ParseError::expect_eq(
            "size (RegionDataHeader)",
            size,
            0x00000020_u32,
        )?;
        crate::parser::ParseError::expect_eq(
            "type (RegionDataHeader)",
            typ,
            0x00000001_u32,
        )?;

        Ok((Self { size, typ, count_rects, rgn_size, bounds }, consumed_bytes))
    }
}
