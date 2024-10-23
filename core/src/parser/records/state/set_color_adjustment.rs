/// The EMR_SETCOLORADJUSTMENT record specifies color adjustment properties in
/// the playback device context.
///
/// Color adjustment values are used to adjust the input color of the source
/// bitmap for graphics operations performed by EMR_STRETCHBLT and
/// EMR_STRETCHDIBITS records when STRETCH_HALFTONE mode is set from the
/// StretchMode enumeration.
///
/// The ColorAdjustment object specified by this record MUST be used in graphics
/// operations that require a ColorAdjustment object, until a different
/// ColorAdjustment object is specified by another EMR_SETCOLORADJUSTMENT
/// record, or until the object is removed by a EMR_DELETEOBJECT record.
#[derive(Clone, Debug)]
pub struct EMR_SETCOLORADJUSTMENT {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETCOLORADJUSTMENT. This value is 0x00000017.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes. This value is 0x00000020.
    pub size: crate::parser::Size,
    /// ColorAdjustment (24 bytes): A ColorAdjustment object that specifies
    /// color adjustment values.
    pub color_adjustment: crate::parser::ColorAdjustment,
}

impl EMR_SETCOLORADJUSTMENT {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_SETCOLORADJUSTMENT {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETCOLORADJUSTMENT as u32,
                    record_type as u32
                ),
            });
        }

        if size.byte_count() != 0x00000020 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "size field must be `0x00000020`, but parsed value is \
                     {:#010X}",
                    size.byte_count(),
                ),
            });
        }

        let (color_adjustment, color_adjustment_bytes) =
            crate::parser::ColorAdjustment::parse(buf)?;

        size.consume(color_adjustment_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, color_adjustment })
    }
}
