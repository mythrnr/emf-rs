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
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let signature = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::FormatSignature::parse,
        )?;
        let version = read_field(buf, &mut consumed_bytes)?;
        let size_data = read_field(buf, &mut consumed_bytes)?;
        let off_data = read_field(buf, &mut consumed_bytes)?;

        // 32-bit alignment: off_data MUST be a multiple of 4. Encode as
        // an `expect_eq("... % 4", x, 0)` so the diagnostic format is
        // consistent with other field-validation errors.
        crate::parser::ParseError::expect_eq(
            "off_data (alignment, mod 4)",
            off_data % 4,
            0_u32,
        )?;

        Ok((Self { signature, version, size_data, off_data }, consumed_bytes))
    }
}
