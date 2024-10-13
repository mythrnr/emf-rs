/// The EMR_INTERSECTCLIPRECT record specifies a new clipping region from the
/// intersection of the current clipping region and the specified rectangle.
///
/// The lower and right edges of the specified rectangle are excluded from the
/// clipping region.
#[derive(Clone, Debug)]
pub struct EMR_INTERSECTCLIPRECT {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_INTERSECTCLIPRECT. This value is 0x0000001E.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// Clip (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19) that
    /// specifies the rectangle in logical units.
    pub clip: wmf_core::parser::RectL,
}

impl EMR_INTERSECTCLIPRECT {
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
        if record_type != crate::parser::RecordType::EMR_INTERSECTCLIPRECT {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_INTERSECTCLIPRECT as u32,
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
