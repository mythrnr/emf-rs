/// The EMR_SCALEWINDOWEXTEX record specifies the current window in the playback
/// device context by using ratios formed by the specified multiplicands and
/// divisors.
///
/// The extent MUST NOT be changed if the current mapping mode is fixed scale.
/// Only MM_ISOTROPIC and MM_ANISOTROPIC are not fixed scale.
///
/// The new window extent is computed as follows.
///
/// ```
/// xNewWE = (xOldWE * xNum) / xDenom
/// yNewWE = (yOldWE * yNum) / yDenom
/// ```
#[derive(Clone, Debug)]
pub struct EMR_SCALEWINDOWEXTEX {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SCALEWINDOWEXTEX. This value is 0x00000020.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// xNum (4 bytes): A signed integer that specifies the horizontal
    /// multiplicand. MUST NOT be zero.
    pub x_num: i32,
    /// xDenom (4 bytes): A signed integer that specifies the horizontal
    /// divisor. MUST NOT be zero.
    pub x_denom: i32,
    /// yNum (4 bytes): A signed integer that specifies the vertical
    /// multiplicand. MUST NOT be zero.
    pub y_num: i32,
    /// yDenom (4 bytes): A signed integer that specifies the vertical divisor.
    /// MUST NOT be zero.
    pub y_denom: i32,
}

impl EMR_SCALEWINDOWEXTEX {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_SCALEWINDOWEXTEX {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SCALEWINDOWEXTEX as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (x_num, x_num_bytes),
            (x_denom, x_denom_bytes),
            (y_num, y_num_bytes),
            (y_denom, y_denom_bytes),
        ) = (
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
        );

        size.consume(x_num_bytes + x_denom_bytes + y_num_bytes + y_denom_bytes);

        if x_num == 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "x_num must not be zero. but parsed value is `{x_num}`",
                ),
            });
        }

        if x_denom == 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "x_denom must not be zero. but parsed value is `{x_denom}`",
                ),
            });
        }

        if y_num == 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "y_num must not be zero. but parsed value is `{y_num}`",
                ),
            });
        }

        if y_denom == 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "y_denom must not be zero. but parsed value is `{y_denom}`",
                ),
            });
        }

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, x_num, x_denom, y_num, y_denom })
    }
}
