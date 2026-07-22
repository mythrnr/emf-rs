use crate::parser::emf_plus::objects::{EmfPlusImage, EmfPlusTransformMatrix};

/// The EmfPlusTextureBrushData object specifies a texture image for a
/// graphics brush (MS-EMFPLUS 2.2.2.45).
///
/// Graphics brushes are specified by EmfPlusBrush objects (section
/// 2.2.1.1). A texture brush paints an image, which in this context is
/// called a "texture". The texture consists of either a portion of an
/// image or a scaled version of an image, which is specified by an
/// EmfPlusImage object (section 2.2.1.4) in the OptionalData field.
///
/// Gamma correction controls the overall brightness and intensity of
/// an image. Uncorrected images can look either bleached out or too
/// dark. Varying the amount of gamma correction changes not only the
/// brightness but also the ratios of red to green to blue. The need
/// for gamma correction arises because an output device might not
/// render colors in the same intensity as the input image.
///
/// The EmfPlusTextureBrushOptionalData object (2.2.2.46) specifies
/// optional data for a texture brush; its fields are inlined into this
/// struct. Note: Each field of this object is optional and might not
/// be present in the OptionalData field of an EmfPlusTextureBrushData
/// object (section 2.2.2.45), depending on the BrushData flags
/// (section 2.1.2.1) set in its BrushDataFlags field. Although it is
/// not practical to represent every possible combination of fields
/// present or absent, this section specifies their relative order in
/// the object. The implementer is responsible for determining which
/// fields are actually present in a given metafile record, and for
/// unmarshaling the data for individual fields separately and
/// appropriately.
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusTextureBrushData {
    /// BrushDataFlags (4 bytes): An unsigned integer that specifies
    /// the data in the OptionalData field. This value MUST be composed
    /// of BrushData flags (section 2.1.2.1). The following flags are
    /// relevant to a texture brush:
    ///
    /// | Name | Value |
    /// |:-|:-|
    /// | BrushDataTransform | 0x00000002 |
    /// | BrushDataIsGammaCorrected | 0x00000080 |
    /// | BrushDataDoNotTransform | 0x00000100 |
    pub brush_data_flags: crate::parser::emf_plus::BrushDataFlags,
    /// WrapMode (4 bytes): A signed integer from the WrapMode
    /// enumeration (section 2.1.1.33) that specifies how to repeat the
    /// texture image across a shape, when the image is smaller than
    /// the area being filled.
    pub wrap_mode: crate::parser::emf_plus::WrapMode,
    /// TransformMatrix (24 bytes): An optional EmfPlusTransformMatrix
    /// object (section 2.2.2.47) that specifies a world space to
    /// device space transform for the texture brush. This field MUST
    /// be present if the BrushDataTransform flag is set in the
    /// BrushDataFlags field of the EmfPlusTextureBrushData object.
    pub transform_matrix: Option<EmfPlusTransformMatrix>,
    /// ImageObject (variable): An optional EmfPlusImage object
    /// (section 2.2.1.4) that specifies the brush texture. This field
    /// MUST be present if the size of the EmfPlusObject record
    /// (section 2.3.5.1) that defines this texture brush is large
    /// enough to accommodate an EmfPlusImage object in addition to the
    /// required fields of the EmfPlusTextureBrushData object and
    /// optionally an EmfPlusTransformMatrix object.
    ///
    /// It is absent when no bytes remain in the brush data.
    pub image_object: Option<EmfPlusImage>,
}

impl EmfPlusTextureBrushData {
    /// Parses texture brush data from at most `available` bytes.
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        available: usize,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::read_with;

        let mut consumed_bytes: usize = 0;
        let brush_data_flags = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::BrushDataFlags::parse,
        )?;
        let wrap_mode = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::WrapMode::parse,
        )?;

        let transform_matrix = if brush_data_flags
            .contains(crate::parser::emf_plus::BrushDataFlags::TRANSFORM)
        {
            Some(read_with(
                buf,
                &mut consumed_bytes,
                EmfPlusTransformMatrix::parse,
            )?)
        } else {
            None
        };

        let remaining = available.saturating_sub(consumed_bytes);
        let image_object = if remaining > 0 {
            let (image, c) = EmfPlusImage::parse(buf, remaining)?;
            consumed_bytes += c;
            Some(image)
        } else {
            None
        };

        Ok((
            Self {
                brush_data_flags,
                wrap_mode,
                transform_matrix,
                image_object,
            },
            consumed_bytes,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::{
        BrushDataFlags, WrapMode,
        objects::{EmfPlusBitmapContent, EmfPlusImageData},
    };

    #[test]
    fn parses_transform_then_image() {
        let mut data = vec![];
        data.extend(BrushDataFlags::TRANSFORM.to_le_bytes());
        data.extend(4_u32.to_le_bytes()); // WrapModeClamp
        for v in [2.0_f32, 0.0, 0.0, 2.0, 1.0, 1.0] {
            data.extend(v.to_le_bytes());
        }
        // EmfPlusImage with compressed content.
        data.extend(0xDBC0_1002_u32.to_le_bytes());
        data.extend(1_u32.to_le_bytes()); // ImageDataTypeBitmap
        data.extend(0_i32.to_le_bytes());
        data.extend(0_i32.to_le_bytes());
        data.extend(0_i32.to_le_bytes());
        data.extend(0_u32.to_le_bytes());
        data.extend(1_u32.to_le_bytes()); // BitmapDataTypeCompressed
        data.extend(b"IMG!");

        let mut buf: &[u8] = &data;
        let (brush, consumed) =
            EmfPlusTextureBrushData::parse(&mut buf, data.len()).unwrap();

        assert_eq!(consumed, data.len());
        assert_eq!(brush.wrap_mode, WrapMode::WrapModeClamp);
        assert!(brush.transform_matrix.is_some());

        let Some(image) = brush.image_object else {
            panic!("expected a texture image");
        };
        let EmfPlusImageData::Bitmap(bitmap) = image.image_data else {
            panic!("expected bitmap image data");
        };
        let EmfPlusBitmapContent::Compressed { compressed_image_data, .. } =
            bitmap.bitmap_data
        else {
            panic!("expected compressed content");
        };
        assert_eq!(compressed_image_data, b"IMG!");
    }

    #[test]
    fn image_is_absent_when_no_bytes_remain() {
        let mut data = vec![];
        data.extend(0_u32.to_le_bytes());
        data.extend(0_u32.to_le_bytes());

        let mut buf: &[u8] = &data;
        let (brush, consumed) =
            EmfPlusTextureBrushData::parse(&mut buf, data.len()).unwrap();

        assert_eq!(consumed, data.len());
        assert!(brush.image_object.is_none());
    }
}
