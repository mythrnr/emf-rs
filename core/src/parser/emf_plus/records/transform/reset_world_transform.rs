/// The EmfPlusResetWorldTransform record resets the current world
/// space transform to the identify matrix.
#[derive(Clone, Debug)]
pub struct EmfPlusResetWorldTransform {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusResetWorldTransform from the RecordType
    /// enumeration. The value MUST be 0x402B.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that is not used. This
    /// field SHOULD be set to zero and MUST be ignored upon receipt.
    pub flags: u16,
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

impl EmfPlusResetWorldTransform {
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
            crate::parser::emf_plus::RecordType::EmfPlusResetWorldTransform
                as u16,
        )?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self { record_type, flags, size, data_size })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_an_empty_record() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);
        let record = EmfPlusResetWorldTransform::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusResetWorldTransform,
            0x0000,
            12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusResetWorldTransform,
        );
        assert_eq!(record.flags, 0x0000);
        assert_eq!(record.size, 0x0000_000C);
    }

    #[test]
    fn rejects_wrong_record_type() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);

        assert!(
            EmfPlusResetWorldTransform::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusSetWorldTransform,
                0x0000,
                12,
                data_size,
            )
            .is_err()
        );
    }
}
