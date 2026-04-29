use crate::imports::*;

/// The EMR_POLYBEZIERTO16 record specifies one or more Bezier curves based on
/// the current drawing position.
#[derive(Clone, Debug)]
pub struct EMR_POLYBEZIERTO16 {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_POLYBEZIERTO16. This value is 0x00000058.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19) that
    /// specifies the bounding rectangle in logical units.
    pub bounds: wmf_core::parser::RectL,
    /// Count (4 bytes): An unsigned integer that specifies the total number of
    /// points. The first curve is drawn from the current position to the third
    /// point by using the first two points as control points. For each
    /// subsequent curve, three more points MUST be specified, and the ending
    /// point of the previous curve MUST be used as the starting point for the
    /// next.
    pub count: u32,
    /// aPoints (variable): An array of PointS objects ([MS-WMF] section
    /// 2.2.2.16), which specify the points of the Bezier curves in logical
    /// units.
    pub a_points: Vec<wmf_core::parser::PointS>,
}

impl EMR_POLYBEZIERTO16 {
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
            crate::parser::RecordType::EMR_POLYBEZIERTO16 as u32,
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
