/// The TriVertex object specifies color and position information for the
/// definition of a rectangle or triangle vertex.
#[derive(Clone, Debug)]
pub struct TriVertex {
    /// x (4 bytes): A signed integer that specifies the horizontal position,
    /// in logical units.
    pub x: i32,
    /// y (4 bytes): A signed integer that specifies the vertical position, in
    /// logical units.
    pub y: i32,
    /// Red (2 bytes): An unsigned integer that specifies the red color value
    /// for the point.
    pub red: u16,
    /// Green (2 bytes): An unsigned integer that specifies the green color
    /// value for the point.
    pub green: u16,
    /// Blue (2 bytes): An unsigned integer that specifies the blue color value
    /// for the point.
    pub blue: u16,
    /// Alpha (2 bytes): An unsigned integer that specifies the alpha
    /// transparency value for the point.
    pub alpha: u16,
}

impl TriVertex {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (x, x_bytes),
            (y, y_bytes),
            (red, red_bytes),
            (green, green_bytes),
            (blue, blue_bytes),
            (alpha, alpha_bytes),
        ) = (
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_u16_from_le_bytes(buf)?,
            crate::parser::read_u16_from_le_bytes(buf)?,
            crate::parser::read_u16_from_le_bytes(buf)?,
            crate::parser::read_u16_from_le_bytes(buf)?,
        );

        Ok((
            Self { x, y, red, green, blue, alpha },
            x_bytes
                + y_bytes
                + red_bytes
                + green_bytes
                + blue_bytes
                + alpha_bytes,
        ))
    }
}
