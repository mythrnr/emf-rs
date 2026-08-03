/// The EmfPlusSetPageTransform record specifies scaling factors and
/// units for converting page space coordinates to device space
/// coordinates.
#[derive(Clone, Debug)]
pub struct EmfPlusSetPageTransform {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusSetPageTransform from the RecordType
    /// enumeration. The value MUST be 0x4030.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    pub flags: u16,
    /// PageUnit (1 byte): The unit of measure for page space
    /// coordinates, from the UnitType enumeration. This value SHOULD
    /// NOT be UnitTypeDisplay or UnitTypeWorld.
    pub page_unit: crate::parser::emf_plus::UnitType,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be 0x00000010.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be 0x00000004.
    pub data_size: crate::parser::Size,
    /// PageScale (4 bytes): A floating-point value that specifies the
    /// scale factor for converting page space coordinates to device
    /// space coordinates.
    pub page_scale: f32,
}

impl EmfPlusSetPageTransform {
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
            crate::parser::emf_plus::RecordType::EmfPlusSetPageTransform as u16,
        )?;

        let page_unit =
            crate::parser::emf_plus::records::page_unit_from_flags(flags)?;
        let page_scale = read_field(buf, &mut data_size)?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self { record_type, flags, page_unit, size, data_size, page_scale })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_page_unit_and_scale() {
        let data = 1.5_f32.to_le_bytes();
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusSetPageTransform::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusSetPageTransform,
            0x0002,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusSetPageTransform,
        );
        assert_eq!(record.flags, 0x0002);
        assert_eq!(record.size, 0x0000_0010);
        assert_eq!(
            record.page_unit,
            crate::parser::emf_plus::UnitType::UnitTypePixel,
        );
        assert_eq!(record.page_scale.to_bits(), 1.5_f32.to_bits());
    }

    #[test]
    fn rejects_an_invalid_page_unit() {
        let data = 1.0_f32.to_le_bytes();
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);

        // The low byte of the flags carries 0xFF, which is not a
        // member of the UnitType enumeration.
        assert!(
            EmfPlusSetPageTransform::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusSetPageTransform,
                0x00FF,
                u32::try_from(data.len()).unwrap() + 12,
                data_size,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_wrong_record_type() {
        let data = 1.0_f32.to_le_bytes();
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);

        assert!(
            EmfPlusSetPageTransform::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusSetWorldTransform,
                0x0002,
                u32::try_from(data.len()).unwrap() + 12,
                data_size,
            )
            .is_err()
        );
    }
}
