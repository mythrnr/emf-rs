/// The BackgroundMode enumeration is used to specify the background mode to be
/// used with text, hatched brushes, and pen styles that are not solid. The
/// background mode determines how to combine the background with foreground
/// text, hatched brushes, and pen styles that are not solid lines.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u16)]
pub enum BackgroundMode {
    /// Background remains untouched.
    TRANSPARENT = 0x0001,
    /// Background is filled with the current background color before the text,
    /// hatched brush, or pen is drawn.
    OPAQUE = 0x0002,
}

crate::parser::constants::impl_parser!(BackgroundMode, u16);
