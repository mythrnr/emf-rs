/// The EMR_DELETECOLORSPACE record deletes a logical color space object.
///
/// Windows NT 3.1, Windows NT 3.5, and Windows NT 3.51 do not support
/// EMR_DELETECOLORSPACE.
///
/// The color space is specified by either a LogColorSpace or LogColorSpaceW
/// object ([MS-WMF] sections 2.2.2.11 and 2.2.2.12, respectively). If the
/// deleted color space is currently selected into the playback device context,
/// the default object MUST be restored.
///
/// An EMR_DELETEOBJECT record SHOULD be used instead of this record to delete a
/// logical color space object.
#[derive(Clone, Debug)]
pub struct EMR_DELETECOLORSPACE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_DELETECOLORSPACE. This value is 0x00000065.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// ihCS (4 bytes): An unsigned integer that specifies the index of a
    /// logical color space object in the EMF object table.
    pub ih_cs: u32,
}

impl EMR_DELETECOLORSPACE {
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
        if record_type != crate::parser::RecordType::EMR_DELETECOLORSPACE {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_DELETECOLORSPACE as u32,
                    record_type as u32
                ),
            });
        }

        let (ih_cs, ih_cs_bytes) = crate::parser::read_u32_from_le_bytes(buf)?;

        size.consume(ih_cs_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, ih_cs })
    }
}
