//! Image objects (MS-EMFPLUS 2.2.1.4 EmfPlusImage, 2.2.2.2
//! EmfPlusBitmap, 2.2.2.3 EmfPlusBitmapData, 2.2.2.10
//! EmfPlusCompressedImage, 2.2.2.27 EmfPlusMetafile).

use crate::{
    imports::*,
    parser::emf_plus::objects::{EmfPlusGraphicsVersion, EmfPlusPalette},
};

/// The EmfPlusImage object specifies a graphics image in the form of a
/// bitmap or metafile (MS-EMFPLUS 2.2.1.4).
///
/// This object is generic and is used to specify different types of
/// image data, including:
///
/// - An EmfPlusBitmap object (section 2.2.2.2); and
/// - An EmfPlusMetafile object (section 2.2.2.27).
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusImage {
    /// Version (4 bytes): An EmfPlusGraphicsVersion object (section
    /// 2.2.2.19) that specifies the version of operating system
    /// graphics that was used to create this object.
    pub version: EmfPlusGraphicsVersion,
    /// ImageData (variable): Variable-length data that defines the
    /// image data specified in the Type field. The content and format
    /// of the data can be different for every image type.
    ///
    /// The Type field of the wire format is implied by the variant.
    pub image_data: EmfPlusImageData,
}

/// The ImageData field of an EmfPlusImage object.
#[derive(Clone, Debug, PartialEq)]
pub enum EmfPlusImageData {
    /// ImageDataTypeUnknown: the content is undefined and kept as raw
    /// bytes.
    Unknown(Vec<u8>),
    /// A bitmap image (MS-EMFPLUS 2.2.2.2).
    Bitmap(EmfPlusBitmap),
    /// A metafile image (MS-EMFPLUS 2.2.2.27).
    Metafile(EmfPlusMetafile),
}

impl EmfPlusImage {
    /// Parses an image from at most `available` bytes of object data.
    /// The budget bounds the pixel/metafile payload, which has no
    /// explicit length of its own in the bitmap case.
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        available: usize,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_bytes_field, read_with};

        let mut consumed_bytes: usize = 0;
        let version =
            read_with(buf, &mut consumed_bytes, EmfPlusGraphicsVersion::parse)?;
        let image_data_type = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::ImageDataType::parse,
        )?;

        let remaining = available.saturating_sub(consumed_bytes);
        let image_data = match image_data_type {
            crate::parser::emf_plus::ImageDataType::ImageDataTypeUnknown => {
                EmfPlusImageData::Unknown(read_bytes_field(
                    buf,
                    &mut consumed_bytes,
                    remaining,
                )?)
            }
            crate::parser::emf_plus::ImageDataType::ImageDataTypeBitmap => {
                let (bitmap, c) = EmfPlusBitmap::parse(buf, remaining)?;
                consumed_bytes += c;
                EmfPlusImageData::Bitmap(bitmap)
            }
            crate::parser::emf_plus::ImageDataType::ImageDataTypeMetafile => {
                let (metafile, c) = EmfPlusMetafile::parse(buf, remaining)?;
                consumed_bytes += c;
                EmfPlusImageData::Metafile(metafile)
            }
        };

        Ok((Self { version, image_data }, consumed_bytes))
    }
}

