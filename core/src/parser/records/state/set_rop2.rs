/// The EMR_SETROP2 record defines a binary raster operation mode.
///
/// Binary raster operation mix modes define how to combine source and
/// destination colors when drawing with the current pen. The mix modes are
/// binary raster operation codes, representing all possible Boolean functions
/// of two variables, using the binary operations AND, OR, and XOR (exclusive
/// OR), and the unary operation NOT. The mix mode is for raster devices only;
/// it is not available for vector devices.
#[derive(Clone, Debug)]
pub struct EMR_SETROP2 {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETROP2. This value is 0x00000014.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// ROP2Mode (4 bytes): An unsigned integer that specifies the raster
    /// operation mode and is in the Binary Raster Op enumeration ([MS-WMF]
    /// section 2.1.1.2).
    pub rop2_mode: wmf_core::parser::BinaryRasterOperation,
}

impl EMR_SETROP2 {
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
        if record_type != crate::parser::RecordType::EMR_SETROP2 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETROP2 as u32,
                    record_type as u32
                ),
            });
        }

        let (rop2_mode, rop2_mode_bytes) =
            wmf_core::parser::BinaryRasterOperation::parse(buf)?;

        size.consume(rop2_mode_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, rop2_mode })
    }
}
