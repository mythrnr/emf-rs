//! Image effects parameter objects (MS-EMFPLUS 2.2.3) and the
//! ImageEffects identifier GUIDs (2.1.3.1). These objects travel in
//! the buffer of EmfPlusSerializableObject records.

use crate::imports::*;

/// A GUID in the MS-DTYP packet representation: Data1/Data2/Data3 are
/// little-endian on the wire, Data4 is a byte array.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl Guid {
    pub const fn new(
        data1: u32,
        data2: u16,
        data3: u16,
        data4: [u8; 8],
    ) -> Self {
        Self { data1, data2, data3, data4 }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_array_field, read_field};

        let mut consumed_bytes: usize = 0;
        let data1 = read_field(buf, &mut consumed_bytes)?;
        let data2 = read_field(buf, &mut consumed_bytes)?;
        let data3 = read_field(buf, &mut consumed_bytes)?;
        let data4 = read_array_field::<R, 8>(buf, &mut consumed_bytes)?;

        Ok((Self { data1, data2, data3, data4 }, consumed_bytes))
    }
}

impl core::fmt::Debug for Guid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:\
             02X}{:02X}}}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7],
        )
    }
}

/// BlurEffectGuid {633C80A4-1843-482B-9EF2-BE2834C5FDD4}: The blur
/// effect (MS-EMFPLUS 2.1.3.1).
pub const BLUR_EFFECT_GUID: Guid = Guid::new(0x633C80A4, 0x1843, 0x482B, [
    0x9E, 0xF2, 0xBE, 0x28, 0x34, 0xC5, 0xFD, 0xD4,
]);
/// BrightnessContrastEffectGuid {D3A1DBE1-8EC4-4C17-9F4C-EA97AD1C343D}:
/// The brightness contrast effect (MS-EMFPLUS 2.1.3.1).
pub const BRIGHTNESS_CONTRAST_EFFECT_GUID: Guid =
    Guid::new(0xD3A1DBE1, 0x8EC4, 0x4C17, [
        0x9F, 0x4C, 0xEA, 0x97, 0xAD, 0x1C, 0x34, 0x3D,
    ]);
/// ColorBalanceEffectGuid {537E597D-251E-48DA-9664-29CA496B70F8}: The
/// color balance effect (MS-EMFPLUS 2.1.3.1).
pub const COLOR_BALANCE_EFFECT_GUID: Guid =
    Guid::new(0x537E597D, 0x251E, 0x48DA, [
        0x96, 0x64, 0x29, 0xCA, 0x49, 0x6B, 0x70, 0xF8,
    ]);
/// ColorCurveEffectGuid {DD6A0022-58E4-4A67-9D9B-D48EB881A53D}: The
/// color curve effect (MS-EMFPLUS 2.1.3.1).
pub const COLOR_CURVE_EFFECT_GUID: Guid =
    Guid::new(0xDD6A0022, 0x58E4, 0x4A67, [
        0x9D, 0x9B, 0xD4, 0x8E, 0xB8, 0x81, 0xA5, 0x3D,
    ]);
/// ColorLookupTableEffectGuid {A7CE72A9-0F7F-40D7-B3CC-D0C02D5C3212}:
/// The color lookup table effect (MS-EMFPLUS 2.1.3.1).
pub const COLOR_LOOKUP_TABLE_EFFECT_GUID: Guid =
    Guid::new(0xA7CE72A9, 0x0F7F, 0x40D7, [
        0xB3, 0xCC, 0xD0, 0xC0, 0x2D, 0x5C, 0x32, 0x12,
    ]);
/// ColorMatrixEffectGuid {718F2615-7933-40E3-A511-5F68FE14DD74}: The
/// color matrix effect (MS-EMFPLUS 2.1.3.1).
pub const COLOR_MATRIX_EFFECT_GUID: Guid =
    Guid::new(0x718F2615, 0x7933, 0x40E3, [
        0xA5, 0x11, 0x5F, 0x68, 0xFE, 0x14, 0xDD, 0x74,
    ]);
