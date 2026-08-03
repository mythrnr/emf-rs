/// The EmfPlusBeginContainer record opens a new graphics state
/// container and specifies a transform for it.
///
/// Each graphics state container MUST be added to an array of saved
/// graphics containers. The graphics state container is not written to
/// the EMF+ metafile, so its format can be determined by the
/// implementation.
#[derive(Clone, Debug)]
pub struct EmfPlusBeginContainer {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusBeginContainer from the RecordType enumeration.
    /// The value MUST be 0x4027.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    pub flags: u16,
    /// PageUnit (1 byte): The unit of measure for page space
    /// coordinates, from the UnitType enumeration. This value SHOULD
    /// NOT be UnitTypeDisplay or UnitTypeWorld.
    ///
    /// The bit diagram of the specification draws PageUnit in the
    /// high byte of the record flags, but GDI+ output (and the Wine
    /// and LibreOffice readers) place it in the low byte, exactly
    /// like EmfPlusSetPageTransform; the low byte is decoded here.
    pub page_unit: crate::parser::emf_plus::UnitType,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be 0x00000030.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be 0x00000024.
    pub data_size: crate::parser::Size,
    /// DestRect (16 bytes): An EmfPlusRectF object that, with SrcRect,
    /// specifies a transform for the container. This transformation
    /// results in SrcRect when applied to DestRect.
    pub dest_rect: crate::parser::emf_plus::objects::EmfPlusRectF,
    /// SrcRect (16 bytes): An EmfPlusRectF rectangle that, with
    /// DestRect, specifies a transform for the container. This
    /// transformation results in SrcRect when applied to DestRect.
    pub src_rect: crate::parser::emf_plus::objects::EmfPlusRectF,
    /// StackIndex (4 bytes): An unsigned integer that specifies an
    /// index to associate with the graphics state container. The index
    /// MUST be referenced by a subsequent EmfPlusEndContainer to close
    /// the graphics state container.
    pub stack_index: u32,
}

impl EmfPlusBeginContainer {
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
        use crate::parser::records::{read_field, read_with};

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u16,
            crate::parser::emf_plus::RecordType::EmfPlusBeginContainer as u16,
        )?;

        let page_unit =
            crate::parser::emf_plus::records::page_unit_from_flags(flags)?;
        let dest_rect = read_with(
            buf,
            &mut data_size,
            crate::parser::emf_plus::objects::EmfPlusRectF::parse,
        )?;
        let src_rect = read_with(
            buf,
            &mut data_size,
            crate::parser::emf_plus::objects::EmfPlusRectF::parse,
        )?;
        let stack_index: u32 = read_field(buf, &mut data_size)?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self {
            record_type,
            flags,
            page_unit,
            size,
            data_size,
            dest_rect,
            src_rect,
            stack_index,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_container_transform() {
        let mut data = vec![];
        for v in [0.0_f32, 0.0, 200.0, 100.0] {
            data.extend(v.to_le_bytes());
        }
        for v in [10.0_f32, 20.0, 50.0, 25.0] {
            data.extend(v.to_le_bytes());
        }
        data.extend(7_u32.to_le_bytes());

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusBeginContainer::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusBeginContainer,
            0x0002,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusBeginContainer,
        );
        assert_eq!(record.flags, 0x0002);
        assert_eq!(record.size, u32::try_from(data.len()).unwrap() + 12);
        assert_eq!(
            record.page_unit,
            crate::parser::emf_plus::UnitType::UnitTypePixel,
        );
        assert_eq!(record.dest_rect.width.to_bits(), 200.0_f32.to_bits());
        assert_eq!(record.dest_rect.height.to_bits(), 100.0_f32.to_bits());
        assert_eq!(record.src_rect.x.to_bits(), 10.0_f32.to_bits());
        assert_eq!(record.src_rect.y.to_bits(), 20.0_f32.to_bits());
        assert_eq!(record.stack_index, 7);
    }

    #[test]
    fn rejects_invalid_page_unit() {
        let data = [0_u8; 36];
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);

        assert!(
            EmfPlusBeginContainer::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusBeginContainer,
                0x00FF,
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
            EmfPlusBeginContainer::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusSave,
                0x0002,
                0x30,
                data_size,
            )
            .is_err()
        );
    }
}
