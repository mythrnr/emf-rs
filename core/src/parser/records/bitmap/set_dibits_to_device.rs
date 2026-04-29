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
        use crate::parser::records::{
            consume_remaining_bytes, read_bytes_field, read_field, read_with,
        };

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_SETDIBITSTODEVICE as u32,
        )?;

        let bounds = read_with(buf, &mut size, wmf_core::parser::RectL::parse)?;
        let x_dest = read_field(buf, &mut size)?;
        let y_dest = read_field(buf, &mut size)?;
        let x_src = read_field(buf, &mut size)?;
        let y_src = read_field(buf, &mut size)?;
        let cx_src = read_field(buf, &mut size)?;
        let cy_src = read_field(buf, &mut size)?;
        let off_bmi_src = read_field(buf, &mut size)?;
        let cb_bmi_src: u32 = read_field(buf, &mut size)?;
        let off_bits_src = read_field(buf, &mut size)?;
        let cb_bits_src: u32 = read_field(buf, &mut size)?;
        let usage_src =
            read_with(buf, &mut size, crate::parser::DIBColors::parse)?;
        let i_start_scan = read_field(buf, &mut size)?;
        let c_scans = read_field(buf, &mut size)?;

        // Defense in depth: reject byte-count fields that exceed the
        // record-size cap before they reach `read_bytes_field`'s
        // `Vec::with_capacity`.
        crate::parser::ParseError::expect_le(
            "cb_bmi_src",
            cb_bmi_src,
            crate::parser::MAX_RECORD_BYTES,
        )?;
        crate::parser::ParseError::expect_le(
            "cb_bits_src",
            cb_bits_src,
            crate::parser::MAX_RECORD_BYTES,
        )?;

        let undef_offset = size.checked_offset(off_bmi_src)?;
        let _ = read_bytes_field(buf, &mut size, undef_offset)?;
        let bmi_src = read_bytes_field(buf, &mut size, cb_bmi_src as usize)?;

        let undef_offset = size.checked_offset(off_bits_src)?;
        let _ = read_bytes_field(buf, &mut size, undef_offset)?;
        let bits_src = read_bytes_field(buf, &mut size, cb_bits_src as usize)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

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
