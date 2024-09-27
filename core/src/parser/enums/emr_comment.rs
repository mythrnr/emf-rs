/// The EmrComment enumeration defines the types of data that a public comment
/// record can contain.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u32)]
pub enum EmrComment {
    /// This comment record contains a specification of an image in WMF.
    EMR_COMMENT_WINDOWS_METAFILE = 0x80000001,
    /// This comment record identifies the beginning of a group of drawing
    /// records.
    EMR_COMMENT_BEGINGROUP = 0x00000002,
    /// This comment record identifies the end of a group of drawing records.
    EMR_COMMENT_ENDGROUP = 0x00000003,
    /// This comment record allows multiple definitions of an image to be
    /// included in the metafile. Using this comment, for example, an
    /// application can include encapsulated PostScript text as well as an EMF
    /// definition of an image.
    EMR_COMMENT_MULTIFORMATS = 0x40000004,
    /// This comment record is reserved and MUST NOT be used.
    EMR_COMMENT_UNICODE_STRING = 0x00000040,
    /// This comment record is reserved and MUST NOT be used.
    EMR_COMMENT_UNICODE_END = 0x00000080,
}

crate::parser::constants::impl_parser!(EmrComment, u32);
