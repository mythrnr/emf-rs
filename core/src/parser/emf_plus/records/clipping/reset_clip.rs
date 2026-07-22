/// The EmfPlusResetClip record resets the current clipping region for
/// the world space to infinity.
#[derive(Clone, Debug)]
pub struct EmfPlusResetClip {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusResetClip from the RecordType enumeration. The
    /// value MUST be 0x4031.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that is reserved and MUST
    /// be ignored.
    pub flags: u16,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned size of the entire record in bytes, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be 0x0000000C.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of data in the record-specific
    /// data that follows. This number does not include the size of the
    /// invariant part of this record. For this record type, the value
    /// MUST be 0x00000000.
    pub data_size: crate::parser::Size,
}

impl EmfPlusResetClip {
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
            crate::parser::emf_plus::RecordType::EmfPlusResetClip as u16,
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
    fn parses_a_record_without_data() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);
        let record = EmfPlusResetClip::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusResetClip,
            0x0000,
            12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusResetClip,
        );
        assert_eq!(record.flags, 0x0000);
        assert_eq!(record.size, 0x0000_000C);
    }

    #[test]
    fn rejects_wrong_record_type() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);

        assert!(
            EmfPlusResetClip::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusOffsetClip,
                0x0000,
                12,
                data_size,
            )
            .is_err()
        );
    }
}
