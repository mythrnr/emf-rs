/// The EmfPlusSetAntiAliasMode record specifies the anti-aliasing mode
/// for text output.
#[derive(Clone, Debug)]
pub struct EmfPlusSetAntiAliasMode {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusSetAntiAliasMode from the RecordType
    /// enumeration. The value MUST be 0x401E.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    pub flags: u16,
    /// SmoothingMode (7 bits): The smoothing mode value, from the
    /// SmoothingMode enumeration.
    ///
    /// Decoded from bits 1-7 of the record flags
    /// (`(flags >> 1) & 0x7F`); the reserved upper bits are ignored as
    /// the specification mandates.
    pub smoothing_mode: crate::parser::emf_plus::SmoothingMode,
    /// A (1 bit): If set, anti-aliasing SHOULD be performed. If clear,
    /// anti-aliasing SHOULD NOT be performed.
    ///
    /// Decoded from the A bit (0x0001) of the record flags.
    pub anti_aliasing: bool,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned size of the entire record in bytes, including
    /// the 12-byte record header and the record-specific buffer data.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of data in the record-specific
    /// data that follows. This number does not include the size of the
    /// invariant part of this record. For this record type, the value
    /// MUST be 0x00000000.
    pub data_size: crate::parser::Size,
}

impl EmfPlusSetAntiAliasMode {
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
            crate::parser::emf_plus::RecordType::EmfPlusSetAntiAliasMode as u16,
        )?;

        // SmoothingMode occupies 7 bits; the bits above it are
        // reserved and MUST be ignored, so they are masked off instead
        // of failing the parse when set.
        let raw = u32::from((flags >> 1) & 0x7F);
        let Some(smoothing_mode) =
            crate::parser::emf_plus::SmoothingMode::from_repr(raw)
        else {
            return Err(crate::parser::ParseError::UnexpectedEnumValue {
                cause: alloc::format!(
                    "unexpected value as SmoothingMode: {raw:#04X}",
                )
                .into(),
            });
        };

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self {
            record_type,
            flags,
            smoothing_mode,
            anti_aliasing: flags & 0x0001 != 0,
            size,
            data_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_flag_encoded_smoothing_mode() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);
        let record = EmfPlusSetAntiAliasMode::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusSetAntiAliasMode,
            0x0009,
            0x0C,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusSetAntiAliasMode,
        );
        assert_eq!(record.flags, 0x0009);
        assert_eq!(record.size, 0x0C);
        assert_eq!(
            record.smoothing_mode,
            crate::parser::emf_plus::SmoothingMode::SmoothingModeAntiAlias8x4,
        );
        assert!(record.anti_aliasing);
    }

    #[test]
    fn ignores_reserved_flag_bits() {
        // Bit 8 is reserved; the record must still parse and the
        // SmoothingMode bits (all clear) must decode to the default.
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);
        let record = EmfPlusSetAntiAliasMode::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusSetAntiAliasMode,
            0x0100,
            0x0C,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.smoothing_mode,
            crate::parser::emf_plus::SmoothingMode::SmoothingModeDefault,
        );
        assert!(!record.anti_aliasing);
    }

    #[test]
    fn rejects_unknown_smoothing_mode() {
        // 0x7F is inside the 7-bit SmoothingMode field but is not a
        // defined enumeration value.
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);

        assert!(
            EmfPlusSetAntiAliasMode::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusSetAntiAliasMode,
                0x7F << 1,
                0x0C,
                data_size,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_wrong_record_type() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);

        assert!(
            EmfPlusSetAntiAliasMode::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusSetTextContrast,
                0x0009,
                0x0C,
                data_size,
            )
            .is_err()
        );
    }
}
