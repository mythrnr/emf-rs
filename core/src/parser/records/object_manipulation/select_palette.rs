/// The EMR_SELECTPALETTE record selects a logical palette into the playback
/// device context.
///
/// The object index MUST NOT be zero, which is reserved and refers to the EMF
/// metafile itself.
///
/// The palette specified by this record MUST be used in subsequent EMF drawing
/// operations, until another EMR_SELECTPALETTE record changes the object or the
/// object is deleted.
#[derive(Clone, Debug)]
pub struct EMR_SELECTPALETTE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SELECTPALETTE. This value is 0x00000030.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes. This value is 0x0000000C.
    pub size: crate::parser::Size,
    /// ihPal (4 bytes): An unsigned integer that specifies either the index of
    /// a LogPalette object in the EMF object table or the value
    /// DEFAULT_PALETTE from the StockObject enumeration, which is the index of
    /// a stock palette.
    pub in_pal: u32,
}

impl EMR_SELECTPALETTE {
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
            crate::parser::RecordType::EMR_SELECTPALETTE as u32,
        )?;
        crate::parser::ParseError::expect_eq(
            "size field",
            size.byte_count() as u32,
            0x0000000C_u32,
        )?;

        let in_pal = read_field(buf, &mut size)?;

        crate::parser::ParseError::expect_ne("in_pal", in_pal, 0_u32)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, in_pal })
    }
}
