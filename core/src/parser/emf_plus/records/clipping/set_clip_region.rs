/// The EmfPlusSetClipRegion record combines the current clipping
/// region with another graphics region.
///
/// The new current clipping region is set to the result of performing
/// the CombineMode operation on the previous current clipping region
/// and the specified EmfPlusRegion object.
#[derive(Clone, Debug)]
pub struct EmfPlusSetClipRegion {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusSetClipRegion from the RecordType enumeration.
    /// The value MUST be 0x4034.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    pub flags: u16,
    /// CM (4 bits): The logical operation for combining two regions.
    /// See the CombineMode enumeration for the meanings of the values.
    pub combine_mode: crate::parser::emf_plus::CombineMode,
    /// ObjectID (1 byte): The index of an EmfPlusRegion object in the
    /// EMF+ Object Table. The value MUST be zero to 63, inclusive.
    pub object_id: u8,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned size of the entire record in bytes, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be 0x0000000C.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of data in the record-specific
    /// data that follows. This number does not include the size of the
    /// invariant part of this record. For this record type, the value
    /// MUST be 0x00000000.
    pub data_size: crate::parser::Size,
}

impl EmfPlusSetClipRegion {
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
            crate::parser::emf_plus::RecordType::EmfPlusSetClipRegion as u16,
        )?;

        let combine_mode =
            crate::parser::emf_plus::records::combine_mode(flags)?;
        let object_id = crate::parser::emf_plus::records::object_id(flags);

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self {
            record_type,
            flags,
            combine_mode,
            object_id,
            size,
            data_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_combine_mode_and_object_id_from_flags() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);
        let record = EmfPlusSetClipRegion::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusSetClipRegion,
            0x0302,
            12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusSetClipRegion,
        );
        assert_eq!(record.flags, 0x0302);
        assert_eq!(record.size, 0x0000_000C);
        assert_eq!(
            record.combine_mode,
            crate::parser::emf_plus::CombineMode::CombineModeXOR,
        );
        assert_eq!(record.object_id, 2);
    }

    #[test]
    fn rejects_invalid_combine_mode() {
        let mut buf: &[u8] = &[];
        let data_size = crate::parser::Size::from(0);

        // CM bits carry 0xF, past the last CombineMode member (0x5).
        assert!(
            EmfPlusSetClipRegion::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusSetClipRegion,
                0x0F00,
                12,
                data_size,
            )
            .is_err()
        );
    }
}
