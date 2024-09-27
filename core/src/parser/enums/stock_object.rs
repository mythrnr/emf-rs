/// The StockObject enumeration specifies the indexes of predefined logical
/// graphics objects that can be used in graphics operations.
///
/// The specific structures of stock objects are implementation-dependent;
/// however, the properties of stock objects SHOULD be equivalent to the
/// properties of explicitly created objects of the same type. These properties
/// are specified where possible for the stock objects defined in this
/// enumeration.
///
/// During metafile processing, stock object indexes can be used by object
/// manipulation records in the same way as indexes of graphics objects that are
/// explicitly created by object creation records. The index of a stock object
/// can be distinguished from the index of an explicit object by the value of
/// the most-significant bit. If that bit is set, the object is a stock object;
/// if the bit is clear, the object was created by a previous metafile record.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u32)]
pub enum StockObject {
    /// A white, solid-color brush that is equivalent to a logical brush
    /// (LogBrushEx object) with the following properties:
    ///
    /// - BrushStyle: BS_SOLID from the BrushStyle enumeration ([MS-WMF]
    ///   section 2.1.1.4)
    /// - Color: 0x00FFFFFF in a ColorRef object ([MS-WMF] section 2.2.2.8)
    WHITE_BRUSH = 0x80000000,
    /// A light gray, solid-color brush that is equivalent to a logical brush
    /// with the following properties:
    /// - BrushStyle: BS_SOLID
    /// - Color: 0x00C0C0C0
    LTGRAY_BRUSH = 0x80000001,
    /// A gray, solid-color brush that is equivalent to a logical brush with
    /// the following properties:
    ///
    /// - BrushStyle: BS_SOLID
    /// - Color: 0x00808080
    GRAY_BRUSH = 0x80000002,
    /// A dark gray, solid color brush that is equivalent to a logical brush
    /// with the following properties:
    ///
    /// - BrushStyle: BS_SOLID
    /// - Color: 0x00404040
    DKGRAY_BRUSH = 0x80000003,
    /// A black, solid color brush that is equivalent to a logical brush with
    /// the following properties:
    ///
    /// - BrushStyle: BS_SOLID
    /// - Color: 0x00000000
    BLACK_BRUSH = 0x80000004,
    /// A null brush that is equivalent to a logical brush with the following
    /// properties:
    ///
    /// - BrushStyle: BS_NULL
    NULL_BRUSH = 0x80000005,
    /// A white, solid-color pen that is equivalent to a logical pen (LogPen
    /// object) with the following properties:
    ///
    /// - PenStyle: PS_COSMETIC + PS_SOLID from the PenStyle enumeration.
    /// - ColorRef: 0x00FFFFFF in a ColorRef object
    WHITE_PEN = 0x80000006,
    /// A black, solid-color pen that is equivalent to a logical pen with the
    /// following properties:
    ///
    /// - PenStyle: PS_COSMETIC + PS_SOLID
    /// - ColorRef: 0x00000000
    BLACK_PEN = 0x80000007,
    /// A null pen that is equivalent to a logical pen with the following
    /// properties:
    ///
    /// - PenStyle: PS_NULL
    NULL_PEN = 0x80000008,
    /// A fixed-width, OEM character set font that is equivalent to a LogFont
    /// object with the following properties:
    ///
    /// - Charset: OEM_CHARSET from the CharacterSet enumeration ([MS-WMF]
    ///   section 2.1.1.5)
    /// - PitchAndFamily: FF_DONTCARE (FamilyFont enumeration, [MS-WMF] section
    ///   2.1.1.8) + FIXED_PITCH (PitchFont enumeration, [MS-WMF] section
    ///   2.1.1.24)
    OEM_FIXED_FONT = 0x8000000A,
    /// A fixed-width font that is equivalent to a LogFont object with the
    /// following properties: On Windows, this is the "Courier" font.
    ///
    /// - Charset: ANSI_CHARSET
    /// - PitchAndFamily: FF_DONTCARE + FIXED_PITCH
    ANSI_FIXED_FONT = 0x8000000B,
    /// A variable-width font that is equivalent to a logical font with the
    /// following properties: On Windows, this is the "MS Sans Serif" font.
    ///
    /// - Charset: ANSI_CHARSET
    /// - PitchAndFamily: FF_DONTCARE + VARIABLE_PITCH
    ANSI_VAR_FONT = 0x8000000C,
    /// A font that is guaranteed to be available in the operating system. The
    /// actual font that is specified by this value is
    /// implementation-dependent.
    ///
    /// On Windows, this is the "Tahoma" font and is used to draw menu text and
    /// dialog box controls. Windows NT 3.1, Windows NT 3.5, Windows NT 3.51,
    /// Windows NT 4.0, Windows 98, and Windows Millennium Edition: The system
    /// font is "MS Sans Serif".
    SYSTEM_FONT = 0x8000000D,
    /// The default font that is provided by the graphics device driver for the
    /// current output device. The actual font that is specified by this value
    /// is implementation-dependent.
    ///
    /// On Windows, this value is considered equivalent to SYSTEM_FONT for the
    /// purposes of the screen display of metafiles. Windows 98 and Windows
    /// Millennium Edition: This value is not supported.
    DEVICE_DEFAULT_FONT = 0x8000000E,
    /// The default palette that is defined for the current output device. The
    /// actual palette that is specified by this value is
    /// implementation-dependent.
    ///
    /// On Windows, this palette consists of the static colors in the system
    /// palette.
    DEFAULT_PALETTE = 0x8000000F,
    /// A fixed-width font that is guaranteed to be available in the operating
    /// system. The actual font that is specified by this value is
    /// implementation-dependent.
    SYSTEM_FIXED_FONT = 0x80000010,
    /// The default font that is used for user interface objects such as menus
    /// and dialog boxes. The actual font that is specified by this value is
    /// implementation-dependent.
    ///
    /// On Windows, the default user interface font is "Tahoma". Windows NT
    /// 3.1, Windows NT 3.5, Windows NT 3.51, Windows NT 4.0, Windows 98, and
    /// Windows Millennium Edition: The default user interface font is "MS Sans
    /// Serif".
    DEFAULT_GUI_FONT = 0x80000011,
    /// The solid-color brush that is currently selected in the playback device
    /// context. The default SHOULD(Windows NT 3.1, Windows NT 3.5, Windows NT
    /// 3.51, Windows NT 4.0, Windows 98, and Windows Millennium Edition: The
    /// default brush is undefined.) be WHITE_BRUSH.
    DC_BRUSH = 0x80000012,
    /// The solid-color pen that is currently selected in the playback device
    /// context. The default SHOULD(Windows NT 3.1, Windows NT 3.5, Windows NT
    /// 3.51, Windows NT 4.0, Windows 98, and Windows Millennium Edition: The
    /// default pen is undefined.) be BLACK_PEN.
    DC_PEN = 0x80000013,
}

crate::parser::constants::impl_parser!(StockObject, u32);
