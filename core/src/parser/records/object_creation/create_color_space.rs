/// The EMR_CREATECOLORSPACE record creates a logical color space object from a
/// color profile with a name consisting of ASCII characters.
///
/// Windows NT 3.1, Windows NT 3.5, and Windows NT 3.51 do not support
/// EMR_CREATECOLORSPACE.
///
/// The logical color space object defined by this record can be selected into
/// the playback device context by an EMR_SETCOLORSPACE record, which defines
/// the logical color space to use in subsequent graphics operations.
#[derive(Clone, Debug)]
pub struct EMR_CREATECOLORSPACE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_CREATECOLORSPACE. This value is 0x00000063.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// ihCS (4 bytes): An unsigned integer that specifies the index of the
    /// logical color space object in the EMF object table. This index MUST be
    /// saved so that this object can be reused or modified.
    pub ih_cs: u32,
    /// lcs (variable): A LogColorSpace object ([MS-WMF] section 2.2.2.11),
    /// which can specify the name of a color profile in ASCII characters.
    pub lcs: wmf_core::parser::LogColorSpace,
}

impl EMR_CREATECOLORSPACE {
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
        if record_type != crate::parser::RecordType::EMR_CREATECOLORSPACE {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_CREATECOLORSPACE as u32,
                    record_type as u32
                ),
            });
        }

        let ((ih_cs, ih_cs_bytes), (lcs, lcs_bytes)) = (
            crate::parser::read_u32_from_le_bytes(buf)?,
            wmf_core::parser::LogColorSpace::parse(buf)?,
        );

        size.consume(ih_cs_bytes + lcs_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, ih_cs, lcs })
    }
}
