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
        let (
            (size, size_bytes),
            (typ, typ_bytes),
            (count_rects, count_rects_bytes),
            (rgn_size, rgn_size_bytes),
            (bounds, bounds_bytes),
        ) = (
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            wmf_core::parser::RectL::parse(buf)?,
        );

        if size != 0x00000020 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "size field must be `0x00000020`, but parsed value is \
                     {size:#010X}"
                ),
            });
        }

        if typ != 0x00000001 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "type field must be `0x00000001`, but parsed value is \
                     {typ:#010X}"
                ),
            });
        }

        Ok((
            Self { size, typ, count_rects, rgn_size, bounds },
            size_bytes
                + typ_bytes
                + count_rects_bytes
                + rgn_size_bytes
                + bounds_bytes,
        ))
    }
}
