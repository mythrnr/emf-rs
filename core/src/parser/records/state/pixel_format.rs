/// The EMR_PIXELFORMAT record specifies the pixel format to use for graphics
/// operations.
///
/// Windows NT 3.1, Windows NT 3.5, and Windows NT 3.51 do not support
/// EMR_PIXELFORMAT.
#[derive(Clone, Debug)]
pub struct EMR_PIXELFORMAT {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_PIXELFORMAT. This value is 0x00000068.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// pfd (40 bytes): A PixelFormatDescriptor object that specifies pixel
    /// format data.
    pub pfd: crate::parser::PixelFormatDescriptor,
}

impl EMR_PIXELFORMAT {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = ?record_type),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{consume_remaining_bytes, read_with};

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_PIXELFORMAT as u32,
        )?;

        let pfd = read_with(
            buf,
            &mut size,
            crate::parser::PixelFormatDescriptor::parse,
        )?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, pfd })
    }
}