/// HueSaturationLightnessEffectGuid
/// {8B2DD6C3-EB07-4D87-A5F0-7108E26A9C5F}: The hue saturation
/// lightness effect (MS-EMFPLUS 2.1.3.1).
pub const HUE_SATURATION_LIGHTNESS_EFFECT_GUID: Guid =
    Guid::new(0x8B2DD6C3, 0xEB07, 0x4D87, [
        0xA5, 0xF0, 0x71, 0x08, 0xE2, 0x6A, 0x9C, 0x5F,
    ]);
/// LevelsEffectGuid {99C354EC-2A31-4F3A-8C34-17A803B33A25}: The levels
/// effect (MS-EMFPLUS 2.1.3.1).
pub const LEVELS_EFFECT_GUID: Guid = Guid::new(0x99C354EC, 0x2A31, 0x4F3A, [
    0x8C, 0x34, 0x17, 0xA8, 0x03, 0xB3, 0x3A, 0x25,
]);
/// RedEyeCorrectionEffectGuid {74D29D05-69A4-4266-9549-3CC52836B632}:
/// The red-eye correction effect (MS-EMFPLUS 2.1.3.1).
pub const RED_EYE_CORRECTION_EFFECT_GUID: Guid =
    Guid::new(0x74D29D05, 0x69A4, 0x4266, [
        0x95, 0x49, 0x3C, 0xC5, 0x28, 0x36, 0xB6, 0x32,
    ]);
/// SharpenEffectGuid {63CBF3EE-C526-402C-8F71-62C540BF5142}: The
/// sharpen effect (MS-EMFPLUS 2.1.3.1).
pub const SHARPEN_EFFECT_GUID: Guid = Guid::new(0x63CBF3EE, 0xC526, 0x402C, [
    0x8F, 0x71, 0x62, 0xC5, 0x40, 0xBF, 0x51, 0x42,
]);
/// TintEffectGuid {1077AF00-2848-4441-9489-44AD4C2D7A2C}: The tint
/// effect (MS-EMFPLUS 2.1.3.1).
pub const TINT_EFFECT_GUID: Guid = Guid::new(0x1077AF00, 0x2848, 0x4441, [
    0x94, 0x89, 0x44, 0xAD, 0x4C, 0x2D, 0x7A, 0x2C,
]);

/// The BlurEffect object specifies a decrease in the difference in
/// intensity between pixels in an image (MS-EMFPLUS 2.2.3.1).
///
/// Bitmap images are specified by EmfPlusBitmap objects (section
/// 2.2.2.2).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlurEffect {
    /// BlurRadius (4 bytes): A floating-point value that specifies the
    /// blur radius in pixels, which determines the number of pixels
    /// involved in calculating the new value of a given pixel. This
    /// value MUST be in the range 0.0 through 255.0.
    ///
    /// As this value increases, the number of pixels involved in the
    /// calculation increases, and the resulting bitmap SHOULD become
    /// more blurry.
    pub blur_radius: f32,
    /// ExpandEdge (4 bytes): A Boolean value that specifies whether
    /// the bitmap expands by an amount equal to the value of the
    /// BlurRadius to produce soft edges. This value MUST be one of the
    /// following:
    ///
    /// - FALSE (0x00000000): The size of the bitmap MUST NOT change, and its
    ///   soft edges SHOULD be clipped to the size of the BlurRadius.
    /// - TRUE (0x00000001): The size of the bitmap SHOULD expand by an amount
    ///   equal to the BlurRadius to produce soft edges.
    ///
    /// Any nonzero value on the wire is stored as `true`.
    pub expand_edge: bool,
}

/// The BrightnessContrastEffect object specifies an expansion or
/// contraction of the lightest and darkest areas of an image
/// (MS-EMFPLUS 2.2.3.2).
///
/// Bitmap images are specified by EmfPlusBitmap objects (section
/// 2.2.2.2).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BrightnessContrastEffect {
    /// BrightnessLevel (4 bytes): A signed integer that specifies the
    /// brightness level. This value MUST be in the range -255 through
    /// 255, with effects as follows:
    ///
    /// - -255 <= value < 0: As the value decreases, the brightness of the
    ///   image SHOULD decrease.
    /// - 0: A value of 0 specifies that the brightness MUST NOT change.
    /// - 0 < value <= 255: As the value increases, the brightness of the image
    ///   SHOULD increase.
    pub brightness_level: i32,
    /// ContrastLevel (4 bytes): A signed integer that specifies the
    /// contrast level. This value MUST be in the range -100 through
    /// 100, with effects as follows:
    ///
    /// - -100 <= value < 0: As the value decreases, the contrast of the image
    ///   SHOULD decrease.
    /// - 0: A value of 0 specifies that the contrast MUST NOT change.
    /// - 0 < value <= 100: As the value increases, the contrast of the image
    ///   SHOULD increase.
    pub contrast_level: i32,
}

