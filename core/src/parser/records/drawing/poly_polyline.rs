use crate::imports::*;

/// The EMR_POLYPOLYLINE record draws multiple series of connected line
/// segments.
///
/// The line segments SHOULD be drawn using the current pen. The figures formed
/// by the segments SHOULD NOT filled. The current drawing position SHOULD
/// neither be used nor updated by this record.
#[derive(Clone, Debug)]
pub struct EMR_POLYPOLYLINE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_POLYPOLYLINE. This value is 0x00000007.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19) that
    /// specifies the bounding rectangle in logical units.
    pub bounds: wmf_core::parser::RectL,
    /// NumberOfPolylines (4 bytes): An unsigned integer that specifies the
    /// number of polylines, which is the number of elements in the
    /// aPolylinePointCount array.
    pub number_of_polylines: u32,
    /// Count (4 bytes): An unsigned integer that specifies the total number of
    /// points in all polylines, which is the number of elements in the aPoints
    /// array.
    ///
    /// | Line width | Device supports wideline | Maximum points allowed |
    /// |:-|:-|:-|
    /// | `1` | `n/a` | `16K` |
    /// | `> 1` | `yes` | `16K` |
    /// | `> 1` | `no` | `1360` |
    ///
    /// Any extra points MUST be ignored.
    pub count: u32,
    /// aPolylinePointCount (variable): A NumberOfPolylines-length array of
    /// 32-bit unsigned integers that specify the point counts for all
    /// polylines. Each value MUST be >= 0x00000002.
    ///
    /// Each point count refers to a number of consecutive elements in the
    /// aPoints array.
    pub a_polyline_point_count: Vec<u32>,
    /// aPoints (variable): A Count-length array of PointL objects ([MS-WMF]
    /// section 2.2.2.15) that specify the point data, in logical units.
    pub a_points: Vec<wmf_core::parser::PointL>,
}

impl EMR_POLYPOLYLINE {
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
        if record_type != crate::parser::RecordType::EMR_POLYPOLYLINE {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_POLYPOLYLINE as u32,
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

        let a_polyline_point_count = {
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
                let (v, b) = wmf_core::parser::PointL::parse(buf)?;

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
            a_polyline_point_count,
            a_points,
        })
    }
}
