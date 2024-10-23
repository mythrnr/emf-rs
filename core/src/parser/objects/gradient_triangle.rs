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
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (vertex1, vertex1_bytes),
            (vertex2, vertex2_bytes),
            (vertex3, vertex3_bytes),
        ) = (
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        Ok((
            Self { vertex1, vertex2, vertex3 },
            vertex1_bytes + vertex2_bytes + vertex3_bytes,
        ))
    }
}