/// The ColorBalanceEffect object specifies adjustments to the relative
/// amounts of red, green, and blue in an image (MS-EMFPLUS 2.2.3.3).
///
/// Bitmap images are specified by EmfPlusBitmap objects (section
/// 2.2.2.2).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorBalanceEffect {
    /// CyanRed (4 bytes): A signed integer that specifies a change in
    /// the amount of red in the image. This value MUST be in the range
    /// -100 through 100, with effects as follows:
    ///
    /// - -100 <= value < 0: As the value decreases, the amount of red in the
    ///   image SHOULD decrease and the amount of cyan SHOULD increase.
    /// - 0: A value of 0 specifies that the amounts of red and cyan MUST NOT
    ///   change.
    /// - 0 < value <= 100: As the value increases, the amount of red in the
    ///   image SHOULD increase and the amount of cyan SHOULD decrease.
    pub cyan_red: i32,
    /// MagentaGreen (4 bytes): A signed integer that specifies a
    /// change in the amount of green in the image. This value MUST be
    /// in the range -100 through 100, with effects as follows:
    ///
    /// - -100 <= value < 0: As the value decreases, the amount of green in the
    ///   image SHOULD decrease and the amount of magenta SHOULD increase.
    /// - 0: A value of 0 specifies that the amounts of green and magenta MUST
    ///   NOT change.
    /// - 0 < value <= 100: As the value increases, the amount of green in the
    ///   image SHOULD increase and the amount of magenta SHOULD decrease.
    pub magenta_green: i32,
    /// YellowBlue (4 bytes): A signed integer that specifies a change
    /// in the amount of blue in the image. This value MUST be in the
    /// range -100 through 100, with effects as follows:
    ///
    /// - -100 <= value < 0: As the value decreases, the amount of blue in the
    ///   image SHOULD decrease and the amount of yellow SHOULD increase.
    /// - 0: A value of 0 specifies that the amounts of blue and yellow MUST
    ///   NOT change.
    /// - 0 < value <= 100: As the value increases, the amount of blue in the
    ///   image SHOULD increase and the amount of yellow SHOULD decrease.
    pub yellow_blue: i32,
}

/// The ColorCurveEffect object specifies one of eight adjustments to
/// the color curve of an image (MS-EMFPLUS 2.2.3.4).
///
/// Bitmap images are specified by EmfPlusBitmap objects (section
/// 2.2.2.2).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorCurveEffect {
    /// CurveAdjustment (4 bytes): An unsigned integer that specifies
    /// the curve adjustment to apply to the colors in bitmap. This
    /// value is defined in the CurveAdjustments enumeration (section
    /// 2.1.1.7).
    pub curve_adjustment: crate::parser::emf_plus::CurveAdjustments,
    /// CurveChannel (4 bytes): An unsigned integer that specifies the
    /// color channel to which the curve adjustment applies. This value
    /// is defined in the CurveChannel enumeration (section 2.1.1.8).
    pub curve_channel: crate::parser::emf_plus::CurveChannel,
    /// AdjustmentIntensity (4 bytes): A signed integer that specifies
    /// the intensity of the curve adjustment to the color channel
    /// specified by CurveChannel. The ranges of meaningful values for
    /// this field vary according to the CurveAdjustment value: -255
    /// through 255 for exposure and density, -100 through 100 for
    /// contrast, highlight, shadow, and midtone, and 0 through 255 for
    /// white saturation and black saturation. See the specification
    /// for the per-adjustment value tables.
    pub adjustment_intensity: i32,
}

