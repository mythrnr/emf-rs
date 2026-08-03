use crate::parser::emf_plus::records::object_id;

/// The EmfPlusDrawPath record specifies drawing a graphics path.
#[derive(Clone, Debug)]
pub struct EmfPlusDrawPath {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusDrawPath from the RecordType enumeration. The
    /// value MUST be 0x4015.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    pub flags: u16,
    /// ObjectId (1 byte): The index of the EmfPlusPath object to draw,
    /// in the EMF+ Object Table. The value MUST be zero to 63,
    /// inclusive.
    pub object_id: u8,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be 0x00000010.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value is 0x00000004.
    pub data_size: crate::parser::Size,
    /// PenId (4 bytes): An unsigned integer that specifies an index in
    /// the EMF+ Object Table for an EmfPlusPen object to use for
    /// drawing the EmfPlusPath. The value MUST be zero to 63,
    /// inclusive.
    pub pen_id: u32,
}

impl EmfPlusDrawPath {
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
            crate::parser::emf_plus::RecordType::EmfPlusDrawPath as u16,
        )?;

        let pen_id: u32 = read_field(buf, &mut data_size)?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self {
            record_type,
            flags,
            object_id: object_id(flags),
            size,
            data_size,
            pen_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_pen_id() {
        let data = 9_u32.to_le_bytes();
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusDrawPath::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusDrawPath,
            0x0003,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusDrawPath,
        );
        assert_eq!(record.flags, 0x0003);
        assert_eq!(record.size, 0x0000_0010);
        assert_eq!(record.object_id, 3);
        assert_eq!(record.pen_id, 9);
    }

    #[test]
    fn rejects_a_truncated_record() {
        let data = [0_u8; 2];
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(4_u32);

        assert!(
            EmfPlusDrawPath::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusDrawPath,
                0x0000,
                0x0000_0010,
                data_size,
            )
            .is_err()
        );
    }
}
