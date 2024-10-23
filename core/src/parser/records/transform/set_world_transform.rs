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
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_SETWORLDTRANSFORM {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETWORLDTRANSFORM as u32,
                    record_type as u32
                ),
            });
        }

        if size.byte_count() != 0x00000020 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "size field must be `0x00000020`, but parsed value is \
                     {:#010X}",
                    size.byte_count(),
                ),
            });
        }

        let (x_form, x_form_bytes) = crate::parser::XForm::parse(buf)?;

        size.consume(x_form_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, x_form })
    }
}
