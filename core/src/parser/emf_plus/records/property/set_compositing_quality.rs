/// The EmfPlusSetCompositingQuality record specifies the desired level
/// of quality for creating composite images from multiple objects.
#[derive(Clone, Debug)]
pub struct EmfPlusSetCompositingQuality {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusSetCompositingQuality from the RecordType
    /// enumeration. The value MUST be 0x4024.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    pub flags: u16,
    /// CompositingQuality (1 byte): The compositing quality value,
    /// from the CompositingQuality enumeration.
    ///
    /// Decoded from the low byte (bits 0-7) of the record flags.
    pub compositing_quality: crate::parser::emf_plus::CompositingQuality,
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

impl EmfPlusSetCompositingQuality {
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
            crate::parser::emf_plus::RecordType::EmfPlusSetCompositingQuality
                as u16,
        )?;

        let compositing_quality = super::enum_from_low_byte(
            flags,
            crate::parser::emf_plus::CompositingQuality::from_repr,
            "CompositingQuality",
        )?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self { record_type, flags, compositing_quality, size, data_size })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::{CompositingQuality, RecordType};

    #[test]
    fn parses_flag_encoded_compositing_quality() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);
        let record = EmfPlusSetCompositingQuality::parse(
            &mut buf,
            RecordType::EmfPlusSetCompositingQuality,
            0x0002,
            0x0C,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            RecordType::EmfPlusSetCompositingQuality,
        );
        assert_eq!(record.flags, 0x0002);
        assert_eq!(record.size, 0x0C);
        assert_eq!(
            record.compositing_quality,
            CompositingQuality::CompositingQualityHighQuality,
        );
    }

    #[test]
    fn parses_zero_flags_as_default_quality() {
        // GDI+ writes the native, zero-based CompositingQuality into the
        // record flags, so 0x0000 is the default quality, not an unknown
        // value. Rejecting it would fail conversion of real metafiles.
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);
        let record = EmfPlusSetCompositingQuality::parse(
            &mut buf,
            RecordType::EmfPlusSetCompositingQuality,
            0x0000,
            0x0C,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.compositing_quality,
            CompositingQuality::CompositingQualityDefault,
        );
    }

    #[test]
    fn rejects_unknown_compositing_quality() {
        // 0x05 is one past AssumeLinear (0x04), the highest defined value.
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);

        assert!(
            EmfPlusSetCompositingQuality::parse(
                &mut buf,
                RecordType::EmfPlusSetCompositingQuality,
                0x0005,
                0x0C,
                data_size,
            )
            .is_err()
        );
    }
}
