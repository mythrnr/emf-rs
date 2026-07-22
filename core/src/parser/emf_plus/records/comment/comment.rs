use crate::imports::*;

/// The EmfPlusComment record specifies arbitrary private data.
#[derive(Clone, Debug)]
pub struct EmfPlusComment {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusComment from the RecordType enumeration. The
    /// value MUST be 0x4003.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that is not used. This
    /// field SHOULD be set to zero and MUST be ignored upon receipt.
    pub flags: u16,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, it MUST be computed as follows:
    /// Size = DataSize + 0x0000000C
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows.
    pub data_size: crate::parser::Size,
    /// PrivateData (variable): A DataSize-length byte array of private
    /// data.
    pub private_data: Vec<u8>,
}

impl EmfPlusComment {
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
        use crate::parser::records::read_bytes_field;

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u16,
            crate::parser::emf_plus::RecordType::EmfPlusComment as u16,
        )?;

        let remaining = data_size.remaining_bytes();
        let private_data = read_bytes_field(buf, &mut data_size, remaining)?;

        Ok(Self { record_type, flags, size, data_size, private_data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_the_private_data_bytes() {
        let data = *b"GDIC1234";
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusComment::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusComment,
            0,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusComment,
        );
        assert_eq!(record.flags, 0);
        assert_eq!(record.size, 20);
        assert_eq!(record.private_data, b"GDIC1234");
    }

    #[test]
    fn rejects_wrong_record_type() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);

        assert!(
            EmfPlusComment::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusGetDC,
                0,
                0x0C,
                data_size,
            )
            .is_err()
        );
    }
}
