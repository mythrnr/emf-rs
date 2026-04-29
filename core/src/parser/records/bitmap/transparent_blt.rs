use crate::imports::*;

/// The EMR_TRANSPARENTBLT record specifies a block transfer of pixels from a
/// source bitmap to a destination rectangle, treating a specified color as
/// transparent, stretching or compressing the output to fit the dimensions of
/// the destination, if necessary.
///
/// Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, and Windows NT 4.0 do not
/// support EMR_TRANSPARENTBLT.
#[derive(Clone, Debug)]
pub struct EMR_TRANSPARENTBLT {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_TRANSPARENTBLT. This value is 0x00000074.
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
    /// the destination rectangle.
    pub cx_dest: i32,
    /// cyDest (4 bytes): A signed integer that specifies the logical height of
    /// the destination rectangle.
    pub cy_dest: i32,
    /// TransparentColor (4 bytes): A ColorRef object ([MS-WMF] section
    /// 2.2.2.8) that specifies the color in the source bitmap to be treated as
    /// transparent.
    pub transparent_color: wmf_core::parser::ColorRef,
    /// xSrc (4 bytes): A signed integer that specifies the logical
    /// x-coordinate of the upper-left corner of the source rectangle.
    pub x_src: i32,
    /// ySrc (4 bytes): A signed integer that specifies the logical
    /// y-coordinate of the upper-left corner of the source rectangle.
    pub y_src: i32,
    /// XformSrc (24 bytes): An XForm object that specifies a world-space to
    /// page- space transform to apply to the source bitmap.
    pub x_form_src: crate::parser::XForm,
    /// BkColorSrc (4 bytes): A ColorRef object that specifies the background
    /// color of the source bitmap.
    pub bk_color_src: wmf_core::parser::ColorRef,
    /// UsageSrc (4 bytes): An unsigned integer that specifies how to interpret
    /// values in the color table in the source bitmap header. This value is in
    /// the DIBColors enumeration.
    pub usage_src: crate::parser::DIBColors,
    /// offBmiSrc (4 bytes): An unsigned integer that specifies the offset in
    /// bytes, from the start of this record to the source bitmap header.
    pub off_bmi_src: u32,
    /// cbBmiSrc (4 bytes): An unsigned integer that specifies the size in
    /// bytes, of the source bitmap header.
    pub cb_bmi_src: u32,
    /// offBitsSrc (4 bytes): An unsigned integer that specifies the offset in
    /// bytes, from the start of this record to the source bitmap bits.
    pub off_bits_src: u32,
    /// cbBitsSrc (4 bytes): An unsigned integer that specifies the size in
    /// bytes, of the source bitmap bits.
    pub cb_bits_src: u32,
    /// cxSrc (4 bytes): A signed integer that specifies the logical width of
    /// the source rectangle.
    pub cx_src: i32,
    /// cySrc (4 bytes): A signed integer that specifies the logical height of
    /// the source rectangle.
    pub cy_src: i32,
    /// BitmapBuffer (variable): A buffer containing the source bitmap, which
    /// is not required to be contiguous with the fixed portion of the
    /// EMR_TRANSPARENTBLT record. Thus, fields in this buffer that are labeled
    /// "UndefinedSpace" are optional and MUST be ignored.
    ///
    /// If the source bitmap color format is 32 bits-per-pixel, only the alpha
    /// transparency value in each pixel SHOULD be copied to the destination.
    /// Windows uses the EMR_ALPHABLEND record to specify a block transfer of a
    /// 32 bits-per-pixel bitmap with alpha transparency.
    ///
    /// BmiSrc (variable): The source bitmap header.
    pub bmi_src: Vec<u8>,
    /// BitsSrc (variable): The source bitmap bits.
    pub bits_src: Vec<u8>,
}

impl EMR_TRANSPARENTBLT {
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
            crate::parser::RecordType::EMR_TRANSPARENTBLT as u32,
        )?;

        let bounds = read_with(buf, &mut size, wmf_core::parser::RectL::parse)?;
        let x_dest = read_field(buf, &mut size)?;
        let y_dest = read_field(buf, &mut size)?;
        let cx_dest = read_field(buf, &mut size)?;
        let cy_dest = read_field(buf, &mut size)?;
        let transparent_color =
            read_with(buf, &mut size, wmf_core::parser::ColorRef::parse)?;
        let x_src = read_field(buf, &mut size)?;
        let y_src = read_field(buf, &mut size)?;
        let x_form_src =
            read_with(buf, &mut size, crate::parser::XForm::parse)?;
        let bk_color_src =
            read_with(buf, &mut size, wmf_core::parser::ColorRef::parse)?;
        let usage_src =
            read_with(buf, &mut size, crate::parser::DIBColors::parse)?;
        let off_bmi_src = read_field(buf, &mut size)?;
        let cb_bmi_src: u32 = read_field(buf, &mut size)?;
        let off_bits_src = read_field(buf, &mut size)?;
        let cb_bits_src: u32 = read_field(buf, &mut size)?;
        let cx_src = read_field(buf, &mut size)?;
        let cy_src = read_field(buf, &mut size)?;

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
            cx_dest,
            cy_dest,
            transparent_color,
            x_src,
            y_src,
            x_form_src,
            bk_color_src,
            usage_src,
            off_bmi_src,
            cb_bmi_src,
            off_bits_src,
            cb_bits_src,
            cx_src,
            cy_src,
            bmi_src,
            bits_src,
        })
    }
}
