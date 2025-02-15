/// An unsigned integer that specifies the nth power gamma correction value for
/// the primary of the source colors. This value SHOULD be in the range from
/// 2,500 to 65,000. A value of 10,000 means gamma correction MUST NOT be
/// performed.
#[derive(Clone, Debug)]
pub struct Gamma(u16);

impl Default for Gamma {
    fn default() -> Self {
        Gamma(10_000)
    }
}

impl Gamma {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (value, value_bytes) = crate::parser::read_u16_from_le_bytes(buf)?;

        if !(2_500..=65_000).contains(&value) {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "Gamma value should be in the range from `2,500` to \
                     `65,000`, but value is {value}"
                ),
            });
        }

        Ok((Self(value), value_bytes))
    }

    pub fn must_not_be_performed(&self) -> bool {
        self.0 == 10_000
    }
}

/// A signed integer that specifies the amount to be applied to the source
/// object. This value SHOULD be in the range from –100 to 100. A value of zero
/// means adjustment MUST NOT be performed.
#[derive(Clone, Debug, Default)]
pub struct Adjustment(i16);

impl Adjustment {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (value, value_bytes) = crate::parser::read_i16_from_le_bytes(buf)?;

        if !(-100..=100).contains(&value) {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "Gamma value should be in the range from `-100` to `100`, \
                     but value is {value}"
                ),
            });
        }

        Ok((Self(value), value_bytes))
    }

    pub fn must_not_be_performed(&self) -> bool {
        self.0 == 0
    }
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct Size(u32, usize);

impl From<u32> for Size {
    fn from(v: u32) -> Self {
        Self(v, 0)
    }
}

impl From<Size> for u32 {
    fn from(v: Size) -> Self {
        v.0
    }
}

impl core::fmt::Display for Size {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#010X}", self.0)
    }
}

impl core::fmt::Debug for Size {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Size(size: {:#010X}, consumed_bytes: {})", self.0, self.1)
    }
}

impl Size {
    pub fn byte_count(&self) -> usize {
        self.0 as usize
    }

    pub fn consume(&mut self, consumed_bytes: usize) {
        self.1 += consumed_bytes;
    }

    pub fn consumed_bytes(&self) -> usize {
        self.1
    }

    pub fn remaining(&self) -> bool {
        self.remaining_bytes() > 0
    }

    pub fn remaining_bytes(&self) -> usize {
        self.byte_count() - self.1
    }
}
