/// The EMR_RESIZEPALETTE record increases or decreases the size of an existing
/// LogPalette object.
///
/// The new size of the LogPalette object MUST be reflected in the
/// NumberOfEntries field in that structure.
#[derive(Clone, Debug)]
pub struct EMR_RESIZEPALETTE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_RESIZEPALETTE. This value is 0x00000033.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// ihPal (4 bytes): An unsigned integer that specifies the index of the
    /// palette object in the EMF object table.
    pub ih_pal: u32,
    /// NumberOfEntries (4 bytes): An unsigned integer that specifies the
    /// number of entries in the palette after resizing. The value MUST be <=
    /// 0x00000400 and > 0x00000000.
    ///
    /// Windows GDI does not perform parameter validation on this value, which
    /// can lead to the generation of EMF metafiles that contain invalid
    /// EMR_RESIZEPALETTE records. Windows ignores such invalid records when
    /// processing metafiles.
    pub number_of_entries: u32,
}

impl EMR_RESIZEPALETTE {
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
        if record_type != crate::parser::RecordType::EMR_RESIZEPALETTE {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_RESIZEPALETTE as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (ih_pal, ih_pal_bytes),
            (number_of_entries, number_of_entries_bytes),
        ) = (
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        size.consume(ih_pal_bytes + number_of_entries_bytes);

        if number_of_entries <= 0x00000000 && number_of_entries > 0x00000400 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "number_of_entries must be greater than zero and lte \
                     `0x00000400`, but parsed value is \
                     `{number_of_entries:#010X}`",
                ),
            });
        }

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, ih_pal, number_of_entries })
    }
}
