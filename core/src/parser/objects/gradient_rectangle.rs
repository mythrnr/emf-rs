/// The GradientRectangle object defines a rectangle using TriVertex objects in
/// an EMR_GRADIENTFILL record.
#[derive(Clone, Debug)]
pub struct GradientRectangle {
    /// UpperLeft (4 bytes): An index into an array of TriVertex objects that
    /// specifies the upper-left vertex of a rectangle. The index MUST be
    /// smaller than the size of the array, as defined by the nVer field of the
    /// EMR_GRADIENTFILL record.
    pub upper_left: u32,
    /// LowerRight (4 bytes): An index into an array of TriVertex objects that
    /// specifies the lower-right vertex of a rectangle. The index MUST be
    /// smaller than the size of the array, as defined by the nVer field of the
    /// EMR_GRADIENTFILL record.
    pub lower_right: u32,
}

impl GradientRectangle {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let ((upper_left, upper_left_bytes), (lower_right, lower_right_bytes)) = (
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        Ok((
            Self { upper_left, lower_right },
            upper_left_bytes + lower_right_bytes,
        ))
    }
}
