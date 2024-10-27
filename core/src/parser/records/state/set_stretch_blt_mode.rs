/// The EMR_SETSTRETCHBLTMODE record specifies bitmap stretch mode.
///
/// The stretching mode specifies how to combine rows or columns of a bitmap
/// with existing pixels on the display device that the EMR_STRETCHBLT record is
/// processed on.
///
/// The STRETCH_ANDSCANS and STRETCH_ORSCANS modes are typically used to
/// preserve foreground pixels in monochrome bitmaps. The STRETCH_DELETESCANS
/// mode is typically used to preserve color in color bitmaps.
///
/// The STRETCH_HALFTONE mode is slower and requires more processing of the
/// source image than the other three modes, but produces higher quality images.
/// Also note that an EMR_SETBRUSHORGEX SHOULD be encountered after setting the
/// STRETCH_HALFTONE mode to avoid brush misalignment.
#[derive(Clone, Debug)]
pub struct EMR_SETSTRETCHBLTMODE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETSTRETCHBLTMODE. This value is 0x00000015.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// StretchMode (4 bytes): An unsigned integer that specifies the stretch
    /// mode and MAY be in the StretchMode enumeration.
    pub stretch_mode: crate::parser::StretchMode,
}

impl EMR_SETSTRETCHBLTMODE {
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
        if record_type != crate::parser::RecordType::EMR_SETSTRETCHBLTMODE {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETSTRETCHBLTMODE as u32,
                    record_type as u32
                ),
            });
        }

        let (stretch_mode, stretch_mode_bytes) =
            crate::parser::StretchMode::parse(buf)?;

        size.consume(stretch_mode_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, stretch_mode })
    }
}
