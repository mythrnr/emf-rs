/// The EMR_FRAMERGN record draws a border around the specified region using the
/// specified brush. The current clipping regions used by this record are
/// maintained in a Regions state element in the playback device context.
#[derive(Clone, Debug)]
pub struct EMR_FRAMERGN {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_FRAMERGN. This value is 0x00000048.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19) that
    /// specifies the destination bounding rectangle in logical coordinates. If
    /// the intersection of this rectangle with the current clipping region is
    /// empty, this record has no effect.
    pub bounds: wmf_core::parser::RectL,
    /// RgnDataSize (4 bytes): An unsigned integer that specifies the size of
    /// region data in bytes.
    pub rgn_data_size: u32,
    /// ihBrush (4 bytes): An unsigned integer that specifies the index of the
    /// brush in the EMF object table index.
    pub ih_brush: u32,
    /// Width (4 bytes): A signed integer that specifies the width of the
    /// vertical brush stroke, in logical units.
    pub width: i32,
    /// Height (4 bytes): A signed integer that specifies the height of the
    /// horizontal brush stroke, in logical units.
    pub height: i32,
    /// RgnData (variable): A RgnDataSize length array of bytes that specifies
    /// the output region in a RegionData object. The bounds specified by the
    /// RegionDataHeader field of this object MAY(The Windows playback
    /// implementation computes the bounding region from the sum of all the
    /// rectangles specified by RgnData field.) be used as the bounding region
    /// when this record is processed.
    pub rgn_data: Vec<crate::parser::RegionData>,
}

impl EMR_FRAMERGN {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_FRAMERGN {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_FRAMERGN as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (bounds, bounds_bytes),
            (rgn_data_size, rgn_data_size_bytes),
            (ih_brush, ih_brush_bytes),
            (width, width_bytes),
            (height, height_bytes),
        ) = (
            wmf_core::parser::RectL::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
        );

        size.consume(
            bounds_bytes
                + rgn_data_size_bytes
                + ih_brush_bytes
                + width_bytes
                + height_bytes,
        );

        let rgn_data = {
            let mut entries = vec![];

            for _ in 0..rgn_data_size {
                let (v, b) = crate::parser::RegionData::parse(buf)?;

                entries.push(v);
                size.consume(b);
            }

            entries
        };

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self {
            record_type,
            size,
            bounds,
            rgn_data_size,
            ih_brush,
            width,
            height,
            rgn_data,
        })
    }
}
