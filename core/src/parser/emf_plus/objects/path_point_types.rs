//! Path point type objects (MS-EMFPLUS 2.2.2.31 EmfPlusPathPointType,
//! 2.2.2.32 EmfPlusPathPointTypeRLE).

use crate::imports::*;

/// The EmfPlusPathPointType object specifies a type value associated
/// with a point on a graphics path (MS-EMFPLUS 2.2.2.31).
///
/// Graphics paths are specified by EmfPlusPath objects (section
/// 2.2.1.6). Every point on a graphics path MUST have a type value
/// associated with it.
///
/// The single byte carries PathPointTypeFlags in the high 4 bits and a
/// PathPointType enumeration value in the low 4 bits.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmfPlusPathPointType {
    /// Flags (4 bits): A flag field that specifies properties of the
    /// path point. This value is one or more of the PathPointType
    /// flags (section 2.1.2.6).
    pub flags: crate::parser::emf_plus::PathPointTypeFlags,
    /// Type (4 bits): An unsigned integer path point type. This value
    /// is defined in the PathPointType enumeration (section 2.1.1.22).
    pub point_type: crate::parser::emf_plus::PathPointType,
}

impl EmfPlusPathPointType {
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
        let value: u8 = read_field(buf, &mut consumed_bytes)?;

        Ok((Self::from_byte(value)?, consumed_bytes))
    }

    /// Decodes the packed flags/type byte.
    pub fn from_byte(value: u8) -> Result<Self, crate::parser::ParseError> {
        let Some(point_type) =
            crate::parser::emf_plus::PathPointType::from_repr(value & 0x0F)
        else {
            return Err(crate::parser::ParseError::UnexpectedEnumValue {
                cause: alloc::format!(
                    "unexpected value as PathPointType: {:#04X}",
                    value & 0x0F,
                )
                .into(),
            });
        };

        Ok(Self {
            flags: crate::parser::emf_plus::PathPointTypeFlags::from_raw(
                u32::from(value >> 4),
            ),
            point_type,
        })
    }

    /// Whether this point closes its subpath.
    pub fn closes_subpath(self) -> bool {
        self.flags.contains(
            crate::parser::emf_plus::PathPointTypeFlags::CLOSE_SUBPATH,
        )
    }
}

/// The EmfPlusPathPointTypeRLE object specifies type values associated
/// with points on a graphics path using RLE compression ([MS-WMF]
/// section 3.1.6) (MS-EMFPLUS 2.2.2.32).
///
/// Graphics paths are specified by EmfPlusPath objects (section
/// 2.2.1.6). Every point on a graphics path MUST have a type value
/// associated with it.
///
/// RLE compression makes it possible to specify an arbitrary number of
/// identical values without a proportional increase in storage
/// requirements.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmfPlusPathPointTypeRLE {
    /// B (1 bit): If set, the path points are on a Bezier curve.
    ///
    /// If clear, the path points are on a graphics line.
    pub bezier: bool,
    /// RunCount (6 bits): The run count, which is the number of path
    /// points to be associated with the type in the PointType field.
    pub run_count: u8,
    /// PointType (1 byte): An EmfPlusPathPointType object (section
    /// 2.2.2.31) that specifies the type to associate with the path
    /// points.
    pub point_type: EmfPlusPathPointType,
}

impl EmfPlusPathPointTypeRLE {
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
        let first: u8 = read_field(buf, &mut consumed_bytes)?;
        let type_byte: u8 = read_field(buf, &mut consumed_bytes)?;

        Ok((
            Self {
                bezier: first & 0x80 != 0,
                run_count: first & 0x3F,
                point_type: EmfPlusPathPointType::from_byte(type_byte)?,
            },
            consumed_bytes,
        ))
    }
}

/// The point type array of a path, keeping the encoding that was used
/// on the wire.
#[derive(Clone, Debug, PartialEq)]
pub enum EmfPlusPathPointTypes {
    /// One EmfPlusPathPointType per path point.
    Plain(Vec<EmfPlusPathPointType>),
    /// Run-length encoded type values.
    Rle(Vec<EmfPlusPathPointTypeRLE>),
}

impl EmfPlusPathPointTypes {
    /// Reads type values for `count` path points. With `rle` set, RLE
    /// entries are read until their run counts cover `count` points.
    pub(crate) fn parse<R: crate::Read>(
        buf: &mut R,
        tracker: &mut impl crate::parser::ConsumeTracker,
        count: u32,
        rle: bool,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::read_with;

        crate::parser::emf_plus::check_element_count(
            "path point type count",
            count,
        )?;

        if rle {
            let mut entries = vec![];
            let mut covered: u64 = 0;

            while covered < u64::from(count) {
                let entry: EmfPlusPathPointTypeRLE =
                    read_with(buf, tracker, EmfPlusPathPointTypeRLE::parse)?;

                // A zero run covers nothing; rejecting it keeps the
                // loop from consuming the buffer without progress.
                crate::parser::ParseError::expect_ne(
                    "RunCount",
                    entry.run_count,
                    0,
                )?;

                covered += u64::from(entry.run_count);
                entries.push(entry);
            }

            return Ok(Self::Rle(entries));
        }

        let mut types = vec![];
        for _ in 0..count {
            types.push(read_with(buf, tracker, EmfPlusPathPointType::parse)?);
        }

        Ok(Self::Plain(types))
    }

    /// Expands the type array to one entry per path point. An RLE run
    /// that overshoots the point count is truncated by the caller if
    /// necessary.
    pub fn expand(&self) -> Vec<EmfPlusPathPointType> {
        match self {
            Self::Plain(v) => v.clone(),
            Self::Rle(entries) => {
                let mut types = vec![];
                for entry in entries {
                    for _ in 0..entry.run_count {
                        types.push(entry.point_type);
                    }
                }
                types
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::{PathPointType, PathPointTypeFlags};

    #[test]
    fn decodes_packed_flags_and_type() {
        // CloseSubpath (0x8 in the high nibble) + Line (0x1).
        let v = EmfPlusPathPointType::from_byte(0x81).unwrap();

        assert_eq!(v.point_type, PathPointType::PathPointTypeLine);
        assert!(v.closes_subpath());
        assert!(!v.flags.contains(PathPointTypeFlags::DASH_MODE));
    }

    #[test]
    fn rejects_undefined_point_type() {
        assert!(EmfPlusPathPointType::from_byte(0x02).is_err());
    }

    #[test]
    fn rle_expands_runs() {
        // Run of 3 bezier points (type 0x03), then 1 start point.
        let data = [0x83, 0x03, 0x01, 0x00];
        let mut buf: &[u8] = &data;
        let mut consumed = 0_usize;

        let types =
            EmfPlusPathPointTypes::parse(&mut buf, &mut consumed, 4, true)
                .unwrap();

        assert_eq!(consumed, 4);

        let expanded = types.expand();
        assert_eq!(expanded.len(), 4);
        assert_eq!(expanded[0].point_type, PathPointType::PathPointTypeBezier,);
        assert_eq!(expanded[3].point_type, PathPointType::PathPointTypeStart,);
    }

    #[test]
    fn rle_rejects_zero_run_count() {
        let data = [0x80, 0x03];
        let mut buf: &[u8] = &data;
        let mut consumed = 0_usize;

        assert!(
            EmfPlusPathPointTypes::parse(&mut buf, &mut consumed, 4, true)
                .is_err()
        );
    }
}
