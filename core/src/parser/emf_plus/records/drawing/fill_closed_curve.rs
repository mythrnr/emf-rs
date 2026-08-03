use crate::parser::emf_plus::records::{
    EmfPlusBrushIdOrColor, FLAG_C, FLAG_P, flag,
};

/// The EmfPlusFillClosedCurve record specifies filling the interior of
/// a closed cardinal spline.
///
/// A "winding" fill operation fills areas according to the "even-odd
/// parity" rule. According to this rule, a test point can be
/// determined to be inside or outside a closed curve as follows: Draw
/// a line from the test point to a point that is distant from the
/// curve. If that line crosses the curve an odd number of times, the
/// test point is inside the curve; otherwise, the test point is
/// outside the curve.
///
/// An "alternate" fill operation fills areas according to the
/// "non-zero" rule. According to this rule, a test point can be
/// determined to be inside or outside a closed curve as follows: Draw
/// a line from a test point to a point that is distant from the curve.
/// Count the number of times the curve crosses the test line from left
/// to right, and count the number of times the curve crosses the test
/// line from right to left. If those two numbers are the same, the
/// test point is outside the curve; otherwise, the test point is
/// inside the curve.
///
/// Note that the two paragraphs above transcribe the specification
/// verbatim, but the specification swaps the rule definitions: in
/// GDI+, a winding fill uses the non-zero rule and an alternate fill
/// uses the even-odd rule. Playback implementations should follow the
/// GDI+ semantics.
#[derive(Clone, Debug)]
pub struct EmfPlusFillClosedCurve {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusFillClosedCurve from the RecordType enumeration.
    /// The value MUST be 0x4016.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    ///
    /// S (1 bit): This bit indicates the type of data in the BrushId
    /// field. If set, BrushId specifies a color as an EmfPlusARGB
    /// object. If clear, BrushId contains the index of an EmfPlusBrush
    /// object in the EMF+ Object Table.
    ///
    /// C (1 bit): This bit indicates whether the PointData field
    /// specifies compressed data. If set, PointData specifies absolute
    /// locations in the coordinate space with 16-bit signed integer
    /// coordinates. If clear, PointData specifies absolute locations
    /// in the coordinate space with 32-bit floating-point coordinates.
    ///
    /// Note: If the P flag (below) is set, this flag is undefined and
    /// MUST be ignored.
    ///
    /// P (1 bit): This bit indicates whether the PointData field
    /// specifies relative or absolute locations. If set, each element
    /// in PointData specifies a location in the coordinate space that
    /// is relative to the location specified by the previous element
    /// in the array. In the case of the first element in PointData, a
    /// previous location at coordinates (0,0) is assumed. If clear,
    /// PointData specifies absolute locations according to the C flag.
    ///
    /// Note: If this flag is set, the C flag (above) is undefined and
    /// MUST be ignored.
    pub flags: u16,
    /// W (1 bit): This bit indicates how to perform the fill
    /// operation. If set, the fill is a "winding" fill. If clear, the
    /// fill is an "alternate" fill.
    ///
    /// Decoded from bit 0x2000 of the record flags.
    pub winding: bool,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record. At least 3
    /// points MUST be specified.
    ///
    /// 0x00000020 <= value: If the P bit is set in the Flags field,
    /// the minimum Size is computed as follows:
    ///
    /// Size = ((((Count * 0x00000002) + 0x00000018 + 0x00000003) / 4) * 4)
    ///
    /// 0x00000024 <= value: If the P bit is clear and the C bit is set
    /// in the Flags field, Size is computed as follows:
    ///
    /// Size = (Count * 0x00000004) + 0x00000018
    ///
    /// 0x00000030 <= value: If the P bit is clear and the C bit is
    /// clear in the Flags field, Size is computed as follows:
    ///
    /// Size = (Count * 0x00000008) + 0x00000018
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record. At least 3
    /// points MUST be specified.
    ///
    /// 0x00000014 <= value: If the P bit is set in the Flags field,
    /// the minimum DataSize is computed as follows:
    ///
    /// DataSize = ((((Count * 0x00000002) + 0x0000000C + 0x00000003) / 4) * 4)
    ///
    /// 0x00000018 <= value: If the P bit is clear and the C bit is set
    /// in the Flags field, DataSize is computed as follows:
    ///
    /// DataSize = (Count * 0x00000004) + 0x0000000C
    ///
    /// 0x00000024 <= value: If the P bit is clear and the C bit is
    /// clear in the Flags field, DataSize is computed as follows:
    ///
    /// DataSize = (Count * 0x00000008) + 0x0000000C
    pub data_size: crate::parser::Size,
    /// BrushId (4 bytes): An unsigned integer that specifies the
    /// EmfPlusBrush, the content of which is determined by the S bit
    /// in the Flags field. This brush is used to fill the interior of
    /// the closed cardinal spline.
    pub brush_id: EmfPlusBrushIdOrColor,
    /// Tension (4 bytes): A floating-point value that specifies how
    /// tightly the spline bends as it passes through the points. A
    /// value of 0.0 specifies that the spline is a sequence of
    /// straight lines. As the value increases, the curve becomes more
    /// rounded. For more information, see [SPLINE77] and [PETZOLD].
    pub tension: f32,
    /// Count (4 bytes): An unsigned integer that specifies the number
    /// of points in the PointData field. At least 3 points MUST be
    /// specified.
    pub count: u32,
    /// PointData (variable): An array of Count points that specify the
    /// endpoints of the lines that define the spline. In a closed
    /// cardinal spline, the curve continues through the last point in
    /// the PointData array and connects with the first point in the
    /// array.
    ///
    /// The type of data in this array is specified by the Flags field,
    /// as follows:
    ///
    /// - EmfPlusPointR object: If the P flag is set in the Flags, the points
    ///   specify relative locations.
    /// - EmfPlusPoint object: If the P bit is clear and the C bit is set in
    ///   the Flags field, the points specify absolute locations with 16-bit
    ///   signed integer values.
    /// - EmfPlusPointF object: If the P bit is clear and the C bit is clear in
    ///   the Flags field, the points specify absolute locations with 32-bit
    ///   floating-point values.
    pub point_data: crate::parser::emf_plus::objects::EmfPlusPoints,
}

