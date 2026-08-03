use crate::{
    converter::PlayError,
    parser::emf_plus::objects::{
        EmfPlusBitmap, EmfPlusBitmapContent, EmfPlusImageData,
        EmfPlusObjectData,
    },
};

/// An EMF+ bitmap object decoded into BMP bytes, plus the dimensions a
/// player needs to place the image.
///
/// This layer covers the MS-EMFPLUS side of image playback: validating
/// the bitmap fields, normalizing the alpha channel, and encoding the
/// pixels as a BMP file. How the bytes reach the output document (for
/// SVG, a data URL) is the player's concern.
#[derive(Clone, Debug)]
pub(crate) struct DecodedBitmap {
    pub width: i32,
    pub height: i32,
    pub bmp: wmf_core::converter::Bitmap,
}

impl DecodedBitmap {
    /// Decodes a raw (uncompressed) EMF+ bitmap, or `None` when the
    /// pixel format is one this converter cannot render.
    fn from_bitmap(bitmap: EmfPlusBitmap) -> Result<Option<Self>, PlayError> {
        let EmfPlusBitmap { width, height, stride, bitmap_data } = bitmap;
        let EmfPlusBitmapContent::Pixel { pixel_format, palette, pixel_data } =
            bitmap_data
        else {
            info!("compressed EMF+ bitmap objects are not implemented");

            return Ok(None);
        };

        if pixel_format.bits_per_pixel() != 32 || pixel_format.is_indexed() {
            info!(
                bits_per_pixel = pixel_format.bits_per_pixel(),
                ?pixel_format,
                "EMF+ bitmap pixel format is not supported",
            );

            return Ok(None);
        }
        if palette.is_some() {
            return Err(PlayError::invalid_record(
                "32bpp EMF+ bitmap must not carry a palette",
            ));
        }
        if width <= 0 || height <= 0 || stride == 0 {
            return Err(PlayError::invalid_record(
                "invalid EMF+ bitmap dimensions",
            ));
        }

        // The dimensions are validated positive above, so the usize
        // casts are lossless; the i32 originals feed the DIB header
        // below.
        let row_size =
            (width.unsigned_abs() as usize).checked_mul(4).ok_or_else(
                || PlayError::invalid_record("EMF+ bitmap row size overflows"),
            )?;
        let stride_size = stride.unsigned_abs() as usize;
        if stride_size != row_size {
            return Err(PlayError::invalid_record(
                "invalid EMF+ bitmap stride",
            ));
        }

        let pixel_size = stride_size
            .checked_mul(height.unsigned_abs() as usize)
            .ok_or_else(|| {
                PlayError::invalid_record("EMF+ bitmap data size overflows")
            })?;
        // The parsed pixel data runs to the end of the object and can
        // carry up to 3 bytes of trailing alignment padding, so trim it
        // to the exact stride * height extent before use.
        if pixel_data.len() < pixel_size {
            return Err(PlayError::invalid_record(
                "truncated EMF+ bitmap pixel data",
            ));
        }
        let mut pixels = pixel_data;
        pixels.truncate(pixel_size);
        normalize_alpha(
            &mut pixels,
            pixel_format.has_alpha(),
            pixel_format.is_premultiplied(),
        );

        // A positive stride is top-down; DIBs encode top-down rows with
        // a negative height, so flip the sign accordingly.
        let dib_height = if stride > 0 { -height } else { height };
        let image_size = u32::try_from(pixel_size).map_err(|_| {
            PlayError::invalid_record("EMF+ bitmap data exceeds u32")
        })?;
        let zero = wmf_core::parser::CIEXYZ { x: 0, y: 0, z: 0 };
        let dib = wmf_core::parser::DeviceIndependentBitmap {
            dib_header_info: wmf_core::parser::BitmapInfoHeader::V4(
                wmf_core::parser::BitmapInfoHeaderV4 {
                    header_size: 108,
                    width,
                    height: dib_height,
                    planes: 1,
                    bit_count: wmf_core::parser::BitCount::BI_BITCOUNT_6,
                    compression: wmf_core::parser::Compression::BI_BITFIELDS,
                    image_size,
                    x_pels_per_meter: 0,
                    y_pels_per_meter: 0,
                    color_used: 0,
                    color_important: 0,
                    red_mask: 0x00FF_0000,
                    green_mask: 0x0000_FF00,
                    blue_mask: 0x0000_00FF,
                    alpha_mask: 0xFF00_0000,
                    color_space_type:
                        wmf_core::parser::LogicalColorSpace::LCS_sRGB,
                    endpoints: wmf_core::parser::CIEXYZTriple {
                        red: zero.clone(),
                        green: zero.clone(),
                        blue: zero,
                    },
                    gamma_red: 0,
                    gamma_green: 0,
                    gamma_blue: 0,
                },
            ),
            colors: wmf_core::parser::Colors::Null,
            bitmap_buffer: wmf_core::parser::BitmapBuffer { a_data: pixels },
        };

        Ok(Some(Self { width, height, bmp: dib.into() }))
    }

