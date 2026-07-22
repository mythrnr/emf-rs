/// The EmfPlusSetInterpolationMode record specifies how image scaling,
/// including stretching and shrinking, is performed.
#[derive(Clone, Debug)]
pub struct EmfPlusSetInterpolationMode {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusSetInterpolationMode from the RecordType
    /// enumeration. The value MUST be 0x4021.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    pub flags: u16,
    /// InterpolationMode (1 byte): The interpolation mode value, from
    /// the InterpolationMode enumeration.
    ///
    /// Decoded from the low byte (bits 0-7) of the record flags.
    pub interpolation_mode: crate::parser::emf_plus::InterpolationMode,
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

impl EmfPlusSetInterpolationMode {
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
            crate::parser::emf_plus::RecordType::EmfPlusSetInterpolationMode
                as u16,
        )?;

        let interpolation_mode = super::enum_from_low_byte(
            flags,
            crate::parser::emf_plus::InterpolationMode::from_repr,
            "InterpolationMode",
        )?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self { record_type, flags, interpolation_mode, size, data_size })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::{InterpolationMode, RecordType};

    #[test]
    fn parses_flag_encoded_interpolation_mode() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);
        let record = EmfPlusSetInterpolationMode::parse(
            &mut buf,
            RecordType::EmfPlusSetInterpolationMode,
            0x0007,
            0x0C,
            data_size,
        )
        .unwrap();

        let expected = RecordType::EmfPlusSetInterpolationMode;
        assert_eq!(record.record_type, expected);
        assert_eq!(record.flags, 0x0007);
        assert_eq!(record.size, 0x0C);
        assert_eq!(
            record.interpolation_mode,
            InterpolationMode::InterpolationModeHighQualityBicubic,
        );
    }

    #[test]
    fn rejects_unknown_interpolation_mode() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);

        assert!(
            EmfPlusSetInterpolationMode::parse(
                &mut buf,
                RecordType::EmfPlusSetInterpolationMode,
                0x0008,
                0x0C,
                data_size,
            )
            .is_err()
        );
    }
}
