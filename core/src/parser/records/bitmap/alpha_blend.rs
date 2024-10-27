use crate::imports::*;

/// The EMR_ALPHABLEND record specifies a block transfer of pixels from a source
/// bitmap to a destination rectangle, including alpha transparency data,
/// according to a specified blending operation.
///
/// Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, and Windows NT 4.0 do not
/// support EMR_ALPHABLEND.
///
/// The following equations show how destination pixels are computed from source
/// pixels using BLENDFUNCTION. In the equations, "dst" refers to the
/// destination bitmap, and "src" refers to the source bitmap. The color and
/// transparency values of the source and destination pixels are denoted by
/// "Red", "Green", "Blue", and "Alpha".
///
/// Case I: The AlphaFormat value is 0, which means the SrcConstantAlpha value
/// MUST be used to blend the source and destination bitmaps, as follows.
///
/// ```
/// dst.Red = src.Red * (SrcConstantAlpha/255.0) +
///     dst.Red * (1.0 - (SrcConstantAlpha/255.0))
/// dst.Green = src.Green * (SrcConstantAlpha/255.0) +
///     dst.Green * (1.0 - (SrcConstantAlpha/255.0))
/// dst.Blue = src.Blue * (SrcConstantAlpha/255.0) +
///     dst.Blue * (1.0 - (SrcConstantAlpha/255.0))
/// ```
///
/// If the destination bitmap has an alpha channel, then it is blended as
/// follows.
///
/// ```
/// dst.Alpha = src.Alpha * (SrcConstantAlpha / 255.0)
///     + dst.Alpha * (1.0 - (SrcConstantAlpha / 255.0))
/// ```
///
/// Note that if SrcConstantAlpha is 0xFF, these equations reduce to a simple
/// source copy to the destination.
///
/// Case II: The AlphaFormat value is AC_SRC_ALPHA, which means the source
/// pixels MUST be premultiplied by SrcConstantAlpha, and then the blend MUST be
/// based on the per-pixel source alpha channel, as follows.
///
/// ```
/// src.Red = src.Red * (SrcConstantAlpha/255.0)
/// src.Green = src.Green * (SrcConstantAlpha/255.0)
/// src.Blue = src.Blue * (SrcConstantAlpha/255.0)
/// dst.Red = src.Red + (1.0 - (src.Alpha/255.0)) * dst.Red
/// dst.Green = src.Green + (1.0 - (src.Alpha/255.0)) * dst.Green
/// dst.Blue = src.Blue + (1.0 - (src.Alpha/255.0)) * dst.Blue
/// ```
///
/// If the destination bitmap has an alpha channel, it is blended as follows.
///
/// ```
/// src.Alpha = src.Alpha * (SrcConstantAlpha)/255.0)
/// dst.Alpha = src.Alpha + (1.0 - (src.Alpha/255.0)) * dst.Alpha
/// ```
///
/// If SrcConstantAlpha is 0xFF, there is in effect no premultiplication of the
/// source values. See section 2.3.1 for more bitmap record types.
#[derive(Clone, Debug)]
pub struct EMR_ALPHABLEND {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_ALPHABLEND. This value is 0x00000072.
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
    /// the destination rectangle. This value MUST be greater than zero.
    pub cx_dest: i32,
    /// cyDest (4 bytes): A signed integer that specifies the logical height of
    /// the destination rectangle. This value MUST be greater than zero.
    pub cy_dest: i32,
    /// BLENDFUNCTION (4 bytes): A structure that specifies the blending
    /// operations for source and destination bitmaps.
    pub blend_function: BlendFunction,
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
    /// bytes from the start of this record to the source bitmap header in the
    /// BitmapBuffer field.
    pub off_bmi_src: u32,
    /// cbBmiSrc (4 bytes): An unsigned integer that specifies the size in
    /// bytes of the source bitmap header.
    pub cb_bmi_src: u32,
    /// offBitsSrc (4 bytes): An unsigned integer that specifies the offset in
    /// bytes from the start of this record to the source bitmap bits in the
    /// BitmapBuffer field.
    pub off_bits_src: u32,
    /// cbBitsSrc (4 bytes): An unsigned integer that specifies the size in
    /// bytes of the source bitmap bits.
    pub cb_bits_src: u32,
    /// cxSrc (4 bytes): A signed integer that specifies the logical width of
    /// the source rectangle. This value MUST be greater than zero.
    pub cx_src: i32,
    /// cySrc (4 bytes): A signed integer that specifies the logical height of
    /// the source rectangle. This value MUST be greater than zero.
    pub cy_src: i32,
    /// BitmapBuffer (variable): A buffer containing the source bitmap, which
    /// is not required to be contiguous with the fixed portion of the
    /// EMR_ALPHABLEND record. Thus, fields in this buffer that are labeled
    /// "UndefinedSpace" are optional and MUST be ignored.
    ///
    /// BmiSrc (variable): The source bitmap header.
    pub bmi_src: Vec<u8>,
    /// BitsSrc (variable): The source bitmap bits.
    pub bits_src: Vec<u8>,
}

