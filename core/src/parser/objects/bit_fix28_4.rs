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
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let ((int_value, int_value_bytes), (frac_value, frac_value_bytes)) =
            (crate::parser::read(buf)?, crate::parser::read(buf)?);

        Ok((Self { int_value, frac_value }, int_value_bytes + frac_value_bytes))
    }
}
