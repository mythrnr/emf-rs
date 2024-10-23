/// The EMR_PIXELFORMAT record specifies the pixel format to use for graphics
/// operations.
///
/// Windows NT 3.1, Windows NT 3.5, and Windows NT 3.51 do not support
/// EMR_PIXELFORMAT.
#[derive(Clone, Debug)]
pub struct EMR_PIXELFORMAT {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_PIXELFORMAT. This value is 0x00000068.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// pfd (40 bytes): A PixelFormatDescriptor object that specifies pixel
    /// format data.
    pub pfd: crate::parser::PixelFormatDescriptor,
}

impl EMR_PIXELFORMAT {
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
        if record_type != crate::parser::RecordType::EMR_PIXELFORMAT {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_PIXELFORMAT as u32,
                    record_type as u32
                ),
            });
        }

        let (pfd, pfd_bytes) =
            crate::parser::PixelFormatDescriptor::parse(buf)?;

        size.consume(pfd_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, pfd })
    }
}
