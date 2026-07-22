use crate::{imports::*, parser::emf_plus::objects::EmfPlusARGB};

/// The EmfPlusPalette object specifies the colors that make up a
/// palette (MS-EMFPLUS 2.2.2.28).
///
/// The PaletteCount field (4 bytes), which specifies the number of
/// entries in the PaletteEntries array, is represented by the length
/// of the vector.
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusPalette {
    /// PaletteStyleFlags (4 bytes): An unsigned integer that specifies
    /// the attributes of data in the palette. This value MUST be
    /// composed of PaletteStyle flags (section 2.1.2.5).
    pub palette_style_flags: crate::parser::emf_plus::PaletteStyleFlags,
    /// PaletteEntries (variable): An array of PaletteCount EmfPlusARGB
    /// objects (section 2.2.2.1) that specify the data in the palette.
    pub palette_entries: Vec<EmfPlusARGB>,
}

impl EmfPlusPalette {
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
        let palette_style_flags = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::PaletteStyleFlags::parse,
        )?;
        let palette_count: u32 = read_field(buf, &mut consumed_bytes)?;

        crate::parser::emf_plus::check_element_count(
            "PaletteCount",
            palette_count,
        )?;

        let mut palette_entries = vec![];
        for _ in 0..palette_count {
            palette_entries.push(read_with(
                buf,
                &mut consumed_bytes,
                EmfPlusARGB::parse,
            )?);
        }

        Ok((Self { palette_style_flags, palette_entries }, consumed_bytes))
    }
}
