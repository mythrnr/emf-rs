/// The EMR_SETMAPMODE record specifies the current mapping mode, which
/// specifies the unit of measure used to transform page space units into device
/// space units, and also specifies the orientation of the device's x-axis and
/// y-axis.
///
/// MM_TEXT mode allows applications to work in device pixels, whose size varies
/// from device to device.
///
/// The MM_HIENGLISH, MM_HIMETRIC, MM_LOENGLISH, MM_LOMETRIC, and MM_TWIPS modes
/// are useful for applications drawing in physically meaningful units such as
/// inches or millimeters.
///
/// MM_ISOTROPIC mode ensures a 1:1 aspect ratio.
///
/// MM_ANISOTROPIC mode allows the x-coordinates and y-coordinates to be
/// adjusted independently.
#[derive(Clone, Debug)]
pub struct EMR_SETMAPMODE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETMAPMODE. This value is 0x00000011.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// MapMode (4 bytes): An unsigned integer from the MapMode enumeration.
    pub map_mode: crate::parser::MapMode,
}

impl EMR_SETMAPMODE {
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
        if record_type != crate::parser::RecordType::EMR_SETMAPMODE {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETMAPMODE as u32,
                    record_type as u32
                ),
            });
        }

        let (map_mode, map_mode_bytes) = crate::parser::MapMode::parse(buf)?;

        size.consume(map_mode_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, map_mode })
    }
}
