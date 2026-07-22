//! Rectangle objects (MS-EMFPLUS 2.2.2.38 EmfPlusRect, 2.2.2.39
//! EmfPlusRectF) and the shared helper that reads rectangle data in
//! whichever encoding the record flags select.

use crate::imports::*;

/// The EmfPlusRect object specifies a rectangle origin, height, and
/// width as integers (MS-EMFPLUS 2.2.2.38).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EmfPlusRect {
    /// X (2 bytes): A signed integer that specifies the horizontal
    /// coordinate of the upper-left corner of the rectangle.
    pub x: i16,
    /// Y (2 bytes): A signed integer that specifies the vertical
    /// coordinate of the upper-left corner of the rectangle.
    pub y: i16,
    /// Width (2 bytes): A signed integer that specifies the width of
    /// the rectangle.
    pub width: i16,
    /// Height (2 bytes): A signed integer that specifies the height of
    /// the rectangle.
    pub height: i16,
}

impl EmfPlusRect {
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
        let width = read_field(buf, &mut consumed_bytes)?;
        let height = read_field(buf, &mut consumed_bytes)?;

        Ok((Self { x, y, width, height }, consumed_bytes))
    }
}

/// The EmfPlusRectF object specifies a rectangle's origin, height, and
/// width as floating-point values (MS-EMFPLUS 2.2.2.39).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EmfPlusRectF {
    /// X (4 bytes): A floating-point value that specifies the
    /// horizontal coordinate of the upper-left corner of the
    /// rectangle.
    pub x: f32,
    /// Y (4 bytes): A floating-point value that specifies the vertical
    /// coordinate of the upper-left corner of the rectangle.
    pub y: f32,
    /// Width (4 bytes): A floating-point value that specifies the
    /// width of the rectangle.
    pub width: f32,
    /// Height (4 bytes): A floating-point value that specifies the
    /// height of the rectangle.
    pub height: f32,
}

impl EmfPlusRectF {
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
        let width = read_field(buf, &mut consumed_bytes)?;
        let height = read_field(buf, &mut consumed_bytes)?;

        Ok((Self { x, y, width, height }, consumed_bytes))
    }
}

/// Rectangle data read from a record, keeping the encoding that was
/// used on the wire. The record's C flag selects EmfPlusRect (set) or
/// EmfPlusRectF (clear).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EmfPlusRectData {
    /// A 16-bit integer rectangle.
    Compressed(EmfPlusRect),
    /// A 32-bit floating-point rectangle.
    Float(EmfPlusRectF),
}

impl EmfPlusRectData {
    /// Reads one rectangle in the encoding selected by the record's C
    /// flag.
    pub(crate) fn parse<R: crate::Read>(
        buf: &mut R,
        tracker: &mut impl crate::parser::ConsumeTracker,
        compressed: bool,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::read_with;

        if compressed {
            Ok(Self::Compressed(read_with(buf, tracker, EmfPlusRect::parse)?))
        } else {
            Ok(Self::Float(read_with(buf, tracker, EmfPlusRectF::parse)?))
        }
    }

    /// Reads `count` rectangles in the encoding selected by the
    /// record's C flag.
    pub(crate) fn parse_list<R: crate::Read>(
        buf: &mut R,
        tracker: &mut impl crate::parser::ConsumeTracker,
        count: u32,
        compressed: bool,
    ) -> Result<Vec<Self>, crate::parser::ParseError> {
        crate::parser::emf_plus::check_element_count("rectangle count", count)?;

        let mut rects = vec![];
        for _ in 0..count {
            rects.push(Self::parse(buf, tracker, compressed)?);
        }

        Ok(rects)
    }

    /// Normalizes the rectangle to floating-point form.
    pub fn as_rect_f(&self) -> EmfPlusRectF {
        match self {
            Self::Compressed(r) => EmfPlusRectF {
                x: f32::from(r.x),
                y: f32::from(r.y),
                width: f32::from(r.width),
                height: f32::from(r.height),
            },
            Self::Float(r) => *r,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_compressed_and_float_forms() {
        let mut data = vec![];
        for v in [1_i16, 2, 3, 4] {
            data.extend(v.to_le_bytes());
        }
        let mut buf: &[u8] = &data;
        let mut consumed = 0_usize;
        let rect =
            EmfPlusRectData::parse(&mut buf, &mut consumed, true).unwrap();

        assert_eq!(consumed, 8);
        assert_eq!(rect.as_rect_f(), EmfPlusRectF {
            x: 1.0,
            y: 2.0,
            width: 3.0,
            height: 4.0
        },);

        let mut data = vec![];
        for v in [5.0_f32, 6.0, 7.0, 8.0] {
            data.extend(v.to_le_bytes());
        }
        let mut buf: &[u8] = &data;
        let mut consumed = 0_usize;
        let rect =
            EmfPlusRectData::parse(&mut buf, &mut consumed, false).unwrap();

        assert_eq!(consumed, 16);
        assert_eq!(rect.as_rect_f(), EmfPlusRectF {
            x: 5.0,
            y: 6.0,
            width: 7.0,
            height: 8.0
        },);
    }
}
