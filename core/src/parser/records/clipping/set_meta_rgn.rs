/// If the current metaregion is null, it is set to the current clipping region.
/// Otherwise, the current metaregion is intersected with the current clipping
/// region, and the result is the new metaregion.
///
/// After the operation, the current clipping region is set to null.
///
/// During playback, drawing occurs only within the intersection of the
/// metaregion and clipping region.
///
/// This EMF record specifies no parameters.
#[derive(Clone, Debug)]
pub struct EMR_SETMETARGN {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETMETARGN. This value is 0x00000043.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
}

impl EMR_SETMETARGN {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = ?record_type),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::consume_remaining_bytes;

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_SETMETARGN as u32,
        )?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size })
    }
}
