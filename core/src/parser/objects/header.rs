/// The Header object defines the EMF metafile header. It specifies properties
/// of the device on which the image in the metafile was created.
#[derive(Clone, Debug)]
pub struct Header {
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19) that
    /// specifies the rectangular inclusive-inclusive bounds in logical units
    /// of the smallest rectangle that can be drawn around the image stored in
    /// the metafile.
    pub bounds: wmf_core::parser::RectL,
    /// Frame (16 bytes): A RectL object that specifies the rectangular
    /// inclusive-inclusive dimensions, in .01 millimeter units, of a rectangle
    /// that surrounds the image stored in the metafile.
    pub frame: wmf_core::parser::RectL,
    /// RecordSignature (4 bytes): An unsigned integer that specifies the
    /// record signature. This MUST be ENHMETA_SIGNATURE, from the
    /// FormatSignature enumeration.
    pub record_signature: crate::parser::FormatSignature,
    /// Version (4 bytes): An unsigned integer that specifies the EMF version
    /// for interoperability. This MAY be 0x00010000.
    pub version: u32,
    /// Bytes (4 bytes): An unsigned integer that specifies the size of the
    /// metafile in bytes.
    pub bytes: u32,
    /// Records (4 bytes): An unsigned integer that specifies the number of
    /// records in the metafile.
    pub records: u32,
    /// Handles (2 bytes): An unsigned integer that specifies the number of
    /// graphics objects that are used during the processing of the metafile.
    pub handles: u16,
    /// Reserved (2 bytes): An unsigned integer that MUST be 0x0000 and MUST be
    /// ignored.
    pub reserved: u16,
    /// nDescription (4 bytes): An unsigned integer that specifies the number
    /// of characters in the array that contains the description of the
    /// metafile's contents. This is zero if there is no description string.
    pub n_description: u32,
    /// offDescription (4 bytes): An unsigned integer that specifies the offset
    /// from the beginning of this record to the array that contains the
    /// description of the metafile's contents.
    pub off_description: u32,
    /// nPalEntries (4 bytes): An unsigned integer that specifies the number of
    /// entries in the metafile palette. The palette is located in the EMR_EOF
    /// record.
    pub n_pal_entries: u32,
    /// Device (8 bytes): A SizeL object ([MS-WMF] section 2.2.2.22) that
    /// specifies the size of the reference device, in pixels.
    pub device: wmf_core::parser::SizeL,
    // Millimeters (8 bytes): A SizeL object that specifies the size of the
    // reference device, in millimeters.
    pub millimeters: wmf_core::parser::SizeL,
}

impl Header {
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
        let bounds = read_with(
            buf,
            &mut consumed_bytes,
            wmf_core::parser::RectL::parse,
        )?;
        let frame = read_with(
            buf,
            &mut consumed_bytes,
            wmf_core::parser::RectL::parse,
        )?;
        let record_signature = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::FormatSignature::parse,
        )?;
        let version = read_field(buf, &mut consumed_bytes)?;
        let bytes = read_field(buf, &mut consumed_bytes)?;
        let records = read_field(buf, &mut consumed_bytes)?;
        let handles = read_field(buf, &mut consumed_bytes)?;
        let reserved = read_field(buf, &mut consumed_bytes)?;
        let n_description = read_field(buf, &mut consumed_bytes)?;
        let off_description = read_field(buf, &mut consumed_bytes)?;
        let n_pal_entries = read_field(buf, &mut consumed_bytes)?;
        let device = read_with(
            buf,
            &mut consumed_bytes,
            wmf_core::parser::SizeL::parse,
        )?;
        let millimeters = read_with(
            buf,
            &mut consumed_bytes,
            wmf_core::parser::SizeL::parse,
        )?;

        crate::parser::ParseError::expect_eq(
            "version",
            version,
            0x00010000_u32,
        )?;
        crate::parser::ParseError::expect_eq("reserved", reserved, 0x0000_u16)?;

        Ok((
            Self {
                bounds,
                frame,
                record_signature,
                version,
                bytes,
                records,
                handles,
                reserved,
                n_description,
                off_description,
                n_pal_entries,
                device,
                millimeters,
            },
            consumed_bytes,
        ))
    }
}