/// The EmfPlusBitmap object specifies a bitmap that contains a graphics
/// image (MS-EMFPLUS 2.2.2.2).
///
/// Graphics images are specified by EmfPlusImage objects (section
/// 2.2.1.4). An EmfPlusBitmap object MUST be present in the ImageData
/// field of an EmfPlusImage object if ImageTypeBitmap is specified in
/// its Type field.
///
/// This object is generic and is used to specify different types of
/// bitmap data, including:
///
/// - An EmfPlusBitmapData object (section 2.2.2.3); and
/// - An EmfPlusCompressedImage object (section 2.2.2.10).
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusBitmap {
    /// Width (4 bytes): A signed integer that specifies the width in
    /// pixels of the area occupied by the bitmap.
    ///
    /// If the image is compressed, according to the Type field, this
    /// value is undefined and MUST be ignored.
    pub width: i32,
    /// Height (4 bytes): A signed integer that specifies the height in
    /// pixels of the area occupied by the bitmap.
    ///
    /// If the image is compressed, according to the Type field, this
    /// value is undefined and MUST be ignored.
    pub height: i32,
    /// Stride (4 bytes): A signed integer that specifies the byte
    /// offset between the beginning of one scan-line and the next.
    /// This value is the number of bytes per pixel, which is specified
    /// in the PixelFormat field, multiplied by the width in pixels,
    /// which is specified in the Width field. The value of this field
    /// MUST be a multiple of four.
    ///
    /// If the image is compressed, according to the Type field, this
    /// value is undefined and MUST be ignored.
    pub stride: i32,
    /// BitmapData (variable): Variable-length data that defines the
    /// bitmap data object specified in the Type field. The content and
    /// format of the data can be different for every bitmap type.
    ///
    /// The PixelFormat and Type fields of the wire format are folded
    /// into the variant.
    pub bitmap_data: EmfPlusBitmapContent,
}

/// The BitmapData field of an EmfPlusBitmap object.
#[derive(Clone, Debug, PartialEq)]
pub enum EmfPlusBitmapContent {
    /// The EmfPlusBitmapData object specifies a bitmap image with
    /// pixel data (MS-EMFPLUS 2.2.2.3).
    ///
    /// Bitmaps are specified by EmfPlusBitmap objects. An
    /// EmfPlusBitmapData object MUST be present in the BitmapData
    /// field of an EmfPlusBitmap object if BitmapDataTypePixel is
    /// specified in its Type field.
    Pixel {
        /// PixelFormat (4 bytes): An unsigned integer that specifies
        /// the format of the pixels that make up the bitmap image.
        /// The supported pixel formats are specified in the
        /// PixelFormat enumeration (section 2.1.1.24).
        pixel_format: crate::parser::emf_plus::PixelFormat,
        /// Colors (variable): An optional EmfPlusPalette object
        /// (section 2.2.2.28), which specifies the palette of colors
        /// used in the pixel data. This field MUST be present if the
        /// I flag is set in the PixelFormat field of the EmfPlusBitmap
        /// object (section 2.2.2.2).
        palette: Option<EmfPlusPalette>,
        /// PixelData (variable): An array of bytes that specify the
        /// pixel data. The size and format of this data can be
        /// computed from fields in the EmfPlusBitmap object, including
        /// the pixel format from the PixelFormat enumeration (section
        /// 2.1.1.24).
        ///
        /// Runs to the end of the object data, so up to 3 bytes of
        /// trailing alignment padding can be included; consumers slice
        /// it by `stride * height`.
        pixel_data: Vec<u8>,
    },
    /// The EmfPlusCompressedImage object specifies an image with
    /// compressed data (MS-EMFPLUS 2.2.2.10).
    ///
    /// Bitmaps are specified by EmfPlusBitmap objects (section
    /// 2.2.2.2). An EmfPlusCompressedImage object MUST be present in
    /// the BitmapData field of an EmfPlusBitmap object if the
    /// BitmapDataTypeCompressed value (section 2.1.1.2) is specified
    /// in its Type field.
    ///
    /// This object is generic and is used for different types of
    /// compressed data, including:
    ///
    /// - Exchangeable Image File Format (EXIF) [EXIF];
    /// - Graphics Interchange Format (GIF) [GIF];
    /// - Joint Photographic Experts Group (JPEG) [JFIF];
    /// - Portable Network Graphics (PNG) [RFC2083] [W3C-PNG]; and
    /// - Tag Image File Format (TIFF) [RFC3302] [TIFF].
    ///
    /// The PixelFormat field is undefined for this variant and its raw
    /// value is preserved.
    Compressed {
        /// The raw value of the undefined PixelFormat field.
        pixel_format_raw: u32,
        /// CompressedImageData (variable): An array of bytes, which
        /// specify the compressed image. The type of compression is
        /// determined from the data itself.
        compressed_image_data: Vec<u8>,
    },
}

