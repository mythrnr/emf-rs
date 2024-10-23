use crate::imports::*;

/// The LogFont object specifies the basic attributes of a logical font.
#[derive(Clone, Debug)]
pub struct LogFont {
    /// Height (4 bytes): A signed integer that specifies the height of the
    /// font's character cell in logical units. The character height value,
    /// also known as the em size, is the character cell height value minus the
    /// internal leading value. The font mapper SHOULD interpret the value
    /// specified in the Height field in the following manner.
    ///
    /// | Value | Meaning |
    /// |:-|:-|
    /// | `0x00000000 < value` | The font mapper transforms this value into device units and matches it against the cell height of the available fonts. |
    /// | `0x00000000` | The font mapper uses a default height value when it searches for a match. |
    /// | `value < 0x00000000` | The font mapper transforms this value into device units and matches its absolute value against the character height of the available fonts. |
    ///
    /// For all height comparisons, the font mapper SHOULD look for the largest
    /// font that does not exceed the requested size.
    pub height: i32,
    /// Width (4 bytes): A signed integer that specifies the average width of
    /// characters in the font in logical units. If the Width field value is
    /// zero, an appropriate value SHOULD (In Windows implementations, the
    /// aspect ratio of the device is matched against the digitization aspect
    /// ratios of the available fonts to find the closest match, determined by
    /// the absolute value of the difference.) be calculated from other values
    /// in this object to find a font that has the typographer's intended
    /// aspect ratio.
    pub width: i32,
    /// Escapement (4 bytes): A signed integer that specifies the angle, in
    /// tenths of degrees, between the escapement vector and the x-axis of the
    /// device. The escapement vector is parallel to the baseline of a row of
    /// text.
    ///
    /// When the graphics mode is set to GM_ADVANCED, the escapement angle of
    /// the string can be specified independently of the orientation angle of
    /// the string's characters.
    pub escapement: i32,
    /// Orientation (4 bytes): A signed integer that specifies the angle, in
    /// tenths of degrees, between each character's baseline and the x-axis of
    /// the device.
    pub orientation: i32,
    /// Weight (4 bytes): A signed integer that specifies the weight of the
    /// font in the range zero through 1000. For example, 400 is normal and 700
    /// is bold. If this value is zero, a default weight can be used.
    ///
    /// Windows uses a weight value of 400 by default.
    ///
    /// | Value | Weight |
    /// |:-|:-|
    /// | Thin | 100 |
    /// | Extra Light (Ultra Light) | 200 |
    /// | Light | 300 |
    /// | Normal (Regular) | 400 |
    /// | Medium | 500 |
    /// | Semi-Bold (Demi-Bold) | 600 |
    /// | Bold | 700 |
    /// | Extra Bold (Ultra Bold) | 800 |
    /// | Heavy (Black) | 900 |
    pub weight: i32,
    /// Italic (1 byte): An unsigned integer that specifies an italic font if
    /// set to 0x01; otherwise, it MUST be set to 0x00.
    pub italic: bool,
    /// Underline (1 byte): An unsigned integer that specifies an underlined
    /// font if set to 0x01; otherwise, it MUST be set to 0x00.
    pub underline: bool,
    /// StrikeOut (1 byte): An unsigned integer that specifies a strikeout font
    /// if set to 0x01; otherwise, it MUST be set to 0x00.
    pub strike_out: bool,
    /// CharSet (1 byte): An unsigned integer that specifies the set of
    /// character glyphs. It MUST be a value in the CharacterSet enumeration
    /// ([MS-WMF] section 2.1.1.5). If the character set is unknown, metafile
    /// processing SHOULD NOT attempt to translate or interpret strings that
    /// are rendered with that font.
    ///
    /// If a typeface name is specified in the Facename field, the CharSet
    /// field value MUST match the character set of that typeface.
    pub charset: wmf_core::parser::CharacterSet,
    /// OutPrecision (1 byte): An unsigned integer that specifies the output
    /// precision. The output precision defines how closely the font is
    /// required to match the requested height, width, character orientation,
    /// escapement, pitch, and font type. It MUST be a value from the
    /// OutPrecision enumeration ([MS-WMF] section 2.1.1.21).
    ///
    /// Applications can use the output precision to control how the font
    /// mapper chooses a font when the operating system contains more than one
    /// font with a specified name. For example, if an operating system
    /// contains a font named Symbol in rasterized and TrueType forms, an
    /// output precision value of OUT_TT_PRECIS forces the font mapper to
    /// choose the TrueType version. A value of OUT_TT_ONLY_PRECIS forces the
    /// font mapper to choose a TrueType font, even if it is necessary to
    /// substitute a TrueType font with another name.
    pub out_precision: wmf_core::parser::OutPrecision,
    /// ClipPrecision (1 byte): An unsigned integer that specifies the clipping
    /// precision. The clipping precision defines how to clip characters that
    /// are partially outside the clipping region. It can be one or more of the
    /// ClipPrecision Flags ([MS-WMF] section 2.1.2.1).
    pub clip_precision: BTreeSet<wmf_core::parser::ClipPrecision>,
    /// Quality (1 byte): An unsigned integer that specifies the output
    /// quality. The output quality defines how closely to attempt to match the
    /// logical-font attributes to those of an actual physical font. It MUST be
    /// one of the values in the FontQuality enumeration ([MS-WMF] section
    /// 2.1.1.10).
    pub quality: wmf_core::parser::FontQuality,
    /// PitchAndFamily (1 byte): A PitchAndFamily object ([MS-WMF] section
    /// 2.2.2.14) that specifies the pitch and family of the font. Font
    /// families describe the look of a font in a general way. They are
    /// intended for specifying a font when the specified typeface is not
    /// available.
    pub pitch_and_family: wmf_core::parser::PitchAndFamily,
    /// Facename (64 bytes): A string of no more than 32 Unicode characters
    /// that specifies the typeface name of the font. If the length of this
    /// string is less than 32 characters, a terminating NULL MUST be present,
    /// after which the remainder of this field MUST be ignored.
    pub facename: String,
}

