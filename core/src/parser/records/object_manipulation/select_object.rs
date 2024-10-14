/// The EMR_SELECTOBJECT record selects a graphics object into the playback
/// device context.
///
/// The object index MUST NOT be zero, which is reserved and refers to the EMF
/// metafile itself.
///
/// The object specified by this record MUST be used in subsequent EMF drawing
/// operations, until another EMR_SELECTOBJECT record changes the object of that
/// type or the object is deleted.
#[derive(Clone, Debug)]
pub struct EMR_SELECTOBJECT {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SELECTOBJECT. This value is 0x00000025.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// ihObject (4 bytes): An unsigned integer that specifies either the index
    /// of a graphics object in the EMF object table or the index of a stock
    /// object in the StockObject enumeration.
    pub in_object: u32,
}

impl EMR_SELECTOBJECT {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_SELECTOBJECT {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SELECTOBJECT as u32,
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
