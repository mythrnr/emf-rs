use crate::{converter::PlayError, imports::*};

// cspell:ignore BITFIELDS CIEXYZ pels

const COMMENT_IDENTIFIER: &[u8; 4] = b"EMF+";
const RECORD_END_OF_FILE: u16 = 0x4002;
const RECORD_OBJECT: u16 = 0x4008;
const RECORD_DRAW_IMAGE: u16 = 0x401A;
const OBJECT_TYPE_IMAGE: u16 = 5;
const IMAGE_DATA_TYPE_BITMAP: u32 = 1;
const BITMAP_DATA_TYPE_PIXEL: u32 = 0;
const UNIT_TYPE_PIXEL: u32 = 2;
const FLAG_OBJECT_CONTINUABLE: u16 = 0x8000;
const FLAG_RECT_COMPRESSED: u16 = 0x4000;
const PIXEL_FORMAT_INDEXED: u32 = 0x0001_0000;
const PIXEL_FORMAT_ALPHA: u32 = 0x0004_0000;
const PIXEL_FORMAT_PREMULTIPLIED: u32 = 0x0008_0000;

#[derive(Clone, Debug)]
struct Bitmap {
    width: i32,
    height: i32,
    href: String,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct RectF {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Debug)]
pub(super) struct DrawImage {
    pub href: String,
    pub image_width: i32,
    pub image_height: i32,
    pub source: RectF,
    pub destination: RectF,
}

#[derive(Clone, Debug, Default)]
pub(super) struct State {
    images: BTreeMap<u8, Bitmap>,
}

impl State {
    pub fn play_comment(
        &mut self,
        data: &[u8],
    ) -> Result<Vec<DrawImage>, PlayError> {
        if !data.starts_with(COMMENT_IDENTIFIER) {
            return Ok(vec![]);
        }

        let data = &data[COMMENT_IDENTIFIER.len()..];
        let mut offset = 0;
        let mut draws = vec![];

        while offset < data.len() {
            let header = take(data, &mut offset, 12)?;
            let record_type = u16::from_le_bytes([header[0], header[1]]);
            let flags = u16::from_le_bytes([header[2], header[3]]);
            let size = usize::try_from(u32::from_le_bytes([
                header[4], header[5], header[6], header[7],
            ]))
            .map_err(|_| {
                invalid_record("EMF+ record size does not fit usize")
            })?;
            let data_size = usize::try_from(u32::from_le_bytes([
                header[8], header[9], header[10], header[11],
            ]))
            .map_err(|_| {
                invalid_record("EMF+ record data size does not fit usize")
            })?;

            if size < 12 || size % 4 != 0 || data_size > size - 12 {
                return Err(invalid_record("invalid EMF+ record size"));
            }

            let record_data = take(data, &mut offset, data_size)?;
            let padding = size - 12 - data_size;
            take(data, &mut offset, padding)?;

            match record_type {
                RECORD_OBJECT => self.read_object(flags, record_data)?,
                RECORD_DRAW_IMAGE => {
                    if let Some(draw) =
                        self.read_draw_image(flags, record_data)?
                    {
                        draws.push(draw);
                    }
                }
                RECORD_END_OF_FILE => break,
                _ => {}
            }
        }

        Ok(draws)
    }

    fn read_object(
        &mut self,
        flags: u16,
        data: &[u8],
    ) -> Result<(), PlayError> {
        if flags & FLAG_OBJECT_CONTINUABLE != 0 {
            info!("continued EMF+ objects are not implemented");
            return Ok(());
        }

        let object_type = (flags >> 8) & 0x007F;
        if object_type != OBJECT_TYPE_IMAGE {
            return Ok(());
        }

        let object_id = flags as u8;
        if let Some(bitmap) = read_bitmap(data)? {
            self.images.insert(object_id, bitmap);
        }

        Ok(())
    }

