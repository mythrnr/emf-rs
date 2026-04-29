use crate::imports::*;

/// The EMR_POLYPOLYGON record specifies a series of closed polygons.
///
/// Each polygon SHOULD be outlined using the current pen, and filled using the
/// current brush and polygon fill mode that are defined in the playback device
/// context. The polygons defined by this record can overlap.
#[derive(Clone, Debug)]
pub struct EMR_POLYPOLYGON {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_POLYPOLYGON. This value is 0x00000008.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19) that
    /// specifies the bounding rectangle in logical units.
    pub bounds: wmf_core::parser::RectL,
    /// NumberOfPolygons (4 bytes): An unsigned integer that specifies the
    /// number of polygons.
    pub number_of_polygons: u32,
    /// Count (4 bytes): An unsigned integer that specifies the total number of
    /// points in all polygons.
    ///
    /// | Line width | Device supports wideline | Maximum points allowed |
    /// |:-|:-|:-|
    /// | `1` | `n/a` | `16K` |
    /// | `> 1` | `yes` | `16K` |
    /// | `> 1` | `no` | `1360` |
    ///
    /// Any extra points MUST be ignored. To draw a line with more points, the
    /// data SHOULD be divided into groups that have less than the maximum
    /// number of points, and an EMR_POLYPOLYGON operation SHOULD be performed
    /// for each group of points.
    pub count: u32,
    /// PolygonPointCount (variable): An array of 32-bit unsigned integers that
    /// specifies the point count for each polygon.
    pub polygon_point_count: Vec<u32>,
    /// aPoints (variable): An array of PointL objects ([MS-WMF] section
    /// 2.2.2.15) that specifies the points for all polygons in logical units.
    /// The number of points is specified by the Count field value.
    pub a_points: Vec<wmf_core::parser::PointL>,
}

impl EMR_POLYPOLYGON {
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
            crate::parser::RecordType::EMR_POLYPOLYGON as u32,
        )?;

        let bounds = read_with(buf, &mut size, wmf_core::parser::RectL::parse)?;
        let number_of_polygons = read_field(buf, &mut size)?;
        let count = read_field(buf, &mut size)?;

        check_total_points(number_of_polygons)?;
        check_total_points(count)?;

        let polygon_point_count = {
            let mut entries: Vec<u32> =
                Vec::with_capacity(number_of_polygons as usize);

            for _ in 0..number_of_polygons {
                entries.push(read_field(buf, &mut size)?);
            }

            entries
        };

        check_polygon_point_count_sum(&polygon_point_count, count)?;

        let a_points = {
            let mut entries = Vec::with_capacity(count as usize);

            for _ in 0..count {
                entries.push(read_with(
                    buf,
                    &mut size,
                    wmf_core::parser::PointL::parse,
                )?);
            }

            entries
        };

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self {
            record_type,
            size,
            bounds,
            number_of_polygons,
            count,
            polygon_point_count,
            a_points,
        })
    }
}