impl LogFont {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use strum::IntoEnumIterator;

        let (
            (height, height_bytes),
            (width, width_bytes),
            (escapement, escapement_bytes),
            (orientation, orientation_bytes),
            (weight, weight_bytes),
            (italic, italic_bytes),
            (underline, underline_bytes),
            (strike_out, strike_out_bytes),
            (charset, charset_bytes),
            (out_precision, out_precision_bytes),
        ) = (
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)
                .map(|(v, b)| (v == 0x01, b))?,
            crate::parser::read_u8_from_le_bytes(buf)
                .map(|(v, b)| (v == 0x01, b))?,
            crate::parser::read_u8_from_le_bytes(buf)
                .map(|(v, b)| (v == 0x01, b))?,
            wmf_core::parser::CharacterSet::parse(buf)?,
            wmf_core::parser::OutPrecision::parse(buf)?,
        );

        let (clip_precision, clip_precision_bytes) = {
            let (v, clip_precision_bytes) =
                crate::parser::read_u8_from_le_bytes(buf)?;

            (
                wmf_core::parser::ClipPrecision::iter()
                    .filter(|c| v & (*c as u8) == (*c as u8))
                    .collect(),
                clip_precision_bytes,
            )
        };

        let (
            (quality, quality_bytes),
            (pitch_and_family, pitch_and_family_bytes),
        ) = (
            wmf_core::parser::FontQuality::parse(buf)?,
            wmf_core::parser::PitchAndFamily::parse(buf)?,
        );

        let (facename, facename_bytes) = {
            let (v, facename_bytes) = crate::parser::read_variable(buf, 64)?;

            (crate::parser::null_terminated_utf16le_string(&v)?, facename_bytes)
        };

        Ok((
            Self {
                height,
                width,
                escapement,
                orientation,
                weight,
                italic,
                underline,
                strike_out,
                charset,
                out_precision,
                clip_precision,
                quality,
                pitch_and_family,
                facename,
            },
            height_bytes
                + width_bytes
                + escapement_bytes
                + orientation_bytes
                + weight_bytes
                + italic_bytes
                + underline_bytes
                + strike_out_bytes
                + charset_bytes
                + out_precision_bytes
                + clip_precision_bytes
                + quality_bytes
                + pitch_and_family_bytes
                + facename_bytes,
        ))
    }
}
