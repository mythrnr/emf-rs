/// The EMR_POLYBEZIER16 record specifies one or more Bezier curves. The curves
/// are drawn using the current pen.
#[derive(Clone, Debug)]
pub struct EMR_POLYBEZIER16 {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_POLYBEZIER16. This value is 0x00000055.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19), which
    /// specifies the inclusive-inclusive bounding rectangle in logical units.
    pub bounds: wmf_core::parser::RectL,
    /// Count (4 bytes): An unsigned integer that specifies the total number of
    /// points. This value MUST be one more than three times the number of
    /// curves to be drawn because each Bezier curve requires two control
    /// points and an endpoint, and the initial curve requires an additional
    /// starting point.
    pub count: u32,
    /// aPoints (variable): An array of PointS objects ([MS-WMF] section
    /// 2.2.2.16), which specify the points of the Bezier curves in logical
    /// units.
    pub a_points: Vec<wmf_core::parser::PointS>,
}

impl EMR_POLYBEZIER16 {
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
        if record_type != crate::parser::RecordType::EMR_POLYBEZIER16 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_POLYBEZIER16 as u32,
                    record_type as u32
                ),
            });
        }

        let ((bounds, bounds_bytes), (count, count_bytes)) = (
            wmf_core::parser::RectL::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        size.consume(bounds_bytes + count_bytes);

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

        Ok(Self { record_type, size, bounds, count, a_points })
    }
}