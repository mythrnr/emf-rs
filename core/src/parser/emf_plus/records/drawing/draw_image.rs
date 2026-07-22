use crate::parser::emf_plus::records::{FLAG_C, flag, object_id};

/// The EmfPlusDrawImage record specifies drawing a scaled image.
#[derive(Clone, Debug)]
pub struct EmfPlusDrawImage {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusDrawImage from the RecordType enumeration. The
    /// value MUST be 0x401A.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    ///
    /// C (1 bit): This bit indicates whether the data in the RectData
    /// field is compressed. If set, RectData contains an EmfPlusRect
    /// object. If clear, RectData contains an EmfPlusRectF object.
    pub flags: u16,
    /// ObjectID (1 byte): The index of an EmfPlusImage object in the
    /// EMF+ Object Table, which specifies the image to render. The
    /// value MUST be zero to 63, inclusive.
    pub object_id: u8,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. For this
    /// record type, the value MUST be one of the following:
    ///
    /// | Value | Meaning |
    /// |:-|:-|
    /// | `0x0000002C` | If the C bit is set in the Flags field. |
    /// | `0x00000034` | If the C bit is clear in the Flags field. |
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. For this record type, the value MUST be one of the
    /// following:
    ///
    /// | Value | Meaning |
    /// |:-|:-|
    /// | `0x00000020` | If the C bit is set in the Flags field. |
    /// | `0x00000028` | If the C bit is clear in the Flags field. |
    pub data_size: crate::parser::Size,
    /// ImageAttributesID (4 bytes): An unsigned integer that specifies
    /// the index of an optional EmfPlusImageAttributes object in the
    /// EMF+ Object Table.
    pub image_attributes_id: u32,
    /// SrcUnit (4 bytes): A signed integer that specifies the units of
    /// the SrcRect field. It MUST be the UnitTypePixel member of the
    /// UnitType enumeration.
    pub src_unit: crate::parser::emf_plus::UnitType,
    /// SrcRect (16 bytes): An EmfPlusRectF object that specifies a
    /// portion of the image to be rendered. The portion of the image
    /// specified by this rectangle is scaled to fit the destination
    /// rectangle specified by the RectData field.
    pub src_rect: crate::parser::emf_plus::objects::EmfPlusRectF,
    /// RectData (variable): Either an EmfPlusRect or EmfPlusRectF
    /// object that defines the bounding box of the image. The portion
    /// of the image specified by the SrcRect field is scaled to fit
    /// this rectangle.
    pub rect_data: crate::parser::emf_plus::objects::EmfPlusRectData,
}

impl EmfPlusDrawImage {
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
            crate::parser::emf_plus::RecordType::EmfPlusDrawImage as u16,
        )?;

        let image_attributes_id: u32 = read_field(buf, &mut data_size)?;
        let src_unit = read_with(
            buf,
            &mut data_size,
            crate::parser::emf_plus::UnitType::parse,
        )?;
        let src_rect = read_with(
            buf,
            &mut data_size,
            crate::parser::emf_plus::objects::EmfPlusRectF::parse,
        )?;
        let rect_data =
            crate::parser::emf_plus::objects::EmfPlusRectData::parse(
                buf,
                &mut data_size,
                flag(flags, FLAG_C),
            )?;

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
            image_attributes_id,
            src_unit,
            src_rect,
            rect_data,
        })
    }
}
