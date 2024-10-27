/// The EMR_STROKEPATH record renders the specified path by using the current
/// pen.
#[derive(Clone, Debug)]
pub struct EMR_STROKEPATH {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_STROKEPATH. This value is 0x00000040.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19) that
    /// specifies the bounding rectangle in logical units.
    pub bounds: wmf_core::parser::RectL,
}

impl EMR_STROKEPATH {
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
        if record_type != crate::parser::RecordType::EMR_STROKEPATH {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_STROKEPATH as u32,
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
