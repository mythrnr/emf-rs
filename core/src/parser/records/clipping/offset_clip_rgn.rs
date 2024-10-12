/// The EMR_OFFSETCLIPRGN record moves the current clipping region in the
/// playback device context by the specified offsets.
#[derive(Clone, Debug)]
pub struct EMR_OFFSETCLIPRGN {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_OFFSETCLIPRGN. This value is 0x0000001A.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: u32,
    /// Offset (8 bytes): A PointL object ([MS-WMF] section 2.2.2.15) that
    /// specifies the horizontal and vertical offsets in logical units.
    pub offset: wmf_core::parser::PointL,
}

impl EMR_OFFSETCLIPRGN {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_OFFSETCLIPRGN {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_OFFSETCLIPRGN as u32,
                    record_type as u32
                ),
            });
        }

        let ((size, size_bytes), (offset, offset_bytes)) = (
            crate::parser::read_u32_from_le_bytes(buf)?,
            wmf_core::parser::PointL::parse(buf)?,
        );
        let consumed_bytes = size_bytes + offset_bytes;

        crate::parser::records::consume_remaining_bytes(
            buf,
            size as usize - consumed_bytes,
        )?;

        Ok(Self { record_type, size, offset })
    }
}
