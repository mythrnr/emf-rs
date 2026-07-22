/// The EmfPlusSetClipRect record combines the current clipping region
/// with a rectangle.
///
/// The new current clipping region is set to the result of the
/// CombineMode operation.
#[derive(Clone, Debug)]
pub struct EmfPlusSetClipRect {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusSetClipRect from the RecordType enumeration. The
    /// value MUST be 0x4032.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    pub flags: u16,
    /// CM (4 bits): The logical operation for combining two regions.
    /// See the CombineMode enumeration for the meanings of the values.
    pub combine_mode: crate::parser::emf_plus::CombineMode,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned size of the entire record in bytes, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be 0x0000001C.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of data in the record-specific
    /// data that follows. This number does not include the size of the
    /// invariant part of this record. For this record type, the value
    /// MUST be 0x00000010.
    pub data_size: crate::parser::Size,
    /// ClipRect (16 bytes): An EmfPlusRectF object that defines the
    /// rectangle to use in the CombineMode operation.
    pub clip_rect: crate::parser::emf_plus::objects::EmfPlusRectF,
}

impl EmfPlusSetClipRect {
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
            crate::parser::emf_plus::RecordType::EmfPlusSetClipRect as u16,
        )?;

        let combine_mode =
            crate::parser::emf_plus::records::combine_mode(flags)?;
        let clip_rect = read_with(
            buf,
            &mut data_size,
            crate::parser::emf_plus::objects::EmfPlusRectF::parse,
        )?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self {
            record_type,
            flags,
            combine_mode,
            size,
            data_size,
            clip_rect,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::objects::EmfPlusRectF;

    #[test]
    fn parses_the_clip_rectangle() {
        let mut data = vec![];
        for v in [1.0_f32, 2.0, 30.0, 40.0] {
            data.extend(v.to_le_bytes());
        }

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusSetClipRect::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusSetClipRect,
            0x0200,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusSetClipRect,
        );
        assert_eq!(record.flags, 0x0200);
        assert_eq!(record.size, 0x0000_001C);
        assert_eq!(
            record.combine_mode,
            crate::parser::emf_plus::CombineMode::CombineModeUnion,
        );
        assert_eq!(record.clip_rect, EmfPlusRectF {
            x: 1.0,
            y: 2.0,
            width: 30.0,
            height: 40.0
        });
    }

    #[test]
    fn rejects_truncated_rectangles() {
        let data = [0_u8; 8];
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);

        assert!(
            EmfPlusSetClipRect::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusSetClipRect,
                0x0000,
                u32::try_from(data.len()).unwrap() + 12,
                data_size,
            )
            .is_err()
        );
    }
}
