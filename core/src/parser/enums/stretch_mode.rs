/// The StretchMode enumeration is used to specify how color data is added to or
/// removed from bitmaps that are stretched or compressed.
///
/// Windows also uses the following symbolic names for the StretchMode
/// enumeration; their meanings are exactly the same as the members with the
/// same values.
///
/// ```
/// #define BLACKONWHITE 1
/// #define WHITEONBLACK 2
/// #define COLORONCOLOR 3
/// #define HALFTONE     4
/// ```
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    strum::FromRepr,
    strum::EnumIter,
)]
#[repr(u32)]
pub enum StretchMode {
    /// Performs a Boolean AND operation using the color values for the
    /// eliminated and existing pixels. If the bitmap is a monochrome bitmap,
    /// this mode preserves black pixels at the expense of white pixels.
    STRETCH_ANDSCANS = 0x01,
    /// Performs a Boolean OR operation using the color values for the
    /// eliminated and existing pixels. If the bitmap is a monochrome bitmap,
    /// this mode preserves white pixels at the expense of black pixels.
    STRETCH_ORSCANS = 0x02,
    /// Deletes the pixels. This mode deletes all eliminated lines of pixels
    /// without trying to preserve their information.
    STRETCH_DELETESCANS = 0x03,
    /// Maps pixels from the source rectangle into blocks of pixels in the
    /// destination rectangle. The average color over the destination block of
    /// pixels approximates the color of the source pixels.
    ///
    /// After setting the STRETCH_HALFTONE stretching mode, the brush origin
    /// SHOULD be defined by an EMR_SETBRUSHORGEX record. If it fails to do so,
    /// brush misalignment can occur.
    STRETCH_HALFTONE = 0x04,
}

crate::parser::enums::impl_parser!(StretchMode, u32);
