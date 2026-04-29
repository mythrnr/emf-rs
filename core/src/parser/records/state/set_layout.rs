/// The EMR_SETLAYOUT record specifies the order in which text and graphics are
/// drawn.
///
/// Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, and Windows NT 4.0 do not
/// support EMR_SETLAYOUT.
#[derive(Clone, Debug)]
pub struct EMR_SETLAYOUT {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETLAYOUT. This value is 0x00000073.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes. This value is 0x0000000C.
    pub size: crate::parser::Size,
    /// LayoutMode (4 bytes): An unsigned integer that specifies the layout
    /// mode as follows:
    pub layout_mode: crate::parser::LayoutMode,
}

impl EMR_SETLAYOUT {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::{read_with, records::consume_remaining_bytes};

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_SETLAYOUT as u32,
        )?;
        crate::parser::ParseError::expect_eq(
            "size",
            size.byte_count(),
            0x0000000C,
        )?;

        let layout_mode =
            read_with(buf, &mut size, crate::parser::LayoutMode::parse)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, layout_mode })
    }
}
