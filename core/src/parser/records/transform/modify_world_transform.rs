/// The EMR_MODIFYWORLDTRANSFORM record modifies the current world-space to
/// page-space transform in the playback device context.
#[derive(Clone, Debug)]
pub struct EMR_MODIFYWORLDTRANSFORM {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_MODIFYWORLDTRANSFORM. This value is 0x00000024.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes. This value is 0x00000024.
    pub size: crate::parser::Size,
    /// Xform (24 bytes): An XForm object that defines a two-dimensional linear
    /// transform in logical units. This transform is used according to the
    /// ModifyWorldTransformMode to define a new value for the world-space to
    /// page-space transform in the playback device context.
    pub x_form: crate::parser::XForm,
    /// ModifyWorldTransformMode (4 bytes): An unsigned integer that specifies
    /// how the transform specified in Xform is used. This value is in the
    /// ModifyWorldTransformMode enumeration.
    pub modify_world_transform_mode: crate::parser::ModifyWorldTransformMode,
}

impl EMR_MODIFYWORLDTRANSFORM {
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
        if record_type != crate::parser::RecordType::EMR_MODIFYWORLDTRANSFORM {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_MODIFYWORLDTRANSFORM as u32,
                    record_type as u32
                ),
            });
        }

        if size.byte_count() != 0x00000024 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "size field must be `0x00000024`, but parsed value is \
                     {:#010X}",
                    size.byte_count(),
                ),
            });
        }

        let (
            (x_form, x_form_bytes),
            (modify_world_transform_mode, modify_world_transform_mode_bytes),
        ) = (
            crate::parser::XForm::parse(buf)?,
            crate::parser::ModifyWorldTransformMode::parse(buf)?,
        );

        size.consume(x_form_bytes + modify_world_transform_mode_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, x_form, modify_world_transform_mode })
    }
}
