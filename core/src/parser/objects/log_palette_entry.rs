/// The LogPaletteEntry object defines the values that make up a single entry in
/// a LogPalette object.
///
/// EMF MUST define colors as device-independent values because the metafile
/// itself is device- independent.
#[derive(Clone, Debug)]
pub struct LogPaletteEntry {
    /// Reserved (1 byte): An unsigned integer that MUST NOT be used and MUST
    /// be ignored.
    pub reserved: u8,
    /// Blue (1 byte): An unsigned integer that defines the blue intensity
    /// value for the entry.
    pub blue: u8,
    /// Green (1 byte): An unsigned integer that defines the green intensity
    /// value for the entry.
    pub green: u8,
    /// Red (1 byte): An unsigned integer that defines the red intensity value
    /// for the entry.
    pub red: u8,
}

impl LogPaletteEntry {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (reserved, reserved_bytes),
            (blue, blue_bytes),
            (green, green_bytes),
            (red, red_bytes),
        ) = (
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
        );

        Ok((
            Self { reserved, blue, green, red },
            reserved_bytes + blue_bytes + green_bytes + red_bytes,
        ))
    }
}
