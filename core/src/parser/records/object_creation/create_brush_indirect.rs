/// The EMR_CREATEBRUSHINDIRECT record defines a logical brush for graphics
/// operations.
#[derive(Clone, Debug)]
pub struct EMR_CREATEBRUSHINDIRECT {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_CREATEBRUSHINDIRECT. This value is 0x00000027.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes,
    /// of this record. This value is 0x00000018.
    pub size: crate::parser::Size,
    /// ihBrush (4 bytes): An unsigned integer that specifies the index of the
    /// logical brush object in the EMF object table. This index is used to
    /// refer to the object, so it can be reused or modified.
    pub ih_brush: u32,
    /// LogBrush (12 bytes): A LogBrushEx object that specifies the style,
    /// color, and pattern of the logical brush. The BrushStyle field in this
    /// object MUST be BS_SOLID, BS_HATCHED, or BS_NULL.
    pub log_brush: crate::parser::LogBrushEx,
}

impl EMR_CREATEBRUSHINDIRECT {
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
        if record_type != crate::parser::RecordType::EMR_CREATEBRUSHINDIRECT {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_CREATEBRUSHINDIRECT as u32,
                    record_type as u32
                ),
            });
        }

        if size.byte_count() != 0x00000018 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "size field must be `0x00000018`, but parsed value is \
                     {:#010X}",
                    size.byte_count(),
                ),
            });
        }

        let ((ih_brush, ih_brush_bytes), (log_brush, log_brush_bytes)) = (
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::LogBrushEx::parse(buf)?,
        );

        size.consume(ih_brush_bytes + log_brush_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, ih_brush, log_brush })
    }
}
