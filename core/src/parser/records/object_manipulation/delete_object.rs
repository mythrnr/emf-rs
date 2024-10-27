/// The EMR_DELETEOBJECT record deletes a graphics object, which is specified by
/// its index in the EMF .object table
///
/// This value MUST NOT be 0, which is a reserved index that refers to the EMF
/// metafile itself; and it MUST NOT be the index of a stock object, which
/// cannot be deleted. Stock object indexes are specified in the StockObject
/// enumeration.
///
/// The object specified by this record MUST be deleted from the EMF object
/// table. If the deleted object is currently selected into the playback device
/// context, the default object MUST be restored.
#[derive(Clone, Debug)]
pub struct EMR_DELETEOBJECT {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_DELETEOBJECT. This value is 0x00000028.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// ihObject (4 bytes): An unsigned integer that specifies the index of a
    /// graphics object in the EMF object table.
    pub in_object: u32,
}

impl EMR_DELETEOBJECT {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_DELETEOBJECT {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_DELETEOBJECT as u32,
                    record_type as u32
                ),
            });
        }

        let (in_object, in_object_bytes) =
            crate::parser::read_u32_from_le_bytes(buf)?;

        size.consume(in_object_bytes);

        if in_object == 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "in_object must not be zero, but parsed value is \
                     `{in_object:#010X}`",
                ),
            });
        }

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, in_object })
    }
}
