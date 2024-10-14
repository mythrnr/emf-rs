/// The EMR_POLYLINETO16 record specifies one or more straight lines based upon
/// the current drawing position.
///
/// A line is drawn from the current drawing position to the first point
/// specified by the aPoints field by using the current pen. For each additional
/// line, drawing is performed from the ending point of the previous line to the
/// next point specified by aPoints.
#[derive(Clone, Debug)]
pub struct EMR_POLYLINETO16 {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_POLYLINETO16. This value is 0x00000059.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19, which
    /// specifies the bounding rectangle in logical units.
    pub bounds: wmf_core::parser::RectL,
    /// Count (4 bytes): An unsigned integer that specifies the number of
    /// points.
    pub count: u32,
    /// aPoints (variable): A Count length array of PointS objects ([MS-WMF]
    /// section 2.2.2.16, which specifies the array of points.
    pub a_points: Vec<wmf_core::parser::PointS>,
}

impl EMR_POLYLINETO16 {
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
        if record_type != crate::parser::RecordType::EMR_POLYLINETO16 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_POLYLINETO16 as u32,
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
