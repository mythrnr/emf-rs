/// The EMR_EXTTEXTOUTW record draws a Unicode text string using the current
/// font and text colors.
///
/// The font and text colors used for output are specified by properties in the
/// current state of EMF metafile playback. A rectangle for clipping and/or
/// opaquing can be defined in the EmrText object that is specified in the
/// aEmrText field.
#[derive(Clone, Debug)]
pub struct EMR_EXTTEXTOUTW {
    /// Type (4 bytes): An unsigned integer that identifies the record type as
    /// EMR_EXTTEXTOUTW from the RecordType enumeration. This value is
    /// 0x00000054.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19). It is
    /// not used and MUST be ignored on receipt.
    pub bounds: wmf_core::parser::RectL,
    /// iGraphicsMode (4 bytes): An unsigned integer that specifies the current
    /// graphics mode from the GraphicsMode enumeration.
    pub i_graphics_mode: crate::parser::GraphicsMode,
    /// exScale (4 bytes): A FLOAT value that specifies the scale factor to
    /// apply along the X axis to convert from page space units to .01mm units.
    /// This is used only if the graphics mode specified by iGraphicsMode is
    /// GM_COMPATIBLE.
    pub ex_scale: f32,
    /// eyScale (4 bytes): A FLOAT value that specifies the scale factor to
    /// apply along the Y axis to convert from page space units to .01mm units.
    /// This is used only if the graphics mode specified by iGraphicsMode is
    /// GM_COMPATIBLE.
    pub ey_scale: f32,
    /// wEmrText (variable): An EmrText object (section 2.2.5) that specifies
    /// the output string in Unicode UTF16-LE characters, with text attributes
    /// and spacing values.
    pub w_emr_text: crate::parser::EmrText,
}

impl EMR_EXTTEXTOUTW {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_EXTTEXTOUTW {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_EXTTEXTOUTW as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (bounds, bounds_bytes),
            (i_graphics_mode, i_graphics_mode_bytes),
            (ex_scale, ex_scale_bytes),
            (ey_scale, ey_scale_bytes),
        ) = (
            wmf_core::parser::RectL::parse(buf)?,
            crate::parser::GraphicsMode::parse(buf)?,
            crate::parser::read_f32_from_le_bytes(buf)?,
            crate::parser::read_f32_from_le_bytes(buf)?,
        );

        size.consume(
            bounds_bytes
                + i_graphics_mode_bytes
                + ex_scale_bytes
                + ey_scale_bytes,
        );

        let (w_emr_text, w_emr_text_bytes) = crate::parser::EmrText::parse(
            buf,
            &record_type,
            size.consumed_bytes(),
        )?;

        size.consume(w_emr_text_bytes);

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
            w_emr_text,
        })
    }
}
