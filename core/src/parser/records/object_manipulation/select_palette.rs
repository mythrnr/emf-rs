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
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_SELECTPALETTE {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SELECTPALETTE as u32,
                    record_type as u32
                ),
            });
        }

        if size.byte_count() != 0x0000000C {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "size field must be `0x0000000C`, but parsed value is \
                     {:#010X}",
                    size.byte_count(),
                ),
            });
        }

        let (in_pal, in_pal_bytes) =
            crate::parser::read_u32_from_le_bytes(buf)?;

        size.consume(in_pal_bytes);

        if in_pal == 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "in_pal must not be zero, but parsed value is \
                     `{in_pal:#010X}`",
                ),
            });
        }

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, in_pal })
    }
}
