/// The EmfPlusTransformMatrix object specifies a world space to device
/// space transform (MS-EMFPLUS 2.2.2.47).
///
/// TransformMatrix (24 bytes): An affine transform, which requires a
/// 2x2 matrix for a linear transformation and a 1x2 matrix for a
/// translation. These values map to the coordinates of the transform
/// matrix as described on each field.
///
/// The matrix maps a point (X, Y) to (X', Y') as:
///
/// ```text
/// X' = M11 * X + M21 * Y + Dx
/// Y' = M12 * X + M22 * Y + Dy
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmfPlusTransformMatrix {
    /// TransformMatrix\[0\]: Corresponds to m11, which is the
    /// coordinate of the first row and first column of the 2x2 matrix.
    pub m11: f32,
    /// TransformMatrix\[1\]: Corresponds to m12, which is the
    /// coordinate of the first row and second column of the 2x2
    /// matrix.
    pub m12: f32,
    /// TransformMatrix\[2\]: Corresponds to m21, which is the
    /// coordinate of the second row and first column of the 2x2
    /// matrix.
    pub m21: f32,
    /// TransformMatrix\[3\]: Corresponds to m22, which is the
    /// coordinate of the second row and second column of the 2x2
    /// matrix.
    pub m22: f32,
    /// TransformMatrix\[4\]: Corresponds to dx, which is the
    /// horizontal displacement in the 1x2 matrix.
    pub dx: f32,
    /// TransformMatrix\[5\]: Corresponds to dy, which is the vertical
    /// displacement in the 1x2 matrix.
    pub dy: f32,
}

impl Default for EmfPlusTransformMatrix {
    fn default() -> Self {
        Self { m11: 1.0, m12: 0.0, m21: 0.0, m22: 1.0, dx: 0.0, dy: 0.0 }
    }
}

impl EmfPlusTransformMatrix {
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
}
