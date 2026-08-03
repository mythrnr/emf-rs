/// The EmfPlusSetTextRenderingHint record specifies the quality of
/// text rendering, including the type of anti-aliasing.
#[derive(Clone, Debug)]
pub struct EmfPlusSetTextRenderingHint {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusSetTextRenderingHint from the RecordType
    /// enumeration. The value MUST be 0x401F.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    pub flags: u16,
    /// TextRenderingHint (1 byte): The text rendering hint value, from
    /// the TextRenderingHint enumeration, which specifies the quality
    /// to use in subsequent text rendering.
    ///
    /// Decoded from the low byte (bits 0-7) of the record flags.
    pub text_rendering_hint: crate::parser::emf_plus::TextRenderingHint,
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

impl EmfPlusSetTextRenderingHint {
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
            crate::parser::emf_plus::RecordType::EmfPlusSetTextRenderingHint
                as u16,
        )?;

        let text_rendering_hint = super::enum_from_low_byte(
            flags,
            crate::parser::emf_plus::TextRenderingHint::from_repr,
            "TextRenderingHint",
        )?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self { record_type, flags, text_rendering_hint, size, data_size })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::{RecordType, TextRenderingHint};

    #[test]
    fn parses_flag_encoded_text_rendering_hint() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);
        let record = EmfPlusSetTextRenderingHint::parse(
            &mut buf,
            RecordType::EmfPlusSetTextRenderingHint,
            0x0005,
            0x0C,
            data_size,
        )
        .unwrap();

        let expected = RecordType::EmfPlusSetTextRenderingHint;
        assert_eq!(record.record_type, expected);
        assert_eq!(record.flags, 0x0005);
        assert_eq!(record.size, 0x0C);
        assert_eq!(
            record.text_rendering_hint,
            TextRenderingHint::TextRenderingHintClearTypeGridFit,
        );
    }

    #[test]
    fn rejects_unknown_text_rendering_hint() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);

        assert!(
            EmfPlusSetTextRenderingHint::parse(
                &mut buf,
                RecordType::EmfPlusSetTextRenderingHint,
                0x0006,
                0x0C,
                data_size,
            )
            .is_err()
        );
    }
}
