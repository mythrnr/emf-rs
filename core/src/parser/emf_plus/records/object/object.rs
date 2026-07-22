use crate::imports::*;

/// The EmfPlusObject record specifies an object for use in graphics
/// operations. The object definition can span multiple records, which
/// is indicated by the value of the Flags field.
///
/// The object data is kept as raw bytes here: an object definition can
/// span multiple records (continued objects), so typing the data
/// requires reassembly first. `EmfPlusObjectAssembler` combines
/// fragments and produces the typed `EmfPlusObjectData`. The
/// specification draws the optional TotalObjectSize field between Size
/// and DataSize, but GDI+ output and the LibreOffice reader place it
/// at the start of the object data of every record of a continued
/// series, which is the layout implemented here.
#[derive(Clone, Debug)]
pub struct EmfPlusObject {
    /// Type (2 bytes): An unsigned integer that defines this record
    /// type as EmfPlusObject from the RecordType enumeration. The
    /// value MUST be 0x4008.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    pub flags: u16,
    /// C (1 bit): Indicates that the object definition continues on in
    /// the next EmfPlusObject record. This flag is never set in the
    /// final record that defines the object.
    pub continuable: bool,
    /// ObjectType (7 bits): The type of object to be created by this
    /// record, from the ObjectType enumeration.
    pub object_type: crate::parser::emf_plus::ObjectType,
    /// ObjectID (1 byte): The index in the EMF+ Object Table to
    /// associate with the object created by this record. The value
    /// MUST be zero to 63, inclusive.
    pub object_id: u8,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned size of the entire record in bytes, including
    /// the 12-byte record header and the record-specific buffer data.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of data in the record-specific
    /// data that follows. This number does not include the size of the
    /// invariant part of this record. For this record type, the value
    /// varies based on the size of object.
    pub data_size: crate::parser::Size,
    /// ObjectData (variable): An array of bytes that contains data for
    /// the type of object specified in the Flags field. The content
    /// and format of the data can be different for each object type.
    /// See the individual object definitions in section 2.2.1 for
    /// additional information.
    ///
    /// For records that belong to a continued object series (including
    /// the final one), the data begins with the 4-byte TotalObjectSize
    /// field.
    pub object_data: Vec<u8>,
}

impl EmfPlusObject {
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
        use crate::parser::records::read_bytes_field;

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u16,
            crate::parser::emf_plus::RecordType::EmfPlusObject as u16,
        )?;

        let object_type_raw = u32::from((flags >> 8) & 0x007F);
        let Some(object_type) =
            crate::parser::emf_plus::ObjectType::from_repr(object_type_raw)
        else {
            return Err(crate::parser::ParseError::UnexpectedEnumValue {
                cause: alloc::format!(
                    "unexpected value as ObjectType: {object_type_raw:#04X}",
                )
                .into(),
            });
        };

        let object_id = crate::parser::emf_plus::records::object_id(flags);

        crate::parser::ParseError::expect_le("ObjectID", object_id, 63)?;

        let remaining = data_size.remaining_bytes();
        let object_data = read_bytes_field(buf, &mut data_size, remaining)?;

        Ok(Self {
            record_type,
            flags,
            continuable: flags & 0x8000 != 0,
            object_type,
            object_id,
            size,
            data_size,
            object_data,
        })
    }
}
