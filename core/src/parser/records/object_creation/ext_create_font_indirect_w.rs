/// The EMR_EXTCREATEFONTINDIRECTW record defines a logical font for graphics
/// operations.
///
/// The logical font object defined by this record can be selected into the
/// playback device context by an EMR_SELECTOBJECT record, which specifies the
/// logical font to use in subsequent graphics operations.
#[derive(Clone, Debug)]
pub struct EMR_EXTCREATEFONTINDIRECTW {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_EXTCREATEFONTINDIRECTW. This value is 0x00000052.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// ihFonts (4 bytes): An unsigned integer that specifies the index of the
    /// logical font object in the EMF object table. This index MUST be saved
    /// so that this object can be reused or modified.
    pub ih_fonts: u32,
    /// elw (variable): A LogFontExDv object, which specifies the logical font.
    /// A LogFont object MAY(Windows NT 3.1, Windows NT 3.5, Windows NT 3.51,
    /// and Windows NT 4.0 metafiles contain a LogFont object in this field.)
    /// be present instead. The process for determining the type of object in
    /// this field is described below.
    ///
    /// The type of logical font object in the elw field of this record is
    /// determined by the following algorithm (all size and length values are
    /// in bytes):
    pub elw: ELW,
}

impl EMR_EXTCREATEFONTINDIRECTW {
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
        if record_type != crate::parser::RecordType::EMR_EXTCREATEFONTINDIRECTW
        {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_EXTCREATEFONTINDIRECTW
                        as u32,
                    record_type as u32
                ),
            });
        }

        let (ih_fonts, ih_fonts_bytes) =
            crate::parser::read_u32_from_le_bytes(buf)?;

        size.consume(ih_fonts_bytes);

        let elw_size = size.remaining_bytes();

        // The size of the elw field must be equal to or greater than
        // the size of a LogFontPanose object.
        if elw_size < 320 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "The size of the elw field must be equal to or greater \
                     than the size of a LogFontPanose object (320 bytes). But \
                     parsed value is `{elw_size:#010X}`",
                ),
            });
        }

        let elw = if elw_size == 320 {
            let (font, font_bytes) = crate::parser::LogFontPanose::parse(buf)?;
            size.consume(font_bytes);

            ELW::LogFontPanose(font)
        } else {
            let mut entries = vec![];

            loop {
                let (v, b) = crate::parser::LogFontExDv::parse(buf)?;

                entries.push(v);
                size.consume(b);

                // log_font_ex (348 bytes) + design_vector (8 to 72 bytes)
                if size.remaining_bytes() < 356 {
                    break;
                }
            }

            ELW::LogFontExDv(entries)
        };

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, ih_fonts, elw })
    }
}

#[derive(Clone, Debug)]
pub enum ELW {
    LogFontPanose(crate::parser::LogFontPanose),
    LogFontExDv(Vec<crate::parser::LogFontExDv>),
}
