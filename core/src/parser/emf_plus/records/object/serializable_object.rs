use crate::parser::emf_plus::objects::{Guid, ImageEffect};

/// The EmfPlusSerializableObject record defines an image effects
/// parameter block that has been serialized into a data buffer.
#[derive(Clone, Debug)]
pub struct EmfPlusSerializableObject {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusSerializableObject from the RecordType
    /// enumeration. The value MUST be 0x4038.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that is not used. This
    /// field SHOULD be set to zero and MUST be ignored upon receipt.
    pub flags: u16,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be computed as follows:
    /// Size = BufferSize + 0x00000020
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be computed as
    /// follows:
    /// DataSize = BufferSize + 0x00000014
    pub data_size: crate::parser::Size,
    /// ObjectGUID (16 bytes): The GUID packet representation value
    /// ([MS-DTYP] section 2.3.4.2) for the image effect. This MUST
    /// correspond to one of the ImageEffects identifiers.
    pub object_guid: Guid,
    /// BufferSize (4 bytes): An unsigned integer that specifies the
    /// size in bytes of the 32-bit-aligned Buffer field.
    pub buffer_size: u32,
    /// Buffer (variable): An array of BufferSize bytes that contain
    /// the serialized image effects parameter block that corresponds
    /// to the GUID in the ObjectGUID field. This MUST be one of the
    /// Image Effects objects, typed here as `ImageEffect`.
    pub buffer: ImageEffect,
}

impl EmfPlusSerializableObject {
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
        use crate::parser::records::{read_bytes_field, read_field, read_with};

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u16,
            crate::parser::emf_plus::RecordType::EmfPlusSerializableObject
                as u16,
        )?;

        let object_guid = read_with(buf, &mut data_size, Guid::parse)?;
        let buffer_size: u32 = read_field(buf, &mut data_size)?;

        crate::parser::ParseError::expect_le(
            "BufferSize",
            buffer_size as usize as u64,
            data_size.remaining_bytes() as u64,
        )?;

        let buffer_bytes =
            read_bytes_field(buf, &mut data_size, buffer_size as usize)?;
        let buffer = ImageEffect::parse(object_guid, &buffer_bytes)?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self {
            record_type,
            flags,
            size,
            data_size,
            object_guid,
            buffer_size,
            buffer,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        imports::*,
        parser::emf_plus::objects::{BLUR_EFFECT_GUID, BlurEffect},
    };

    /// The GUID packet representation of `BLUR_EFFECT_GUID` followed
    /// by BufferSize and the serialized BlurEffect parameter block.
    fn record_data(buffer_size: u32) -> Vec<u8> {
        let mut data = vec![];
        data.extend(0x633C_80A4_u32.to_le_bytes());
        data.extend(0x1843_u16.to_le_bytes());
        data.extend(0x482B_u16.to_le_bytes());
        data.extend([0x9E, 0xF2, 0xBE, 0x28, 0x34, 0xC5, 0xFD, 0xD4]);
        data.extend(buffer_size.to_le_bytes());
        data.extend(2.5_f32.to_le_bytes()); // BlurRadius
        data.extend(1_u32.to_le_bytes()); // ExpandEdge
        data
    }

    #[test]
    fn parses_a_blur_effect_parameter_block() {
        let data = record_data(8);
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusSerializableObject::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusSerializableObject,
            0,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusSerializableObject,
        );
        assert_eq!(record.flags, 0);
        assert_eq!(record.size, 0x28);
        assert_eq!(record.object_guid, BLUR_EFFECT_GUID);
        assert_eq!(record.buffer_size, 8);
        assert_eq!(
            record.buffer,
            ImageEffect::Blur(BlurEffect {
                blur_radius: 2.5,
                expand_edge: true,
            }),
        );
    }

    #[test]
    fn rejects_buffer_size_exceeding_the_record_data() {
        // BufferSize claims more bytes than the record data holds;
        // the guard must fail the parse before the buffer is sized.
        let data = record_data(64);
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);

        assert!(
            EmfPlusSerializableObject::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusSerializableObject,
                0,
                u32::try_from(data.len()).unwrap() + 12,
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
            EmfPlusSerializableObject::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusObject,
                0,
                0x0C,
                data_size,
            )
            .is_err()
        );
    }
}
