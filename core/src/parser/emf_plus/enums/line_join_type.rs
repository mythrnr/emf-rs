/// The LineJoinType enumeration defines ways to join two lines that are
/// drawn by the same graphics pen and whose ends meet
/// (MS-EMFPLUS 2.1.1.19).
///
/// Graphics lines are specified by EmfPlusPen objects. A line join makes
/// the intersection of the two line ends look more continuous.
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
pub enum LineJoinType {
    /// A mitered line join.
    LineJoinTypeMiter = 0x00000000,
    /// A beveled line join.
    LineJoinTypeBevel = 0x00000001,
    /// A rounded line join.
    LineJoinTypeRound = 0x00000002,
    /// A clipped mitered line join.
    LineJoinTypeMiterClipped = 0x00000003,
}

crate::parser::enums::impl_parser!(LineJoinType, u32);
