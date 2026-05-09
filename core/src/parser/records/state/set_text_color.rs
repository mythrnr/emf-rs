/// The EMR_SETTEXTCOLOR record defines the current text foreground color.
#[derive(Clone, Debug)]
pub struct EMR_SETTEXTCOLOR {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETTEXTCOLOR. This value is 0x00000018.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Color (4 bytes): A ColorRef object ([MS-WMF] section 2.2.2.8) that
    /// specifies the text foreground color.
    pub color: wmf_core::parser::ColorRef,
}

impl EMR_SETTEXTCOLOR {
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
            crate::parser::RecordType::EMR_SETTEXTCOLOR as u32,
        )?;

        let color =
            read_with(buf, &mut size, wmf_core::parser::ColorRef::parse)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, color })
    }
}
