use crate::imports::*;

/// The EMR_MASKBLT record specifies a block transfer of pixels from a source
/// bitmap to a destination rectangle, optionally in combination with a brush
/// pattern and with the application of a color mask bitmap, according to
/// specified foreground and background raster operations.
///
/// The mask bitmap MUST be monochrome; that is, each pixel value MUST be zero
/// or one. A pixel value of one in the mask indicates that the color of the
/// corresponding pixel in the source bitmap SHOULD be copied to the
/// destination. A value of zero in the mask indicates that the destination
/// pixel color SHOULD NOT be changed. If the mask rectangle is smaller than the
/// source and destination rectangles, the mask pattern MUST be replicated as
/// necessary.
#[derive(Clone, Debug)]
pub struct EMR_MASKBLT {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_MASKBLT. This value is 0x0000004E.
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
    /// ROP4 (4 bytes): A quaternary raster operation, which specifies ternary
    /// raster operations for the foreground and background colors of a bitmap.
    /// These values define how the color data of the source rectangle is to be
    /// combined with the color data of the destination rectangle.
    pub rop4: ROP4,
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
    /// bytes of the source bitmap header.
    pub cb_bmi_src: u32,
    /// offBitsSrc (4 bytes): An unsigned integer that specifies the offset in
    /// bytes, from the start of this record to the source bitmap bits in the
    /// BitmapBuffer field.
    pub off_bits_src: u32,
    /// cbBitsSrc (4 bytes): An unsigned integer that specifies the size in
    /// bytes, of the source bitmap bits.
    pub cb_bits_src: u32,
    /// xMask (4 bytes): A signed integer that specifies the logical
    /// x-coordinate of the upper-left corner of the mask bitmap.
    pub x_mask: i32,
    /// yMask (4 bytes): A signed integer that specifies the logical
    /// y-coordinate of the upper-left corner of the mask bitmap.
    pub y_mask: i32,
    /// UsageMask (4 bytes): An unsigned integer that specifies how to
    /// interpret values in the color table in the mask bitmap header. This
    /// value is in the DIBColors enumeration.
    pub usage_mask: crate::parser::DIBColors,
    /// offBmiMask (4 bytes): An unsigned integer that specifies the offset in
    /// bytes, from the start of this record to the mask bitmap header in the
    /// BitmapBuffer field.
    pub off_bmi_mask: u32,
    /// cbBmiMask (4 bytes): An unsigned integer that specifies the size in
    /// bytes, of the mask bitmap header.
    pub cb_bmi_mask: u32,
    /// offBitsMask (4 bytes): An unsigned integer that specifies the offset in
    /// bytes, from the start of this record to the mask bitmap bits in the
    /// BitmapBuffer field.
    pub off_bits_mask: u32,
    /// cbBitsMask (4 bytes): An unsigned integer that specifies the size in
    /// bytes, of the mask bitmap bits.
    pub cb_bits_mask: u32,
    /// BitmapBuffer (variable): A buffer containing the source and mask
    /// bitmaps, which are not required to be contiguous with the fixed portion
    /// of the EMR_MASKBLT record or with each other. Thus, fields in this
    /// buffer that are labeled "UndefinedSpace" are optional and MUST be
    /// ignored.
    ///
    /// BmiSrc (variable): The source bitmap header.
    pub bmi_src: Vec<u8>,
    /// BitsSrc (variable): The source bitmap bits.
    pub bits_src: Vec<u8>,
    /// BmiMask (variable): The mask bitmap header.
    pub bmi_mask: Vec<u8>,
    /// BitsMask (variable): The mask bitmap bits.
    pub bits_mask: Vec<u8>,
}

