/// The EMR_EXCLUDECLIPRECT record excludes the specified rectangle from the
/// current clipping region.
///
/// The result of the intersection is saved as the new current clipping region.
/// The lower and right edges of the specified rectangle MUST NOT be excluded
/// from clipping.
#[derive(Clone, Debug)]
pub struct EMR_EXCLUDECLIPRECT {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_EXCLUDECLIPRECT. This value is 0x0000001D.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// Clip (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19) that
    /// specifies a rectangle in logical units.
    pub clip: wmf_core::parser::RectL,
}

impl EMR_EXCLUDECLIPRECT {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_EXCLUDECLIPRECT {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_EXCLUDECLIPRECT as u32,
                    record_type as u32
                ),
            });
        }

        let (clip, clip_bytes) = wmf_core::parser::RectL::parse(buf)?;
        size.consume(clip_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, clip })
    }
}
