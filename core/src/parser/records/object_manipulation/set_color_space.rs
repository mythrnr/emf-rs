/// The EMR_SETCOLORSPACE record selects a logical color space into the playback
/// device context.
///
/// Windows NT 3.1, Windows NT 3.5, and Windows NT 3.51 do not support
/// EMR_SETCOLORSPACE.
///
/// This object is either a LogColorSpace or LogColorSpaceW object ([MS-WMF]
/// sections 2.2.2.11 and 2.2.2.12, respectively).
///
/// The color space specified by this record MUST be used in subsequent EMF
/// drawing operations, until another EMR_SETCOLORSPACE record changes the
/// object or the object is deleted.
#[derive(Clone, Debug)]
pub struct EMR_SETCOLORSPACE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETCOLORSPACE. This value is 0x00000064.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes. This value is 0x0000000C.
    pub size: crate::parser::Size,
    /// ihCS (4 bytes): An unsigned integer that specifies the index of a
    /// logical color space object in the EMF object table.
    pub in_cs: u32,
}

impl EMR_SETCOLORSPACE {
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
        if record_type != crate::parser::RecordType::EMR_SETCOLORSPACE {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETCOLORSPACE as u32,
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

        let (in_cs, in_cs_bytes) = crate::parser::read_u32_from_le_bytes(buf)?;

        size.consume(in_cs_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, in_cs })
    }
}
