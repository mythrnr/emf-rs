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
        if record_type != crate::parser::RecordType::EMR_POLYPOLYGON16 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_POLYPOLYGON16 as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (bounds, bounds_bytes),
            (number_of_polygons, number_of_polygons_bytes),
            (count, count_bytes),
        ) = (
            wmf_core::parser::RectL::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        size.consume(bounds_bytes + number_of_polygons_bytes + count_bytes);

        let polygon_point_count = {
            let mut entries = vec![];

            for _ in 0..number_of_polygons {
                let (v, b) = crate::parser::read_u32_from_le_bytes(buf)?;

                entries.push(v);
                size.consume(b);
            }

            entries
        };

        let a_points = {
            let mut entries = vec![];

            for _ in 0..count {
                let (v, b) = wmf_core::parser::PointS::parse(buf)?;

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
            number_of_polygons,
            count,
            polygon_point_count,
            a_points,
        })
    }
}
