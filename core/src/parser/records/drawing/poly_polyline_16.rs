use crate::imports::*;

/// The EMR_POLYPOLYLINE16 record specifies multiple series of connected line
/// segments.
#[derive(Clone, Debug)]
pub struct EMR_POLYPOLYLINE16 {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_POLYPOLYLINE16. This value is 0x0000005A.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19), which
    /// specifies the bounding rectangle in logical units.
    pub bounds: wmf_core::parser::RectL,
    /// NumberOfPolylines (4 bytes): An unsigned integer that specifies the
    /// number of polylines.
    pub number_of_polylines: u32,
    /// Count (4 bytes): An unsigned integer that specifies the total number of
    /// points in all polylines.
    pub count: u32,
    /// PolylinePointCount (variable): A NumberOfPolylines length array of
    /// 32-bit unsigned integers that specifies the point counts for each
    /// polyline.
    pub polyline_point_count: Vec<u32>,
    /// aPoints (variable): A Count length array of PointS objects ([MS-WMF]
    /// section 2.2.2.16), which specifies the array of points.
    pub a_points: Vec<wmf_core::parser::PointS>,
}

impl EMR_POLYPOLYLINE16 {
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
        use crate::parser::records::{
            check_polygon_point_count_sum, check_total_points,
            consume_remaining_bytes, read_field, read_with,
        };

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_POLYPOLYLINE16 as u32,
        )?;

        let bounds = read_with(buf, &mut size, wmf_core::parser::RectL::parse)?;
        let number_of_polylines = read_field(buf, &mut size)?;
        let count = read_field(buf, &mut size)?;

        check_total_points(number_of_polylines)?;
        check_total_points(count)?;

        let polyline_point_count = {
            let mut entries: Vec<u32> = vec![];

            for _ in 0..number_of_polylines {
                entries.push(read_field(buf, &mut size)?);
            }

            entries
        };

        check_polygon_point_count_sum(&polyline_point_count, count)?;

        let a_points = {
            let mut entries = vec![];

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

        Ok(Self {
            record_type,
            size,
            bounds,
            number_of_polylines,
            count,
            polyline_point_count,
            a_points,
        })
    }
}
