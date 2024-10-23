/// The EMR_SETTEXTJUSTIFICATION record specifies the amount of extra space to
/// add to break characters for text justification.
///
/// Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, Windows NT 4.0, Windows 98,
/// and Windows Millennium Edition do not support the EMR_SETTEXTJUSTIFICATION
/// record type.
///
/// Instead of using this record, an implementation SHOULD use EMR_EXTTEXTOUTW
/// to perform this function.
#[derive(Clone, Debug)]
pub struct EMR_SETTEXTJUSTIFICATION {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETTEXTJUSTIFICATION. This value is 0x00000078.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// nBreakExtra (4 bytes): A signed integer that specifies the total amount
    /// of extra space to add in logical units.
    pub n_break_extra: i32,
    /// nBreakCount (4 bytes): A signed integer that specifies the number of
    /// break characters.
    pub n_break_count: i32,
}

impl EMR_SETTEXTJUSTIFICATION {
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
        if record_type != crate::parser::RecordType::EMR_SETTEXTJUSTIFICATION {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETTEXTJUSTIFICATION as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (n_break_extra, n_break_extra_bytes),
            (n_break_count, n_break_count_bytes),
        ) = (
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
        );

        size.consume(n_break_extra_bytes + n_break_count_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, n_break_extra, n_break_count })
    }
}
