/// The FormatSignature enumeration defines values that are used to identify the
/// format of embedded data in EMF metafiles.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u32)]
pub enum FormatSignature {
    /// The sequence of ASCII characters "FME ", which denotes EMF data. The
    /// reverse of the string is " EMF".
    ///
    /// Note: The space character in the string is significant and MUST be
    /// present.
    ///
    /// This signature is used in the following structures:
    /// - EMR_HEADER records to identify the EMF metafile
    /// - The EmrFormat object in EMR_COMMENT_MULTIFORMATS records, to specify
    ///   embedded EMF records.
    ENHMETA_SIGNATURE = 0x464D4520,
    /// The value of this member is the sequence of ASCII characters "FSPE",
    /// which denotes encapsulated PostScript (EPS) data. The reverse of the
    /// string is "EPSF".
    ///
    /// This signature is used in EmrFormat objects to specify embedded
    /// PostScript data in the EpsData object in EMR_COMMENT_MULTIFORMATS
    /// records.
    EPS_SIGNATURE = 0x46535045,
}

crate::parser::constants::impl_parser!(FormatSignature, u32);
