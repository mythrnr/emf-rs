/// The EMR_EXTTEXTOUTA record draws an ASCII text string using the current font
/// and text colors.
///
/// The font and text colors used for output are specified by the state of the
/// current graphics environment. A rectangle for clipping and/or opaquing can
/// be defined in the EmrText object in the aEmrText field.
///
/// This record SHOULD(Windows GDI emulates EMR_EXTTEXTOUTA with an
/// EMR_EXTTEXTOUTW record.) be emulated with an EMR_EXTTEXTOUTW record, which
/// requires the ASCII text string in the EmrText object to be converted to
/// Unicode UTF16-LE encoding.
#[derive(Clone, Debug)]
pub struct EMR_EXTTEXTOUTA {
    /// Type (4 bytes): An unsigned integer that identifies the record type as
    /// EMR_EXTTEXTOUTA from the RecordType enumeration. This value is
    /// 0x00000053.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19), which is
    /// not used and MUST be ignored on receipt.
    pub bounds: wmf_core::parser::RectL,
    /// iGraphicsMode (4 bytes): An unsigned integer that specifies the
    /// graphics mode from the GraphicsMode enumeration.
    pub i_graphics_mode: crate::parser::GraphicsMode,
    /// exScale (4 bytes): A FLOAT value that specifies the scale factor to
    /// apply along the X axis to convert from page space units to .01mm units.
    /// This SHOULD be used only if the graphics mode specified by
    /// iGraphicsMode is GM_COMPATIBLE.
    pub ex_scale: f32,
    /// eyScale (4 bytes): A FLOAT value that specifies the scale factor to
    /// apply along the Y axis to convert from page space units to .01mm units.
    /// This SHOULD be used only if the graphics mode specified by
    /// iGraphicsMode is GM_COMPATIBLE.
    pub ey_scale: f32,
    /// aEmrText (variable): An EmrText object that specifies the output string
    /// in 8-bit ASCII characters, text attributes, and spacing values.
    pub a_emr_text: crate::parser::EmrText,
}

impl EMR_EXTTEXTOUTA {
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
        if record_type != crate::parser::RecordType::EMR_EXTTEXTOUTA {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_EXTTEXTOUTA as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (bounds, bounds_bytes),
            (i_graphics_mode, i_graphics_mode_bytes),
            (ex_scale, ex_scale_bytes),
            (ey_scale, ey_scale_bytes),
            (a_emr_text, a_emr_text_bytes),
        ) = (
            wmf_core::parser::RectL::parse(buf)?,
            crate::parser::GraphicsMode::parse(buf)?,
            crate::parser::read_f32_from_le_bytes(buf)?,
            crate::parser::read_f32_from_le_bytes(buf)?,
            crate::parser::EmrText::parse(buf, &record_type)?,
        );

        size.consume(
            bounds_bytes
                + i_graphics_mode_bytes
                + ex_scale_bytes
                + ey_scale_bytes
                + a_emr_text_bytes,
        );

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self {
            record_type,
            size,
            bounds,
            i_graphics_mode,
            ex_scale,
            ey_scale,
            a_emr_text,
        })
    }
}
