/// The EmfPlusMultiplyWorldTransform record multiplies the current
/// world space transform by a specified transform matrix.
#[derive(Clone, Debug)]
pub struct EmfPlusMultiplyWorldTransform {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusMultiplyWorldTransform from the RecordType
    /// enumeration. The value MUST be 0x402C.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    pub flags: u16,
    /// A (1 bit): If set, the transform matrix is post-multiplied. If
    /// clear, it is pre-multiplied.
    ///
    /// The A bit is the 0x2000 bit of the record flags.
    pub post_multiply: bool,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, this value MUST be 0x00000024.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data. For
    /// this record type, this value MUST be 0x00000018.
    pub data_size: crate::parser::Size,
    /// MatrixData (24 bytes): An EmfPlusTransformMatrix object that
    /// defines the multiplication matrix.
    pub matrix_data: crate::parser::emf_plus::objects::EmfPlusTransformMatrix,
}

impl EmfPlusMultiplyWorldTransform {
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
            crate::parser::emf_plus::RecordType::EmfPlusMultiplyWorldTransform
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

        Ok(Self {
            record_type,
            flags,
            post_multiply: flags & super::FLAG_POST_MULTIPLY != 0,
            size,
            data_size,
            matrix_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_multiplication_matrix() {
        let mut data = vec![];
        for v in [1.0_f32, 2.0, 3.0, 4.0, 5.0, 6.0] {
            data.extend(v.to_le_bytes());
        }

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusMultiplyWorldTransform::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusMultiplyWorldTransform,
            0x2000,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusMultiplyWorldTransform,
        );
        assert_eq!(record.flags, 0x2000);
        assert_eq!(record.size, 0x0000_0024);
        assert!(record.post_multiply);
        assert_eq!(record.matrix_data.m11.to_bits(), 1.0_f32.to_bits());
        assert_eq!(record.matrix_data.m12.to_bits(), 2.0_f32.to_bits());
        assert_eq!(record.matrix_data.m21.to_bits(), 3.0_f32.to_bits());
        assert_eq!(record.matrix_data.m22.to_bits(), 4.0_f32.to_bits());
        assert_eq!(record.matrix_data.dx.to_bits(), 5.0_f32.to_bits());
        assert_eq!(record.matrix_data.dy.to_bits(), 6.0_f32.to_bits());
    }

    #[test]
    fn rejects_wrong_record_type() {
        let data = [0_u8; 24];
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);

        assert!(
            EmfPlusMultiplyWorldTransform::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusSetWorldTransform,
                0x0000,
                u32::try_from(data.len()).unwrap() + 12,
                data_size,
            )
            .is_err()
        );
    }
}
