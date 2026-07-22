//! Bit flag constants defined in Section 2.1.2 of the MS-EMFPLUS
//! specifications.
//!
//! Unlike the enumerations in 2.1.1, these values combine freely, so
//! each set is modeled as a newtype over the raw integer with named
//! mask constants and a `contains` test instead of `strum::FromRepr`.

/// Generates a bit-flag newtype over `u32`: the raw value survives as-is
/// (unknown bits included) so unparsed writer-specific bits are not lost,
/// and `contains` tests a mask.
macro_rules! impl_bit_flags {
    (
        $(#[$doc:meta])* $t:ident,
        $($(#[$const_doc:meta])* $name:ident = $value:expr;)+
    ) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct $t(u32);

        impl $t {
            $($(#[$const_doc])* pub const $name: u32 = $value;)+

            #[cfg_attr(feature = "tracing", tracing::instrument(
                level = tracing::Level::TRACE,
                skip_all,
                err(level = tracing::Level::ERROR, Display),
            ))]
            pub fn parse<R: crate::Read>(
                buf: &mut R,
            ) -> Result<(Self, usize), crate::parser::ParseError> {
                let (value, consumed_bytes) =
                    <u32 as crate::parser::ReadLeField>::read_le(buf)?;

                Ok((Self(value), consumed_bytes))
            }

            /// Constructs the flags from a raw value (used when the
            /// flags travel inside another field).
            pub fn from_raw(value: u32) -> Self {
                Self(value)
            }

            /// The raw value as read from the stream.
            pub fn raw(self) -> u32 {
                self.0
            }

            /// Whether every bit of `mask` is set.
            pub fn contains(self, mask: u32) -> bool {
                self.0 & mask == mask
            }
        }
    };
}

impl_bit_flags! {
    /// The BrushData flags specify properties of graphics brushes,
    /// including the presence of optional data fields. These flags can
    /// be combined to specify multiple options (MS-EMFPLUS 2.1.2.1).
    ///
    /// Graphics brushes are specified by EmfPlusBrush objects.
    BrushDataFlags,
    /// This flag is meaningful in EmfPlusPathGradientBrushData objects.
    ///
    /// If set, an EmfPlusBoundaryPathData object is specified in the
    /// BoundaryData field of the brush data object.
    ///
    /// If clear, an EmfPlusBoundaryPointData object is specified in the
    /// BoundaryData field of the brush data object.
    PATH = 0x0000_0001;
    /// This flag is meaningful in EmfPlusLinearGradientBrushData
    /// objects, EmfPlusPathGradientBrushData objects, and
    /// EmfPlusTextureBrushData objects.
    ///
    /// If set, a 2x3 world space to device space transform matrix is
    /// specified in the OptionalData field of the brush data object.
    TRANSFORM = 0x0000_0002;
    /// This flag is meaningful in EmfPlusLinearGradientBrushData and
    /// EmfPlusPathGradientBrushData objects.
    ///
    /// If set, an EmfPlusBlendColors object is specified in the
    /// OptionalData field of the brush data object.
    PRESET_COLORS = 0x0000_0004;
    /// This flag is meaningful in EmfPlusLinearGradientBrushData and
    /// EmfPlusPathGradientBrushData objects.
    ///
    /// If set, an EmfPlusBlendFactors object that specifies a blend
    /// pattern along a horizontal gradient is specified in the
    /// OptionalData field of the brush data object.
    BLEND_FACTORS_H = 0x0000_0008;
    /// This flag is meaningful in EmfPlusLinearGradientBrushData
    /// objects.
    ///
    /// If set, an EmfPlusBlendFactors object that specifies a blend
    /// pattern along a vertical gradient is specified in the
    /// OptionalData field of the brush data object.
    BLEND_FACTORS_V = 0x0000_0010;
    /// This flag is meaningful in EmfPlusPathGradientBrushData objects.
    ///
    /// If set, an EmfPlusFocusScaleData object is specified in the
    /// OptionalData field of the brush data object.
    FOCUS_SCALES = 0x0000_0040;
    /// This flag is meaningful in EmfPlusLinearGradientBrushData,
    /// EmfPlusPathGradientBrushData, and EmfPlusTextureBrushData
    /// objects.
    ///
    /// If set, the brush MUST already be gamma corrected; that is,
    /// output brightness and intensity have been corrected to match the
    /// input image.
    IS_GAMMA_CORRECTED = 0x0000_0080;
    /// This flag is meaningful in EmfPlusTextureBrushData objects.
    ///
    /// If set, a world space to device space transform SHOULD NOT be
    /// applied to the texture brush.
    DO_NOT_TRANSFORM = 0x0000_0100;
}

impl_bit_flags! {
    /// The CustomLineCapData flags specify data for custom line caps.
    /// These flags can be combined to specify multiple options
    /// (MS-EMFPLUS 2.1.2.2).
    ///
    /// Custom graphics line caps are specified by EmfPlusCustomLineCap
    /// objects.
    CustomLineCapDataFlags,
    /// If set, an EmfPlusFillPath object is specified in the
    /// OptionalData field of the EmfPlusCustomLineCapData object for
    /// filling the custom line cap.
    FILL_PATH = 0x0000_0001;
    /// If set, an EmfPlusLinePath object is specified in the
    /// OptionalData field of the EmfPlusCustomLineCapData object for
    /// outlining the custom line cap.
    LINE_PATH = 0x0000_0002;
}

impl_bit_flags! {
    /// The DriverStringOptions flags specify properties of graphics
    /// text positioning and rendering. These flags can be combined to
    /// specify multiple options (MS-EMFPLUS 2.1.2.3).
    ///
    /// Graphics text output is specified in EmfPlusDrawDriverString
    /// records.
    DriverStringOptionsFlags,
    /// If set, the positions of character glyphs SHOULD be specified in
    /// a character map lookup table.
    ///
    /// If clear, the glyph positions SHOULD be obtained from an array
    /// of coordinates.
    CMAP_LOOKUP = 0x0000_0001;
    /// If set, the string SHOULD be rendered vertically.
    ///
    /// If clear, the string SHOULD be rendered horizontally.
    VERTICAL = 0x0000_0002;
    /// If set, character glyph positions SHOULD be calculated relative
    /// to the position of the first glyph.
    ///
    /// If clear, the glyph positions SHOULD be obtained from an array
    /// of coordinates.
    REALIZED_ADVANCE = 0x0000_0004;
    /// If set, less memory SHOULD be used to cache anti-aliased glyphs,
    /// which produces lower quality text rendering.
    ///
    /// If clear, more memory SHOULD be used, which produces higher
    /// quality text rendering.
    LIMIT_SUBPIXEL = 0x0000_0008;
}

impl_bit_flags! {
    /// The FontStyle flags specify styles of graphics font typefaces.
    /// These flags can be combined to specify multiple options
    /// (MS-EMFPLUS 2.1.2.4).
    ///
    /// Graphics font typefaces are specified by EmfPlusFont objects.
    FontStyleFlags,
    /// If set, the font typeface is rendered with a heavier weight or
    /// thickness.
    ///
    /// If clear, the font typeface is rendered with a normal thickness.
    BOLD = 0x0000_0001;
    /// If set, the font typeface is rendered with the vertical stems of
    /// the characters at an increased angle or slant relative to the
    /// baseline.
    ///
    /// If clear, the font typeface is rendered with the vertical stems
    /// of the characters at a normal angle.
    ITALIC = 0x0000_0002;
    /// If set, the font typeface is rendered with a line underneath the
    /// baseline of the characters.
    ///
    /// If clear, the font typeface is rendered without a line
    /// underneath the baseline.
    UNDERLINE = 0x0000_0004;
    /// If set, the font typeface is rendered with a line parallel to
    /// the baseline drawn through the middle of the characters.
    ///
    /// If clear, the font typeface is rendered without a line through
    /// the characters.
    STRIKEOUT = 0x0000_0008;
}

impl_bit_flags! {
    /// The PaletteStyle flags specify properties of graphics palettes.
    /// These flags can be combined to specify multiple options
    /// (MS-EMFPLUS 2.1.2.5).
    ///
    /// Graphics palettes are specified by EmfPlusPalette objects.
    PaletteStyleFlags,
    /// If set, one or more of the palette entries MUST contain alpha
    /// transparency information.
    HAS_ALPHA = 0x0000_0001;
    /// If set, the palette MUST contain only grayscale entries.
    GRAY_SCALE = 0x0000_0002;
    /// If set, the palette MUST contain discrete color values that can
    /// be used for halftoning.
    HALFTONE = 0x0000_0004;
}

impl_bit_flags! {
    /// The PathPointType flags specify type properties of points on
    /// graphics paths. These flags can be combined to specify multiple
    /// options (MS-EMFPLUS 2.1.2.6).
    ///
    /// Graphics paths are specified by EmfPlusPath objects.
    ///
    /// They occupy the high 4 bits of an EmfPlusPathPointType object,
    /// and the masks here apply to that 4-bit field.
    PathPointTypeFlags,
    /// A line segment that passes through the point is dashed.
    DASH_MODE = 0x0000_0001;
    /// The point is a position marker.
    PATH_MARKER = 0x0000_0002;
    /// The point is the endpoint of a subpath.
    CLOSE_SUBPATH = 0x0000_0008;
}

impl_bit_flags! {
    /// The PenData flags specify properties of graphics pens, including
    /// the presence of optional data fields. These flags can be
    /// combined to specify multiple options (MS-EMFPLUS 2.1.2.7).
    ///
    /// Graphics pens are specified by EmfPlusPen objects.
    ///
    /// When multiple optional fields are present, they appear in the
    /// EmfPlusPenOptionalData object in the order of these masks.
    PenDataFlags,
    /// If set, a 2x3 transform matrix is specified in the OptionalData
    /// field of an EmfPlusPenData object.
    TRANSFORM = 0x0000_0001;
    /// If set, the style of a starting line cap is specified in the
    /// OptionalData field of an EmfPlusPenData object.
    START_CAP = 0x0000_0002;
    /// Indicates whether the style of an ending line cap is specified
    /// in the OptionalData field of an EmfPlusPenData object.
    END_CAP = 0x0000_0004;
    /// Indicates whether a line join type is specified in the
    /// OptionalData field of an EmfPlusPenData object.
    JOIN = 0x0000_0008;
    /// Indicates whether a miter limit is specified in the OptionalData
    /// field of an EmfPlusPenData object.
    MITER_LIMIT = 0x0000_0010;
    /// Indicates whether a line style is specified in the OptionalData
    /// field of an EmfPlusPenData object.
    LINE_STYLE = 0x0000_0020;
    /// Indicates whether a dashed line cap is specified in the
    /// OptionalData field of an EmfPlusPenData object.
    DASHED_LINE_CAP = 0x0000_0040;
    /// Indicates whether a dashed line offset is specified in the
    /// OptionalData field of an EmfPlusPenData object.
    DASHED_LINE_OFFSET = 0x0000_0080;
    /// Indicates whether an EmfPlusDashedLineData object is specified
    /// in the OptionalData field of an EmfPlusPenData object.
    DASHED_LINE = 0x0000_0100;
    /// Indicates whether a pen alignment is specified in the
    /// OptionalData field of an EmfPlusPenData object.
    NON_CENTER = 0x0000_0200;
    /// Indicates whether the length and content of a
    /// EmfPlusCompoundLineData object are present in the OptionalData
    /// field of an EmfPlusPenData object.
    COMPOUND_LINE = 0x0000_0400;
    /// Indicates whether an EmfPlusCustomStartCapData object is
    /// specified in the OptionalData field of an EmfPlusPenData object.
    CUSTOM_START_CAP = 0x0000_0800;
    /// Indicates whether an EmfPlusCustomEndCapData object is specified
    /// in the OptionalData field of an EmfPlusPenData object.
    CUSTOM_END_CAP = 0x0000_1000;
}

impl_bit_flags! {
    /// The StringFormat flags specify options for graphics text layout,
    /// including direction, clipping and font handling. These flags can
    /// be combined to specify multiple options (MS-EMFPLUS 2.1.2.8).
    ///
    /// Graphics text layout is specified by EmfPlusStringFormat objects.
    StringFormatFlags,
    /// If set, the reading order of the string SHOULD be right to left.
    /// For horizontal text, this means that characters are read from
    /// right to left. For vertical text, this means that columns are
    /// read from right to left.
    ///
    /// If clear, horizontal or vertical text SHOULD be read from left
    /// to right.
    DIRECTION_RIGHT_TO_LEFT = 0x0000_0001;
    /// If set, individual lines of text SHOULD be drawn vertically on
    /// the display device.
    ///
    /// If clear, individual lines of text SHOULD be drawn horizontally,
    /// with each new line below the previous line.
    DIRECTION_VERTICAL = 0x0000_0002;
    /// If set, parts of characters MUST be allowed to overhang the text
    /// layout rectangle.
    ///
    /// If clear, characters that overhang the boundaries of the text
    /// layout rectangle MUST be repositioned to avoid overhang.
    ///
    /// An italic, "f" is an example of a character that can have
    /// overhanging parts.
    NO_FIT_BLACK_BOX = 0x0000_0004;
    /// If set, control characters SHOULD appear in the output as
    /// representative Unicode glyphs.
    DISPLAY_FORMAT_CONTROL = 0x0000_0020;
    /// If set, an alternate font SHOULD be used for characters that are
    /// not supported in the requested font.
    ///
    /// If clear, a character missing from the requested font SHOULD
    /// appear as a "font missing" character, which MAY be an open
    /// square.
    NO_FONT_FALLBACK = 0x0000_0400;
    /// If set, the space at the end of each line MUST be included in
    /// measurements of string length.
    ///
    /// If clear, the space at the end of each line MUST be excluded
    /// from measurements of string length.
    MEASURE_TRAILING_SPACES = 0x0000_0800;
    /// If set, a string that extends past the end of the text layout
    /// rectangle MUST NOT be wrapped to the next line.
    ///
    /// If clear, a string that extends past the end of the text layout
    /// rectangle MUST be broken at the last word boundary within the
    /// bounding rectangle, and the remainder of the string MUST be
    /// wrapped to the next line.
    NO_WRAP = 0x0000_1000;
    /// If set, whole lines of text SHOULD be output and SHOULD NOT be
    /// clipped by the string's layout rectangle.
    ///
    /// If clear, text layout SHOULD continue until all lines are
    /// output, or until additional lines would not be visible as a
    /// result of clipping.
    ///
    /// This flag can be used either to deny or allow a line of text to
    /// be partially obscured by a layout rectangle that is not a
    /// multiple of line height. For all text to be visible, a layout
    /// rectangle at least as tall as the height of one line.
    LINE_LIMIT = 0x0000_2000;
    /// If set, text extending outside the string layout rectangle
    /// SHOULD be allowed to show.
    ///
    /// If clear, all text that extends outside the layout rectangle
    /// SHOULD be clipped.
    NO_CLIP = 0x0000_4000;
    /// This flag MAY be used to specify an implementation-specific
    /// process for rendering text.
    BYPASS_GDI = 0x8000_0000;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_requires_every_bit_of_the_mask() {
        let flags = PenDataFlags::from_raw(
            PenDataFlags::TRANSFORM | PenDataFlags::START_CAP,
        );

        assert!(flags.contains(PenDataFlags::TRANSFORM));
        assert!(
            flags.contains(PenDataFlags::TRANSFORM | PenDataFlags::START_CAP)
        );
        assert!(!flags.contains(PenDataFlags::END_CAP));
        assert!(
            !flags.contains(PenDataFlags::TRANSFORM | PenDataFlags::END_CAP)
        );
    }

    #[test]
    fn parse_keeps_unknown_bits() {
        let bytes = 0xDEAD_BEEF_u32.to_le_bytes();
        let mut buf: &[u8] = &bytes;
        let (flags, consumed) = BrushDataFlags::parse(&mut buf).unwrap();

        assert_eq!(consumed, 4);
        assert_eq!(flags.raw(), 0xDEAD_BEEF);
    }
}