impl EMR_ALPHABLEND {
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
        if record_type != crate::parser::RecordType::EMR_ALPHABLEND {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_ALPHABLEND as u32,
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
            (blend_function, blend_function_bytes),
            (x_src, x_src_bytes),
            (y_src, y_src_bytes),
            (x_form_src, x_form_src_bytes),
            (bk_color_src, bk_color_src_bytes),
            (usage_src, usage_src_bytes),
            (off_bmi_src, off_bmi_src_bytes),
            (cb_bmi_src, cb_bmi_src_bytes),
            (off_bits_src, off_bits_src_bytes),
            (cb_bits_src, cb_bits_src_bytes),
            (cx_src, cx_src_bytes),
            (cy_src, cy_src_bytes),
        ) = (
            wmf_core::parser::RectL::parse(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            BlendFunction::parse(buf)?,
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
        );

        size.consume(
            bounds_bytes
                + x_dest_bytes
                + y_dest_bytes
                + cx_dest_bytes
                + cy_dest_bytes
                + blend_function_bytes
                + x_src_bytes
                + y_src_bytes
                + x_form_src_bytes
                + bk_color_src_bytes
                + usage_src_bytes
                + off_bmi_src_bytes
                + cb_bmi_src_bytes
                + off_bits_src_bytes
                + cb_bits_src_bytes
                + cx_src_bytes
                + cy_src_bytes,
        );

        if cx_dest <= 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "cx_dest must be greater than zero, but parsed value is \
                     `{cx_dest:#010X}`",
                ),
            });
        }

        if cy_dest <= 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "cy_dest must be greater than zero, but parsed value is \
                     `{cy_dest:#010X}`",
                ),
            });
        }

        if cx_src <= 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "cx_src must be greater than zero, but parsed value is \
                     `{cx_src:#010X}`",
                ),
            });
        }

        if cy_src <= 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "cy_src must be greater than zero, but parsed value is \
                     `{cy_src:#010X}`",
                ),
            });
        }

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
            cx_dest,
            cy_dest,
            blend_function,
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

#[derive(Clone, Debug)]
pub struct BlendFunction {
    /// BlendOperation (1 byte): The blend operation code. The only source and
    /// destination blend operation that has been defined is 0x00, which
    /// specifies that the source bitmap MUST be combined with the destination
    /// bitmap based on the alpha transparency values of the source pixels. See
    /// the following equations for details.
    pub blend_operation: u8,
    /// BlendFlags (1 byte): This value is 0x00 and MUST be ignored.
    pub blend_flags: u8,
    /// SrcConstantAlpha (1 byte): An unsigned integer that specifies alpha
    /// transparency, which determines the blend of the source and destination
    /// bitmaps. This value MUST be used on the entire source bitmap. The
    /// minimum alpha transparency value, zero, corresponds to completely
    /// transparent; the maximum value, 0xFF, corresponds to completely opaque.
    /// In effect, a value of 0xFF specifies that the per-pixel alpha values
    /// determine the blend of the source and destination bitmaps. See the
    /// equations later in this section for details.
    pub src_constant_alpha: u8,
    /// AlphaFormat (1 byte): A structure that specifies how source and
    /// destination pixels are interpreted with respect to alpha transparency.
    pub alpha_format: u8,
}

impl BlendFunction {
    fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (blend_operation, blend_operation_bytes),
            (blend_flags, blend_flags_bytes),
            (src_constant_alpha, src_constant_alpha_bytes),
            (alpha_format, alpha_format_bytes),
        ) = (
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
        );

        if blend_flags != 0x00 {
            warn!(
                "blend_flags field must be `0x00`, but parsed value is \
                 {blend_flags:#04X}",
            );
        }

        Ok((
            Self {
                blend_operation,
                blend_flags,
                src_constant_alpha,
                alpha_format,
            },
            blend_operation_bytes
                + blend_flags_bytes
                + src_constant_alpha_bytes
                + alpha_format_bytes,
        ))
    }
}
