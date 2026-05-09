/// The XForm object defines a two-dimensional, linear transform matrix.
///
/// The following equations specify how the matrix values are used to transform
/// a point (X,Y) to a new point (X',Y'):
///
/// ```
/// X' = M11 * X + M21 * Y + Dx
/// Y' = M12 * X + M22 * Y + Dy
/// ```
#[derive(Clone, Debug)]
pub struct XForm {
    /// M11 (4 bytes): A FLOAT matrix value.
    pub m11: f32,
    /// M12 (4 bytes): A FLOAT matrix value.
    pub m12: f32,
    /// M21 (4 bytes): A FLOAT matrix value.
    pub m21: f32,
    /// M22 (4 bytes): A FLOAT matrix value.
    pub m22: f32,
    /// Dx (4 bytes): A FLOAT value that contains a horizontal translation
    /// component, in logical units.
    pub dx: f32,
    /// Dy (4 bytes): A FLOAT value that contains a vertical translation
    /// component, in logical units.
    pub dy: f32,
}

impl XForm {
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
        let m11 = read_field(buf, &mut consumed_bytes)?;
        let m12 = read_field(buf, &mut consumed_bytes)?;
        let m21 = read_field(buf, &mut consumed_bytes)?;
        let m22 = read_field(buf, &mut consumed_bytes)?;
        let dx = read_field(buf, &mut consumed_bytes)?;
        let dy = read_field(buf, &mut consumed_bytes)?;

        Ok((Self { m11, m12, m21, m22, dx, dy }, consumed_bytes))
    }

    pub fn calc_scale(&self) -> f32 {
        (self.m11 * self.m22 - self.m12 * self.m21).sqrt()
    }
}

impl Default for XForm {
    fn default() -> Self {
        Self { m11: 1.0, m12: 0.0, m21: 0.0, m22: 1.0, dx: 0.0, dy: 0.0 }
    }
}
