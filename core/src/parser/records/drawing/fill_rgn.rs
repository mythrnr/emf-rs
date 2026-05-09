use crate::imports::*;

/// The EMR_FILLRGN record fills the specified region by using the specified
/// brush. The current clipping regions used by this record are maintained in a
/// Regions state element in the playback device context.
#[derive(Clone, Debug)]
pub struct EMR_FILLRGN {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_FILLRGN. This value is 0x00000047.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19) that
    /// specifies the destination bounding rectangle in logical units. If the
    /// intersection of this rectangle with the current clipping region is
    /// empty, this record has no effect.
    pub bounds: wmf_core::parser::RectL,
    /// RgnDataSize (4 bytes): An unsigned integer that specifies the size of
    /// region data in bytes.
    pub rgn_data_size: u32,
    /// ihBrush (4 bytes): An unsigned integer that specifies the index of the
    /// brush in the EMF object table for filling the region.
    pub ih_brush: u32,
    /// RgnData (variable): A RgnDataSize length array of bytes that specifies
    /// the output region in a RegionData object. The bounds specified by the
    /// RegionDataHeader field of this object MAY(The Windows playback
    /// implementation computes the bounding region from the sum of all the
    /// rectangles specified by the RegionData object in the RgnData field.) be
    /// used as the bounding region when this record is processed.
    pub rgn_data: Vec<crate::parser::RegionData>,
}

impl EMR_FILLRGN {
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
            crate::parser::RecordType::EMR_FILLRGN as u32,
        )?;

        let bounds = read_with(buf, &mut size, wmf_core::parser::RectL::parse)?;
        let rgn_data_size: u32 = read_field(buf, &mut size)?;
        let ih_brush = read_field(buf, &mut size)?;

        // Cap `rgn_data_size` so a crafted u32::MAX cannot drive the
        // loop to exhaustion. Pre-allocating `Vec::with_capacity` is
        // intentionally skipped: a single `RegionData` already owns
        // a nested `Vec<RectL>`, so 16 Mi entries would still exceed
        // the record-size cap.
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

        Ok(Self {
            record_type,
            size,
            bounds,
            rgn_data_size,
            ih_brush,
            rgn_data,
        })
    }
}
