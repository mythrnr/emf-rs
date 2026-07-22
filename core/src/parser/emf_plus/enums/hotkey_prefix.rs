/// The HotkeyPrefix enumeration defines output options for hotkey
/// prefixes in graphics text (MS-EMFPLUS 2.1.1.14).
///
/// Graphics text is specified by EmfPlusStringFormat objects.
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
pub enum HotkeyPrefix {
    /// The hotkey prefix SHOULD NOT be displayed.
    HotkeyPrefixNone = 0x00000000,
    /// No hotkey prefix is defined.
    HotkeyPrefixShow = 0x00000001,
    /// The hotkey prefix SHOULD be displayed.
    HotkeyPrefixHide = 0x00000002,
}

crate::parser::enums::impl_parser!(HotkeyPrefix, u32);
