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
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (m11, m11_bytes),
            (m12, m12_bytes),
            (m21, m21_bytes),
            (m22, m22_bytes),
            (dx, dx_bytes),
            (dy, dy_bytes),
        ) = (
            crate::parser::read_f32_from_le_bytes(buf)?,
            crate::parser::read_f32_from_le_bytes(buf)?,
            crate::parser::read_f32_from_le_bytes(buf)?,
            crate::parser::read_f32_from_le_bytes(buf)?,
            crate::parser::read_f32_from_le_bytes(buf)?,
            crate::parser::read_f32_from_le_bytes(buf)?,
        );

        Ok((
            Self { m11, m12, m21, m22, dx, dy },
            m11_bytes + m12_bytes + m21_bytes + m22_bytes + dx_bytes + dy_bytes,
        ))
    }
}
