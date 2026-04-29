/// The EMR_SETMITERLIMIT record specifies the limit for the length of miter
/// joins.
#[derive(Clone, Debug)]
pub struct EMR_SETMITERLIMIT {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETMITERLIMIT. This value is 0x0000003A.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// MiterLimit (4 bytes): An unsigned integer that specifies the new miter
    /// length limit.
    ///
    /// Windows GDI accepts a FLOAT value for the miter length limit value.
    pub miter_limit: u32,
}

impl EMR_SETMITERLIMIT {
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
        use crate::parser::records::{consume_remaining_bytes, read_field};

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_SETMITERLIMIT as u32,
        )?;

        let miter_limit = read_field(buf, &mut size)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, miter_limit })
    }
}
