/// The HeaderExtension1 object defines the first extension to the EMF metafile
/// header. It adds support for a PixelFormatDescriptor object and OpenGL
/// [OPENGL] records.
#[derive(Clone, Debug)]
pub struct HeaderExtension1 {
    /// cbPixelFormat (4 bytes): An unsigned integer that specifies the size of
    /// the PixelFormatDescriptor object. This value is 0x00000000 if no pixel
    /// format is set.
    pub cb_pixel_format: u32,
    /// offPixelFormat (4 bytes): An unsigned integer that specifies the offset
    /// to the PixelFormatDescriptor object. This value is 0x00000000 if no
    /// pixel format is set.
    pub off_pixel_format: u32,
    /// bOpenGL (4 bytes): An unsigned integer that indicates whether OpenGL
    /// commands are present in the metafile.
    ///
    /// | Value | Meaning |
    /// |:-|:-|
    /// | `0x00000000` | OpenGL records are not present in the metafile. |
    /// | `0x00000001` | OpenGL records are present in the metafile. |
    pub b_open_gl: u32,
}

impl HeaderExtension1 {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (cb_pixel_format, cb_pixel_format_bytes),
            (off_pixel_format, off_pixel_format_bytes),
            (b_open_gl, b_open_gl_bytes),
        ) = (
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        Ok((
            Self { cb_pixel_format, off_pixel_format, b_open_gl },
            cb_pixel_format_bytes + off_pixel_format_bytes + b_open_gl_bytes,
        ))
    }
}
