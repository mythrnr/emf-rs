use crate::{
    converter::{PlayError, emf_plus::bitmap::DecodedBitmap},
    imports::*,
    parser::emf_plus::{
        EmfPlusDrawImage, UnitType,
        objects::{EmfPlusObjectData, EmfPlusRectF},
    },
};

/// The data an SVG image element needs for one DrawImage record,
/// borrowing the encoded image from the stored bitmap.
#[derive(Clone, Debug)]
pub(super) struct DrawImage<'a> {
    pub href: &'a str,
    pub image_width: i32,
    pub image_height: i32,
    pub source: EmfPlusRectF,
    pub destination: EmfPlusRectF,
}

/// The EMF+ bitmaps the SVG player has tracked so far.
///
/// The object table and Dual flag persist across the EMR_COMMENT records
/// of a metafile: an image object created in one comment can be drawn by
/// a DrawImage record in a later comment. Framing and record parsing are
/// handled by [`EmfPlusDispatcher`](crate::converter::EmfPlusDispatcher)
/// and bitmap decoding by [`DecodedBitmap`]; this layer only turns the
/// decoded bitmaps and draws into the data an SVG image element needs.
#[derive(Clone, Debug, Default)]
pub(super) struct State {
    images: BTreeMap<u8, StoredBitmap>,
    // In an EMF+ Dual metafile the same graphics are also described by
    // EMF records, which the EMF playback path renders; drawing the EMF+
    // records too would double-draw, so EMF+ drawing is suppressed and
    // the EMF path is trusted. An EMF+ Only metafile has no EMF fallback,
    // so its EMF+ records must be rendered.
    dual: bool,
}

impl State {
    /// Records whether the metafile is EMF+ Dual, from the EmfPlusHeader.
    pub fn set_dual(&mut self, dual: bool) {
        self.dual = dual;
    }

    /// Stores an EMF+ object for later drawing, keeping only the image
    /// objects this layer can render.
    ///
    /// The EMF+ object table is shared by every object type, so an id
    /// reused by an object this layer cannot render must evict the
    /// image stored under it; otherwise a later DrawImage would render
    /// the replaced bitmap. For a Dual metafile nothing is stored at
    /// all: [`State::resolve_draw_image`] discards every draw there, so
    /// encoding the bitmaps would only waste time and memory.
    pub fn store_object(
        &mut self,
        object_id: u8,
        object: EmfPlusObjectData,
    ) -> Result<(), PlayError> {
        if self.dual {
            return Ok(());
        }

        if let Some(decoded) = DecodedBitmap::from_object(object)? {
            self.images.insert(object_id, StoredBitmap {
                width: decoded.width,
                height: decoded.height,
                href: decoded.bmp.as_data_url(),
            });
        } else {
            self.images.remove(&object_id);
        }

        Ok(())
    }

    /// Resolves a DrawImage record against the tracked objects, or `None`
    /// when the image is unknown, its unit is unsupported, or EMF+
    /// drawing is suppressed for a Dual metafile.
    pub fn resolve_draw_image(
        &self,
        record: &EmfPlusDrawImage,
    ) -> Option<DrawImage<'_>> {
        if self.dual {
            return None;
        }

        let bitmap = self.images.get(&record.object_id)?;

        if record.src_unit != UnitType::UnitTypePixel {
            info!(
                src_unit = ?record.src_unit,
                "EMF+ DrawImage source unit is not supported",
            );

            return None;
        }

        Some(DrawImage {
            href: &bitmap.href,
            image_width: bitmap.width,
            image_height: bitmap.height,
            source: record.src_rect,
            destination: record.rect_data.as_rect_f(),
        })
    }
}