impl EmfPlusBitmap {
    /// Parses a bitmap from at most `available` bytes; the pixel or
    /// compressed data runs to the end of that budget.
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        available: usize,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_bytes_field, read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let width: i32 = read_field(buf, &mut consumed_bytes)?;
        let height: i32 = read_field(buf, &mut consumed_bytes)?;
        let stride: i32 = read_field(buf, &mut consumed_bytes)?;
        let pixel_format_raw: u32 = read_field(buf, &mut consumed_bytes)?;
        let bitmap_data_type = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::BitmapDataType::parse,
        )?;

        use crate::parser::emf_plus::BitmapDataType;

        let bitmap_data = match bitmap_data_type {
            BitmapDataType::BitmapDataTypePixel => {
                let Some(pixel_format) =
                    crate::parser::emf_plus::PixelFormat::from_repr(
                        pixel_format_raw,
                    )
                else {
                    return Err(
                        crate::parser::ParseError::UnexpectedEnumValue {
                            cause: alloc::format!(
                                "unexpected value as PixelFormat: \
                                 {pixel_format_raw:#010X}",
                            )
                            .into(),
                        },
                    );
                };

                let palette = if pixel_format.is_indexed() {
                    Some(read_with(
                        buf,
                        &mut consumed_bytes,
                        EmfPlusPalette::parse,
                    )?)
                } else {
                    None
                };

                let remaining = available.saturating_sub(consumed_bytes);
                let pixel_data =
                    read_bytes_field(buf, &mut consumed_bytes, remaining)?;

                EmfPlusBitmapContent::Pixel {
                    pixel_format,
                    palette,
                    pixel_data,
                }
            }
            BitmapDataType::BitmapDataTypeCompressed => {
                let remaining = available.saturating_sub(consumed_bytes);
                let compressed_image_data =
                    read_bytes_field(buf, &mut consumed_bytes, remaining)?;

                EmfPlusBitmapContent::Compressed {
                    pixel_format_raw,
                    compressed_image_data,
                }
            }
        };

        Ok((Self { width, height, stride, bitmap_data }, consumed_bytes))
    }
}

/// The EmfPlusMetafileData object specifies a metafile that contains a
/// graphics image (MS-EMFPLUS 2.2.2.27).
///
/// Graphics images are specified by EmfPlusImage objects (section
/// 2.2.1.4). An EmfPlusMetafile object MUST be present in the
/// ImageData field of an EmfPlusImage object if ImageTypeMetafile is
/// specified in its Type field.
///
/// This object is generic and is used for different types of data,
/// including:
///
/// - A WMF metafile [MS-WMF];
/// - A WMF metafile which can be placed;
/// - An EMF metafile [MS-EMF];
/// - An EMF+ metafile that specifies graphics operations with EMF+ records
///   only; and
/// - An EMF+ metafile that specifies graphics operations with both EMF+ and EMF
///   records ([MS-EMF] section 2.3).
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusMetafile {
    /// Type (4 bytes): An unsigned integer that specifies the type of
    /// metafile that is embedded in the MetafileData field. This value
    /// is defined in the MetafileDataType enumeration (section
    /// 2.1.1.20).
    pub metafile_type: crate::parser::emf_plus::MetafileDataType,
    /// MetafileData (variable): Variable-length data that specifies
    /// the embedded metafile. The content and format of the data can
    /// be different for each metafile type.
    ///
    /// Nested metafiles are not parsed recursively here; conversion
    /// decides whether and how to replay them. The MetafileDataSize
    /// field of the wire format is consumed at parse time and not
    /// stored.
    pub metafile_data: Vec<u8>,
}

