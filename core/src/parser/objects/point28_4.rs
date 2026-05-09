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
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::read_with;

        let mut consumed_bytes: usize = 0;
        let x = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::BitFIX28_4::parse,
        )?;
        let y = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::BitFIX28_4::parse,
        )?;

        Ok((Self { x, y }, consumed_bytes))
    }
}
