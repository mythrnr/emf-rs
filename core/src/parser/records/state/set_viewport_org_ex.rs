/// The EMR_SETVIEWPORTORGEX record defines the viewport origin.
#[derive(Clone, Debug)]
pub struct EMR_SETVIEWPORTORGEX {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETVIEWPORTORGEX. This value is 0x0000000C.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Origin (8 bytes): A PointL object ([MS-WMF] section 2.2.2.15) that
    /// specifies the window horizontal and vertical origin in device units.
    pub origin: wmf_core::parser::PointL,
}

impl EMR_SETVIEWPORTORGEX {
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
            crate::parser::RecordType::EMR_SETVIEWPORTORGEX as u32,
        )?;

        let origin =
            read_with(buf, &mut size, wmf_core::parser::PointL::parse)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, origin })
    }
}
