/// The EMR_POLYTEXTOUTA record draws one or more ASCII text strings using the
/// current font and text colors.
///
/// The font and text colors used for output are specified by properties in the
/// current state of EMF metafile playback.
///
/// EMR_POLYTEXTOUTA SHOULD<70> be emulated with a series of EMR_EXTTEXTOUTW
/// records, one per string. This requires the ASCII text string in each EmrText
/// object to be converted to Unicode UTF16-LE encoding.
#[derive(Clone, Debug)]
pub struct EMR_POLYTEXTOUTA {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_POLYTEXTOUTA. This value is 0x00000060.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19), which
    /// specifies the bounding rectangle in logical units.
    pub bounds: wmf_core::parser::RectL,
    /// iGraphicsMode (4 bytes): An unsigned integer that specifies the current
    /// graphics mode, from the GraphicsMode enumeration.
    pub i_graphics_mode: crate::parser::GraphicsMode,
    /// exScale (4 bytes): A FLOAT value that specifies the X scale from page
    /// units to .01mm units if graphics mode is GM_COMPATIBLE.
    pub ex_scale: f32,
    /// eyScale (4 bytes): A FLOAT value that specifies the Y scale from page
    /// units to .01mm units if graphics mode is GM_COMPATIBLE.
    pub ey_scale: f32,
    /// cStrings (4 bytes): An unsigned integer that specifies the number of
    /// EmrText objects.
    pub c_strings: u32,
    /// aEmrText (variable): An array of EmrText objects that specify the
    /// output strings in 8-bit ASCII characters, with text attributes, and
    /// spacing values. The number of EmrText objects is specified by cStrings.
    pub a_emr_text: Vec<crate::parser::EmrText>,
}

impl EMR_POLYTEXTOUTA {
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
        if record_type != crate::parser::RecordType::EMR_POLYTEXTOUTA {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_POLYTEXTOUTA as u32,
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

        let a_emr_text = {
            let mut entries = vec![];

            for _ in 0..c_strings {
                let (v, b) = crate::parser::EmrText::parse(buf, &record_type)?;

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
            a_emr_text,
        })
    }
}
