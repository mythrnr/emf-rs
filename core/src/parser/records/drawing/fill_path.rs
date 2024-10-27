/// The EMR_FILLPATH record closes any open figures in the current path bracket
/// and fills the path's interior by using the current brush and polygon-filling
/// mode.
#[derive(Clone, Debug)]
pub struct EMR_FILLPATH {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_FILLPATH. This value is 0x0000003E.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19), which
    /// specifies the bounding rectangle in logical units.
    pub bounds: wmf_core::parser::RectL,
}

impl EMR_FILLPATH {
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
        if record_type != crate::parser::RecordType::EMR_FILLPATH {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_FILLPATH as u32,
                    record_type as u32
                ),
            });
        }

        let (bounds, bounds_bytes) = wmf_core::parser::RectL::parse(buf)?;

        size.consume(bounds_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, bounds })
    }
}
