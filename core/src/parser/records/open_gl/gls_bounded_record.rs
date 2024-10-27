use crate::imports::*;

/// The EMR_GLSBOUNDEDRECORD record specifies an OpenGL function with a bounding
/// rectangle for output.
#[derive(Clone, Debug)]
pub struct EMR_GLSBOUNDEDRECORD {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_GLSBOUNDEDRECORD. This value is 0x00000067.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19) that
    /// defines a bounding rectangle in logical units, for output produced by
    /// executing the OpenGL function.
    pub bounds: wmf_core::parser::RectL,
    /// cbData (4 bytes): An unsigned integer that specifies the size of the
    /// Data field in bytes. If this value is zero, no data is attached to this
    /// record.
    pub cb_data: u32,
    /// Data (variable, optional): An array of bytes that specifies data for
    /// the OpenGL function.
    pub data: Vec<u8>,
}

impl EMR_GLSBOUNDEDRECORD {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_GLSBOUNDEDRECORD {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_GLSBOUNDEDRECORD as u32,
                    record_type as u32
                ),
            });
        }

        let ((bounds, bounds_bytes), (cb_data, cb_data_bytes)) = (
            wmf_core::parser::RectL::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        let (data, data_bytes) =
            crate::parser::read_variable(buf, cb_data as usize)?;

        size.consume(bounds_bytes + cb_data_bytes + data_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, bounds, cb_data, data })
    }
}
