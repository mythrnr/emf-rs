//! Point objects (MS-EMFPLUS 2.2.2.35 EmfPlusPoint, 2.2.2.36
//! EmfPlusPointF, 2.2.2.37 EmfPlusPointR) and the shared helper that
//! reads a point array in whichever of the three encodings the record
//! flags select.

use crate::imports::*;

/// The EmfPlusPoint object specifies an ordered pair of integer (X,Y)
/// values that define an absolute location in a coordinate space
/// (MS-EMFPLUS 2.2.2.35).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EmfPlusPoint {
    /// X (2 bytes): A signed integer that defines the horizontal
    /// coordinate.
    pub x: i16,
    /// Y (2 bytes): A signed integer that defines the vertical
    /// coordinate.
    pub y: i16,
}

impl EmfPlusPoint {
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
        let x = read_field(buf, &mut consumed_bytes)?;
        let y = read_field(buf, &mut consumed_bytes)?;

        Ok((Self { x, y }, consumed_bytes))
    }
}

/// The EmfPlusPointF object specifies an ordered pair of
/// floating-point (X,Y) values that define an absolute location in a
/// coordinate space (MS-EMFPLUS 2.2.2.36).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EmfPlusPointF {
    /// X (4 bytes): A floating-point value that specifies the
    /// horizontal coordinate.
    pub x: f32,
    /// Y (4 bytes): A floating-point value that specifies the vertical
    /// coordinate.
    pub y: f32,
}

impl EmfPlusPointF {
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
        let x = read_field(buf, &mut consumed_bytes)?;
        let y = read_field(buf, &mut consumed_bytes)?;

        Ok((Self { x, y }, consumed_bytes))
    }
}

/// The EmfPlusPointR object specifies an ordered pair of integer (X,Y)
/// values that define a relative location in a coordinate space
/// (MS-EMFPLUS 2.2.2.37).
///
/// Note: The object that specifies the horizontal coordinate is not
/// required to be the same type as the object that specifies the
/// vertical coordinate; that is, one can be 7 bits and the other can
/// be 15 bits.
///
/// Each coordinate is an EmfPlusInteger7 or EmfPlusInteger15 object
/// (2.2.2.21 / 2.2.2.22); the decoded signed deltas are stored here.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EmfPlusPointR {
    /// X (variable): A signed integer that specifies the horizontal
    /// coordinate. This value MUST be specified by either an
    /// EmfPlusInteger7 object (section 2.2.2.21) or an
    /// EmfPlusInteger15 object (section 2.2.2.22).
    ///
    /// The decoded delta relative to the previous point is stored.
    pub dx: i16,
    /// Y (variable): A signed integer that specifies the vertical
    /// coordinate. This value MUST be specified by either an
    /// EmfPlusInteger7 object or an EmfPlusInteger15 object.
    ///
    /// The decoded delta relative to the previous point is stored.
    pub dy: i16,
}

impl EmfPlusPointR {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let mut consumed_bytes: usize = 0;
        let dx = read_emf_plus_integer(buf, &mut consumed_bytes)?;
        let dy = read_emf_plus_integer(buf, &mut consumed_bytes)?;

        Ok((Self { dx, dy }, consumed_bytes))
    }
}

/// Bit 7 of the first byte distinguishes EmfPlusInteger15 (set) from
/// EmfPlusInteger7 (clear).
const INTEGER_15_FLAG: u8 = 0x80;
/// Bit 6 of the first byte is the sign bit of the encoded value.
const SIGN_BIT: u8 = 0x40;
/// The value bits carried by the first byte.
const VALUE_MASK: u8 = 0x7F;

