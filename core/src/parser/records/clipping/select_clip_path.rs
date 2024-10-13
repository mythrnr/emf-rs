/// The EMR_SELECTCLIPPATH record sets the current clipping region in the
/// playback device context to the current clipping region combined with current
/// path bracket.
#[derive(Clone, Debug)]
pub struct EMR_SELECTCLIPPATH {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SELECTCLIPPATH. This value is 0x00000043.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// RegionMode (4 bytes): An unsigned integer that specifies how to combine
    /// the current clipping region with the current path bracket. This value
    /// is in the RegionMode enumeration.
    pub region_mode: crate::parser::RegionMode,
}

impl EMR_SELECTCLIPPATH {
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
        if record_type != crate::parser::RecordType::EMR_SELECTCLIPPATH {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SELECTCLIPPATH as u32,
                    record_type as u32
                ),
            });
        }

        let (region_mode, region_mode_bytes) =
            crate::parser::RegionMode::parse(buf)?;
        size.consume(region_mode_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, region_mode })
    }
}
