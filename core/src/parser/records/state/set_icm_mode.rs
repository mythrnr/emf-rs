/// The EMR_SETICMMODE record specifies the mode of Image Color Management (ICM)
/// for graphics operations.
///
/// Windows NT 3.1, Windows NT 3.5, and Windows NT 3.51 do not support
/// EMR_SETICMMODE.
///
/// When ICM mode is enabled in the playback device context, colors specified in
/// EMF records SHOULD be color matched, whereas the default color profile
/// SHOULD be used when a bit-block transfer is performed. If the default color
/// profile is not desired, ICM mode SHOULD be turned off before performing the
/// bit-block transfer.
#[derive(Clone, Debug)]
pub struct EMR_SETICMMODE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETICMMODE. This value is 0x00000062.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes. This value is 0x0000000C.
    pub size: crate::parser::Size,
    /// ICMMode (4 bytes): An unsigned integer that specifies whether to enable
    /// or disable ICM, from the ICMMode enumeration.
    pub icm_mode: crate::parser::ICMMode,
}

impl EMR_SETICMMODE {
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
            crate::parser::RecordType::EMR_SETICMMODE as u32,
        )?;
        crate::parser::ParseError::expect_eq(
            "size",
            size.byte_count(),
            0x0000000C,
        )?;

        let icm_mode =
            read_with(buf, &mut size, crate::parser::ICMMode::parse)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, icm_mode })
    }
}
