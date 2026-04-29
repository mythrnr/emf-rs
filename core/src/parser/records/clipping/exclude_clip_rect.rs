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
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = ?record_type),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{consume_remaining_bytes, read_with};

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_EXCLUDECLIPRECT as u32,
        )?;

        let clip = read_with(buf, &mut size, wmf_core::parser::RectL::parse)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, clip })
    }
}