    fn read_draw_image(
        &self,
        flags: u16,
        data: &[u8],
    ) -> Result<Option<DrawImage>, PlayError> {
        let object_id = flags as u8;
        let Some(bitmap) = self.images.get(&object_id) else {
            return Ok(None);
        };

        let mut offset = 0;
        let _image_attributes_id = read_u32(data, &mut offset)?;
        let source_unit = read_u32(data, &mut offset)?;
        if source_unit != UNIT_TYPE_PIXEL {
            info!(source_unit, "EMF+ DrawImage source unit is not supported");
            return Ok(None);
        }

        let source = read_rect_f(data, &mut offset)?;
        let destination = if flags & FLAG_RECT_COMPRESSED == 0 {
            read_rect_f(data, &mut offset)?
        } else {
            read_rect(data, &mut offset)?
        };

        Ok(Some(DrawImage {
            href: bitmap.href.clone(),
            image_width: bitmap.width,
            image_height: bitmap.height,
            source,
            destination,
        }))
    }
}

fn read_bitmap(data: &[u8]) -> Result<Option<Bitmap>, PlayError> {
    let mut offset = 0;
    let _version = read_u32(data, &mut offset)?;
    let image_data_type = read_u32(data, &mut offset)?;
    if image_data_type != IMAGE_DATA_TYPE_BITMAP {
        return Ok(None);
    }

    let width = read_i32(data, &mut offset)?;
    let height = read_i32(data, &mut offset)?;
    let stride = read_i32(data, &mut offset)?;
    let pixel_format = read_u32(data, &mut offset)?;
    let bitmap_data_type = read_u32(data, &mut offset)?;

    if bitmap_data_type != BITMAP_DATA_TYPE_PIXEL {
        info!("compressed EMF+ bitmap objects are not implemented");
        return Ok(None);
    }
    if width <= 0 || height <= 0 || stride == 0 {
        return Err(invalid_record("invalid EMF+ bitmap dimensions"));
    }

    let bits_per_pixel = (pixel_format >> 8) & 0xFF;
    if bits_per_pixel != 32 || pixel_format & PIXEL_FORMAT_INDEXED != 0 {
        info!(
            bits_per_pixel,
            pixel_format, "EMF+ bitmap pixel format is not supported",
        );
        return Ok(None);
    }

    let width = usize::try_from(width)
        .map_err(|_| invalid_record("EMF+ bitmap width does not fit usize"))?;
    let height = usize::try_from(height)
        .map_err(|_| invalid_record("EMF+ bitmap height does not fit usize"))?;
    let row_size = width
        .checked_mul(4)
        .ok_or_else(|| invalid_record("EMF+ bitmap row size overflows"))?;
    let stride_size = usize::try_from(stride.unsigned_abs())
        .map_err(|_| invalid_record("EMF+ bitmap stride does not fit usize"))?;
    if stride_size != row_size || stride_size % 4 != 0 {
        return Err(invalid_record("invalid EMF+ bitmap stride"));
    }

    let pixel_size = stride_size
        .checked_mul(height)
        .ok_or_else(|| invalid_record("EMF+ bitmap data size overflows"))?;
    let mut pixels = take(data, &mut offset, pixel_size)?.to_vec();
    normalize_alpha(&mut pixels, pixel_format);

    let dib_height = if stride > 0 {
        -i32::try_from(height)
            .map_err(|_| invalid_record("EMF+ bitmap height exceeds i32"))?
    } else {
        i32::try_from(height)
            .map_err(|_| invalid_record("EMF+ bitmap height exceeds i32"))?
    };
    let image_size = u32::try_from(pixel_size)
        .map_err(|_| invalid_record("EMF+ bitmap data exceeds u32"))?;
    let zero = wmf_core::parser::CIEXYZ { x: 0, y: 0, z: 0 };
    let dib = wmf_core::parser::DeviceIndependentBitmap {
        dib_header_info: wmf_core::parser::BitmapInfoHeader::V4(
            wmf_core::parser::BitmapInfoHeaderV4 {
                header_size: 108,
                width: i32::try_from(width).map_err(|_| {
                    invalid_record("EMF+ bitmap width exceeds i32")
                })?,
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
                color_space_type: wmf_core::parser::LogicalColorSpace::LCS_sRGB,
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
    let bitmap: wmf_core::converter::Bitmap = dib.into();

    Ok(Some(Bitmap {
        width: i32::try_from(width)
            .map_err(|_| invalid_record("EMF+ bitmap width exceeds i32"))?,
        height: i32::try_from(height)
            .map_err(|_| invalid_record("EMF+ bitmap height exceeds i32"))?,
        href: bitmap.as_data_url(),
    }))
}

fn normalize_alpha(pixels: &mut [u8], pixel_format: u32) {
    let has_alpha = pixel_format & PIXEL_FORMAT_ALPHA != 0;
    let premultiplied = pixel_format & PIXEL_FORMAT_PREMULTIPLIED != 0;

    for pixel in pixels.chunks_exact_mut(4) {
        if !has_alpha {
            pixel[3] = u8::MAX;
        } else if premultiplied && pixel[3] != 0 {
            let alpha = u16::from(pixel[3]);
            for component in &mut pixel[..3] {
                let value = u16::from(*component) * u16::from(u8::MAX) / alpha;
                *component = u8::try_from(value.min(u16::from(u8::MAX)))
                    .expect("component is clamped to u8");
            }
        }
    }
}

fn read_rect_f(data: &[u8], offset: &mut usize) -> Result<RectF, PlayError> {
    Ok(RectF {
        x: read_f32(data, offset)?,
        y: read_f32(data, offset)?,
        width: read_f32(data, offset)?,
        height: read_f32(data, offset)?,
    })
}

fn read_rect(data: &[u8], offset: &mut usize) -> Result<RectF, PlayError> {
    Ok(RectF {
        x: f32::from(read_i16(data, offset)?),
        y: f32::from(read_i16(data, offset)?),
        width: f32::from(read_i16(data, offset)?),
        height: f32::from(read_i16(data, offset)?),
    })
}

fn read_u32(data: &[u8], offset: &mut usize) -> Result<u32, PlayError> {
    let value = take(data, offset, 4)?;
    Ok(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}

fn read_i32(data: &[u8], offset: &mut usize) -> Result<i32, PlayError> {
    let value = take(data, offset, 4)?;
    Ok(i32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}

fn read_f32(data: &[u8], offset: &mut usize) -> Result<f32, PlayError> {
    let value = take(data, offset, 4)?;
    Ok(f32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}

fn read_i16(data: &[u8], offset: &mut usize) -> Result<i16, PlayError> {
    let value = take(data, offset, 2)?;
    Ok(i16::from_le_bytes([value[0], value[1]]))
}

fn take<'a>(
    data: &'a [u8],
    offset: &mut usize,
    len: usize,
) -> Result<&'a [u8], PlayError> {
    let end = offset
        .checked_add(len)
        .ok_or_else(|| invalid_record("EMF+ offset overflows"))?;
    let value = data
        .get(*offset..end)
        .ok_or_else(|| invalid_record("truncated EMF+ record"))?;
    *offset = end;
    Ok(value)
}

fn invalid_record(cause: &'static str) -> PlayError {
    PlayError::InvalidRecord { cause: cause.into() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_premultiplied_alpha() {
        let mut pixels = [10, 20, 30, 128];

        normalize_alpha(
            &mut pixels,
            PIXEL_FORMAT_ALPHA | PIXEL_FORMAT_PREMULTIPLIED,
        );

        assert_eq!(pixels, [19, 39, 59, 128]);
    }

    #[test]
    fn fills_missing_alpha_as_opaque() {
        let mut pixels = [10, 20, 30, 0];

        normalize_alpha(&mut pixels, 0);

        assert_eq!(pixels, [10, 20, 30, 255]);
    }
}
