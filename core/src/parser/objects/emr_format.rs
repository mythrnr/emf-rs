/// The EmrFormat object contains information that identifies the format of
/// image data in an EMR_COMMENT_MULTIFORMATS record.
#[derive(Clone, Debug)]
pub struct EmrFormat {
    /// Signature (4 bytes): An unsigned integer that specifies the format of
    /// the image data. This value is in the FormatSignature enumeration.
    pub signature: crate::parser::FormatSignature,
    /// Version (4 bytes): An unsigned integer that specifies the format
    /// version number. If the Signature field specifies encapsulated
    /// PostScript (EPS), this value is 0x00000001; otherwise, this value is
    /// ignored.
    pub version: u32,
    /// SizeData (4 bytes): An unsigned integer that specifies the size of the
    /// data in bytes.
    pub size_data: u32,
    /// offData (4 bytes): An unsigned integer that specifies the offset to the
    /// data from the start of the identifier field in an EMR_COMMENT_PUBLIC
    /// record. The offset MUST be 32-bit aligned.
    pub off_data: u32,
}

impl EmrFormat {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (signature, signature_bytes),
            (version, version_bytes),
            (size_data, size_data_bytes),
            (off_data, off_data_bytes),
        ) = (
            crate::parser::FormatSignature::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        if off_data % 4 != 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "off_data field in EmrFormat must be 32-bit aligned, but \
                     parsed value is {off_data:#010X}"
                ),
            });
        }

        Ok((
            Self { signature, version, size_data, off_data },
            signature_bytes + version_bytes + size_data_bytes + off_data_bytes,
        ))
    }
}
