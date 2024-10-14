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
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_POLYPOLYLINE16 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_POLYPOLYLINE16 as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (bounds, bounds_bytes),
            (number_of_polylines, number_of_polylines_bytes),
            (count, count_bytes),
        ) = (
            wmf_core::parser::RectL::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        size.consume(bounds_bytes + number_of_polylines_bytes + count_bytes);

        let polyline_point_count = {
            let mut entries = vec![];

            for _ in 0..number_of_polylines {
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
            number_of_polylines,
            count,
            polyline_point_count,
            a_points,
        })
    }
}
