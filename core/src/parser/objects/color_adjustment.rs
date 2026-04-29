use crate::imports::*;

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
    pub values: BTreeSet<crate::parser::enums::ColorAdjustmentEnum>,
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
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use strum::IntoEnumIterator;

        use crate::parser::records::{read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let size = read_field(buf, &mut consumed_bytes)?;

        crate::parser::ParseError::expect_eq(
            "size (ColorAdjustment)",
            size,
            0x0018_u16,
        )?;

        let values = {
            let v: u16 = read_field(buf, &mut consumed_bytes)?;

            crate::parser::enums::ColorAdjustmentEnum::iter()
                .filter(|c| v & (*c as u16) == (*c as u16))
                .collect()
        };

        let illuminant_index = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::Illuminant::parse,
        )?;
        let red_gamma =
            read_with(buf, &mut consumed_bytes, crate::parser::Gamma::parse)?;
        let green_gamma =
            read_with(buf, &mut consumed_bytes, crate::parser::Gamma::parse)?;
        let blue_gamma =
            read_with(buf, &mut consumed_bytes, crate::parser::Gamma::parse)?;
        let reference_black = read_field(buf, &mut consumed_bytes)?;
        let reference_white = read_field(buf, &mut consumed_bytes)?;
        let contrast = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::Adjustment::parse,
        )?;
        let brightness = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::Adjustment::parse,
        )?;
        let colorfulness = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::Adjustment::parse,
        )?;
        let red_green_tint = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::Adjustment::parse,
        )?;

        crate::parser::ParseError::expect_le(
            "reference_black (ColorAdjustment)",
            reference_black,
            4_000_u16,
        )?;
        crate::parser::ParseError::expect_in_range(
            "reference_white (ColorAdjustment)",
            reference_white,
            6_000_u16,
            10_000_u16,
        )?;

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
            consumed_bytes,
        ))
    }
}

impl Default for ColorAdjustment {
    fn default() -> Self {
        Self {
            size: 0x0018,
            values: BTreeSet::new(),
            illuminant_index:
                crate::parser::Illuminant::ILLUMINANT_DEVICE_DEFAULT,
            red_gamma: crate::parser::Gamma::default(),
            green_gamma: crate::parser::Gamma::default(),
            blue_gamma: crate::parser::Gamma::default(),
            reference_black: 0,
            reference_white: 10_000,
            contrast: crate::parser::Adjustment::default(),
            brightness: crate::parser::Adjustment::default(),
            colorfulness: crate::parser::Adjustment::default(),
            red_green_tint: crate::parser::Adjustment::default(),
        }
    }
}