/// A stored bitmap, already encoded as the data URL an SVG image
/// element consumes.
#[derive(Clone, Debug)]
struct StoredBitmap {
    width: i32,
    height: i32,
    href: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        converter::emf_plus::bitmap::test_support::bitmap_object,
        parser::emf_plus::{PixelFormat, RecordType, objects::EmfPlusRectData},
    };

    fn draw_image_record(
        object_id: u8,
        src_unit: UnitType,
    ) -> EmfPlusDrawImage {
        EmfPlusDrawImage {
            record_type: RecordType::EmfPlusDrawImage,
            flags: u16::from(object_id),
            object_id,
            size: 0x34,
            data_size: crate::parser::Size::from(0x28_u32),
            image_attributes_id: 0,
            src_unit,
            src_rect: EmfPlusRectF { x: 1.0, y: 2.0, width: 3.0, height: 4.0 },
            rect_data: EmfPlusRectData::Float(EmfPlusRectF {
                x: 5.0,
                y: 6.0,
                width: 7.0,
                height: 8.0,
            }),
        }
    }

    #[test]
    fn stores_the_bitmap_dimensions_and_href() {
        let mut state = State::default();
        state
            .store_object(3, bitmap_object(PixelFormat::PixelFormat32bppARGB))
            .unwrap();

        let bitmap = state.images.get(&3).unwrap();

        assert_eq!((bitmap.width, bitmap.height), (2, 1));
        assert!(bitmap.href.starts_with("data:image/bmp;base64,"));
    }

    #[test]
    fn skips_storing_for_dual_metafiles() {
        let mut state = State::default();
        state.set_dual(true);
        state
            .store_object(3, bitmap_object(PixelFormat::PixelFormat32bppARGB))
            .unwrap();

        assert!(state.images.is_empty());
    }

    #[test]
    fn evicts_the_stale_image_when_an_object_id_is_reused() {
        let mut state = State::default();
        state
            .store_object(3, bitmap_object(PixelFormat::PixelFormat32bppARGB))
            .unwrap();
        state
            .store_object(3, bitmap_object(PixelFormat::PixelFormat24bppRGB))
            .unwrap();

        assert!(state.images.is_empty());
    }

    #[test]
    fn resolves_a_draw_image_against_a_stored_object() {
        let mut state = State::default();
        state
            .store_object(3, bitmap_object(PixelFormat::PixelFormat32bppARGB))
            .unwrap();

        let draw = state
            .resolve_draw_image(&draw_image_record(3, UnitType::UnitTypePixel))
            .unwrap();

        assert_eq!((draw.image_width, draw.image_height), (2, 1));
        assert!(draw.href.starts_with("data:image/bmp;base64,"));
        assert_eq!(draw.source, EmfPlusRectF {
            x: 1.0,
            y: 2.0,
            width: 3.0,
            height: 4.0,
        });
        assert_eq!(draw.destination, EmfPlusRectF {
            x: 5.0,
            y: 6.0,
            width: 7.0,
            height: 8.0,
        });
    }

    #[test]
    fn suppresses_draws_for_dual_metafiles() {
        let mut state = State::default();
        state
            .store_object(3, bitmap_object(PixelFormat::PixelFormat32bppARGB))
            .unwrap();
        state.set_dual(true);

        assert!(
            state
                .resolve_draw_image(&draw_image_record(
                    3,
                    UnitType::UnitTypePixel,
                ))
                .is_none()
        );
    }

    #[test]
    fn skips_draws_for_unknown_object_ids() {
        let state = State::default();

        assert!(
            state
                .resolve_draw_image(&draw_image_record(
                    3,
                    UnitType::UnitTypePixel,
                ))
                .is_none()
        );
    }

    #[test]
    fn skips_draws_with_unsupported_source_units() {
        let mut state = State::default();
        state
            .store_object(3, bitmap_object(PixelFormat::PixelFormat32bppARGB))
            .unwrap();

        assert!(
            state
                .resolve_draw_image(&draw_image_record(
                    3,
                    UnitType::UnitTypeWorld,
                ))
                .is_none()
        );
    }
}
