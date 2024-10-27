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
        let (signature, signature_bytes) =
            crate::parser::read_u32_from_le_bytes(buf)?;

        if signature != 0x08007664 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "signature field in DesignVector must be `0x08007664`, \
                     but parsed value is {signature:#010X}"
                ),
            });
        }

        let (num_axes, num_axes_bytes) =
            crate::parser::read_u32_from_le_bytes(buf)?;

        if num_axes > 16 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "num_axes field in DesignVector must be less than `16`, \
                     but parsed value is {num_axes:#010X}"
                ),
            });
        }

        let mut values = vec![];
        let mut values_bytes = 0;

        for _ in 0..num_axes {
            let (value, value_bytes) =
                crate::parser::read_i32_from_le_bytes(buf)?;

            values.push(value);
            values_bytes += value_bytes;
        }

        Ok((
            Self { signature, num_axes, values },
            signature_bytes + num_axes_bytes + values_bytes,
        ))
    }
}
