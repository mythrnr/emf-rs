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

    pub fn transform_point_l(
        &self,
        p: wmf_core::parser::PointL,
    ) -> wmf_core::parser::PointL {
        wmf_core::parser::PointL {
            x: (f64::from(self.m11) * f64::from(p.x)
                + f64::from(self.m21) * f64::from(p.y)
                + f64::from(self.dx)) as i32,
            y: (f64::from(self.m12) * f64::from(p.x)
                + f64::from(self.m22) * f64::from(p.y)
                + f64::from(self.dy)) as i32,
        }
    }

    pub fn transform_point_s(
        &self,
        p: wmf_core::parser::PointS,
    ) -> wmf_core::parser::PointS {
        wmf_core::parser::PointS {
            x: (self.m11 * f32::from(p.x) + self.m21 * f32::from(p.y) + self.dx)
                as i16,
            y: (self.m12 * f32::from(p.x) + self.m22 * f32::from(p.y) + self.dy)
                as i16,
        }
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
