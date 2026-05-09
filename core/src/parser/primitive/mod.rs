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
        let (value, value_bytes) =
            <u16 as crate::parser::ReadLeField>::read_le(buf)?;

        crate::parser::ParseError::expect_in_range(
            "Gamma value",
            value,
            2_500_u16,
            65_000_u16,
        )?;

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
        let (value, value_bytes) =
            <i16 as crate::parser::ReadLeField>::read_le(buf)?;

        crate::parser::ParseError::expect_in_range(
            "Adjustment value",
            value,
            -100_i16,
            100_i16,
        )?;

        Ok((Self(value), value_bytes))
    }

    pub fn must_not_be_performed(&self) -> bool {
        self.0 == 0
    }
}

/// Upper bound on a single EMR record's `Size` field, in bytes.
///
/// 64 MiB is several orders of magnitude larger than any practical EMR
/// record (an A0-resolution screen capture stretches into the megabyte
/// range, not the gigabyte range). The bound exists to short-circuit
/// crafted inputs that would otherwise drive `Vec::with_capacity`
/// through `byte_count()` into oversized allocations.
pub const MAX_RECORD_BYTES: u32 = 64 * 1024 * 1024;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct Size {
    /// Total record length in bytes, taken verbatim from the
    /// `Size` field of the record header.
    byte_count_raw: u32,
    /// Number of bytes already consumed from the record payload, used
    /// to detect overruns and to skip the trailing reserved area.
    consumed_bytes: usize,
}

impl From<u32> for Size {
    fn from(v: u32) -> Self {
        Self { byte_count_raw: v, consumed_bytes: 0 }
    }
}

impl From<Size> for u32 {
    fn from(v: Size) -> Self {
        v.byte_count_raw
    }
}

impl core::fmt::Display for Size {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#010X}", self.byte_count_raw)
    }
}

impl core::fmt::Debug for Size {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Size(size: {:#010X}, consumed_bytes: {})",
            self.byte_count_raw, self.consumed_bytes
        )
    }
}

impl Size {
    /// Constructs a `Size` from the raw `Size` field of an EMR record
    /// header, rejecting values past `MAX_RECORD_BYTES` so that
    /// downstream `byte_count()` consumers cannot inadvertently allocate
    /// gigabytes from a malformed stream.
    pub fn parse(
        byte_count_raw: u32,
    ) -> Result<Self, crate::parser::ParseError> {
        crate::parser::ParseError::expect_le(
            "record_size (bytes)",
            byte_count_raw,
            MAX_RECORD_BYTES,
        )?;

        Ok(Self { byte_count_raw, consumed_bytes: 0 })
    }

    #[inline]
    pub fn byte_count(&self) -> usize {
        self.byte_count_raw as usize
    }

    /// Advances the consumed-bytes counter via `checked_add`. Saturates
    /// on overflow rather than panicking; an overrun is then surfaced
    /// through `is_overrun()` and `remaining_bytes()` (which clamps to
    /// 0) so the caller still observes the malformed state without an
    /// unwind.
    #[inline]
    pub fn consume(&mut self, consumed_bytes: usize) {
        self.consumed_bytes =
            self.consumed_bytes.saturating_add(consumed_bytes);
    }

    #[inline]
    pub fn consumed_bytes(&self) -> usize {
        self.consumed_bytes
    }

    /// Subtracts consumed bytes from the given offset, returning a
    /// `ParseError` on underflow.
    pub fn checked_offset(
        &self,
        offset: u32,
    ) -> Result<usize, crate::parser::ParseError> {
        let offset = offset as usize;

        offset.checked_sub(self.consumed_bytes).ok_or_else(|| {
            crate::parser::ParseError::UnexpectedPattern {
                cause: alloc::format!(
                    "offset ({offset:#010X}) is less than consumed bytes ({})",
                    self.consumed_bytes,
                )
                .into(),
            }
        })
    }

    /// Returns true when `consumed_bytes` has exceeded `byte_count`,
    /// indicating a malformed record or a parser bug.
    #[inline]
    pub fn is_overrun(&self) -> bool {
        self.consumed_bytes > self.byte_count()
    }

