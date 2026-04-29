use crate::imports::*;

/// The EMR_BITBLT record specifies a block transfer of pixels from a source
/// bitmap to a destination rectangle, optionally in combination with a brush
/// pattern, according to a specified raster operation.
#[derive(Clone, Debug)]
pub struct EMR_BITBLT {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_BITBLT. This value is 0x0000004C.
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
    /// cxDest (4 bytes): A signed integer that specifies the logical width of
    /// the source and destination rectangles.
    pub cx_dest: i32,
    /// cyDest (4 bytes): A signed integer that specifies the logical height of
    /// the source and destination rectangles.
    pub cy_dest: i32,
    /// BitBltRasterOperation (4 bytes): An unsigned integer that specifies the
    /// raster operation code. This code defines how the color data of the
    /// source rectangle is to be combined with the color data of the
    /// destination rectangle and optionally a brush pattern, to achieve the
    /// final color.
    ///
    /// This value is in the Ternary Raster Operation enumeration ([MS-WMF]
    /// section 2.1.1.31).
    pub bit_blt_raster_operation: wmf_core::parser::TernaryRasterOperation,
    /// xSrc (4 bytes): A signed integer that specifies the logical
    /// x-coordinate of the upper-left corner of the source rectangle.
    pub x_src: i32,
    /// ySrc (4 bytes): A signed integer that specifies the logical
    /// y-coordinate of the upper-left corner of the source rectangle.
    pub y_src: i32,
    /// XformSrc (24 bytes): An XForm object that specifies a world-space to
    /// page- space transform to apply to the source bitmap.
    pub x_form_src: crate::parser::XForm,
    /// BkColorSrc (4 bytes): A ColorRef object ([MS-WMF] section 2.2.2.8) that
    /// specifies the background color of the source bitmap.
    pub bk_color_src: wmf_core::parser::ColorRef,
    /// UsageSrc (4 bytes): An unsigned integer that specifies how to interpret
    /// values in the color table in the source bitmap header. This value is in
    /// the DIBColors enumeration.
    pub usage_src: crate::parser::DIBColors,
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
    /// BitmapBuffer (variable): A buffer containing the source bitmap, which
    /// is not required to be contiguous with the fixed portion of the
    /// EMR_BITBLT record. Thus, fields in this buffer that are labeled
    /// "UndefinedSpace" are optional and MUST be ignored.
    ///
    /// If the raster operation specified by BitBltRasterOperation does not
    /// require a source bitmap, the source bitmap can be omitted.
    ///
    /// BmiSrc (variable): The source bitmap header.
    pub bmi_src: Vec<u8>,
    /// BitsSrc (variable): The source bitmap bits.
    pub bits_src: Vec<u8>,
}

impl EMR_BITBLT {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = ?record_type),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{
            consume_remaining_bytes, discard_bytes_field, read_bytes_field,
            read_field, read_with,
        };

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_BITBLT as u32,
        )?;

        let bounds = read_with(buf, &mut size, wmf_core::parser::RectL::parse)?;
        let x_dest = read_field(buf, &mut size)?;
        let y_dest = read_field(buf, &mut size)?;
        let cx_dest = read_field(buf, &mut size)?;
        let cy_dest = read_field(buf, &mut size)?;
        let bit_blt_raster_operation = read_with(
            buf,
            &mut size,
            wmf_core::parser::TernaryRasterOperation::parse,
        )?;
        let x_src = read_field(buf, &mut size)?;
        let y_src = read_field(buf, &mut size)?;
        let x_form_src =
            read_with(buf, &mut size, crate::parser::XForm::parse)?;
        let bk_color_src =
            read_with(buf, &mut size, wmf_core::parser::ColorRef::parse)?;
        let usage_src =
            read_with(buf, &mut size, crate::parser::DIBColors::parse)?;
        let off_bmi_src: u32 = read_field(buf, &mut size)?;
        let cb_bmi_src: u32 = read_field(buf, &mut size)?;
        let off_bits_src: u32 = read_field(buf, &mut size)?;
        let cb_bits_src: u32 = read_field(buf, &mut size)?;

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

        let bmi_src = if off_bmi_src > 0 && cb_bmi_src > 0 {
            let undef_offset = size.checked_offset(off_bmi_src)?;
            discard_bytes_field(buf, &mut size, undef_offset)?;
            read_bytes_field(buf, &mut size, cb_bmi_src as usize)?
        } else {
            vec![]
        };

        let bits_src = if off_bits_src > 0 && cb_bits_src > 0 {
            let undef_offset = size.checked_offset(off_bits_src)?;
            discard_bytes_field(buf, &mut size, undef_offset)?;
            read_bytes_field(buf, &mut size, cb_bits_src as usize)?
        } else {
            vec![]
        };

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self {
            record_type,
            size,
            bounds,
            x_dest,
            y_dest,
            cx_dest,
            cy_dest,
            bit_blt_raster_operation,
            x_src,
            y_src,
            x_form_src,
            bk_color_src,
            usage_src,
            off_bmi_src,
            cb_bmi_src,
            off_bits_src,
            cb_bits_src,
            bmi_src,
            bits_src,
        })
    }
}
