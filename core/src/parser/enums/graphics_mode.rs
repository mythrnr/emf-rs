/// The GraphicsMode enumeration is used to specify how to interpret shape data
/// such as rectangle coordinates.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u32)]
pub enum GraphicsMode {
    /// TrueType text MUST be written from left to right and right side up,
    /// even if the rest of the graphics are rotated about the x-axis or y-axis
    /// because of the current world-to-device transform. Only the height of
    /// the text SHOULD be scaled. GM_COMPATIBLE graphics mode is used for
    /// compatibility between 16-bit and 32-bit systems.
    ///
    /// The world-to-device transform is modified by changing the window and
    /// viewport extents and origins, using the EMR_SETWINDOWEXTEX and
    /// EMR_SETVIEWPORTEXTEX records, and the EMR_SETWINDOWORGEX and
    /// EMR_SETVIEWPORTORGEX records, respectively.
    ///
    /// The world-to-device transform can be changed by
    /// EMR_MODIFYWORLDTRANSFORM and EMR_SETWORLDTRANSFORM records.
    ///
    /// In GM_COMPATIBLE graphics mode, bottom and rightmost edges MUST be
    /// excluded when rectangles are drawn.
    GM_COMPATIBLE = 0x00000001,
    /// TrueType text output SHOULD(Windows NT 3.1, Windows NT 3.5, Windows NT
    /// 3.51, Windows 98, Windows Millennium Edition, Windows NT 4.0, and
    /// Windows 2000: GM_ADVANCED is not supported.) fully conform to the
    /// current world-to-device transform.
    ///
    /// Arcs MUST be drawn in the counterclockwise direction in world space;
    /// however, both arc control points and the arcs themselves MUST reflect
    /// the current world-to-device transform.
    ///
    /// The world-to-device transform can be modified directly by
    /// EMR_MODIFYWORLDTRANSFORM and EMR_SETWORLDTRANSFORM records, or
    /// indirectly by changing the window and viewport extents and origins,
    /// using the EMR_SETWINDOWEXTEX and EMR_SETVIEWPORTEXTEX records, and the
    /// EMR_SETWINDOWORGEX and EMR_SETVIEWPORTORGEX records, respectively.
    ///
    /// In GM_ADVANCED graphics mode, bottom and rightmost edges MUST be
    /// included when rectangles are drawn.
    GM_ADVANCED = 0x00000002,
}

crate::parser::constants::impl_parser!(GraphicsMode, u32);