/// The ColorLookupTableEffect object specifies adjustments to the
/// colors in an image (MS-EMFPLUS 2.2.3.5).
///
/// Bitmap images are specified by EmfPlusBitmap objects (section
/// 2.2.2.2).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorLookupTableEffect {
    /// BlueLookupTable (256 bytes): An array of 256 bytes that
    /// specifies the adjustment for the blue color channel.
    pub blue_lookup_table: [u8; 256],
    /// GreenLookupTable (256 bytes): An array of 256 bytes that
    /// specifies the adjustment for the green color channel.
    pub green_lookup_table: [u8; 256],
    /// RedLookupTable (256 bytes): An array of 256 bytes that
    /// specifies the adjustment for the red color channel.
    pub red_lookup_table: [u8; 256],
    /// AlphaLookupTable (256 bytes): An array of 256 bytes that
    /// specifies the adjustment for the alpha color channel.
    pub alpha_lookup_table: [u8; 256],
}

/// The ColorMatrixEffect object specifies an affine transform to be
/// applied to an image (MS-EMFPLUS 2.2.3.6).
///
/// Bitmap images are specified by EmfPlusBitmap objects (section
/// 2.2.2.2). A color matrix effect is performed by multiplying a color
/// vector by a ColorMatrixEffect object. A 5x5 color matrix can
/// perform a linear transform, including reflection, rotation,
/// shearing, or scaling followed by a translation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorMatrixEffect {
    /// Matrix (100 bytes): a 5x5 color transform matrix in row-major
    /// order. The wire layout consists of five 20-byte fields:
    ///
    /// - Matrix_N_0 (20 bytes): Matrix\[N\]\[0\] of the 5x5 color matrix. This
    ///   row is used for transforms.
    /// - Matrix_N_1 (20 bytes): Matrix\[N\]\[1\] of the 5x5 color matrix. This
    ///   row is used for transforms.
    /// - Matrix_N_2 (20 bytes): Matrix\[N\]\[2\] of the 5x5 color matrix. This
    ///   row is used for transforms.
    /// - Matrix_N_3 (20 bytes): Matrix\[N\]\[3\] of the 5x5 color matrix. This
    ///   row is used for transforms.
    /// - Matrix_N_4 (20 bytes): Matrix\[N\]\[4\] of the 5x5 color matrix. This
    ///   row is used for color translations.
    ///
    /// Notable elements: Matrix_0_0 is the factor for the color red;
    /// Matrix_1_1, the factor for the color green; Matrix_2_2, the
    /// factor for the color blue; Matrix_3_3, the factor for the alpha
    /// (transparency) value. Matrix_4_0 through Matrix_4_3 MUST be
    /// 0.0, and Matrix_4_4 SHOULD be 1.0.
    ///
    /// `matrix[i][j]` holds the j-th element of the i-th 20-byte wire
    /// group, so it corresponds to Matrix_j_i (that is,
    /// Matrix\[j\]\[i\]) in the specification.
    pub matrix: [[f32; 5]; 5],
}

/// The HueSaturationLightnessEffect object specifies adjustments to
/// the hue, saturation, and lightness of an image
/// (MS-EMFPLUS 2.2.3.7).
///
/// Bitmap images are specified by EmfPlusBitmap objects (section
/// 2.2.2.2).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HueSaturationLightnessEffect {
    /// HueLevel (4 bytes): The adjustment to the hue.
    ///
    /// - -180 <= value < 0: Negative values specify clockwise rotation on the
    ///   color wheel.
    /// - 0: A value of 0 specifies that the hue MUST NOT change.
    /// - 0 < value <= 180: Positive values specify counter-clockwise rotation
    ///   on the color wheel.
    pub hue_level: i32,
    /// SaturationLevel (4 bytes): The adjustment to the saturation.
    ///
    /// - -100 <= value < 0: Negative values specify decreasing saturation.
    /// - 0: A value of 0 specifies that the saturation MUST NOT change.
    /// - 0 < value <= 100: Positive values specify increasing saturation.
    pub saturation_level: i32,
    /// LightnessLevel (4 bytes): The adjustment to the lightness.
    ///
    /// - -100 <= value < 0: Negative values specify decreasing lightness.
    /// - 0: A value of 0 specifies that the lightness MUST NOT change.
    /// - 0 < value <= 100: Positive values specify increasing lightness.
    pub lightness_level: i32,
}