impl EmfPlusMetafile {
    /// Parses an embedded metafile from at most `available` bytes.
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        available: usize,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_bytes_field, read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let metafile_type = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::MetafileDataType::parse,
        )?;
        let metafile_data_size: u32 = read_field(buf, &mut consumed_bytes)?;

        // The declared size must fit in what the enclosing object
        // actually carries; otherwise the read would cross into
        // sibling data of the enclosing record.
        let remaining = available.saturating_sub(consumed_bytes);
        crate::parser::ParseError::expect_le(
            "MetafileDataSize",
            u64::from(metafile_data_size),
            remaining as u64,
        )?;

        let metafile_data = read_bytes_field(
            buf,
            &mut consumed_bytes,
            metafile_data_size as usize,
        )?;

        Ok((Self { metafile_type, metafile_data }, consumed_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::PixelFormat;

    fn version_bytes() -> [u8; 4] {
        0xDBC0_1002_u32.to_le_bytes()
    }

    #[test]
    fn parses_raw_pixel_bitmap() {
        let mut data = vec![];
        data.extend(version_bytes());
        data.extend(1_u32.to_le_bytes()); // ImageDataTypeBitmap
        data.extend(2_i32.to_le_bytes()); // Width
        data.extend(1_i32.to_le_bytes()); // Height
        data.extend(8_i32.to_le_bytes()); // Stride
        data.extend((PixelFormat::PixelFormat32bppARGB as u32).to_le_bytes());
        data.extend(0_u32.to_le_bytes()); // BitmapDataTypePixel
        data.extend([0xAA; 8]); // pixel data

        let mut buf: &[u8] = &data;
        let (image, consumed) =
            EmfPlusImage::parse(&mut buf, data.len()).unwrap();

        assert_eq!(consumed, data.len());

        let EmfPlusImageData::Bitmap(bitmap) = image.image_data else {
            panic!("expected bitmap image data");
        };
        assert_eq!((bitmap.width, bitmap.height, bitmap.stride), (2, 1, 8));

        let EmfPlusBitmapContent::Pixel { pixel_format, palette, pixel_data } =
            bitmap.bitmap_data
        else {
            panic!("expected raw pixel content");
        };
        assert_eq!(pixel_format, PixelFormat::PixelFormat32bppARGB);
        assert!(palette.is_none());
        assert_eq!(pixel_data, vec![0xAA; 8]);
    }

    #[test]
    fn parses_compressed_bitmap_with_undefined_pixel_format() {
        let mut data = vec![];
        data.extend(version_bytes());
        data.extend(1_u32.to_le_bytes()); // ImageDataTypeBitmap
        data.extend(0_i32.to_le_bytes()); // Width (undefined)
        data.extend(0_i32.to_le_bytes()); // Height (undefined)
        data.extend(0_i32.to_le_bytes()); // Stride (undefined)
        // An arbitrary value that is not a defined PixelFormat variant;
        // it must not fail the parse for compressed content.
        data.extend(0x1234_5678_u32.to_le_bytes());
        data.extend(1_u32.to_le_bytes()); // BitmapDataTypeCompressed
        data.extend(b"PNG!");

        let mut buf: &[u8] = &data;
        let (image, consumed) =
            EmfPlusImage::parse(&mut buf, data.len()).unwrap();

        assert_eq!(consumed, data.len());

        let EmfPlusImageData::Bitmap(bitmap) = image.image_data else {
            panic!("expected bitmap image data");
        };
        let EmfPlusBitmapContent::Compressed {
            pixel_format_raw,
            compressed_image_data,
        } = bitmap.bitmap_data
        else {
            panic!("expected compressed content");
        };
        assert_eq!(pixel_format_raw, 0x1234_5678);
        assert_eq!(compressed_image_data, b"PNG!");
    }

    #[test]
    fn metafile_size_must_fit_in_the_object() {
        let mut data = vec![];
        data.extend(version_bytes());
        data.extend(2_u32.to_le_bytes()); // ImageDataTypeMetafile
        data.extend(3_u32.to_le_bytes()); // MetafileDataTypeEmf
        data.extend(100_u32.to_le_bytes()); // larger than remaining
        data.extend([0x00; 4]);

        let mut buf: &[u8] = &data;

        assert!(EmfPlusImage::parse(&mut buf, data.len()).is_err());
    }
}
