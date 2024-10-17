/// The EMR_SETMAPPERFLAGS record specifies parameters for the process of
/// matching logical fonts to physical fonts, which is performed by the font
/// mapper.
///
/// Only Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, and Windows NT 4.0
/// support EMR_SETMAPPERFLAGS.
#[derive(Clone, Debug)]
pub struct EMR_SETMAPPERFLAGS {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETMAPPERFLAGS. This value is 0x00000010.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes. This value is 0x0000000C.
    pub size: crate::parser::Size,
    /// Flags (4 bytes): An unsigned integer that specifies parameters for the
    /// font matching process.
    ///
    /// | Value | Meaning |
    /// |:-|:-|
    /// | `0x00000000` | The font mapper is not limited to fonts that match the aspect ratio of the output device. |
    /// | `0x00000001` | The font mapper SHOULD select only fonts that match the aspect ratio of the output device. |
    pub flags: u32,
}

impl EMR_SETMAPPERFLAGS {
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
        if record_type != crate::parser::RecordType::EMR_SETMAPPERFLAGS {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETMAPPERFLAGS as u32,
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

        let (flags, flags_bytes) = crate::parser::read_u32_from_le_bytes(buf)?;

        size.consume(flags_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, flags })
    }
}
