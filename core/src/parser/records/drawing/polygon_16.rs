use crate::imports::*;

/// The EMR_POLYGON16 record specifies a polygon consisting of two or more
/// vertexes connected by straight lines. The polygon is outlined by using the
/// current pen and filled by using the current brush and polygon fill mode. The
/// polygon is closed automatically by drawing a line from the last vertex to
/// the first.
#[derive(Clone, Debug)]
pub struct EMR_POLYGON16 {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_POLYGON16. This value is 0x00000056.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object, specified in [MS-WMF] section
    /// 2.2.2.19, which specifies the bounding rectangle in logical units.
    pub bounds: wmf_core::parser::RectL,
    /// Count (4 bytes): An unsigned integer that specifies the total number of
    /// points.
    pub count: u32,
    /// aPoints (variable): A Count length array of PointS objects, specified
    /// in [MS-WMF] section 2.2.2.16, which specifies the array of points.
    pub a_points: Vec<wmf_core::parser::PointS>,
}

impl EMR_POLYGON16 {
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
            crate::parser::RecordType::EMR_POLYGON16 as u32,
        )?;

        let bounds = read_with(buf, &mut size, wmf_core::parser::RectL::parse)?;
        let count = read_field(buf, &mut size)?;

        check_total_points(count)?;

        let a_points = {
            let mut entries = Vec::with_capacity(count as usize);

            for _ in 0..count {
                entries.push(read_with(
                    buf,
                    &mut size,
                    wmf_core::parser::PointS::parse,
                )?);
            }

            entries
        };

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, bounds, count, a_points })
    }
}