/// Reads one EmfPlusInteger7 or EmfPlusInteger15 object
/// (MS-EMFPLUS 2.2.2.21 / 2.2.2.22).
///
/// The EmfPlusInteger7 object specifies a 7-bit signed integer in an
/// 8-bit field. Value (7 bits): A 7-bit signed integer between -64 and
/// 63, inclusive.
///
/// The EmfPlusInteger15 object specifies a 15-bit signed integer in a
/// 16-bit field. Value (15 bits): A 15-bit signed integer between
/// -16,384 and 16,383, inclusive.
///
/// Both objects are used to specify point coordinates in EmfPlusPointR
/// objects (section 2.2.2.37).
///
/// The first byte carries the format flag (bit 7), the sign bit
/// (bit 6), and the high value bits. EmfPlusInteger15 carries the low
/// 8 value bits in a second byte, so the 15-bit value is assembled in
/// big-endian order, unlike every fixed-width field in the format.
/// OR-ing the sign bit back into the removed flag-bit position
/// completes the two's complement representation of negative values.
fn read_emf_plus_integer<R: crate::Read>(
    buf: &mut R,
    tracker: &mut usize,
) -> Result<i16, crate::parser::ParseError> {
    use crate::parser::records::read_field;

    let first: u8 = read_field(buf, tracker)?;
    let negative = first & SIGN_BIT != 0;
    let mut high = first & VALUE_MASK;

    if negative {
        high |= INTEGER_15_FLAG;
    }

    if first & INTEGER_15_FLAG == 0 {
        return Ok(i16::from(high as i8));
    }

    let low: u8 = read_field(buf, tracker)?;

    Ok(i16::from_le_bytes([low, high]))
}

/// A point array read from a drawing record or a path object, keeping
/// the encoding that was used on the wire.
///
/// The encoding is selected by record flags: the P flag (relative
/// EmfPlusPointR), else the C flag (compressed EmfPlusPoint), else
/// EmfPlusPointF.
#[derive(Clone, Debug, PartialEq)]
pub enum EmfPlusPoints {
    /// Absolute 16-bit integer locations.
    Absolute(Vec<EmfPlusPoint>),
    /// Absolute 32-bit floating-point locations.
    AbsoluteF(Vec<EmfPlusPointF>),
    /// Deltas relative to the previous point; the first delta is
    /// relative to (0, 0).
    Relative(Vec<EmfPlusPointR>),
}

impl EmfPlusPoints {
    /// Reads `count` points in the encoding selected by `relative` (the
    /// record's P flag) and `compressed` (the record's C flag). The C
    /// flag is undefined and ignored when P is set.
    pub(crate) fn parse<R: crate::Read>(
        buf: &mut R,
        tracker: &mut impl crate::parser::ConsumeTracker,
        count: u32,
        relative: bool,
        compressed: bool,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::read_with;

        crate::parser::emf_plus::check_element_count("point count", count)?;

        // Grow-on-push instead of `with_capacity(count)`: the count is
        // bounded above, but a crafted count paired with a truncated
        // stream must fail on read, not pre-allocate first.
        if relative {
            let mut points = vec![];
            for _ in 0..count {
                points.push(read_with(buf, tracker, EmfPlusPointR::parse)?);
            }
            return Ok(Self::Relative(points));
        }

        if compressed {
            let mut points = vec![];
            for _ in 0..count {
                points.push(read_with(buf, tracker, EmfPlusPoint::parse)?);
            }
            return Ok(Self::Absolute(points));
        }

        let mut points = vec![];
        for _ in 0..count {
            points.push(read_with(buf, tracker, EmfPlusPointF::parse)?);
        }
        Ok(Self::AbsoluteF(points))
    }

