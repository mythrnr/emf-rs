use crate::imports::*;

/// The EMR_SETDIBITSTODEVICE record specifies a block transfer of pixels from
/// specified scanlines of a source bitmap to a destination rectangle.
///
/// This record supports source images in JPEG and PNG format. The Compression
/// field in the source bitmap header specifies the image format.
#[derive(Clone, Debug)]
pub struct EMR_SETDIBITSTODEVICE {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETDIBITSTODEVICE. This value is 0x00000050.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19) that
    /// specifies the destination bounding rectangle in logical coordinates. If
    /// the intersection of this rectangle with the current clipping regions in
    /// the playback device context is empty, this record has no effect.
    pub bounds: wmf_core::parser::RectL,
    /// xDest (4 bytes): A signed integer that specifies the logical
    /// x-coordinate of the upper-left corner of the destination rectangle.
    pub x_dest: i32,
    /// yDest (4 bytes): A signed integer that specifies the logical
    /// y-coordinate of the upper-left corner of the destination rectangle.
    pub y_dest: i32,
    /// xSrc (4 bytes): A signed integer that specifies the x-coordinate in
    /// pixels of the lower-left corner of the source rectangle.
    pub x_src: i32,
    /// ySrc (4 bytes): A signed integer that specifies the y-coordinate in
    /// pixels of the lower-left corner of the source rectangle.
    pub y_src: i32,
    /// cxSrc (4 bytes): A signed integer that specifies the width in pixels of
    /// the source rectangle.
    pub cx_src: i32,
    /// cySrc (4 bytes): A signed integer that specifies the height in pixels
    /// of the source rectangle.
    pub cy_src: i32,
    /// offBmiSrc (4 bytes): An unsigned integer that specifies the offset in
    /// bytes, from the start of this record to the source bitmap header in the
    /// BitmapBuffer field.
    pub off_bmi_src: u32,
    /// cbBmiSrc (4 bytes): An unsigned integer that specifies the size in
    /// bytes, of the source bitmap header.
    pub cb_bmi_src: u32,
    /// offBitsSrc (4 bytes): An unsigned integer that specifies the offset in
    /// bytes, from the start of this record to the source bitmap bits in the
    /// BitmapBuffer field.
    pub off_bits_src: u32,
    /// cbBitsSrc (4 bytes): An unsigned integer that specifies the size in
    /// bytes, of the source bitmap bits.
    pub cb_bits_src: u32,
    /// UsageSrc (4 bytes): An unsigned integer that specifies how to interpret
    /// values in the color table in the source bitmap header. This value is in
    /// the DIBColors enumeration.
    pub usage_src: crate::parser::DIBColors,
    /// iStartScan (4 bytes): An unsigned integer that specifies the first scan
    /// line in the array.
    pub i_start_scan: u32,
    /// cScans (4 bytes): An unsigned integer that specifies the number of scan
    /// lines.
    pub c_scans: u32,
    /// BitmapBuffer (variable): A buffer containing the source bitmap, which
    /// is not required to be contiguous with the fixed portion of the
    /// EMR_SETDIBITSTODEVICE record. Thus, fields in this buffer that are
    /// labeled "UndefinedSpace" are optional and MUST be ignored.
    ///
    /// BmiSrc (variable): The source bitmap header.
    pub bmi_src: Vec<u8>,
    /// BitsSrc (variable): The source bitmap bits.
    pub bits_src: Vec<u8>,
}

impl EMR_SETDIBITSTODEVICE {
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
        if record_type != crate::parser::RecordType::EMR_SETDIBITSTODEVICE {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETDIBITSTODEVICE as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (bounds, bounds_bytes),
            (x_dest, x_dest_bytes),
            (y_dest, y_dest_bytes),
            (x_src, x_src_bytes),
            (y_src, y_src_bytes),
            (cx_src, cx_src_bytes),
            (cy_src, cy_src_bytes),
            (off_bmi_src, off_bmi_src_bytes),
            (cb_bmi_src, cb_bmi_src_bytes),
            (off_bits_src, off_bits_src_bytes),
            (cb_bits_src, cb_bits_src_bytes),
            (usage_src, usage_src_bytes),
            (i_start_scan, i_start_scan_bytes),
            (c_scans, c_scans_bytes),
        ) = (
            wmf_core::parser::RectL::parse(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::DIBColors::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        size.consume(
            bounds_bytes
                + x_dest_bytes
                + y_dest_bytes
                + x_src_bytes
                + y_src_bytes
                + cx_src_bytes
                + cy_src_bytes
                + off_bmi_src_bytes
                + cb_bmi_src_bytes
                + off_bits_src_bytes
                + cb_bits_src_bytes
                + usage_src_bytes
                + i_start_scan_bytes
                + c_scans_bytes,
        );

        let ((_, undef_space_bytes), (bmi_src, bmi_src_bytes)) = (
            crate::parser::read_variable(
                buf,
                off_bmi_src as usize - size.consumed_bytes(),
            )?,
            crate::parser::read_variable(buf, cb_bmi_src as usize)?,
        );

        size.consume(undef_space_bytes + bmi_src_bytes);

        let ((_, undef_space_bytes), (bits_src, bits_src_bytes)) = (
            crate::parser::read_variable(
                buf,
                off_bits_src as usize - size.consumed_bytes(),
            )?,
            crate::parser::read_variable(buf, cb_bits_src as usize)?,
        );

        size.consume(undef_space_bytes + bits_src_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self {
            record_type,
            size,
            bounds,
            x_dest,
            y_dest,
            x_src,
            y_src,
            cx_src,
            cy_src,
            off_bmi_src,
            cb_bmi_src,
            off_bits_src,
            cb_bits_src,
            usage_src,
            i_start_scan,
            c_scans,
            bmi_src,
            bits_src,
        })
    }
}
