use crate::{
    imports::*,
    parser::emf_plus::objects::{
        EmfPlusCharacterRange, EmfPlusGraphicsVersion,
    },
};

/// The EmfPlusStringFormat object specifies text layout, display
/// manipulations, and language identification (MS-EMFPLUS 2.2.1.9),
/// with its trailing EmfPlusStringFormatData (2.2.2.44) inlined.
///
/// The EmfPlusStringFormatData object specifies tab stops and
/// character positions for a graphics string. Graphics strings are
/// specified by EmfPlusStringFormat objects.
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusStringFormat {
    /// Version (4 bytes): An EmfPlusGraphicsVersion object (section
    /// 2.2.2.19) that specifies the version of operating system
    /// graphics that was used to create this object.
    pub version: EmfPlusGraphicsVersion,
    /// StringFormatFlags (4 bytes): An unsigned integer that specifies
    /// text layout options for formatting, clipping and font handling.
    /// This value is composed of StringFormat flags (section 2.1.2.8).
    pub string_format_flags: crate::parser::emf_plus::StringFormatFlags,
    /// Language (4 bytes): An EmfPlusLanguageIdentifier object
    /// (section 2.2.2.23) that specifies the language to use for the
    /// string.
    pub language: crate::parser::emf_plus::LanguageIdentifier,
    /// StringAlignment (4 bytes): An unsigned integer that specifies
    /// how to align the string horizontally in the layout rectangle.
    /// This value is defined in the StringAlignment enumeration
    /// (section 2.1.1.28).
    pub string_alignment: crate::parser::emf_plus::StringAlignment,
    /// LineAlign (4 bytes): An unsigned integer that specifies how to
    /// align the string vertically in the layout rectangle. This value
    /// is defined in the StringAlignment enumeration.
    pub line_align: crate::parser::emf_plus::StringAlignment,
    /// DigitSubstitution (4 bytes): An unsigned integer that specifies
    /// how to substitute numeric digits in the string according to a
    /// locale or language. This value is defined in the
    /// StringDigitSubstitution enumeration (section 2.1.1.29).
    pub digit_substitution: crate::parser::emf_plus::StringDigitSubstitution,
    /// DigitLanguage (4 bytes): An EmfPlusLanguageIdentifier object
    /// that specifies the language to use for numeric digits in the
    /// string. For example, if this string contains Arabic digits,
    /// this field MUST contain a language identifier that specifies an
    /// Arabic language.
    pub digit_language: crate::parser::emf_plus::LanguageIdentifier,
    /// FirstTabOffset (4 bytes): A floating-point value that specifies
    /// the number of spaces between the beginning of a text line and
    /// the first tab stop.
    pub first_tab_offset: f32,
    /// HotkeyPrefix (4 bytes): A signed integer that specifies the
    /// type of processing that is performed on a string when a
    /// keyboard shortcut prefix (that is, an ampersand) is
    /// encountered. Basically, this field specifies whether to display
    /// keyboard shortcut prefixes that relate to text. The value is
    /// defined in the HotkeyPrefix enumeration (section 2.1.1.14).
    pub hotkey_prefix: crate::parser::emf_plus::HotkeyPrefix,
    /// LeadingMargin (4 bytes): A floating-point value that specifies
    /// the length of the space to add to the starting position of a
    /// string. The default is 1/6 inch; for typographic fonts, the
    /// default value is 0.
    pub leading_margin: f32,
    /// TrailingMargin (4 bytes): A floating-point value that specifies
    /// the length of the space to leave following a string. The
    /// default is 1/6 inch; for typographic fonts, the default value
    /// is 0.
    pub trailing_margin: f32,
    /// Tracking (4 bytes): A floating-point value that specifies the
    /// ratio of the horizontal space allotted to each character in a
    /// specified string to the font-defined width of the character.
    /// Large values for this property specify ample space between
    /// characters; values less than 1 can produce character overlap.
    /// The default is 1.03; for typographic fonts, the default value
    /// is 1.00.
    pub tracking: f32,
    /// Trimming (4 bytes): How to trim characters from a string that
    /// is too large to fit into a layout rectangle. This value is
    /// defined in the StringTrimming enumeration (section 2.1.1.30).
    pub trimming: crate::parser::emf_plus::StringTrimming,
    /// TabStops (variable): An optional array of 32-bit floating-point
    /// values that specify the optional tab stop locations for this
    /// object. Each tab stop value represents the number of spaces
    /// between tab stops or, for the first tab stop, the number of
    /// spaces between the beginning of a line of text and the first
    /// tab stop.
    ///
    /// This field MUST be present if the value of the TabStopCount
    /// field in the EmfPlusStringFormat object (section 2.2.1.9) is
    /// greater than 0.
    pub tab_stops: Vec<f32>,
    /// CharRange (variable): An optional array of RangeCount
    /// EmfPlusCharacterRange objects (section 2.2.2.8) that specify
    /// the range of character positions within a string of text. The
    /// bounding region is defined by the area of the display that is
    /// occupied by a group of characters specified by the character
    /// range.
    ///
    /// This field MUST be present if the value of the RangeCount field
    /// in the EmfPlusStringFormat object is greater than 0.
    pub char_range: Vec<EmfPlusCharacterRange>,
}

