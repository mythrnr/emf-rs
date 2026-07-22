/// The EmfPlusSetWorldTransform record sets the world transform
/// according to the values in a specified transform matrix.
#[derive(Clone, Debug)]
pub struct EmfPlusSetWorldTransform {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusSetWorldTransform from the RecordType
    /// enumeration. The value MUST be 0x402A.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that is not used. This
    /// field SHOULD be set to zero and MUST be ignored upon receipt.
    pub flags: u16,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be 0x00000024.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be 0x00000018.
    pub data_size: crate::parser::Size,
    /// MatrixData (24 bytes): An EmfPlusTransformMatrix object that
    /// defines the new current world transform.
    pub matrix_data: crate::parser::emf_plus::objects::EmfPlusTransformMatrix,
}

impl EmfPlusSetWorldTransform {
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
        use crate::parser::records::read_with;

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u16,
            crate::parser::emf_plus::RecordType::EmfPlusSetWorldTransform
                as u16,
        )?;

        let matrix_data = read_with(
            buf,
            &mut data_size,
            crate::parser::emf_plus::objects::EmfPlusTransformMatrix::parse,
        )?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self { record_type, flags, size, data_size, matrix_data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_world_transform_matrix() {
        let mut data = vec![];
        for v in [1.0_f32, 0.0, 0.0, 1.0, 10.0, 20.0] {
            data.extend(v.to_le_bytes());
        }

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusSetWorldTransform::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusSetWorldTransform,
            0x0000,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusSetWorldTransform,
        );
        assert_eq!(record.flags, 0x0000);
        assert_eq!(record.size, 0x0000_0024);
        assert_eq!(record.matrix_data.m11.to_bits(), 1.0_f32.to_bits());
        assert_eq!(record.matrix_data.m22.to_bits(), 1.0_f32.to_bits());
        assert_eq!(record.matrix_data.dx.to_bits(), 10.0_f32.to_bits());
        assert_eq!(record.matrix_data.dy.to_bits(), 20.0_f32.to_bits());
    }

    #[test]
    fn rejects_wrong_record_type() {
        let data = [0_u8; 24];
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);

        assert!(
            EmfPlusSetWorldTransform::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusResetWorldTransform,
                0x0000,
                u32::try_from(data.len()).unwrap() + 12,
                data_size,
            )
            .is_err()
        );
    }
}