    #[inline]
    pub fn remaining(&self) -> bool {
        !self.is_overrun() && self.remaining_bytes() > 0
    }

    /// Bytes left in the record. Saturates to 0 on overrun so callers
    /// that thread this into `read_variable` or `Vec::with_capacity`
    /// cannot underflow; pair with `is_overrun()` when overrun must be
    /// distinguished from a normal end-of-record.
    #[inline]
    pub fn remaining_bytes(&self) -> usize {
        self.byte_count().saturating_sub(self.consumed_bytes)
    }
}

#[cfg(test)]
mod size_tests {
    use super::*;

    #[test]
    fn from_u32_zero_consumed() {
        let s = Size::from(16);
        assert_eq!(s.byte_count(), 16);
        assert_eq!(s.consumed_bytes(), 0);
        assert_eq!(s.remaining_bytes(), 16);
        assert!(!s.is_overrun());
    }

    #[test]
    fn parse_rejects_over_max() {
        assert!(Size::parse(MAX_RECORD_BYTES).is_ok());
        assert!(Size::parse(MAX_RECORD_BYTES + 1).is_err());
    }

    #[test]
    fn consume_then_remaining() {
        let mut s = Size::from(20);
        s.consume(8);
        assert_eq!(s.consumed_bytes(), 8);
        assert_eq!(s.remaining_bytes(), 12);
        assert!(s.remaining());
    }

    #[test]
    fn overrun_saturates_remaining() {
        let mut s = Size::from(10);
        s.consume(20);
        // remaining_bytes saturates to 0 instead of panicking.
        assert_eq!(s.remaining_bytes(), 0);
        assert!(s.is_overrun());
        // remaining() must report false once overrun is detected.
        assert!(!s.remaining());
    }

    #[test]
    fn checked_offset_underflow_errors() {
        let mut s = Size::from(20);
        s.consume(15);
        assert!(s.checked_offset(10).is_err());
        // Equal offset returns 0.
        assert_eq!(s.checked_offset(15).unwrap(), 0);
    }
}

#[cfg(test)]
mod gamma_adjustment_tests {
    use super::*;

    fn read_gamma(value: u16) -> Result<Gamma, crate::parser::ParseError> {
        let bytes = value.to_le_bytes();
        let mut buf: &[u8] = &bytes;
        Gamma::parse(&mut buf).map(|(v, _)| v)
    }

    fn read_adjustment(
        value: i16,
    ) -> Result<Adjustment, crate::parser::ParseError> {
        let bytes = value.to_le_bytes();
        let mut buf: &[u8] = &bytes;
        Adjustment::parse(&mut buf).map(|(v, _)| v)
    }

    #[test]
    fn gamma_accepts_lower_bound() {
        assert!(read_gamma(2_500).is_ok());
    }

    #[test]
    fn gamma_accepts_upper_bound() {
        assert!(read_gamma(65_000).is_ok());
    }

    #[test]
    fn gamma_rejects_below_lower_bound() {
        assert!(read_gamma(2_499).is_err());
    }

    #[test]
    fn gamma_rejects_above_upper_bound() {
        // value 65_001 cannot be represented in u16, so use the next
        // edge: 65_001 wraps to ... actually we can't test above 65_535.
        // The bound is 65_000, so 65_001 fits in u16.
        assert!(read_gamma(65_001).is_err());
    }

    #[test]
    fn gamma_default_signals_no_correction() {
        let g = Gamma::default();
        assert!(g.must_not_be_performed());
    }

    #[test]
    fn adjustment_accepts_inclusive_bounds() {
        assert!(read_adjustment(-100).is_ok());
        assert!(read_adjustment(100).is_ok());
    }

    #[test]
    fn adjustment_rejects_out_of_range() {
        assert!(read_adjustment(-101).is_err());
        assert!(read_adjustment(101).is_err());
    }

    #[test]
    fn adjustment_zero_signals_no_correction() {
        let a = read_adjustment(0).unwrap();
        assert!(a.must_not_be_performed());
    }
}
