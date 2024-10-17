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
        if record_type != crate::parser::RecordType::EMR_SETARCDIRECTION {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETARCDIRECTION as u32,
                    record_type as u32
                ),
            });
        }

        if size.byte_count() != 0x0000000C {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "size field must be `0x0000000C`, but parsed value is \
                     {:#010X}",
                    size.byte_count(),
                ),
            });
        }

        let (arc_direction, arc_direction_bytes) =
            crate::parser::ArcDirection::parse(buf)?;

        size.consume(arc_direction_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, arc_direction })
    }
}
