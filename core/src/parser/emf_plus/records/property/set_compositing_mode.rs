/// The EmfPlusSetCompositingMode record specifies how source colors
/// are combined with background colors.
#[derive(Clone, Debug)]
pub struct EmfPlusSetCompositingMode {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusSetCompositingMode from the RecordType
    /// enumeration. The value MUST be 0x4023.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    pub flags: u16,
    /// CompositingMode (1 byte): The compositing mode value, from the
    /// CompositingMode enumeration. Compositing can be expressed as
    /// the state of alpha blending, which can either be on or off.
    ///
    /// Decoded from the low byte (bits 0-7) of the record flags.
    pub compositing_mode: crate::parser::emf_plus::CompositingMode,
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

impl EmfPlusSetCompositingMode {
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
            crate::parser::emf_plus::RecordType::EmfPlusSetCompositingMode
                as u16,
        )?;

        let compositing_mode = super::enum_from_low_byte(
            flags,
            crate::parser::emf_plus::CompositingMode::from_repr,
            "CompositingMode",
        )?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self { record_type, flags, compositing_mode, size, data_size })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_flag_encoded_compositing_mode() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);
        let record = EmfPlusSetCompositingMode::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusSetCompositingMode,
            0x0001,
            0x0C,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusSetCompositingMode,
        );
        assert_eq!(record.flags, 0x0001);
        assert_eq!(record.size, 0x0C);
        assert_eq!(
            record.compositing_mode,
            crate::parser::emf_plus::CompositingMode::CompositingModeSourceCopy,
        );
    }

    #[test]
    fn rejects_unknown_compositing_mode() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);

        assert!(
            EmfPlusSetCompositingMode::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusSetCompositingMode,
                0x0002,
                0x0C,
                data_size,
            )
            .is_err()
        );
    }
}
