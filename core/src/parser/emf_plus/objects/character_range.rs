/// The EmfPlusCharacterRange object specifies a range of character
/// positions for a text string (MS-EMFPLUS 2.2.2.8).
///
/// Graphics strings are specified by EmfPlusStringFormat objects
/// (section 2.2.1.9).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EmfPlusCharacterRange {
    /// First (4 bytes): A signed integer that specifies the first
    /// position of this range.
    pub first: i32,
    /// Length (4 bytes): A signed integer that specifies the number of
    /// positions in this range.
    pub length: i32,
}

impl EmfPlusCharacterRange {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::read_field;

        let mut consumed_bytes: usize = 0;
        let first = read_field(buf, &mut consumed_bytes)?;
        let length = read_field(buf, &mut consumed_bytes)?;

        Ok((Self { first, length }, consumed_bytes))
    }
}
