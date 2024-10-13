/// The EMR_SETPOLYFILLMODE record defines polygon fill mode.
///
/// In general, the modes differ only in cases where a complex, overlapping
/// polygon MUST be filled; for example, a five-sided polygon that forms a
/// five-pointed star with a pentagon in the center. In such cases, ALTERNATE
/// mode SHOULD fill every other enclosed region within the polygon (the points
/// of the star), but WINDING mode SHOULD fill all regions (the points of the
/// star and the pentagon).
///
/// When the fill mode is ALTERNATE, the area between odd-numbered and
/// even-numbered polygon sides on each scan line SHOULD be filled. That is, the
/// area between the first and second side SHOULD be filled, and between the
/// third and fourth side, and so on.
///
/// When the fill mode is WINDING, any region that has a nonzero winding value
/// SHOULD be filled. The winding value is the number of times a pen used to
/// draw the polygon would go around the region. The direction of each edge of
/// the polygon is significant.
#[derive(Clone, Debug)]
pub struct EMR_SETPOLYFILLMODE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETPOLYFILLMODE. This value is 0x00000013.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// PolygonFillMode (4 bytes): An unsigned integer that specifies the
    /// polygon fill mode and is in the PolygonFillMode enumeration.
    pub polygon_fill_mode: crate::parser::PolygonFillMode,
}

impl EMR_SETPOLYFILLMODE {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_SETPOLYFILLMODE {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETPOLYFILLMODE as u32,
                    record_type as u32
                ),
            });
        }

        let (polygon_fill_mode, polygon_fill_mode_bytes) =
            crate::parser::PolygonFillMode::parse(buf)?;

        size.consume(polygon_fill_mode_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, polygon_fill_mode })
    }
}
