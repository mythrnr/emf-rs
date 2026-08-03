/// The StringDigitSubstitution enumeration defines ways to substitute
/// digits in a string according to a user's locale or language
/// (MS-EMFPLUS 2.1.1.30).
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
pub enum StringDigitSubstitution {
    /// Use an implementation-defined substitution scheme.
    StringDigitSubstitutionUser = 0x00000000,
    /// Disable substitutions.
    StringDigitSubstitutionNone = 0x00000001,
    /// Substitute digits that correspond with the official national
    /// language of the user's locale.
    StringDigitSubstitutionNational = 0x00000002,
    /// Substitute digits that correspond to the user's native script or
    /// language, which can be different from the official national
    /// language of the user's locale.
    StringDigitSubstitutionTraditional = 0x00000003,
}

crate::parser::enums::impl_parser!(StringDigitSubstitution, u32);