    /// Decodes a bitmap from a completed EMF+ object, or `None` when
    /// the object is not an image this converter can render.
    pub(crate) fn from_object(
        object: EmfPlusObjectData,
    ) -> Result<Option<Self>, PlayError> {
        let EmfPlusObjectData::Image(image) = object else {
            return Ok(None);
        };
        let EmfPlusImageData::Bitmap(bitmap) = image.image_data else {
            info!("EMF+ metafile image objects are not implemented");

            return Ok(None);
        };

        Self::from_bitmap(bitmap)
    }
}

/// Rewrites the pixels in place so the alpha channel is straight
/// (non-premultiplied) and fully populated, which is what the BMP
/// encoding expects.
fn normalize_alpha(pixels: &mut [u8], has_alpha: bool, premultiplied: bool) {
    // Straight alpha is already the target representation, so the most
    // common pixel format skips the pass over the pixels entirely.
    if has_alpha && !premultiplied {
        return;
    }

    if !has_alpha {
        for pixel in pixels.chunks_exact_mut(4) {
            pixel[3] = u8::MAX;
        }

        return;
    }

    for pixel in pixels.chunks_exact_mut(4) {
        let alpha = u16::from(pixel[3]);
        if alpha == 0 {
            continue;
        }

        for component in &mut pixel[..3] {
            let value = u16::from(*component) * u16::from(u8::MAX) / alpha;
            *component = u8::try_from(value.min(u16::from(u8::MAX)))
                .expect("component is clamped to u8");
        }
    }
}

/// Wire-level image object builders shared with the SVG player tests.
#[cfg(test)]
pub(crate) mod test_support {
    use super::*;
    use crate::parser::emf_plus::{PixelFormat, objects::EmfPlusImage};

    pub(crate) fn bitmap_object(
        pixel_format: PixelFormat,
    ) -> EmfPlusObjectData {
        bitmap_object_with(pixel_format, 2, 1, 8, &[
            0, 0, 255, 255, 0, 255, 0, 255,
        ])
    }

    pub(crate) fn bitmap_object_with(
        pixel_format: PixelFormat,
        width: i32,
        height: i32,
        stride: i32,
        pixel_data: &[u8],
    ) -> EmfPlusObjectData {
        let mut data = vec![];
        data.extend(0xDBC0_1002_u32.to_le_bytes()); // version
        data.extend(1_u32.to_le_bytes()); // ImageDataTypeBitmap
        data.extend(width.to_le_bytes());
        data.extend(height.to_le_bytes());
        data.extend(stride.to_le_bytes());
        data.extend((pixel_format as u32).to_le_bytes());
        data.extend(0_u32.to_le_bytes()); // BitmapDataTypePixel
        data.extend(pixel_data);

        let mut buf: &[u8] = &data;
        let (image, _) = EmfPlusImage::parse(&mut buf, data.len()).unwrap();

        EmfPlusObjectData::Image(image)
    }
}

#[cfg(test)]
mod tests {
    use super::{test_support::*, *};
    use crate::parser::emf_plus::PixelFormat;

    /// Reads the Height field of the BMP info header: 14 bytes of file
    /// header, then HeaderSize (4) and Width (4).
    fn bmp_height(bitmap: &DecodedBitmap) -> i32 {
        i32::from_le_bytes(bitmap.bmp.as_slice()[22..26].try_into().unwrap())
    }

    fn bmp_pixels(bitmap: &DecodedBitmap) -> &[u8] {
        let bmp = bitmap.bmp.as_slice();
        // The pixel data offset is stored at byte 10 of the BMP file
        // header.
        let offset =
            u32::from_le_bytes(bmp[10..14].try_into().unwrap()) as usize;

        &bmp[offset..]
    }

    #[test]
    fn normalizes_alpha_per_pixel_format() {
        let cases = [
            ("straight alpha is untouched", true, false, [10, 20, 30, 40], [
                10, 20, 30, 40,
            ]),
            (
                "missing alpha is forced opaque",
                false,
                false,
                [10, 20, 30, 0],
                [10, 20, 30, 255],
            ),
            (
                "premultiplied alpha becomes straight",
                true,
                true,
                [10, 20, 30, 128],
                [19, 39, 59, 128],
            ),
            ("zero alpha keeps the components", true, true, [10, 20, 30, 0], [
                10, 20, 30, 0,
            ]),
        ];

        for (name, has_alpha, premultiplied, input, expected) in cases {
            let mut pixels = input;
            normalize_alpha(&mut pixels, has_alpha, premultiplied);

            assert_eq!(pixels, expected, "{name}");
        }
    }

    #[test]
    fn decodes_only_supported_pixel_formats() {
        let cases = [
            (PixelFormat::PixelFormat32bppARGB, true),
            (PixelFormat::PixelFormat32bppPARGB, true),
            (PixelFormat::PixelFormat32bppRGB, true),
            (PixelFormat::PixelFormat24bppRGB, false),
            (PixelFormat::PixelFormat16bppRGB555, false),
        ];

        for (pixel_format, decoded) in cases {
            let result =
                DecodedBitmap::from_object(bitmap_object(pixel_format))
                    .unwrap();

            assert_eq!(result.is_some(), decoded, "{pixel_format:?}");
        }
    }

