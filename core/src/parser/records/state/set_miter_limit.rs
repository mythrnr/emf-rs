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
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_SETMITERLIMIT {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETMITERLIMIT as u32,
                    record_type as u32
                ),
            });
        }

        let (miter_limit, miter_limit_bytes) =
            crate::parser::read_u32_from_le_bytes(buf)?;

        size.consume(miter_limit_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, miter_limit })
    }
}
