use crate::imports::*;

/// The EMR_PLGBLT record specifies a block transfer of pixels from a source
/// bitmap to a destination parallelogram, with the application of a color mask
/// bitmap.
///
/// The mask bitmap MUST be monochrome; that is, each pixel value MUST be zero
/// or one. A pixel value of one in the mask indicates that the color of the
/// corresponding pixel in the source bitmap SHOULD be copied to the
/// destination. A value of zero in the mask indicates that the destination
/// pixel color SHOULD NOT be changed. If the mask rectangle is smaller than the
/// source and destination rectangles, the mask pattern MUST be replicated as
/// necessary.
#[derive(Clone, Debug)]
pub struct EMR_PLGBLT {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_PLGBLT. This value is 0x0000004F.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// Bounds (16 bytes): A RectL object ([MS-WMF] section 2.2.2.19) that
    /// specifies the destination bounding rectangle in logical coordinates. If
    /// the intersection of this rectangle with the current clipping regions in
    /// the playback device context is empty, this record has no effect.
    pub bounds: wmf_core::parser::RectL,
    /// aptlDest (24 bytes): An array of three PointL objects ([MS-WMF] section
    /// 2.2.2.15) that specifies three corners a parallelogram destination area
    /// for the block transfer.
    ///
    /// The upper-left corner of the source rectangle is mapped to the first
    /// point in this array, the upper-right corner to the second point, and
    /// the lower-left corner to the third point. The lower-right corner of the
    /// source rectangle is mapped to the implicit fourth point in the
    /// parallelogram, which is computed from the first three points (A, B, and
    /// C) by treating them as vectors.
    ///
    /// ```
    /// D = B + C A
    /// ```
    pub aptl_dest: [wmf_core::parser::PointL; 3],
    /// xSrc (4 bytes): A signed integer that specifies the logical
    /// x-coordinate of the upper-left corner of the source rectangle.
    pub x_src: i32,
    /// ySrc (4 bytes): A signed integer that specifies the logical
    /// y-coordinate of the upper-left corner of the source rectangle.
    pub y_src: i32,
    /// cxSrc (4 bytes): A signed integer that specifies the logical width of
    /// the source rectangle.
    pub cx_src: i32,
    /// cySrc (4 bytes): A signed integer that specifies the logical height of
    /// the source rectangle.
    pub cy_src: i32,
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
    /// bytes, of the source bitmap.
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
    /// bytes, from the start of this record to the header of the mask bitmap
    /// in the BitmapBuffer field.
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
    /// of the EMR_PLGBLT record or with each other. Thus, fields in this
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

impl EMR_PLGBLT {
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
            consume_remaining_bytes, discard_bytes_field, read_bytes_field,
            read_field, read_with,
        };

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_PLGBLT as u32,
        )?;

        let bounds = read_with(buf, &mut size, wmf_core::parser::RectL::parse)?;
        let aptl_dest = {
            let p1 =
                read_with(buf, &mut size, wmf_core::parser::PointL::parse)?;
            let p2 =
                read_with(buf, &mut size, wmf_core::parser::PointL::parse)?;
            let p3 =
                read_with(buf, &mut size, wmf_core::parser::PointL::parse)?;
            [p1, p2, p3]
        };
        let x_src = read_field(buf, &mut size)?;
        let y_src = read_field(buf, &mut size)?;
        let cx_src = read_field(buf, &mut size)?;
        let cy_src = read_field(buf, &mut size)?;
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
        let x_mask = read_field(buf, &mut size)?;
        let y_mask = read_field(buf, &mut size)?;
        let usage_mask =
            read_with(buf, &mut size, crate::parser::DIBColors::parse)?;
        let off_bmi_mask = read_field(buf, &mut size)?;
        let cb_bmi_mask: u32 = read_field(buf, &mut size)?;
        let off_bits_mask = read_field(buf, &mut size)?;
        let cb_bits_mask: u32 = read_field(buf, &mut size)?;

        // Defense in depth: reject byte-count fields that exceed the
        // record-size cap before they reach `read_bytes_field`'s
        // `Vec::with_capacity`.
        for (name, n) in [
            ("cb_bmi_src", cb_bmi_src),
            ("cb_bits_src", cb_bits_src),
            ("cb_bmi_mask", cb_bmi_mask),
            ("cb_bits_mask", cb_bits_mask),
        ] {
            crate::parser::ParseError::expect_le(
                name,
                n,
                crate::parser::MAX_RECORD_BYTES,
            )?;
        }

        let undef_offset = size.checked_offset(off_bmi_src)?;
        discard_bytes_field(buf, &mut size, undef_offset)?;
        let bmi_src = read_bytes_field(buf, &mut size, cb_bmi_src as usize)?;

        let undef_offset = size.checked_offset(off_bits_src)?;
        discard_bytes_field(buf, &mut size, undef_offset)?;
        let bits_src = read_bytes_field(buf, &mut size, cb_bits_src as usize)?;

        let undef_offset = size.checked_offset(off_bmi_mask)?;
        discard_bytes_field(buf, &mut size, undef_offset)?;
        let bmi_mask = read_bytes_field(buf, &mut size, cb_bmi_mask as usize)?;

        let undef_offset = size.checked_offset(off_bits_mask)?;
        discard_bytes_field(buf, &mut size, undef_offset)?;
        let bits_mask =
            read_bytes_field(buf, &mut size, cb_bits_mask as usize)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self {
            record_type,
            size,
            bounds,
            aptl_dest,
            x_src,
            y_src,
            cx_src,
            cy_src,
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
