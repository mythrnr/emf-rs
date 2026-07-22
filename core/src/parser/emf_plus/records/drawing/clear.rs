/// The EmfPlusClear record clears the output coordinate space and
/// initializes it with a background color and transparency.
#[derive(Clone, Debug)]
pub struct EmfPlusClear {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusClear from the RecordType enumeration. The value
    /// MUST be 0x4009.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that is not used. This
    /// field SHOULD be set to zero and MUST be ignored upon receipt.
    pub flags: u16,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, it MUST be 0x00000010.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, it MUST be 0x00000004.
    pub data_size: crate::parser::Size,
    /// Color (4 bytes): An EmfPlusARGB object that defines the color
    /// to paint the screen. All colors are specified in [IEC-RGB],
    /// unless otherwise noted.
    pub color: crate::parser::emf_plus::objects::EmfPlusARGB,
}

impl EmfPlusClear {
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
            crate::parser::emf_plus::RecordType::EmfPlusClear as u16,
        )?;

        let color = read_with(
            buf,
            &mut data_size,
            crate::parser::emf_plus::objects::EmfPlusARGB::parse,
        )?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self { record_type, flags, size, data_size, color })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::objects::EmfPlusARGB;

    #[test]
    fn parses_the_background_color_and_stores_the_header() {
        let data = [0x01, 0x02, 0x03, 0xFF];
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);

        let record = EmfPlusClear::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusClear,
            0x0000,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(record.record_type as u16, 0x4009);
        assert_eq!(record.flags, 0x0000);
        assert_eq!(record.size, 0x0000_0010);
        assert_eq!(record.color, EmfPlusARGB {
            blue: 0x01,
            green: 0x02,
            red: 0x03,
            alpha: 0xFF,
        });
    }
}
