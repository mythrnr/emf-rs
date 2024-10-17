/// The EMR_POLYBEZIERTO record specifies one or more Bezier curves based upon
/// the current drawing position.
#[derive(Clone, Debug)]
pub struct EMR_POLYBEZIERTO {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_POLYBEZIERTO. This value is 0x00000005.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19) that
    /// specifies the bounding rectangle in logical units.
    pub bounds: wmf_core::parser::RectL,
    /// Count (4 bytes): An unsigned integer that specifies the number of
    /// points in the aPoints array. The first curve MUST be drawn from the
    /// current position to the third point by using the first two points as
    /// control points. For each subsequent curve, exactly three more points
    /// MUST be specified, and the ending point of the previous curve MUST be
    /// used as the starting point for the next.
    ///
    /// | Line width | Device supports wideline | Maximum points allowed |
    /// |:-|:-|:-|
    /// | `1` | `n/a` | `16K` |
    /// | `> 1` | `yes` | `16K` |
    /// | `> 1` | `no` | `1360` |
    ///
    /// Any extra points MUST be ignored.
    pub count: u32,
    /// aPoints (variable): An array of PointL objects ([MS-WMF] section
    /// 2.2.2.15), which specify the endpoints and control points of the Bezier
    /// curves in logical units.
    pub a_points: Vec<wmf_core::parser::PointL>,
}

impl EMR_POLYBEZIERTO {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_POLYBEZIERTO {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_POLYBEZIERTO as u32,
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

        Ok(Self { record_type, size, bounds, count, a_points })
    }
}
