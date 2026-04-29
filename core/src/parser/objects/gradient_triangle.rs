/// The GradientTriangle object defines a triangle using TriVertex objects in an
/// EMR_GRADIENTFILL record.
#[derive(Clone, Debug)]
pub struct GradientTriangle {
    /// Vertex1 (4 bytes): An index into an array of TriVertex objects that
    /// specifies a vertex of a triangle. The index MUST be smaller than the
    /// size of the array, as defined by the nVer field of the EMR_GRADIENTFILL
    /// record.
    pub vertex1: u32,
    /// Vertex2 (4 bytes): An index into an array of TriVertex objects that
    /// specifies a vertex of a triangle. The index MUST be smaller than the
    /// size of the array, as defined by the nVer field of the EMR_GRADIENTFILL
    /// record.
    pub vertex2: u32,
    /// Vertex3 (4 bytes): An index into an array of TriVertex objects that
    /// specifies a vertex of a triangle. The index MUST be smaller than the
    /// size of the array, as defined by the nVer field of the EMR_GRADIENTFILL
    /// record.
    pub vertex3: u32,
}

impl GradientTriangle {
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
        let vertex1 = read_field(buf, &mut consumed_bytes)?;
        let vertex2 = read_field(buf, &mut consumed_bytes)?;
        let vertex3 = read_field(buf, &mut consumed_bytes)?;

        Ok((Self { vertex1, vertex2, vertex3 }, consumed_bytes))
    }
}
