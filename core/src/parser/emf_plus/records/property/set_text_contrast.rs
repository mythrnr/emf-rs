/// The EmfPlusSetTextContrast record specifies text contrast according
/// to the gamma correction value.
#[derive(Clone, Debug)]
pub struct EmfPlusSetTextContrast {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusSetTextContrast from the RecordType enumeration.
    /// The value MUST be 0x4020.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    pub flags: u16,
    /// TextContrast (12 bits): The gamma correction value X 1000,
    /// which will be applied to subsequent text rendering operations.
    /// The allowable range is 1000 to 2200, representing text gamma
    /// values of 1.0 to 2.2.
    ///
    /// Decoded from bits 0-11 of the record flags (`flags & 0x0FFF`).
    pub text_contrast: u16,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be 0x0000000C.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be 0x00000000.
    pub data_size: crate::parser::Size,
}

impl EmfPlusSetTextContrast {
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
        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u16,
            crate::parser::emf_plus::RecordType::EmfPlusSetTextContrast as u16,
        )?;

        let text_contrast = flags & 0x0FFF;

        crate::parser::ParseError::expect_in_range(
            "TextContrast",
            text_contrast,
            1000_u16,
            2200_u16,
        )?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self { record_type, flags, text_contrast, size, data_size })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::RecordType;

    #[test]
    fn parses_flag_encoded_text_contrast() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);
        // Bit 12 is outside the TextContrast sub-field and must be
        // masked away by the decoder.
        let record = EmfPlusSetTextContrast::parse(
            &mut buf,
            RecordType::EmfPlusSetTextContrast,
            0x15DC,
            0x0C,
            data_size,
        )
        .unwrap();

        assert_eq!(record.record_type, RecordType::EmfPlusSetTextContrast);
        assert_eq!(record.flags, 0x15DC);
        assert_eq!(record.size, 0x0C);
        assert_eq!(record.text_contrast, 1500);
    }

    #[test]
    fn rejects_text_contrast_out_of_range() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);

        assert!(
            EmfPlusSetTextContrast::parse(
                &mut buf,
                RecordType::EmfPlusSetTextContrast,
                999,
                0x0C,
                data_size,
            )
            .is_err()
        );
    }
}
