/// The EmfPlusRotateWorldTransform record performs a rotation on the
/// current world space transform.
#[derive(Clone, Debug)]
pub struct EmfPlusRotateWorldTransform {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusRotateWorldTransform from the RecordType
    /// enumeration. The value MUST be 0x402F.
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
    /// record type, the value MUST be 0x00000010.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be 0x00000004.
    pub data_size: crate::parser::Size,
    /// Angle (4 bytes): A floating-point value that specifies the
    /// angle of rotation in degrees. The operation is performed by
    /// constructing a new transform matrix from the rotation matrix
    /// diagram of the specification (Figure 3: Rotation Transform
    /// Matrix).
    ///
    /// The current world space transform is multiplied by this matrix,
    /// and the result becomes the new current world space transform.
    /// The Flags field determines the order of multiplication.
    pub angle: f32,
}

impl EmfPlusRotateWorldTransform {
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
            crate::parser::emf_plus::RecordType::EmfPlusRotateWorldTransform
                as u16,
        )?;

        let angle = read_field(buf, &mut data_size)?;

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
            angle,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_rotation_angle() {
        let data = 90.0_f32.to_le_bytes();
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusRotateWorldTransform::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusRotateWorldTransform,
            0x2000,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusRotateWorldTransform,
        );
        assert_eq!(record.flags, 0x2000);
        assert_eq!(record.size, 0x0000_0010);
        assert!(record.post_multiply);
        assert_eq!(record.angle.to_bits(), 90.0_f32.to_bits());
    }

    #[test]
    fn rejects_wrong_record_type() {
        let data = [0_u8; 4];
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);

        assert!(
            EmfPlusRotateWorldTransform::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusScaleWorldTransform,
                0x0000,
                u32::try_from(data.len()).unwrap() + 12,
                data_size,
            )
            .is_err()
        );
    }
}
