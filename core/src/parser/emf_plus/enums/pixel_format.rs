/// The PixelFormat enumeration defines pixel formats that are supported
/// in EMF+ bitmaps (MS-EMFPLUS 2.1.1.25).
///
/// Pixel formats are specified by EmfPlusBitmap objects. They are
/// encoded as follows:
///
/// - Bits 0-7: Enumeration of the pixel format constants, starting at zero.
/// - Bits 8-15: The total number of bits per pixel.
/// - Bit 16: If set, the color value is indexed into a palette.
/// - Bit 17: If set, the color value is in a GDI-supported format.
/// - Bit 18: If set, the color value has an alpha component.
/// - Bit 19: If set, the color value has a premultiplied alpha component.
/// - Bit 20: If set, extended colors, 16 bits per channel, are supported.
/// - Bits 21-31: Reserved.
///
/// The accessor methods below decompose these bit fields.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    strum::FromRepr,
    strum::EnumIter,
)]
#[repr(u32)]
pub enum PixelFormat {
    /// The format is not specified.
    PixelFormatUndefined = 0x00000000,
    /// 1 bit per pixel, indexed.
    PixelFormat1bppIndexed = 0x00030101,
    /// 4 bits per pixel, indexed.
    PixelFormat4bppIndexed = 0x00030402,
    /// 8 bits per pixel, indexed.
    PixelFormat8bppIndexed = 0x00030803,
    /// 16 bits per pixel, grayscale.
    PixelFormat16bppGrayScale = 0x00101004,
    /// 16 bits per pixel, RGB 5-5-5.
    PixelFormat16bppRGB555 = 0x00021005,
    /// 16 bits per pixel, RGB 5-6-5.
    PixelFormat16bppRGB565 = 0x00021006,
    /// 16 bits per pixel, ARGB 1-5-5-5.
    PixelFormat16bppARGB1555 = 0x00061007,
    /// 24 bits per pixel, RGB 8-8-8.
    PixelFormat24bppRGB = 0x00021808,
    /// 32 bits per pixel, RGB 8-8-8 with the high byte unused.
    PixelFormat32bppRGB = 0x00022009,
    /// 32 bits per pixel, ARGB 8-8-8-8.
    PixelFormat32bppARGB = 0x0026200A,
    /// 32 bits per pixel, premultiplied ARGB 8-8-8-8.
    PixelFormat32bppPARGB = 0x000E200B,
    /// 48 bits per pixel, RGB 16-16-16.
    PixelFormat48bppRGB = 0x0010300C,
    /// 64 bits per pixel, ARGB 16-16-16-16.
    PixelFormat64bppARGB = 0x0034400D,
    /// 64 bits per pixel, premultiplied ARGB 16-16-16-16.
    PixelFormat64bppPARGB = 0x001A400E,
}

crate::parser::enums::impl_parser!(PixelFormat, u32);

impl PixelFormat {
    /// Bits per pixel (bits 8-15 of the raw value).
    pub fn bits_per_pixel(self) -> u32 {
        ((self as u32) >> 8) & 0xFF
    }

    /// Whether the pixel values are indexes into a palette.
    pub fn is_indexed(self) -> bool {
        (self as u32) & 0x0001_0000 != 0
    }

    /// Whether the pixel format includes an alpha component.
    pub fn has_alpha(self) -> bool {
        (self as u32) & 0x0004_0000 != 0
    }

    /// Whether each color component is premultiplied by the alpha
    /// component.
    pub fn is_premultiplied(self) -> bool {
        (self as u32) & 0x0008_0000 != 0
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn roundtrip_all_variants() {
        for v in PixelFormat::iter() {
            assert_eq!(PixelFormat::from_repr(v as u32), Some(v));
        }
    }

    #[test]
    fn decomposes_bit_fields() {
        let v = PixelFormat::PixelFormat32bppPARGB;
        assert_eq!(v.bits_per_pixel(), 32);
        assert!(!v.is_indexed());
        assert!(v.has_alpha());
        assert!(v.is_premultiplied());

        let v = PixelFormat::PixelFormat8bppIndexed;
        assert_eq!(v.bits_per_pixel(), 8);
        assert!(v.is_indexed());
        assert!(!v.has_alpha());
    }
}
