/// The EmfPlusOffsetClip record applies a translation transform on the
/// current clipping region for the world space.
///
/// The new current clipping region is set to the result of the
/// translation transform.
#[derive(Clone, Debug)]
pub struct EmfPlusOffsetClip {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusOffsetClip from the RecordType enumeration. The
    /// value MUST be 0x4035.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that is reserved and MUST
    /// be ignored.
    pub flags: u16,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned size of the entire record in bytes, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be 0x00000014.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of data in the record-specific
    /// data that follows. This number does not include the size of the
    /// invariant part of this record. For this record type, the value
    /// MUST be 0x00000008.
    pub data_size: crate::parser::Size,
    /// dx (4 bytes): A floating-point value that specifies the
    /// horizontal offset for the translation.
    pub dx: f32,
    /// dy (4 bytes): A floating-point value that specifies the
    /// vertical offset for the translation.
    pub dy: f32,
}

impl EmfPlusOffsetClip {
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
            crate::parser::emf_plus::RecordType::EmfPlusOffsetClip as u16,
        )?;

        let dx = read_field(buf, &mut data_size)?;
        let dy = read_field(buf, &mut data_size)?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self { record_type, flags, size, data_size, dx, dy })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_translation_offsets() {
        let mut data = vec![];
        data.extend(1.5_f32.to_le_bytes());
        data.extend((-2.5_f32).to_le_bytes());

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusOffsetClip::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusOffsetClip,
            0x0000,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusOffsetClip,
        );
        assert_eq!(record.flags, 0x0000);
        assert_eq!(record.size, 0x0000_0014);
        assert_eq!(record.dx.to_bits(), 1.5_f32.to_bits());
        assert_eq!(record.dy.to_bits(), (-2.5_f32).to_bits());
    }

    #[test]
    fn rejects_wrong_record_type() {
        let data = [0_u8; 8];
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);

        assert!(
            EmfPlusOffsetClip::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusResetClip,
                0x0000,
                u32::try_from(data.len()).unwrap() + 12,
                data_size,
            )
            .is_err()
        );
    }
}
