/// The EMR_SETWORLDTRANSFORM record specifies a transform for the current
/// world-space to page- space transform in the playback device context.
#[derive(Clone, Debug)]
pub struct EMR_SETWORLDTRANSFORM {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETWORLDTRANSFORM. This value is 0x00000023.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes. This value is 0x00000020.
    pub size: crate::parser::Size,
    /// Xform (24 bytes): An XForm object that specifies a two-dimensional
    /// linear transform in logical units. This transform defines a new value
    /// for the current world-space to page-space transform.
    pub x_form: crate::parser::XForm,
}

impl EMR_SETWORLDTRANSFORM {
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
            crate::parser::RecordType::EMR_SETWORLDTRANSFORM as u32,
        )?;
        crate::parser::ParseError::expect_eq(
            "size field",
            size.byte_count() as u32,
            0x00000020_u32,
        )?;

        let x_form = read_with(buf, &mut size, crate::parser::XForm::parse)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, x_form })
    }
}