/// The LevelsEffect object specifies adjustments to the highlights,
/// midtones, and shadows of an image (MS-EMFPLUS 2.2.3.8).
///
/// Bitmap images are specified by EmfPlusBitmap objects (section
/// 2.2.2.2).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LevelsEffect {
    /// Highlight (4 bytes): How much to lighten the highlights of an
    /// image. The color channel values at the high end of the
    /// intensity range are altered more than values near the middle or
    /// low ends, which means an image can be lightened without losing
    /// the contrast between the darker portions of the image.
    ///
    /// - 0 <= value < 100: Highlights with a percent of intensity above this
    ///   threshold SHOULD be increased.
    /// - 100: Highlights MUST NOT change.
    pub highlight: i32,
    /// MidTone (4 bytes): How much to lighten or darken the midtones
    /// of an image. Color channel values in the middle of the
    /// intensity range are altered more than values near the high or
    /// low ends, which means an image can be lightened or darkened
    /// without losing the contrast between the darkest and lightest
    /// portions of the image.
    ///
    /// - -100 <= value < 0: Midtones are made darker.
    /// - 0: Midtones MUST NOT change.
    /// - 0 < value <= 100: Midtones are made lighter.
    pub mid_tone: i32,
    /// Shadow (4 bytes): How much to darken the shadows of an image.
    /// Color channel values at the low end of the intensity range are
    /// altered more than values near the middle or high ends, which
    /// means an image can be darkened without losing the contrast
    /// between the lighter portions of the image.
    ///
    /// - 0: Shadows MUST NOT change.
    /// - 0 < value <= 100: Shadows with a percent of intensity below this
    ///   threshold are made darker.
    pub shadow: i32,
}

/// The RedEyeCorrectionEffect object specifies areas of an image to
/// which a red-eye correction is applied (MS-EMFPLUS 2.2.3.9).
///
/// Bitmap images are specified by EmfPlusBitmap objects (section
/// 2.2.2.2).
///
/// The wire format begins with NumberOfAreas (4 bytes): a signed
/// integer that specifies the number of rectangles in the Areas field.
/// It is not stored here; the length of `areas` carries it.
#[derive(Clone, Debug)]
pub struct RedEyeCorrectionEffect {
    /// Areas (variable): An array of NumberOfAreas WMF RectL objects
    /// ([MS-WMF] section 2.2.2.19). Each rectangle specifies an area
    /// of the bitmap image to which the red-eye correction effect
    /// SHOULD be applied.
    pub areas: Vec<wmf_core::parser::RectL>,
}

// Manual implementation because `wmf_core::parser::RectL` does not
// implement `PartialEq`.
impl PartialEq for RedEyeCorrectionEffect {
    fn eq(&self, other: &Self) -> bool {
        self.areas.len() == other.areas.len()
            && self.areas.iter().zip(other.areas.iter()).all(|(a, b)| {
                a.left == b.left
                    && a.top == b.top
                    && a.right == b.right
                    && a.bottom == b.bottom
            })
    }
}

/// The SharpenEffect object specifies an increase in the difference in
/// intensity between pixels in an image (MS-EMFPLUS 2.2.3.10).
///
/// Bitmap images are specified by EmfPlusBitmap objects (section
/// 2.2.2.2).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SharpenEffect {
    /// Radius (4 bytes): A floating-point value that specifies the
    /// sharpening radius in pixels, which determines the number of
    /// pixels involved in calculating the new value of a given pixel.
    ///
    /// As this value increases, the number of pixels involved in the
    /// calculation increases, and the resulting bitmap SHOULD become
    /// sharper.
    pub radius: f32,
    /// Amount (4 bytes): A floating-point value that specifies the
    /// difference in intensity between a given pixel and the
    /// surrounding pixels.
    ///
    /// - 0: Sharpening MUST NOT be performed.
    /// - 0 < value <= 100: As this value increases, the difference in
    ///   intensity between pixels SHOULD increase.
    pub amount: f32,
}

