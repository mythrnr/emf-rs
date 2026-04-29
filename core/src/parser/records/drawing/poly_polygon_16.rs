use crate::imports::*;

/// The EMR_POLYPOLYGON16 record specifies a series of closed polygons. Each
/// polygon is outlined using the current pen, and filled using the current
/// brush and polygon fill mode. The polygons drawn by this record can overlap.
#[derive(Clone, Debug)]
pub struct EMR_POLYPOLYGON16 {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_POLYPOLYGON16. This value is 0x0000005B.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19), which
    /// specifies the bounding rectangle in logical units.
    pub bounds: wmf_core::parser::RectL,
    /// NumberOfPolygons (4 bytes): An unsigned integer that specifies the
    /// number of polygons.
    pub number_of_polygons: u32,
    /// Count (4 bytes): An unsigned integer that specifies the total number of
    /// points in all polygons.
    pub count: u32,
    /// PolygonPointCount (variable): A NumberOfPolygons length array of 32-bit
    /// unsigned integers that specifies the point counts for each polygon.
    pub polygon_point_count: Vec<u32>,
    /// aPoints (variable): A Count length array of PointS objects ([MS-WMF]
    /// section 2.2.2.16), which specifies the array of points.
    pub a_points: Vec<wmf_core::parser::PointS>,
}

impl EMR_POLYPOLYGON16 {
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
            crate::parser::RecordType::EMR_POLYPOLYGON16 as u32,
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
            number_of_polygons,
            count,
            polygon_point_count,
            a_points,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{RecordType, Size};

    fn record_size(byte_count: u32) -> Size {
        let mut size = Size::from(byte_count);
        size.consume(8);
        size
    }

    #[test]
    fn rejects_count_above_max_total_points() {
        // Build a payload where `count` exceeds `MAX_TOTAL_POINTS`.
        // Record bytes: 8 (header) + 16 (bounds) + 4 (number_of_polygons)
        // + 4 (count). The points arrays do not need to be present in
        // the buffer because the bound check fires before they are
        // read.
        let oversized: u32 = crate::parser::records::MAX_TOTAL_POINTS + 1;
        let mut payload = Vec::with_capacity(24);
        payload.extend_from_slice(&[0_u8; 16]); // bounds (zeroed)
        payload.extend_from_slice(&1_u32.to_le_bytes()); // number_of_polygons
        payload.extend_from_slice(&oversized.to_le_bytes()); // count

        let mut buf: &[u8] = &payload;
        let result = EMR_POLYPOLYGON16::parse(
            &mut buf,
            RecordType::EMR_POLYPOLYGON16,
            record_size(8 + 24),
        );
        assert!(result.is_err(), "oversized count should be rejected");
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("total point count"),
            "diagnostic should mention the bound: {msg}",
        );
    }

    #[test]
    fn rejects_polygon_point_count_sum_overflow() {
        // Build a payload where polygon_point_count entries sum to
        // exceed `count`. The bound check fires before allocating the
        // points array.
        let mut payload = Vec::with_capacity(36);
        payload.extend_from_slice(&[0_u8; 16]); // bounds
        payload.extend_from_slice(&2_u32.to_le_bytes()); // number_of_polygons
        payload.extend_from_slice(&3_u32.to_le_bytes()); // count = 3
        // Two entries summing to 5 (> count of 3).
        payload.extend_from_slice(&3_u32.to_le_bytes());
        payload.extend_from_slice(&2_u32.to_le_bytes());

        let mut buf: &[u8] = &payload;
        let result = EMR_POLYPOLYGON16::parse(
            &mut buf,
            RecordType::EMR_POLYPOLYGON16,
            record_size(8 + 32),
        );
        assert!(
            result.is_err(),
            "polygon_point_count sum > count must be rejected",
        );
    }
}
