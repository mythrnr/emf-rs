/// The EMR_EXTFLOODFILL record fills an area of the display surface with the
/// current brush.
#[derive(Clone, Debug)]
pub struct EMR_EXTFLOODFILL {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_EXTFLOODFILL. This value is 0x00000035.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Start (8 bytes): A PointL object ([MS-WMF] section 2.2.2.15), which
    /// specifies the coordinates, in logical units, where filling begins.
    pub start: wmf_core::parser::PointL,
    /// Color (4 bytes): A ColorRef object ([MS-WMF] section 2.2.2.8), which is
    /// used with the FloodFillMode to determine the area to fill.
    pub color: wmf_core::parser::ColorRef,
    /// FloodFillMode (4 bytes): An unsigned integer that specifies how to use
    /// the Color value to determine the area for the flood fill operation.
    /// This value is in the FloodFill enumeration.
    pub flood_fill_mode: crate::parser::FloodFill,
}

impl EMR_EXTFLOODFILL {
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
        use crate::parser::records::{consume_remaining_bytes, read_with};

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_EXTFLOODFILL as u32,
        )?;

        let start = read_with(buf, &mut size, wmf_core::parser::PointL::parse)?;
        let color =
            read_with(buf, &mut size, wmf_core::parser::ColorRef::parse)?;
        let flood_fill_mode =
            read_with(buf, &mut size, crate::parser::FloodFill::parse)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, start, color, flood_fill_mode })
    }
}
