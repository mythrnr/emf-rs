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
        use crate::parser::records::{read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let version = read_field(buf, &mut consumed_bytes)?;
        let number_of_entries: u16 = read_field(buf, &mut consumed_bytes)?;

        crate::parser::ParseError::expect_eq(
            "version (LogPalette)",
            version,
            0x0300_u16,
        )?;

        // `number_of_entries` is a u16, so `as usize` is at most
        // 65535. A single LogPaletteEntry is 4 bytes, capping the
        // pre-allocation at 256 KiB even for the worst-case input.
        let palette_entries = {
            let mut entries = Vec::with_capacity(number_of_entries as usize);

            for _ in 0..number_of_entries {
                entries.push(read_with(
                    buf,
                    &mut consumed_bytes,
                    crate::parser::LogPaletteEntry::parse,
                )?);
            }

            entries
        };

        Ok((
            Self { version, number_of_entries, palette_entries },
            consumed_bytes,
        ))
    }
}
