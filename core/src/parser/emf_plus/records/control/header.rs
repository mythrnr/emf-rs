/// The EmfPlusHeader record specifies the start of EMF+ data in the
/// metafile.
///
/// The EmfPlusHeader record MUST be embedded in an EMF
/// EMR_COMMENT_EMFPLUS record, which MUST be the record immediately
/// following the EMF header in the metafile.
#[derive(Clone, Debug)]
pub struct EmfPlusHeader {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusHeader from the RecordType enumeration. The
    /// value MUST be 0x4001.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about the structure of the metafile.
    pub flags: u16,
    /// D (1 bit): If set, this flag indicates that this metafile is
    /// EMF+ Dual, which means that it contains two sets of records,
    /// each of which completely specifies the graphics content. If
    /// clear, the graphics content is specified by EMF+ records, and
    /// possibly EMF records ([MS-EMF] section 2.3) that are preceded
    /// by an EmfPlusGetDC record. If this flag is set, EMF records
    /// alone SHOULD suffice to define the graphics content. Note that
    /// whether the EMF+ Dual flag is set or not, some EMF records are
    /// always present, namely EMF control records and the EMF records
    /// that contain EMF+ records.
    pub dual: bool,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned size of the entire record in bytes, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value is 0x0000001C.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of data in the record-specific
    /// data that follows. This number does not include the size of the
    /// invariant part of this record. For this record type, the value
    /// is 0x00000010.
    pub data_size: crate::parser::Size,
    /// Version (4 bytes): An EmfPlusGraphicsVersion object that
    /// specifies the version of operating system graphics that was
    /// used to create this metafile.
    pub version: crate::parser::emf_plus::objects::EmfPlusGraphicsVersion,
    /// EmfPlusFlags (4 bytes): An unsigned integer that contains
    /// information about how this metafile was recorded.
    pub emf_plus_flags: u32,
    /// V (1 bit): If set, this flag indicates that the metafile was
    /// recorded with a reference device context for a video display.
    /// If clear, the metafile was recorded with a reference device
    /// context for a printer.
    pub video_display: bool,
    /// LogicalDpiX (4 bytes): An unsigned integer that specifies the
    /// horizontal resolution for which the metafile was recorded, in
    /// units of pixels per inch.
    pub logical_dpi_x: u32,
    /// LogicalDpiY (4 bytes): An unsigned integer that specifies the
    /// vertical resolution for which the metafile was recorded, in
    /// units of lines per inch.
    pub logical_dpi_y: u32,
}

impl EmfPlusHeader {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = ?record_type),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::emf_plus::RecordType,
        flags: u16,
        size: u32,
        mut data_size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u16,
            crate::parser::emf_plus::RecordType::EmfPlusHeader as u16,
        )?;

        let version = read_with(
            buf,
            &mut data_size,
            crate::parser::emf_plus::objects::EmfPlusGraphicsVersion::parse,
        )?;

        // The specification mandates the metafile signature value in
        // the Version field of this record; elsewhere the raw value is
        // kept without validation.
        version.validate_signature()?;

        let emf_plus_flags: u32 = read_field(buf, &mut data_size)?;
        let logical_dpi_x: u32 = read_field(buf, &mut data_size)?;
        let logical_dpi_y: u32 = read_field(buf, &mut data_size)?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self {
            record_type,
            flags,
            dual: flags & 0x0001 != 0,
            size,
            data_size,
            version,
            emf_plus_flags,
            video_display: emf_plus_flags & 0x0000_0001 != 0,
            logical_dpi_x,
            logical_dpi_y,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_dual_header() {
        let mut data = vec![];
        data.extend(0xDBC0_1002_u32.to_le_bytes());
        data.extend(1_u32.to_le_bytes());
        data.extend(96_u32.to_le_bytes());
        data.extend(96_u32.to_le_bytes());

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let header = EmfPlusHeader::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusHeader,
            0x0001,
            0x1C,
            data_size,
        )
        .unwrap();

        assert_eq!(
            header.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusHeader,
        );
        assert_eq!(header.flags, 0x0001);
        assert_eq!(header.size, 0x1C);
        assert!(header.dual);
        assert!(header.video_display);
        assert_eq!(header.logical_dpi_x, 96);
        assert_eq!(header.logical_dpi_y, 96);
    }

    #[test]
    fn rejects_wrong_metafile_signature() {
        let mut data = vec![];
        data.extend(0x0000_0002_u32.to_le_bytes());
        data.extend(0_u32.to_le_bytes());
        data.extend(96_u32.to_le_bytes());
        data.extend(96_u32.to_le_bytes());

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);

        assert!(
            EmfPlusHeader::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusHeader,
                0,
                0x1C,
                data_size,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_wrong_record_type() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);

        assert!(
            EmfPlusHeader::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusEndOfFile,
                0,
                0x1C,
                data_size,
            )
            .is_err()
        );
    }
}
