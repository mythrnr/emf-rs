/// The EMR_POLYTEXTOUTW record draws one or more Unicode text strings using the
/// current font and text colors.
///
/// The font and text colors used for output are specified by properties in the
/// current state of the playback device context.
///
/// EMR_POLYTEXTOUTW SHOULD be emulated with a series of EMR_EXTTEXTOUTW
/// records, one per string. Windows NT 3.1 is the only Windows version in which
/// GDI uses EMR_POLYTEXTOUTW records for text output. All other versions
/// emulate EMR_POLYTEXTOUTW with EMR_EXTTEXTOUTW records.
#[derive(Clone, Debug)]
pub struct EMR_POLYTEXTOUTW {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_POLYTEXTOUTW. This value is 0x00000061.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19), which
    /// specifies the bounding rectangle in logical units.
    pub bounds: wmf_core::parser::RectL,
    /// iGraphicsMode (4 bytes): An unsigned integer that specifies the current
    /// graphics mode.
    pub i_graphics_mode: crate::parser::GraphicsMode,
    /// exScale (4 bytes): A FLOAT value that specifies the X scale from page
    /// units to .01mm units if graphics mode is GM_COMPATIBLE.
    pub ex_scale: f32,
    /// eyScale (4 bytes): A FLOAT value that specifies the X scale from page
    /// units to .01mm units if graphics mode is GM_COMPATIBLE.
    pub ey_scale: f32,
    /// cStrings (4 bytes): An unsigned integer that specifies the number of
    /// EmrText objects.
    pub c_strings: u32,
    /// wEmrText (variable): An array of EmrText objects that specify the
    /// output strings in Unicode UTF16-LE characters, with text attributes and
    /// spacing values. The number of EmrText objects is specified by cStrings.
    pub w_emr_text: Vec<crate::parser::EmrText>,
}

impl EMR_POLYTEXTOUTW {
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
        if record_type != crate::parser::RecordType::EMR_POLYTEXTOUTW {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_POLYTEXTOUTW as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (bounds, bounds_bytes),
            (i_graphics_mode, i_graphics_mode_bytes),
            (ex_scale, ex_scale_bytes),
            (ey_scale, ey_scale_bytes),
            (c_strings, c_strings_bytes),
        ) = (
            wmf_core::parser::RectL::parse(buf)?,
            crate::parser::GraphicsMode::parse(buf)?,
            crate::parser::read_f32_from_le_bytes(buf)?,
            crate::parser::read_f32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        size.consume(
            bounds_bytes
                + i_graphics_mode_bytes
                + ex_scale_bytes
                + ey_scale_bytes
                + c_strings_bytes,
        );

        let w_emr_text = {
            let mut entries = vec![];

            for _ in 0..c_strings {
                let (v, b) = crate::parser::EmrText::parse(
                    buf,
                    &record_type,
                    size.consumed_bytes(),
                )?;

                entries.push(v);
                size.consume(b);
            }

            entries
        };

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
            c_strings,
            w_emr_text,
        })
    }
}
