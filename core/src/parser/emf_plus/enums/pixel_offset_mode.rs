/// The PixelOffsetMode enumeration defines how pixels are offset, which
/// specifies the trade-off between rendering speed and quality
/// (MS-EMFPLUS 2.1.1.26).
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
pub enum PixelOffsetMode {
    /// Pixels are centered on integer coordinates, specifying speed over
    /// quality.
    PixelOffsetModeDefault = 0x00000000,
    /// Pixels are centered on integer coordinates, as with
    /// PixelOffsetModeNone. Higher speed at the expense of quality is
    /// specified.
    PixelOffsetModeHighSpeed = 0x00000001,
    /// Pixels are centered on half-integer coordinates, as with
    /// PixelOffsetModeHalf. Higher quality at the expense of speed is
    /// specified.
    PixelOffsetModeHighQuality = 0x00000002,
    /// Pixels are centered on the origin, which means that the pixel
    /// covers the area from -0.5 to 0.5 on both the x and y axes and its
    /// center is at (0,0).
    PixelOffsetModeNone = 0x00000003,
    /// Pixels are centered on half-integer coordinates, which means that
    /// the pixel covers the area from 0 to 1 on both the x and y axes
    /// and its center is at (0.5,0.5). By offsetting pixels during
    /// rendering, the render quality can be improved at the cost of
    /// render speed.
    PixelOffsetModeHalf = 0x00000004,
}

crate::parser::enums::impl_parser!(PixelOffsetMode, u32);
