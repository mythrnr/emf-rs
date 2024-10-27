/// The EMR_SETVIEWPORTEXTEX record defines the viewport extent.
#[derive(Clone, Debug)]
pub struct EMR_SETVIEWPORTEXTEX {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETVIEWPORTEXTEX. This value is 0x0000000B.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Extent (8 bytes): A SizeL object ([MS-WMF] section 2.2.2.22) that
    /// specifies the horizontal and vertical extents in device units.
    pub extent: wmf_core::parser::SizeL,
}

impl EMR_SETVIEWPORTEXTEX {
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
        if record_type != crate::parser::RecordType::EMR_SETVIEWPORTEXTEX {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETVIEWPORTEXTEX as u32,
                    record_type as u32
                ),
            });
        }

        let (extent, extent_bytes) = wmf_core::parser::SizeL::parse(buf)?;

        size.consume(extent_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, extent })
    }
}
