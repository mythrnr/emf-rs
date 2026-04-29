/// The EMR_SETARCDIRECTION record specifies the drawing direction to be used
/// for arc and rectangle output.
///
/// The arc direction affects the direction in which the following records draw:
///
/// - EMR_ARC
/// - EMR_ARCTO
/// - EMR_CHORD
/// - EMR_ELLIPSE
/// - EMR_PIE
/// - EMR_RECTANGLE
/// - EMR_ROUNDRECT
#[derive(Clone, Debug)]
pub struct EMR_SETARCDIRECTION {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETARCDIRECTION. This value is 0x00000039.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes. This value is 0x0000000C.
    pub size: crate::parser::Size,
    /// ArcDirection (4 bytes): An unsigned integer that specifies the arc
    /// direction. This value is in the ArcDirection enumeration. The default
    /// direction is counterclockwise.
    pub arc_direction: crate::parser::ArcDirection,
}

impl EMR_SETARCDIRECTION {
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
        use crate::parser::{read_with, records::consume_remaining_bytes};

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_SETARCDIRECTION as u32,
        )?;
        crate::parser::ParseError::expect_eq(
            "size",
            size.byte_count(),
            0x0000000C,
        )?;

        let arc_direction =
            read_with(buf, &mut size, crate::parser::ArcDirection::parse)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, arc_direction })
    }
}
