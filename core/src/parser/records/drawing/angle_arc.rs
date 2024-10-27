/// The EMR_ANGLEARC record specifies a line segment of an arc. The line
/// segment is drawn from the current position to the beginning of the arc. The
/// arc is drawn along the perimeter of a circle with the given radius and
/// center. The length of the arc is defined by the given start and sweep
/// angles.
///
/// The arc is drawn by recording an imaginary circle around the specified
/// center point with the specified radius. The starting point of the arc is
/// determined by measuring counterclockwise from the x-axis of the circle by
/// the number of degrees in the start angle. The ending point is similarly
/// located by measuring counterclockwise from the starting point by the number
/// of degrees in the sweep angle.
///
/// If the sweep angle is greater than 360 degrees, the arc is swept multiple
/// times.
///
/// This record specifies lines by using the current pen. The figure is not
/// filled.
#[derive(Clone, Debug)]
pub struct EMR_ANGLEARC {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_ANGLEARC. This value is 0x00000029.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// Center (8 bytes): A PointL object ([MS-WMF] section 2.2.2.15), which
    /// specifies the logical coordinates of the circle's center.
    pub center: wmf_core::parser::PointL,
    /// Radius (4 bytes): An unsigned integer that specifies the circle's
    /// radius, in logical units.
    pub radius: u32,
    /// StartAngle (4 bytes): A 32-bit float that specifies the arc's start
    /// angle, in degrees.
    pub start_angle: f32,
    /// SweepAngle (4 bytes): A 32-bit float that specifies the arc's sweep
    /// angle, in degrees.
    pub sweep_angle: f32,
}

impl EMR_ANGLEARC {
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
        if record_type != crate::parser::RecordType::EMR_ANGLEARC {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_ANGLEARC as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (center, center_bytes),
            (radius, radius_bytes),
            (start_angle, start_angle_bytes),
            (sweep_angle, sweep_angle_bytes),
        ) = (
            wmf_core::parser::PointL::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_f32_from_le_bytes(buf)?,
            crate::parser::read_f32_from_le_bytes(buf)?,
        );

        size.consume(
            center_bytes + radius_bytes + start_angle_bytes + sweep_angle_bytes,
        );

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, center, radius, start_angle, sweep_angle })
    }
}
