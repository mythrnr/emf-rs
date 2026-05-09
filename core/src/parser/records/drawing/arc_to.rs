/// The EMR_ARCTO record specifies an elliptical arc. It resets the current
/// drawing position to the endpoint of the arc.
#[derive(Clone, Debug)]
pub struct EMR_ARCTO {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_ARCTO. This value is 0x00000037.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Box (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19), which
    /// specifies the inclusive-inclusive bounding rectangle in logical units.
    pub bx: wmf_core::parser::RectL,
    /// Start (8 bytes): A PointL object ([MS-WMF] section 2.2.2.15), which
    /// specifies the coordinates, in logical units, of the first radial ending
    /// point, in logical units.
    pub start: wmf_core::parser::PointL,
    /// End (8 bytes): A PointL object that specifies the coordinates of the
    /// second radial ending point, in logical units.
    pub end: wmf_core::parser::PointL,
}

impl EMR_ARCTO {
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
        use crate::parser::records::{consume_remaining_bytes, read_with};

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_ARCTO as u32,
        )?;

        let bx = read_with(buf, &mut size, wmf_core::parser::RectL::parse)?;
        let start = read_with(buf, &mut size, wmf_core::parser::PointL::parse)?;
        let end = read_with(buf, &mut size, wmf_core::parser::PointL::parse)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, bx, start, end })
    }
}
