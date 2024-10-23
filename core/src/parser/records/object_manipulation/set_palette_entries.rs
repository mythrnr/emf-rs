use crate::imports::*;

/// The EMR_SETPALETTEENTRIES record defines RGB color values in a range of
/// entries for an existing logical palette.
#[derive(Clone, Debug)]
pub struct EMR_SETPALETTEENTRIES {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETPALETTEENTRIES. This value is 0x00000032.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// ihPal (4 bytes): An unsigned integer that specifies an index of a
    /// LogPalette object in the EMF object table.
    pub ih_pal: u32,
    /// Start (4 bytes): An unsigned integer that specifies the index in the
    /// palette of the first entry to set.
    pub start: u32,
    /// NumberofEntries (4 bytes): An unsigned integer that specifies the
    /// number of entries in the aPalEntries array.
    pub number_of_entries: u32,
    /// aPalEntries (variable): An array of LogPaletteEntry objects that
    /// specify the palette data.
    pub a_pal_entries: Vec<crate::parser::LogPaletteEntry>,
}

impl EMR_SETPALETTEENTRIES {
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
        if record_type != crate::parser::RecordType::EMR_SETPALETTEENTRIES {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETPALETTEENTRIES as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (ih_pal, ih_pal_bytes),
            (start, start_bytes),
            (number_of_entries, number_of_entries_bytes),
        ) = (
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        size.consume(ih_pal_bytes + start_bytes + number_of_entries_bytes);

        let a_pal_entries = {
            let mut entries = vec![];

            for _ in 0..number_of_entries {
                let (v, b) = crate::parser::LogPaletteEntry::parse(buf)?;

                entries.push(v);
                size.consume(b);
            }

            entries
        };

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self {
            record_type,
            size,
            ih_pal,
            start,
            number_of_entries,
            a_pal_entries,
        })
    }
}