/// The TintEffect object specifies an addition of black or white to a
/// specified hue in an image (MS-EMFPLUS 2.2.3.11).
///
/// Bitmap images are specified by EmfPlusBitmap objects (section
/// 2.2.2.2).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TintEffect {
    /// Hue (4 bytes): A signed integer that specifies the hue to which
    /// the tint effect is applied.
    ///
    /// - -180 <= value < 0: The color at a specified counter-clockwise
    ///   rotation of the color wheel, starting from blue.
    /// - 0: A value of 0 specifies the color blue on the color wheel.
    /// - 0 < value <= 180: The color at a specified clockwise rotation of the
    ///   color wheel, starting from blue.
    pub hue: i32,
    /// Amount (4 bytes): A signed integer that specifies how much the
    /// hue is strengthened or weakened.
    ///
    /// - -100 <= value < 0: Negative values specify how much the hue is
    ///   weakened, which equates to the addition of black.
    /// - 0: A value of 0 specifies that the tint MUST NOT change.
    /// - 0 < value <= 100: Positive values specify how much the hue is
    ///   strengthened, which equates to the addition of white.
    pub amount: i32,
}

/// An image effect parameter block, typed by the ObjectGUID of the
/// enclosing EmfPlusSerializableObject record.
#[derive(Clone, Debug, PartialEq)]
pub enum ImageEffect {
    Blur(BlurEffect),
    BrightnessContrast(BrightnessContrastEffect),
    ColorBalance(ColorBalanceEffect),
    ColorCurve(ColorCurveEffect),
    ColorLookupTable(Box<ColorLookupTableEffect>),
    ColorMatrix(ColorMatrixEffect),
    HueSaturationLightness(HueSaturationLightnessEffect),
    Levels(LevelsEffect),
    RedEyeCorrection(RedEyeCorrectionEffect),
    Sharpen(SharpenEffect),
    Tint(TintEffect),
    /// The GUID does not match any effect defined in MS-EMFPLUS
    /// 2.1.3.1; the buffer is kept as raw bytes.
    Unknown {
        object_guid: Guid,
        buffer: Vec<u8>,
    },
}

