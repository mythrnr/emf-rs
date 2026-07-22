/// The EmfPlusTranslateWorldTransform record performs a translation
/// on the current world space transform.
#[derive(Clone, Debug)]
pub struct EmfPlusTranslateWorldTransform {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusTranslateWorldTransform from the RecordType
    /// enumeration. The value MUST be 0x402D.
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
    /// record type, the value MUST be 0x00000014.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be 0x00000008.
    pub data_size: crate::parser::Size,
    /// dx (4 bytes): A floating-point value that defines the
    /// horizontal distance. The translation is performed by
    /// constructing a new world transform matrix from the dx and dy
    /// fields.
    pub dx: f32,
    /// dy (4 bytes): A floating-point value that defines the vertical
    /// distance value.
    pub dy: f32,
}

impl EmfPlusTranslateWorldTransform {
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
            crate::parser::emf_plus::RecordType::EmfPlusTranslateWorldTransform
                as u16,
        )?;

        let dx = read_field(buf, &mut data_size)?;
        let dy = read_field(buf, &mut data_size)?;

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
            dx,
            dy,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_translation_distances() {
        let mut data = vec![];
        data.extend(10.0_f32.to_le_bytes());
        data.extend((-5.0_f32).to_le_bytes());

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusTranslateWorldTransform::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusTranslateWorldTransform,
            0x2000,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusTranslateWorldTransform,
        );
        assert_eq!(record.flags, 0x2000);
        assert_eq!(record.size, 0x0000_0014);
        assert!(record.post_multiply);
        assert_eq!(record.dx.to_bits(), 10.0_f32.to_bits());
        assert_eq!(record.dy.to_bits(), (-5.0_f32).to_bits());
    }

    #[test]
    fn rejects_wrong_record_type() {
        let data = [0_u8; 8];
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);

        assert!(
            EmfPlusTranslateWorldTransform::parse(
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
