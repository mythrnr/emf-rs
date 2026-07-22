/// The EmfPlusSetRenderingOrigin record specifies the rendering origin
/// for graphics output.
#[derive(Clone, Debug)]
pub struct EmfPlusSetRenderingOrigin {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusSetRenderingOrigin from the RecordType
    /// enumeration. The value MUST be 0x401D.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that is not used. This
    /// field SHOULD be set to zero and MUST be ignored upon receipt.
    pub flags: u16,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be 0x00000014.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be 0x00000008.
    pub data_size: crate::parser::Size,
    /// x (4 bytes): A signed integer that defines the horizontal
    /// coordinate value of the rendering origin.
    pub x: i32,
    /// y (4 bytes): A signed integer that defines the vertical
    /// coordinate value of the rendering origin.
    pub y: i32,
}

impl EmfPlusSetRenderingOrigin {
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
            crate::parser::emf_plus::RecordType::EmfPlusSetRenderingOrigin
                as u16,
        )?;

        let x = read_field(buf, &mut data_size)?;
        let y = read_field(buf, &mut data_size)?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self { record_type, flags, size, data_size, x, y })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::RecordType;

    #[test]
    fn parses_origin_coordinates() {
        let mut data = vec![];
        data.extend((-3_i32).to_le_bytes());
        data.extend(7_i32.to_le_bytes());

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusSetRenderingOrigin::parse(
            &mut buf,
            RecordType::EmfPlusSetRenderingOrigin,
            0x0000,
            0x14,
            data_size,
        )
        .unwrap();

        assert_eq!(record.record_type, RecordType::EmfPlusSetRenderingOrigin);
        assert_eq!(record.flags, 0x0000);
        assert_eq!(record.size, 0x14);
        assert_eq!(record.x, -3);
        assert_eq!(record.y, 7);
    }

    #[test]
    fn rejects_truncated_data() {
        let data = 1_i32.to_le_bytes();
        let mut buf: &[u8] = &data[..];
        let data_size = crate::parser::Size::from(8);

        assert!(
            EmfPlusSetRenderingOrigin::parse(
                &mut buf,
                RecordType::EmfPlusSetRenderingOrigin,
                0x0000,
                0x14,
                data_size,
            )
            .is_err()
        );
    }
}
