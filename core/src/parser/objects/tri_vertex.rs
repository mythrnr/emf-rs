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
        use crate::parser::records::read_field;

        let mut consumed_bytes: usize = 0;
        let x = read_field(buf, &mut consumed_bytes)?;
        let y = read_field(buf, &mut consumed_bytes)?;
        let red = read_field(buf, &mut consumed_bytes)?;
        let green = read_field(buf, &mut consumed_bytes)?;
        let blue = read_field(buf, &mut consumed_bytes)?;
        let alpha = read_field(buf, &mut consumed_bytes)?;

        Ok((Self { x, y, red, green, blue, alpha }, consumed_bytes))
    }
}
