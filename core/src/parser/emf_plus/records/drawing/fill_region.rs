use crate::parser::emf_plus::records::{EmfPlusBrushIdOrColor, object_id};

/// The EmfPlusFillRegion record specifies filling the interior of a
/// graphics region.
#[derive(Clone, Debug)]
pub struct EmfPlusFillRegion {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusFillRegion from the RecordType enumeration. The
    /// value MUST be 0x4013.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    ///
    /// S (1 bit): This bit specifies the type of data in the BrushId
    /// field. If set, BrushId specifies a color as an EmfPlusARGB
    /// object. If clear, BrushId contains the index of an EmfPlusBrush
    /// object in the EMF+ Object Table.
    pub flags: u16,
    /// ObjectId (1 byte): The index of the EmfPlusRegion object to
    /// fill, in the EMF+ Object Table. The value MUST be zero to 63,
    /// inclusive.
    pub object_id: u8,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned size of the entire record in bytes, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be 0x00000010.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be 0x00000004.
    pub data_size: crate::parser::Size,
    /// BrushId (4 bytes): An unsigned integer that defines the brush,
    /// the content of which is determined by the S bit in the Flags
    /// field.
    pub brush_id: EmfPlusBrushIdOrColor,
}

impl EmfPlusFillRegion {
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
            crate::parser::emf_plus::RecordType::EmfPlusFillRegion as u16,
        )?;

        let brush_id =
            EmfPlusBrushIdOrColor::parse(buf, &mut data_size, flags)?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self {
            record_type,
            flags,
            object_id: object_id(flags),
            size,
            data_size,
            brush_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_brush_id_fill_and_object_id() {
        let data = 2_u32.to_le_bytes();

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusFillRegion::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusFillRegion,
            0x0007,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusFillRegion,
        );
        assert_eq!(record.flags, 0x0007);
        assert_eq!(record.size, 0x0000_0010);
        assert_eq!(record.object_id, 7);
        assert_eq!(record.brush_id, EmfPlusBrushIdOrColor::BrushId(2));
    }
}
