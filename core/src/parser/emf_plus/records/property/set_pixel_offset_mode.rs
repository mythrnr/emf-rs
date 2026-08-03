/// The EmfPlusSetPixelOffsetMode record specifies how pixels are
/// centered with respect to the coordinates of the drawing surface.
#[derive(Clone, Debug)]
pub struct EmfPlusSetPixelOffsetMode {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusSetPixelOffsetMode from the RecordType
    /// enumeration. The value MUST be 0x4022.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    pub flags: u16,
    /// PixelOffsetMode (1 byte): The pixel offset mode value, from the
    /// PixelOffsetMode enumeration.
    ///
    /// Decoded from the low byte (bits 0-7) of the record flags.
    pub pixel_offset_mode: crate::parser::emf_plus::PixelOffsetMode,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be 0x0000000C.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be 0x00000000.
    pub data_size: crate::parser::Size,
}

impl EmfPlusSetPixelOffsetMode {
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
        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u16,
            crate::parser::emf_plus::RecordType::EmfPlusSetPixelOffsetMode
                as u16,
        )?;

        let pixel_offset_mode = super::enum_from_low_byte(
            flags,
            crate::parser::emf_plus::PixelOffsetMode::from_repr,
            "PixelOffsetMode",
        )?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self { record_type, flags, pixel_offset_mode, size, data_size })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::{PixelOffsetMode, RecordType};

    #[test]
    fn parses_flag_encoded_pixel_offset_mode() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);
        let record = EmfPlusSetPixelOffsetMode::parse(
            &mut buf,
            RecordType::EmfPlusSetPixelOffsetMode,
            0x0004,
            0x0C,
            data_size,
        )
        .unwrap();

        assert_eq!(record.record_type, RecordType::EmfPlusSetPixelOffsetMode);
        assert_eq!(record.flags, 0x0004);
        assert_eq!(record.size, 0x0C);
        assert_eq!(
            record.pixel_offset_mode,
            PixelOffsetMode::PixelOffsetModeHalf,
        );
    }

    #[test]
    fn rejects_unknown_pixel_offset_mode() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);

        assert!(
            EmfPlusSetPixelOffsetMode::parse(
                &mut buf,
                RecordType::EmfPlusSetPixelOffsetMode,
                0x0005,
                0x0C,
                data_size,
            )
            .is_err()
        );
    }
}
