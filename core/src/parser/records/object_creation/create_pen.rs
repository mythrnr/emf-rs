/// The EMR_CREATEPEN record defines a logical pen for graphics operations.
///
/// The logical pen object defined by this record can be selected into the
/// playback device context by an EMR_SELECTOBJECT record, which specifies the
/// logical pen to use in subsequent graphics operations.
#[derive(Clone, Debug)]
pub struct EMR_CREATEPEN {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_CREATEPEN. This value is 0x00000026.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes,
    /// of this record. This value is 0x0000001C.
    pub size: crate::parser::Size,
    /// ihPen (4 bytes): An unsigned integer that specifies the index of the
    /// logical pen object in the EMF object table. This index MUST be saved so
    /// that this object can be reused or modified.
    pub ih_pen: u32,
    /// LogPen (16 bytes): A LogPen object that specifies the style, width, and
    /// color of the logical pen.
    pub log_pen: crate::parser::LogPen,
}

impl EMR_CREATEPEN {
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
        if record_type != crate::parser::RecordType::EMR_CREATEPEN {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_CREATEPEN as u32,
                    record_type as u32
                ),
            });
        }

        if size.byte_count() != 0x0000001C {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "size field must be `0x0000001C`, but parsed value is \
                     {:#010X}",
                    size.byte_count(),
                ),
            });
        }

        let ((ih_pen, ih_pen_bytes), (log_pen, log_pen_bytes)) = (
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::LogPen::parse(buf)?,
        );

        size.consume(ih_pen_bytes + log_pen_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, ih_pen, log_pen })
    }
}