impl EmfPlusFillClosedCurve {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = ?record_type),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::emf_plus::RecordType,
        flags: u16,
        size: u32,
        mut data_size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::read_field;

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u16,
            crate::parser::emf_plus::RecordType::EmfPlusFillClosedCurve as u16,
        )?;

        let brush_id =
            EmfPlusBrushIdOrColor::parse(buf, &mut data_size, flags)?;
        let tension = read_field(buf, &mut data_size)?;
        let count: u32 = read_field(buf, &mut data_size)?;
        let point_data =
            crate::parser::emf_plus::objects::EmfPlusPoints::parse(
                buf,
                &mut data_size,
                count,
                flag(flags, FLAG_P),
                flag(flags, FLAG_C),
            )?;

        crate::parser::emf_plus::records::consume_remaining(
            buf,
            &mut data_size,
        )?;

        Ok(Self {
            record_type,
            flags,
            winding: flags & 0x2000 != 0,
            size,
            data_size,
            brush_id,
            tension,
            count,
            point_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::objects::{
        EmfPlusARGB, EmfPlusPoint, EmfPlusPoints,
    };

    #[test]
    fn parses_winding_fill_with_compressed_points() {
        let mut data = vec![];
        data.extend([0x01, 0x02, 0x03, 0xFF]); // blue, green, red, alpha
        data.extend(0.5_f32.to_le_bytes());
        data.extend(3_u32.to_le_bytes());
        for v in [0_i16, 0, 10, 0, 10, 20] {
            data.extend(v.to_le_bytes());
        }

        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusFillClosedCurve::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusFillClosedCurve,
            0xE000,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusFillClosedCurve,
        );
        assert_eq!(record.flags, 0xE000);
        assert_eq!(record.size, 0x0000_0024);
        assert!(record.winding);
        assert_eq!(
            record.brush_id,
            EmfPlusBrushIdOrColor::Color(EmfPlusARGB {
                blue: 0x01,
                green: 0x02,
                red: 0x03,
                alpha: 0xFF,
            }),
        );
        assert_eq!(record.tension.to_bits(), 0.5_f32.to_bits());
        assert_eq!(record.count, 3);
        assert_eq!(
            record.point_data,
            EmfPlusPoints::Absolute(vec![
                EmfPlusPoint { x: 0, y: 0 },
                EmfPlusPoint { x: 10, y: 0 },
                EmfPlusPoint { x: 10, y: 20 },
            ]),
        );
    }
}