impl EMR_MASKBLT {
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
        if record_type != crate::parser::RecordType::EMR_MASKBLT {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_MASKBLT as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (bounds, bounds_bytes),
            (x_dest, x_dest_bytes),
            (y_dest, y_dest_bytes),
            (cx_dest, cx_dest_bytes),
            (cy_dest, cy_dest_bytes),
            (rop4, rop4_bytes),
            (x_src, x_src_bytes),
            (y_src, y_src_bytes),
            (x_form_src, x_form_src_bytes),
            (bk_color_src, bk_color_src_bytes),
            (usage_src, usage_src_bytes),
            (off_bmi_src, off_bmi_src_bytes),
            (cb_bmi_src, cb_bmi_src_bytes),
            (off_bits_src, off_bits_src_bytes),
            (cb_bits_src, cb_bits_src_bytes),
            (x_mask, x_mask_bytes),
            (y_mask, y_mask_bytes),
            (usage_mask, usage_mask_bytes),
            (off_bmi_mask, off_bmi_mask_bytes),
            (cb_bmi_mask, cb_bmi_mask_bytes),
            (off_bits_mask, off_bits_mask_bytes),
            (cb_bits_mask, cb_bits_mask_bytes),
        ) = (
            wmf_core::parser::RectL::parse(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            ROP4::parse(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::XForm::parse(buf)?,
            wmf_core::parser::ColorRef::parse(buf)?,
            crate::parser::DIBColors::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::DIBColors::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        size.consume(
            bounds_bytes
                + x_dest_bytes
                + y_dest_bytes
                + cx_dest_bytes
                + cy_dest_bytes
                + rop4_bytes
                + x_src_bytes
                + y_src_bytes
                + x_form_src_bytes
                + bk_color_src_bytes
                + usage_src_bytes
                + off_bmi_src_bytes
                + cb_bmi_src_bytes
                + off_bits_src_bytes
                + cb_bits_src_bytes
                + x_mask_bytes
                + y_mask_bytes
                + usage_mask_bytes
                + off_bmi_mask_bytes
                + cb_bmi_mask_bytes
                + off_bits_mask_bytes
                + cb_bits_mask_bytes,
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

        let ((_, undef_space_bytes), (bmi_mask, bmi_mask_bytes)) = (
            crate::parser::read_variable(
                buf,
                off_bmi_mask as usize - size.consumed_bytes(),
            )?,
            crate::parser::read_variable(buf, cb_bmi_mask as usize)?,
        );

        size.consume(undef_space_bytes + bmi_mask_bytes);

        let ((_, undef_space_bytes), (bits_mask, bits_mask_bytes)) = (
            crate::parser::read_variable(
                buf,
                off_bits_mask as usize - size.consumed_bytes(),
            )?,
            crate::parser::read_variable(buf, cb_bits_mask as usize)?,
        );

        size.consume(undef_space_bytes + bits_mask_bytes);

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
            cx_dest,
            cy_dest,
            rop4,
            x_src,
            y_src,
            x_form_src,
            bk_color_src,
            usage_src,
            off_bmi_src,
            cb_bmi_src,
            off_bits_src,
            cb_bits_src,
            x_mask,
            y_mask,
            usage_mask,
            off_bmi_mask,
            cb_bmi_mask,
            off_bits_mask,
            cb_bits_mask,
            bmi_src,
            bits_src,
            bmi_mask,
            bits_mask,
        })
    }
}

#[derive(Clone, Debug)]
pub struct ROP4 {
    /// Reserved (2 bytes): This field SHOULD be 0x0000 and MUST be ignored.
    pub reserved: [u8; 2],
    /// BackgroundROP3 (1 byte): The unsigned, most-significant 8 bits of a
    /// 24-bit ternary raster operation value from the Ternary Raster Operation
    /// enumeration ([MS-WMF] section 2.1.1.31). This code defines how to
    /// combine the background color data of the source and destination bitmaps
    /// and brush pattern.
    pub background_rop3: u8,
    /// ForegroundROP3 (1 byte): The unsigned, most-significant 8 bits of a
    /// 24-bit ternary raster operation value from the Ternary Raster Operation
    /// enumeration. This code defines how to combine the foreground color data
    /// of the source and destination bitmaps and brush pattern.
    pub foreground_rop3: u8,
}

impl ROP4 {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (reserved, reserved_bytes),
            (background_rop3, background_rop3_bytes),
            (foreground_rop3, foreground_rop3_bytes),
        ) = (
            crate::parser::read::<_, 2>(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
        );

        Ok((
            Self { reserved, background_rop3, foreground_rop3 },
            reserved_bytes + background_rop3_bytes + foreground_rop3_bytes,
        ))
    }
}