impl ImageEffect {
    /// Types the buffer of an EmfPlusSerializableObject record based
    /// on its ObjectGUID.
    pub fn parse(
        object_guid: Guid,
        buffer: &[u8],
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{read_array_field, read_field, read_with};

        let buf = &mut &buffer[..];
        let mut consumed: usize = 0;

        let effect = if object_guid == BLUR_EFFECT_GUID {
            let blur_radius = read_field(buf, &mut consumed)?;
            let expand_edge: u32 = read_field(buf, &mut consumed)?;
            Self::Blur(BlurEffect {
                blur_radius,
                expand_edge: expand_edge != 0,
            })
        } else if object_guid == BRIGHTNESS_CONTRAST_EFFECT_GUID {
            let brightness_level = read_field(buf, &mut consumed)?;
            let contrast_level = read_field(buf, &mut consumed)?;
            Self::BrightnessContrast(BrightnessContrastEffect {
                brightness_level,
                contrast_level,
            })
        } else if object_guid == COLOR_BALANCE_EFFECT_GUID {
            let cyan_red = read_field(buf, &mut consumed)?;
            let magenta_green = read_field(buf, &mut consumed)?;
            let yellow_blue = read_field(buf, &mut consumed)?;
            Self::ColorBalance(ColorBalanceEffect {
                cyan_red,
                magenta_green,
                yellow_blue,
            })
        } else if object_guid == COLOR_CURVE_EFFECT_GUID {
            let curve_adjustment = read_with(
                buf,
                &mut consumed,
                crate::parser::emf_plus::CurveAdjustments::parse,
            )?;
            let curve_channel = read_with(
                buf,
                &mut consumed,
                crate::parser::emf_plus::CurveChannel::parse,
            )?;
            let adjustment_intensity = read_field(buf, &mut consumed)?;
            Self::ColorCurve(ColorCurveEffect {
                curve_adjustment,
                curve_channel,
                adjustment_intensity,
            })
        } else if object_guid == COLOR_LOOKUP_TABLE_EFFECT_GUID {
            let blue_lookup_table = read_array_field(buf, &mut consumed)?;
            let green_lookup_table = read_array_field(buf, &mut consumed)?;
            let red_lookup_table = read_array_field(buf, &mut consumed)?;
            let alpha_lookup_table = read_array_field(buf, &mut consumed)?;
            Self::ColorLookupTable(Box::new(ColorLookupTableEffect {
                blue_lookup_table,
                green_lookup_table,
                red_lookup_table,
                alpha_lookup_table,
            }))
        } else if object_guid == COLOR_MATRIX_EFFECT_GUID {
            let mut matrix = [[0.0_f32; 5]; 5];
            for row in &mut matrix {
                for v in row.iter_mut() {
                    *v = read_field(buf, &mut consumed)?;
                }
            }
            Self::ColorMatrix(ColorMatrixEffect { matrix })
        } else if object_guid == HUE_SATURATION_LIGHTNESS_EFFECT_GUID {
            let hue_level = read_field(buf, &mut consumed)?;
            let saturation_level = read_field(buf, &mut consumed)?;
            let lightness_level = read_field(buf, &mut consumed)?;
            Self::HueSaturationLightness(HueSaturationLightnessEffect {
                hue_level,
                saturation_level,
                lightness_level,
            })
        } else if object_guid == LEVELS_EFFECT_GUID {
            let highlight = read_field(buf, &mut consumed)?;
            let mid_tone = read_field(buf, &mut consumed)?;
            let shadow = read_field(buf, &mut consumed)?;
            Self::Levels(LevelsEffect { highlight, mid_tone, shadow })
        } else if object_guid == RED_EYE_CORRECTION_EFFECT_GUID {
            let number_of_areas: i32 = read_field(buf, &mut consumed)?;
            let count = u32::try_from(number_of_areas).map_err(|_| {
                crate::parser::ParseError::UnexpectedPattern {
                    cause: alloc::format!(
                        "NumberOfAreas is negative: {number_of_areas}",
                    )
                    .into(),
                }
            })?;

            crate::parser::emf_plus::check_element_count(
                "NumberOfAreas",
                count,
            )?;

            let mut areas = vec![];
            for _ in 0..count {
                areas.push(read_with(
                    buf,
                    &mut consumed,
                    wmf_core::parser::RectL::parse,
                )?);
            }
            Self::RedEyeCorrection(RedEyeCorrectionEffect { areas })
        } else if object_guid == SHARPEN_EFFECT_GUID {
            let radius = read_field(buf, &mut consumed)?;
            let amount = read_field(buf, &mut consumed)?;
            Self::Sharpen(SharpenEffect { radius, amount })
        } else if object_guid == TINT_EFFECT_GUID {
            let hue = read_field(buf, &mut consumed)?;
            let amount = read_field(buf, &mut consumed)?;
            Self::Tint(TintEffect { hue, amount })
        } else {
            Self::Unknown { object_guid, buffer: buffer.to_vec() }
        };

        Ok(effect)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guid_debug_renders_braced_form() {
        assert_eq!(
            alloc::format!("{BLUR_EFFECT_GUID:?}"),
            "{633C80A4-1843-482B-9EF2-BE2834C5FDD4}",
        );
    }

    #[test]
    fn guid_parse_reads_mixed_endian_layout() {
        let bytes = [
            0xA4, 0x80, 0x3C, 0x63, 0x43, 0x18, 0x2B, 0x48, 0x9E, 0xF2, 0xBE,
            0x28, 0x34, 0xC5, 0xFD, 0xD4,
        ];
        let mut buf: &[u8] = &bytes;
        let (guid, consumed) = Guid::parse(&mut buf).unwrap();

        assert_eq!(consumed, 16);
        assert_eq!(guid, BLUR_EFFECT_GUID);
    }

    #[test]
    fn types_a_blur_effect_buffer() {
        let mut buffer = vec![];
        buffer.extend(2.5_f32.to_le_bytes());
        buffer.extend(1_u32.to_le_bytes());

        let effect = ImageEffect::parse(BLUR_EFFECT_GUID, &buffer).unwrap();

        assert_eq!(
            effect,
            ImageEffect::Blur(BlurEffect {
                blur_radius: 2.5,
                expand_edge: true,
            }),
        );
    }

    #[test]
    fn keeps_unknown_guid_buffers_raw() {
        let guid = Guid::new(0, 0, 0, [0; 8]);
        let effect = ImageEffect::parse(guid, &[0xAB, 0xCD]).unwrap();

        assert_eq!(effect, ImageEffect::Unknown {
            object_guid: guid,
            buffer: vec![0xAB, 0xCD],
        },);
    }
}
