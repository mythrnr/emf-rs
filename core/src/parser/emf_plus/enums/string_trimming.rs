/// The StringTrimming enumeration defines how to trim characters from a
/// string that is too large for the text layout rectangle
/// (MS-EMFPLUS 2.1.1.31).
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
pub enum StringTrimming {
    /// No trimming is done.
    StringTrimmingNone = 0x00000000,
    /// The string is broken at the boundary of the last character that
    /// is inside the layout rectangle. This is the default.
    StringTrimmingCharacter = 0x00000001,
    /// The string is broken at the boundary of the last word that is
    /// inside the layout rectangle.
    StringTrimmingWord = 0x00000002,
    /// The string is broken at the boundary of the last character that
    /// is inside the layout rectangle, and an ellipsis (...) is inserted
    /// after the character.
    StringTrimmingEllipsisCharacter = 0x00000003,
    /// The string is broken at the boundary of the last word that is
    /// inside the layout rectangle, and an ellipsis (...) is inserted
    /// after the word.
    StringTrimmingEllipsisWord = 0x00000004,
    /// The center is removed from the string and replaced by an
    /// ellipsis. The algorithm keeps as much of the last portion of the
    /// string as possible.
    StringTrimmingEllipsisPath = 0x00000005,
}

crate::parser::enums::impl_parser!(StringTrimming, u32);
