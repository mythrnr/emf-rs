/// The Point28_4 object represents the location of a point on a device surface
/// with coordinates in 28.4 bit FIX notation.
#[derive(Clone, Debug)]
pub struct Point28_4 {
    /// x (4 bytes): A BitFIX28_4 object that represents the horizontal
    /// coordinate of the point.
    pub x: crate::parser::BitFIX28_4,
    /// y (4 bytes): A BitFIX28_4 object that represents the vertical
    /// coordinate of the point.
    pub y: crate::parser::BitFIX28_4,
}

impl Point28_4 {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let ((x, x_bytes), (y, y_bytes)) = (
            crate::parser::BitFIX28_4::parse(buf)?,
            crate::parser::BitFIX28_4::parse(buf)?,
        );

        Ok((Self { x, y }, x_bytes + y_bytes))
    }
}
