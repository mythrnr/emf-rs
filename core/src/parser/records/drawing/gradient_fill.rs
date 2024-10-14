/// The EMR_GRADIENTFILL record specifies filling rectangles or triangles with
/// gradients of color.
///
/// Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, and Windows NT 4.0 do not
/// support EMR_GRADIENTFILL.
///
/// An EMR_GRADIENTFILL record that specifies that the three vertexes of a
/// triangle SHOULD fill the figure with smooth gradients of colors. Windows
/// uses true colors in 24-bits-per-pixel (bpp) and 32-bpp formats, and
/// dithering in 4-bpp, 8-bpp, and 16-bpp formats.
///
/// An EMR_GRADIENTFILL record that specifies that the upper-left and
/// lower-right vertexes of a rectangle SHOULD fill the figure with smooth
/// gradients of color. There are two gradient fill modes in the GradientFill
/// enumeration that can be used when drawing a rectangle. In
/// GRADIENT_FILL_RECT_H mode, the rectangle is filled from left to right. In
/// GRADIENT_FILL_RECT_V mode, the rectangle is filled from top to bottom.
///
/// An EMR_GRADIENTFILL record MUST ignore the Alpha fields in the TriVertex
/// objects. An EMR_ALPHABLEND record that immediately follows the
/// EMR_GRADIENTFILL record can be used to apply an alpha transparency gradient
/// to the filled area.
#[derive(Clone, Debug)]
pub struct EMR_GRADIENTFILL {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_GRADIENTFILL. This value is 0x00000076.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19), which
    /// specifies the inclusive- inclusive bounding rectangle in logical units.
    pub bounds: wmf_core::parser::RectL,
    /// nVer (4 bytes): An unsigned integer that specifies the number of
    /// vertexes.
    pub n_ver: u32,
    /// nTri (4 bytes): An unsigned integer that specifies the number of
    /// rectangles or triangles to fill.
    pub n_tri: u32,
    /// ulMode (4 bytes): An unsigned integer that specifies the gradient fill
    /// mode. This value is in the GradientFill enumeration.
    pub ul_mode: crate::parser::GradientFill,
    /// VertexData (variable): Objects that specify the vertexes of either
    /// rectangles or triangles and the colors that correspond to them.
    pub vertex_data: VertexData,
}

impl EMR_GRADIENTFILL {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_GRADIENTFILL {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_GRADIENTFILL as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (bounds, bounds_bytes),
            (n_ver, n_ver_bytes),
            (n_tri, n_tri_bytes),
            (ul_mode, ul_mode_bytes),
        ) = (
            wmf_core::parser::RectL::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::GradientFill::parse(buf)?,
        );

        size.consume(bounds_bytes + n_ver_bytes + n_tri_bytes + ul_mode_bytes);

        let vertex_objects = {
            let mut entries = vec![];

            for _ in 0..n_ver {
                let (v, b) = crate::parser::TriVertex::parse(buf)?;

                entries.push(v);
                size.consume(b);
            }

            entries
        };
        let vertex_indexes =
            if ul_mode == crate::parser::GradientFill::GRADIENT_FILL_TRIANGLE {
                let mut entries = vec![];

                for _ in 0..n_tri {
                    let (v, b) = crate::parser::GradientTriangle::parse(buf)?;

                    entries.push(v);
                    size.consume(b);
                }

                VertexIndexes::GradientTriangle(entries)
            } else {
                let mut entries = vec![];

                for _ in 0..n_tri {
                    let (v, b) = crate::parser::GradientRectangle::parse(buf)?;

                    entries.push(v);
                    size.consume(b);
                }

                VertexIndexes::GradientRectangle(entries)
            };
        let vertex_padding =
            if ul_mode == crate::parser::GradientFill::GRADIENT_FILL_TRIANGLE {
                let (v, b) =
                    crate::parser::read_variable(buf, (n_tri * 4) as usize)?;
                size.consume(b);

                v
            } else {
                vec![]
            };

        let vertex_data =
            VertexData { vertex_objects, vertex_indexes, vertex_padding };

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self {
            record_type,
            size,
            bounds,
            n_ver,
            n_tri,
            ul_mode,
            vertex_data,
        })
    }
}

#[derive(Clone, Debug)]
pub struct VertexData {
    /// VertexObjects (variable): An array of nVer TriVertex objects. Each
    /// object specifies the position and color of a vertex of either a
    /// rectangle or a triangle, depending on the value of the ulMode field.
    pub vertex_objects: Vec<crate::parser::TriVertex>,
    /// VertexIndexes (variable): An array of nTri GradientRectangle objects or
    /// GradientTriangle objects, depending on the value of the ulMode field.
    /// Each object specifies indexes into the array of TriVertex objects in
    /// the VertexObjects field.
    pub vertex_indexes: VertexIndexes,
    /// VertexPadding (variable, optional): An array of nTri times four bytes
    /// that MUST be present if the value of the ulMode field indicates
    /// GradientRectangle objects. If the value of the ulMode field indicates
    /// GradientTriangle objects, no VertexPadding is present. This field MUST
    /// be ignored.
    pub vertex_padding: Vec<u8>,
}

#[derive(Clone, Debug)]
pub enum VertexIndexes {
    GradientRectangle(Vec<crate::parser::GradientRectangle>),
    GradientTriangle(Vec<crate::parser::GradientTriangle>),
}
