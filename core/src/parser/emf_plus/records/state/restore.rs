/// The EmfPlusRestore record restores the graphics state, identified
/// by a specified index, from a stack of saved graphics states.
///
/// Each graphics state MUST be popped off a stack of saved graphics
/// states. The graphics state information is not written to the EMF+
/// metafile, so its format can be determined by the implementation.
#[derive(Clone, Debug)]
pub struct EmfPlusRestore {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusRestore from the RecordType enumeration. The
    /// value MUST be 0x4026.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that is not used. This
    /// field SHOULD be set to zero and MUST be ignored upon receipt.
    pub flags: u16,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of records in the entire record,
    /// including the 12-byte record header and record-specific data.
    /// For this record type, the value MUST be 0x00000010.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be 0x00000004.
    pub data_size: crate::parser::Size,
    /// StackIndex (4 bytes): An unsigned integer that specifies the
    /// level associated with a graphics state. The level value was
    /// assigned to the graphics state by a previous EmfPlusSave
    /// record.
    pub stack_index: u32,
}

impl EmfPlusRestore {
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
        use crate::parser::records::read_field;

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u16,
            crate::parser::emf_plus::RecordType::EmfPlusRestore as u16,
        )?;

        let stack_index: u32 = read_field(buf, &mut data_size)?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self { record_type, flags, size, data_size, stack_index })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_stack_index() {
        let data = 2_u32.to_le_bytes();
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusRestore::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusRestore,
            0x0000,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusRestore,
        );
        assert_eq!(record.flags, 0x0000);
        assert_eq!(record.size, 0x10);
        assert_eq!(record.stack_index, 2);
    }

    #[test]
    fn rejects_wrong_record_type() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);

        assert!(
            EmfPlusRestore::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusSave,
                0x0000,
                0x10,
                data_size,
            )
            .is_err()
        );
    }
}