    #[test]
    fn decodes_the_dimensions_and_bmp_signature() {
        let bitmap = DecodedBitmap::from_object(bitmap_object(
            PixelFormat::PixelFormat32bppARGB,
        ))
        .unwrap()
        .unwrap();

        assert_eq!((bitmap.width, bitmap.height), (2, 1));
        assert!(bitmap.bmp.as_slice().starts_with(b"BM"));
    }

    #[test]
    fn keeps_the_pixel_bytes_after_the_bmp_header() {
        let pixels = [0, 0, 255, 255, 0, 255, 0, 255];
        let bitmap = DecodedBitmap::from_object(bitmap_object_with(
            PixelFormat::PixelFormat32bppARGB,
            2,
            1,
            8,
            &pixels,
        ))
        .unwrap()
        .unwrap();

        assert_eq!(bmp_pixels(&bitmap), pixels);
    }

    #[test]
    fn rejects_malformed_bitmaps() {
        let cases = [
            ("zero width", 0, 1, 8, 8),
            ("negative width", -2, 1, 8, 8),
            ("zero height", 2, 0, 8, 8),
            ("zero stride", 2, 1, 0, 8),
            ("stride not matching the row size", 2, 1, 12, 12),
            ("truncated pixel data", 2, 1, 8, 4),
        ];

        for (name, width, height, stride, data_len) in cases {
            let object = bitmap_object_with(
                PixelFormat::PixelFormat32bppARGB,
                width,
                height,
                stride,
                &vec![0; data_len],
            );

            assert!(DecodedBitmap::from_object(object).is_err(), "{name}");
        }
    }

    #[test]
    fn ignores_trailing_alignment_padding() {
        let pixels = [0, 0, 255, 255, 0, 255, 0, 255];
        let mut padded = pixels.to_vec();
        padded.extend([0, 0, 0]);

        let exact = DecodedBitmap::from_object(bitmap_object_with(
            PixelFormat::PixelFormat32bppARGB,
            2,
            1,
            8,
            &pixels,
        ))
        .unwrap()
        .unwrap();
        let trimmed = DecodedBitmap::from_object(bitmap_object_with(
            PixelFormat::PixelFormat32bppARGB,
            2,
            1,
            8,
            &padded,
        ))
        .unwrap()
        .unwrap();

        assert_eq!(exact.bmp.as_slice(), trimmed.bmp.as_slice());
    }

    #[test]
    fn encodes_the_stride_direction_in_the_height_sign() {
        let pixels = [0, 0, 255, 255, 0, 255, 0, 255];
        let top_down = DecodedBitmap::from_object(bitmap_object_with(
            PixelFormat::PixelFormat32bppARGB,
            2,
            1,
            8,
            &pixels,
        ))
        .unwrap()
        .unwrap();
        let bottom_up = DecodedBitmap::from_object(bitmap_object_with(
            PixelFormat::PixelFormat32bppARGB,
            2,
            1,
            -8,
            &pixels,
        ))
        .unwrap()
        .unwrap();

        assert_eq!(bmp_height(&top_down), -1);
        assert_eq!(bmp_height(&bottom_up), 1);
    }

    #[test]
    fn encodes_equal_bitmaps_for_normalized_alpha_variants() {
        // The DIB header does not depend on the pixel format, so
        // formats that normalize to the same straight-alpha pixels
        // must produce the same BMP bytes.
        //
        // The unused 4th byte of a no-alpha format is forced opaque.
        let no_alpha = DecodedBitmap::from_object(bitmap_object_with(
            PixelFormat::PixelFormat32bppRGB,
            2,
            1,
            8,
            &[10, 20, 30, 0, 40, 50, 60, 9],
        ))
        .unwrap()
        .unwrap();
        let opaque = DecodedBitmap::from_object(bitmap_object_with(
            PixelFormat::PixelFormat32bppARGB,
            2,
            1,
            8,
            &[10, 20, 30, 255, 40, 50, 60, 255],
        ))
        .unwrap()
        .unwrap();
        // Premultiplied components are divided by alpha 51
        // (255 / 51 = 5).
        let premultiplied = DecodedBitmap::from_object(bitmap_object_with(
            PixelFormat::PixelFormat32bppPARGB,
            2,
            1,
            8,
            &[51, 17, 0, 51, 0, 34, 51, 51],
        ))
        .unwrap()
        .unwrap();
        let straight = DecodedBitmap::from_object(bitmap_object_with(
            PixelFormat::PixelFormat32bppARGB,
            2,
            1,
            8,
            &[255, 85, 0, 51, 0, 170, 255, 51],
        ))
        .unwrap()
        .unwrap();

        assert_eq!(no_alpha.bmp.as_slice(), opaque.bmp.as_slice());
        assert_eq!(premultiplied.bmp.as_slice(), straight.bmp.as_slice());
    }
}
