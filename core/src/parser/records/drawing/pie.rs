/// The EMR_PIE record specifies a pie-shaped wedge bounded by the intersection
/// of an ellipse and two radials. The pie is outlined by using the current pen
/// and filled by using the current brush.
///
/// The curve of the pie is defined by an ellipse that fits the specified
/// bounding rectangle. The curve begins at the point where the ellipse
/// intersects the first radial and extends counterclockwise to the point where
/// the ellipse intersects the second radial.
///
/// The current drawing position is neither used nor updated by this record.
#[derive(Clone, Debug)]
pub struct EMR_PIE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_PIE. This value is 0x0000002F.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Box (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19), which
    /// specifies the inclusive- inclusive bounding rectangle in logical units.
    pub bx: wmf_core::parser::RectL,
    /// Start (8 bytes): A PointL object ([MS-WMF] section 2.2.2.15), which
    /// specifies the coordinates, in logical units, of the endpoint of the
    /// first radial.
    pub start: wmf_core::parser::PointL,
    /// End (8 bytes): A PointL object that specifies the coordinates, in
    /// logical units, of the endpoint of the second radial.
    pub end: wmf_core::parser::PointL,
}

impl EMR_PIE {
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
            crate::parser::RecordType::EMR_PIE as u32,
        )?;

        let bx = read_with(buf, &mut size, wmf_core::parser::RectL::parse)?;
        let start = read_with(buf, &mut size, wmf_core::parser::PointL::parse)?;
        let end = read_with(buf, &mut size, wmf_core::parser::PointL::parse)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, bx, start, end })
    }
}
