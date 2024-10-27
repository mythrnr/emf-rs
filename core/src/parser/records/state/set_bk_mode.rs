/// The EMR_SETBKMODE record specifies the background mix mode to use with text,
/// hatched brushes, and pens that are not solid lines.
#[derive(Clone, Debug)]
pub struct EMR_SETBKMODE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETBKMODE. This value is 0x00000012.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// BackgroundMode (4 bytes): An unsigned integer that specifies the
    /// background mode, from the BackgroundMode enumeration.
    pub background_mode: crate::parser::BackgroundMode,
}

impl EMR_SETBKMODE {
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
        if record_type != crate::parser::RecordType::EMR_SETBKMODE {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETBKMODE as u32,
                    record_type as u32
                ),
            });
        }

        let (background_mode, background_mode_bytes) =
            crate::parser::BackgroundMode::parse(buf)?;

        size.consume(background_mode_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, background_mode })
    }
}
