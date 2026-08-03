/// The EmfPlusScaleWorldTransform record performs a scaling on the
/// current world space transform.
#[derive(Clone, Debug)]
pub struct EmfPlusScaleWorldTransform {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusScaleWorldTransform from the RecordType
    /// enumeration. The value MUST be 0x402E.
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
    /// Sx (4 bytes): A floating-point value that defines the
    /// horizontal scale factor. The scaling is performed by
    /// constructing a new transform matrix from the Sx and Sy field
    /// values, as shown in the scale matrix diagram of the
    /// specification (Figure 4: Scale Transform Matrix).
    pub sx: f32,
    /// Sy (4 bytes): A floating-point value that defines the vertical
    /// scale factor.
    pub sy: f32,
}

impl EmfPlusScaleWorldTransform {
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
            crate::parser::emf_plus::RecordType::EmfPlusScaleWorldTransform
                as u16,
        )?;

        let sx = read_field(buf, &mut data_size)?;
        let sy = read_field(buf, &mut data_size)?;

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
            sx,
            sy,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_scale_factors() {
        let mut data = vec![];
        data.extend(2.0_f32.to_le_bytes());
        data.extend(0.5_f32.to_le_bytes());

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusScaleWorldTransform::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusScaleWorldTransform,
            0x2000,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusScaleWorldTransform,
        );
        assert_eq!(record.flags, 0x2000);
        assert_eq!(record.size, 0x0000_0014);
        assert!(record.post_multiply);
        assert_eq!(record.sx.to_bits(), 2.0_f32.to_bits());
        assert_eq!(record.sy.to_bits(), 0.5_f32.to_bits());
    }

    #[test]
    fn rejects_wrong_record_type() {
        let data = [0_u8; 8];
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);

        assert!(
            EmfPlusScaleWorldTransform::parse(
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
