use crate::imports::*;

/// The LogPalette object specifies a logical_palette that contains
/// device-independent color definitions.
///
/// EMF MUST define colors as device-independent values because the metafile
/// itself is device-independent.
#[derive(Clone, Debug)]
pub struct LogPalette {
    /// Version (2 bytes): An unsigned integer that specifies the version
    /// number of the system. This value is 0x0300.
    pub version: u16,
    /// NumberOfEntries (2 bytes): An unsigned integer that specifies the
    /// number of entries in the PaletteEntries field.
    pub number_of_entries: u16,
    /// PaletteEntries (variable): An array of LogPaletteEntry objects that
    /// defines the color and usage of each entry in the logical_palette.
    pub palette_entries: Vec<crate::parser::LogPaletteEntry>,
}

impl LogPalette {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (version, version_bytes),
            (number_of_entries, number_of_entries_bytes),
        ) = (
            crate::parser::read_u16_from_le_bytes(buf)?,
            crate::parser::read_u16_from_le_bytes(buf)?,
        );

        if version != 0x0300 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "version field in LogPalette must be `0x0300`, but parsed \
                     value is {version:#06X}"
                ),
            });
        }

        let (palette_entries, palette_entries_bytes) = {
            let mut entries = vec![];
            let mut bytes = 0;

            for _ in 0..number_of_entries {
                let (e, b) = crate::parser::LogPaletteEntry::parse(buf)?;

                entries.push(e);
                bytes += b;
            }

            (entries, bytes)
        };

        Ok((
            Self { version, number_of_entries, palette_entries },
            version_bytes + number_of_entries_bytes + palette_entries_bytes,
        ))
    }
}
