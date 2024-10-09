/// The ColorAdjustment object defines values for adjusting the colors in source
/// bitmaps in bit-block transfers.
///
/// Windows 98 and Windows Millennium Edition do not support the ColorAdjustment
/// object.
///
/// The ColorAdjustment object is used in bit-block transfers performed by
/// EMR_STRETCHBLT and EMR_STRETCHDIBITS records when the StretchMode
/// enumeration value is STRETCH_HALFTONE. The color adjustment values can apply
/// a color filter or lighten or darken an image.
///
/// An EMR_SETCOLORADJUSTMENT record sets the current ColorAdjustment object in
/// the playback device context. That ColorAdjustment object affects all
/// subsequent EMR_STRETCHBLT and EMR_STRETCHDIBITS records until a different
/// ColorAdjustment object is specified by another EMR_SETCOLORADJUSTMENT
/// record, or until the object is removed by a EMR_DELETEOBJECT record.
#[derive(Clone, Debug)]
pub struct ColorAdjustment {
    /// Size (2 bytes): An unsigned integer that specifies the size in bytes of
    /// this object. This value is 0x0018.
    pub size: u16,
    /// Values (2 bytes): An unsigned integer that specifies how to prepare the
    /// output image. This field can be set to NULL or to any combination of
    /// values in the ColorAdjustment enumeration.
    pub values: Vec<crate::parser::enums::ColorAdjustment>,
    /// IlluminantIndex (2 bytes): An unsigned integer that specifies the type
    /// of standard light source under which the image is viewed, from the
    /// Illuminant enumeration.
    pub illuminant_index: crate::parser::Illuminant,
    /// RedGamma (2 bytes): An unsigned integer that specifies the nth power
    /// gamma correction value for the red primary of the source colors. This
    /// value SHOULD be in the range from 2,500 to 65,000.(Windows can generate
    /// ColorAdjustment objects with values outside their valid ranges. Such
    /// objects are ignored.) A value of 10,000 means gamma correction MUST NOT
    /// be performed.
    pub red_gamma: crate::parser::Gamma,
    /// GreenGamma (2 bytes): An unsigned integer that specifies the nth power
    /// gamma correction value for the green primary of the source colors. This
    /// value SHOULD be in the range from 2,500 to 65,000. A value of 10,000
    /// means gamma correction MUST NOT be performed.
    pub green_gamma: crate::parser::Gamma,
    /// BlueGamma (2 bytes): An unsigned integer that specifies the nth power
    /// gamma correction value for the blue primary of the source colors. This
    /// value SHOULD be in the range from 2,500 to 65,000. A value of 10,000
    /// means gamma correction MUST NOT be performed.
    pub blue_gamma: crate::parser::Gamma,
    /// ReferenceBlack (2 bytes): An unsigned integer that specifies the black
    /// reference for the source colors. Any colors that are darker than this
    /// are treated as black. This value SHOULD be in the range from zero to
    /// 4,000.
    pub reference_black: u16,
    /// ReferenceWhite (2 bytes): An unsigned integer that specifies the white
    /// reference for the source colors. Any colors that are lighter than this
    /// are treated as white. This value SHOULD be in the range from 6,000 to
    /// 10,000.
    pub reference_white: u16,
    /// Contrast (2 bytes): A signed integer that specifies the amount of
    /// contrast to be applied to the source object. This value SHOULD be in
    /// the range from –100 to 100. A value of zero means contrast adjustment
    /// MUST NOT be performed.
    pub contrast: crate::parser::Adjustment,
    /// Brightness (2 bytes): A signed integer that specifies the amount of
    /// brightness to be applied to the source object. This value SHOULD be in
    /// the range from –100 to 100. A value of zero means brightness adjustment
    /// MUST NOT be performed.
    pub brightness: crate::parser::Adjustment,
    /// Colorfulness (2 bytes): A signed integer that specifies the amount of
    /// colorfulness to be applied to the source object. This value SHOULD be
    /// in the range from –100 to 100. A value of zero means colorfulness
    /// adjustment MUST NOT be performed.
    pub colorfulness: crate::parser::Adjustment,
    /// RedGreenTint (2 bytes): A signed integer that specifies the amount of
    /// red or green tint adjustment to be applied to the source object. This
    /// value SHOULD be in the range from –100 to 100. Positive numbers adjust
    /// towards red and negative numbers adjust towards green. A value of zero
    /// means tint adjustment MUST NOT be performed.
    pub red_green_tint: crate::parser::Adjustment,
}

impl ColorAdjustment {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use strum::IntoEnumIterator;

        let (size, size_bytes) = crate::parser::read_u16_from_le_bytes(buf)?;
        if size != 0x0018 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "size field in ColorAdjustment must be `0x0018`, but \
                     parsed value is {size:#06X}"
                ),
            });
        }

        let (values, values_bytes) = {
            let (v, values_bytes) = crate::parser::read_u16_from_le_bytes(buf)?;

            (
                crate::parser::enums::ColorAdjustment::iter()
                    .filter(|c| v & (*c as u16) == (*c as u16))
                    .collect(),
                values_bytes,
            )
        };

        let (
            (illuminant_index, illuminant_index_bytes),
            (red_gamma, red_gamma_bytes),
            (green_gamma, green_gamma_bytes),
            (blue_gamma, blue_gamma_bytes),
            (reference_black, reference_black_bytes),
            (reference_white, reference_white_bytes),
            (contrast, contrast_bytes),
            (brightness, brightness_bytes),
            (colorfulness, colorfulness_bytes),
            (red_green_tint, red_green_tint_bytes),
        ) = (
            crate::parser::Illuminant::parse(buf)?,
            crate::parser::Gamma::parse(buf)?,
            crate::parser::Gamma::parse(buf)?,
            crate::parser::Gamma::parse(buf)?,
            crate::parser::read_u16_from_le_bytes(buf)?,
            crate::parser::read_u16_from_le_bytes(buf)?,
            crate::parser::Adjustment::parse(buf)?,
            crate::parser::Adjustment::parse(buf)?,
            crate::parser::Adjustment::parse(buf)?,
            crate::parser::Adjustment::parse(buf)?,
        );

        if reference_black > 4_000 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "reference_black field in ColorAdjustment must be range \
                     in zero to `4,000`, but parsed value is \
                     {reference_black:#06X}"
                ),
            });
        }

        if reference_white < 6_000 || 10_000 < reference_white {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "reference_white field in ColorAdjustment must be range \
                     in `6,000` to `10,000`, but parsed value is \
                     {reference_white:#06X}"
                ),
            });
        }

        Ok((
            Self {
                size,
                values,
                illuminant_index,
                red_gamma,
                green_gamma,
                blue_gamma,
                reference_black,
                reference_white,
                contrast,
                brightness,
                colorfulness,
                red_green_tint,
            },
            size_bytes
                + values_bytes
                + illuminant_index_bytes
                + red_gamma_bytes
                + green_gamma_bytes
                + blue_gamma_bytes
                + reference_black_bytes
                + reference_white_bytes
                + contrast_bytes
                + brightness_bytes
                + colorfulness_bytes
                + red_green_tint_bytes,
        ))
    }
}