    /// The number of points.
    pub fn len(&self) -> usize {
        match self {
            Self::Absolute(v) => v.len(),
            Self::AbsoluteF(v) => v.len(),
            Self::Relative(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Normalizes the points to absolute floating-point locations.
    /// Relative deltas are accumulated starting from (0, 0) as
    /// specified for EmfPlusPointR.
    pub fn as_points_f(&self) -> Vec<EmfPlusPointF> {
        match self {
            Self::Absolute(v) => v
                .iter()
                .map(|p| EmfPlusPointF { x: f32::from(p.x), y: f32::from(p.y) })
                .collect(),
            Self::AbsoluteF(v) => v.clone(),
            Self::Relative(v) => {
                let mut points = Vec::with_capacity(v.len());
                // i64 accumulators: MAX_ELEMENT_COUNT deltas of +/-16383
                // stay below 2^38, while an i32 would overflow after
                // about 131 thousand maximum deltas and panic under
                // overflow checks.
                let (mut x, mut y) = (0_i64, 0_i64);

                for p in v {
                    x += i64::from(p.dx);
                    y += i64::from(p.dy);
                    points.push(EmfPlusPointF { x: x as f32, y: y as f32 });
                }

                points
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_integer(bytes: &[u8]) -> (i16, usize) {
        let mut buf = bytes;
        let mut consumed = 0;
        let v = read_emf_plus_integer(&mut buf, &mut consumed).unwrap();
        (v, consumed)
    }

    #[test]
    fn integer7_positive() {
        assert_eq!(read_integer(&[0x0A]), (10, 1));
        // Maximum: 0x3F = 63.
        assert_eq!(read_integer(&[0x3F]), (63, 1));
    }

    #[test]
    fn integer7_negative() {
        // 0x4A: sign bit set, 7-bit two's complement of -54.
        assert_eq!(read_integer(&[0x4A]), (-54, 1));
        // Minimum: 0x40 = -64.
        assert_eq!(read_integer(&[0x40]), (-64, 1));
        // 0x7F = -1.
        assert_eq!(read_integer(&[0x7F]), (-1, 1));
    }

    #[test]
    fn integer15_positive() {
        // 0x92 0x34: flag bit set, value 0x1234 (big-endian assembly).
        assert_eq!(read_integer(&[0x92, 0x34]), (0x1234, 2));
        // Maximum: 0xBF 0xFF = 16383.
        assert_eq!(read_integer(&[0xBF, 0xFF]), (16383, 2));
    }

    #[test]
    fn integer15_negative() {
        // 0xC0 0x00: sign bit set, 15-bit two's complement of -16384.
        assert_eq!(read_integer(&[0xC0, 0x00]), (-16384, 2));
        // 0xFF 0xFF = -1.
        assert_eq!(read_integer(&[0xFF, 0xFF]), (-1, 2));
    }

    #[test]
    fn relative_points_accumulate_from_origin() {
        // (10, 20), then delta (-1, -2).
        let data = [0x0A, 0x14, 0x7F, 0x7E];
        let mut buf: &[u8] = &data;
        let mut consumed = 0_usize;

        let points =
            EmfPlusPoints::parse(&mut buf, &mut consumed, 2, true, false)
                .unwrap();

        assert_eq!(consumed, 4);
        assert_eq!(points.as_points_f(), vec![
            EmfPlusPointF { x: 10.0, y: 20.0 },
            EmfPlusPointF { x: 9.0, y: 18.0 },
        ],);
    }

    #[test]
    fn compressed_points_parse_as_i16() {
        let mut data = vec![];
        for v in [1_i16, -2, 3, -4] {
            data.extend(v.to_le_bytes());
        }
        let mut buf: &[u8] = &data;
        let mut consumed = 0_usize;

        let points =
            EmfPlusPoints::parse(&mut buf, &mut consumed, 2, false, true)
                .unwrap();

        assert_eq!(consumed, 8);
        assert_eq!(
            points,
            EmfPlusPoints::Absolute(vec![
                EmfPlusPoint { x: 1, y: -2 },
                EmfPlusPoint { x: 3, y: -4 },
            ]),
        );
    }

    #[test]
    fn relative_accumulation_survives_extreme_deltas() {
        // Enough maximum deltas to overflow an i32 accumulator; the
        // i64 accumulation must neither panic nor wrap.
        let count = 200_000_usize;
        let points = EmfPlusPoints::Relative(vec![
            EmfPlusPointR {
                dx: 16383,
                dy: -16384
            };
            count
        ]);

        let out = points.as_points_f();

        assert_eq!(out.len(), count);
        assert_eq!(out[count - 1], EmfPlusPointF {
            x: (16383_i64 * count as i64) as f32,
            y: (-16384_i64 * count as i64) as f32,
        },);
    }

    #[test]
    fn rejects_absurd_point_count() {
        let mut buf: &[u8] = &[];
        let mut consumed = 0_usize;

        assert!(
            EmfPlusPoints::parse(
                &mut buf,
                &mut consumed,
                u32::MAX,
                false,
                false,
            )
            .is_err()
        );
    }
}
