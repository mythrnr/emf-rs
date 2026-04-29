/// The BitFIX28_4 object defines a numeric value in 28.4 bit FIX notation.
///
/// The real number represented by this object is computed as follows:
///
/// ```
/// IntValue + (FracValue / 16)
/// ```
#[derive(Clone, Debug)]
pub struct BitFIX28_4 {
    /// IntValue (28 bits): The signed, integral part of the number.
    pub int_value: [u8; 28],
    /// FracValue (4 bits): The unsigned fractional part of the number, in
    /// units of one-sixteenth.
    pub frac_value: [u8; 4],
}

impl BitFIX28_4 {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::read_array_field;

        let mut consumed_bytes: usize = 0;
        let int_value = read_array_field(buf, &mut consumed_bytes)?;
        let frac_value = read_array_field(buf, &mut consumed_bytes)?;

        Ok((Self { int_value, frac_value }, consumed_bytes))
    }
}
