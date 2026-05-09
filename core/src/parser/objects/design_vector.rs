use crate::imports::*;

/// The DesignVector object defines the design vector, which specifies values
/// for the font axes of a multiple master font.
#[derive(Clone, Debug)]
pub struct DesignVector {
    /// Signature (4 bytes): An unsigned integer that MUST be set to the value
    /// 0x08007664.
    pub signature: u32,
    /// NumAxes (4 bytes): An unsigned integer that specifies the number of
    /// elements in the Values array. It MUST be in the range 0 to 16,
    /// inclusive.
    pub num_axes: u32,
    /// Values (variable, optional): An array of 32-bit signed integers that
    /// specify the values of the font axes of a multiple master, OpenType
    /// font. The maximum number of values in the array is 16.
    pub values: Vec<i32>,
}

impl DesignVector {
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
        let signature = read_field(buf, &mut consumed_bytes)?;

        crate::parser::ParseError::expect_eq(
            "signature (DesignVector)",
            signature,
            0x08007664_u32,
        )?;

        let num_axes = read_field(buf, &mut consumed_bytes)?;

        crate::parser::ParseError::expect_le(
            "num_axes (DesignVector)",
            num_axes,
            16_u32,
        )?;

        let mut values: Vec<i32> = vec![];

        for _ in 0..num_axes {
            values.push(read_field(buf, &mut consumed_bytes)?);
        }

        Ok((Self { signature, num_axes, values }, consumed_bytes))
    }
}
