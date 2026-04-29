/// The EMR_CHORD record specifies a chord, which is a region bounded by the
/// intersection of an ellipse and a line segment, called a secant. The chord is
/// outlined by using the current pen and filled by using the current brush.
///
/// The curve of the chord is defined by an ellipse that fits the specified
/// bounding rectangle. The curve begins at the point where the ellipse
/// intersects the first radial and extends counterclockwise to the point where
/// the ellipse intersects the second radial. The chord is closed by drawing a
/// line from the intersection of the first radial and the curve to the
/// intersection of the second radial and the curve.
///
/// If the starting point and ending point of the curve are the same, a complete
/// ellipse is drawn.
///
/// The current drawing position is neither used nor updated by processing this
/// record.
#[derive(Clone, Debug)]
pub struct EMR_CHORD {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_CHORD. This value is 0x0000002E.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Box (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19), which
    /// specifies the inclusive-inclusive bounding rectangle in logical units.
    pub bx: wmf_core::parser::RectL,
    /// Start (8 bytes): A PointL object ([MS-WMF] section 2.2.2.15), which
    /// specifies the coordinates, in logical units, of the endpoint of the
    /// radial defining the beginning of the chord.
    pub start: wmf_core::parser::PointL,
    /// End (8 bytes): A PointL object that specifies the logical coordinates
    /// of the endpoint of the radial defining the end of the chord.
    pub end: wmf_core::parser::PointL,
}

impl EMR_CHORD {
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
            crate::parser::RecordType::EMR_CHORD as u32,
        )?;

        let bx = read_with(buf, &mut size, wmf_core::parser::RectL::parse)?;
        let start = read_with(buf, &mut size, wmf_core::parser::PointL::parse)?;
        let end = read_with(buf, &mut size, wmf_core::parser::PointL::parse)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, bx, start, end })
    }
}
