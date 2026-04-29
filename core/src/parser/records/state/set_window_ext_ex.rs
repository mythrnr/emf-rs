/// The EMR_SETWINDOWEXTEX record defines the window extent.
#[derive(Clone, Debug)]
pub struct EMR_SETWINDOWEXTEX {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETWINDOWEXTEX. This value is 0x00000009.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Extent (8 bytes): A SizeL object ([MS-WMF] section 2.2.2.22) that
    /// specifies the horizontal and vertical extents in logical units.
    pub extent: wmf_core::parser::SizeL,
}

impl EMR_SETWINDOWEXTEX {
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
            crate::parser::RecordType::EMR_SETWINDOWEXTEX as u32,
        )?;

        let extent = read_with(buf, &mut size, wmf_core::parser::SizeL::parse)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, extent })
    }
}