impl EmfPlusStringFormat {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let version =
            read_with(buf, &mut consumed_bytes, EmfPlusGraphicsVersion::parse)?;
        let string_format_flags = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::StringFormatFlags::parse,
        )?;
        let language = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::LanguageIdentifier::parse,
        )?;
        let string_alignment = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::StringAlignment::parse,
        )?;
        let line_align = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::StringAlignment::parse,
        )?;
        let digit_substitution = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::StringDigitSubstitution::parse,
        )?;
        let digit_language = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::LanguageIdentifier::parse,
        )?;
        let first_tab_offset = read_field(buf, &mut consumed_bytes)?;
        let hotkey_prefix = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::HotkeyPrefix::parse,
        )?;
        let leading_margin = read_field(buf, &mut consumed_bytes)?;
        let trailing_margin = read_field(buf, &mut consumed_bytes)?;
        let tracking = read_field(buf, &mut consumed_bytes)?;
        let trimming = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::StringTrimming::parse,
        )?;
        let tab_stop_count: i32 = read_field(buf, &mut consumed_bytes)?;
        let range_count: i32 = read_field(buf, &mut consumed_bytes)?;

        let tab_stop_count = to_count("TabStopCount", tab_stop_count)?;
        let range_count = to_count("RangeCount", range_count)?;

        crate::parser::emf_plus::check_element_count(
            "TabStopCount",
            tab_stop_count,
        )?;
        crate::parser::emf_plus::check_element_count(
            "RangeCount",
            range_count,
        )?;

        let mut tab_stops = vec![];
        for _ in 0..tab_stop_count {
            tab_stops.push(read_field(buf, &mut consumed_bytes)?);
        }

        let mut char_range = vec![];
        for _ in 0..range_count {
            char_range.push(read_with(
                buf,
                &mut consumed_bytes,
                EmfPlusCharacterRange::parse,
            )?);
        }

        Ok((
            Self {
                version,
                string_format_flags,
                language,
                string_alignment,
                line_align,
                digit_substitution,
                digit_language,
                first_tab_offset,
                hotkey_prefix,
                leading_margin,
                trailing_margin,
                tracking,
                trimming,
                tab_stops,
                char_range,
            },
            consumed_bytes,
        ))
    }
}

/// The count fields of EmfPlusStringFormat are signed on the wire; a
/// negative count has no valid interpretation.
fn to_count(
    field: &'static str,
    value: i32,
) -> Result<u32, crate::parser::ParseError> {
    u32::try_from(value).map_err(|_| {
        crate::parser::ParseError::UnexpectedPattern {
            cause: alloc::format!("{field} is negative: {value}").into(),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::{StringAlignment, StringTrimming};

    #[test]
    fn parses_tab_stops_and_char_ranges() {
        let mut data = vec![];
        data.extend(0xDBC0_1002_u32.to_le_bytes()); // Version
        data.extend(0x0000_0001_u32.to_le_bytes()); // RTL flag
        data.extend(0x0411_u32.to_le_bytes()); // Language: ja-JP
        data.extend(1_u32.to_le_bytes()); // StringAlignmentCenter
        data.extend(2_u32.to_le_bytes()); // LineAlign: Far
        data.extend(1_u32.to_le_bytes()); // DigitSubstitutionNone
        data.extend(0_u32.to_le_bytes()); // DigitLanguage
        data.extend(4.0_f32.to_le_bytes()); // FirstTabOffset
        data.extend(0_u32.to_le_bytes()); // HotkeyPrefixNone
        data.extend(1.0_f32.to_le_bytes()); // LeadingMargin
        data.extend(2.0_f32.to_le_bytes()); // TrailingMargin
        data.extend(1.0_f32.to_le_bytes()); // Tracking
        data.extend(3_u32.to_le_bytes()); // EllipsisCharacter
        data.extend(2_i32.to_le_bytes()); // TabStopCount
        data.extend(1_i32.to_le_bytes()); // RangeCount
        data.extend(8.0_f32.to_le_bytes());
        data.extend(16.0_f32.to_le_bytes());
        data.extend(1_i32.to_le_bytes()); // CharRange.first
        data.extend(5_i32.to_le_bytes()); // CharRange.length

        let mut buf: &[u8] = &data;
        let (format, consumed) = EmfPlusStringFormat::parse(&mut buf).unwrap();

        assert_eq!(consumed, data.len());
        assert_eq!(
            format.string_alignment,
            StringAlignment::StringAlignmentCenter,
        );
        assert_eq!(format.line_align, StringAlignment::StringAlignmentFar);
        assert_eq!(
            format.trimming,
            StringTrimming::StringTrimmingEllipsisCharacter,
        );
        assert_eq!(format.language.primary_language_id(), 0x11);
        assert_eq!(format.tab_stops, vec![8.0, 16.0]);
        assert_eq!(format.char_range, vec![EmfPlusCharacterRange {
            first: 1,
            length: 5
        }],);
    }

    #[test]
    fn rejects_negative_counts() {
        let mut data = vec![];
        data.extend(0xDBC0_1002_u32.to_le_bytes());
        data.extend(0_u32.to_le_bytes());
        data.extend(0_u32.to_le_bytes());
        data.extend(0_u32.to_le_bytes());
        data.extend(0_u32.to_le_bytes());
        data.extend(0_u32.to_le_bytes());
        data.extend(0_u32.to_le_bytes());
        data.extend(0.0_f32.to_le_bytes());
        data.extend(0_u32.to_le_bytes());
        data.extend(0.0_f32.to_le_bytes());
        data.extend(0.0_f32.to_le_bytes());
        data.extend(0.0_f32.to_le_bytes());
        data.extend(0_u32.to_le_bytes());
        data.extend((-1_i32).to_le_bytes()); // TabStopCount
        data.extend(0_i32.to_le_bytes());

        let mut buf: &[u8] = &data;

        assert!(EmfPlusStringFormat::parse(&mut buf).is_err());
    }
}
