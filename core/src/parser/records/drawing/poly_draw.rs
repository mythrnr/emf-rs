/// The EMR_POLYDRAW record specifies a set of line segments and Bezier curves.
#[derive(Clone, Debug)]
pub struct EMR_POLYDRAW {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_POLYDRAW. This value is 0x00000038.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object, specified in [MS-WMF] section
    /// 2.2.2.19, which specifies the bounding rectangle in logical units.
    pub bounds: wmf_core::parser::RectL,
    /// Count (4 bytes): An unsigned integer that specifies the number of
    /// points in the aPoints field.
    pub count: u32,
    /// aPoints (variable): An array of PointL objects ([MS-WMF] section
    /// 2.2.2.15), which specify the points in logical units.
    pub a_points: Vec<wmf_core::parser::PointL>,
    /// abTypes (variable): A Count length array of byte values that specifies
    /// how each point in the aPoints array is used. This value is in the Point
    /// enumeration.
    pub ab_types: Vec<crate::parser::Point>,
}

impl EMR_POLYDRAW {
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
        if record_type != crate::parser::RecordType::EMR_POLYDRAW {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_POLYDRAW as u32,
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

        let ab_types = {
            let mut entries = vec![];

            for _ in 0..count {
                let (v, b) = crate::parser::Point::parse(buf)?;

                entries.push(v);
                size.consume(b);
            }

            entries
        };

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, bounds, count, a_points, ab_types })
    }
}
