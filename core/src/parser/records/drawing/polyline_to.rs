use crate::imports::*;

/// The EMR_POLYLINETO record specifies one or more straight lines based upon
/// the current drawing position.
///
/// A line SHOULD be drawn from the current position to the first point
/// specified by the aPoints field using the current pen. Each additional line
/// SHOULD be drawn from the ending point of the previous line to the next point
/// specified by aPoints.
#[derive(Clone, Debug)]
pub struct EMR_POLYLINETO {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_POLYLINETO. This value is 0x00000006.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object, specified in [MS-WMF] section
    /// 2.2.2.19, which specifies the bounding rectangle in logical units.
    pub bounds: wmf_core::parser::RectL,
    /// Count (4 bytes): An unsigned integer that specifies the number of
    /// points in the aPoints array.
    ///
    /// | Line width | Device supports wideline | Maximum points allowed |
    /// |:-|:-|:-|
    /// | `1` | `n/a` | `16K` |
    /// | `> 1` | `yes` | `16K` |
    /// | `> 1` | `no` | `1360` |
    ///
    /// Any extra points MUST be ignored.
    pub count: u32,
    /// aPoints (variable): A Count length array of PointL objects ([MS-WMF]
    /// section 2.2.2.15, which specifies the point data, in logical units.
    pub a_points: Vec<wmf_core::parser::PointL>,
}

impl EMR_POLYLINETO {
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
        if record_type != crate::parser::RecordType::EMR_POLYLINETO {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_POLYLINETO as u32,
                    record_type as u32
                ),
            });
        }

        let ((bounds, bounds_bytes), (count, count_bytes)) = (
            wmf_core::parser::RectL::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        size.consume(bounds_bytes + count_bytes);

        let a_points = {
            let mut entries = vec![];

            for _ in 0..count {
                let (v, b) = wmf_core::parser::PointL::parse(buf)?;

                entries.push(v);
                size.consume(b);
            }

            entries
        };

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, bounds, count, a_points })
    }
}
