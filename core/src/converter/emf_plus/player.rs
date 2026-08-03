//! Playback of the EMF+ records embedded in EMR_COMMENT records.
//!
//! [`EmfPlusPlayer`] is the EMF+ counterpart of the EMF [`Player`] trait:
//! one method per MS-EMFPLUS record, every method a no-op by default so
//! that a player overrides only the records it renders. It is a separate
//! trait, so an EMF-only player is unaffected and opts in with an empty
//! `impl EmfPlusPlayer for MyPlayer {}`.
//!
//! [`EmfPlusDispatcher`] drives the trait: it reads the record header,
//! parses each record, reassembles continued object records, and calls
//! the matching player method. The dispatcher is purely mechanical — the
//! EMF+ Dual/Only rendering policy lives in the player, which learns the
//! Dual flag from the [`EmfPlusHeader`] record delivered to
//! [`EmfPlusPlayer::emf_plus_header`].
//!
//! [`Player`]: crate::converter::Player

use crate::{
    converter::PlayError,
    parser::{
        Size,
        emf_plus::{objects::EmfPlusObjectData, *},
    },
};

/// Generates a default no-op handler for each `(method, record type)`
/// pair. The record is consumed so the default body neither warns about
/// an unused value nor invites a by-reference signature.
macro_rules! record_handlers {
    ($($method:ident: $record:ty),+ $(,)?) => {
        $(
            fn $method(
                self,
                record_number: usize,
                record: $record,
            ) -> Result<Self, PlayError> {
                let _ = (record_number, record);

                Ok(self)
            }
        )+
    };
}

/// Processes the EMF+ records embedded in a metafile.
///
/// Mirrors the EMF [`Player`](crate::converter::Player) trait: methods
/// take `self` by value and return `Self`, so a player folds its state
/// through the record stream. Every method defaults to a no-op; a player
/// implements only the records it renders.
///
/// Object records are delivered pre-assembled: continued object records
/// are recombined by the dispatcher, so [`emf_plus_object`] receives the
/// object table index and the typed [`EmfPlusObjectData`], never a raw
/// fragment.
///
/// [`emf_plus_object`]: EmfPlusPlayer::emf_plus_object
pub trait EmfPlusPlayer: Sized {
    /// Handles a completed object definition (MS-EMFPLUS 2.3.5.1),
    /// identified by its object table index. Continued objects are
    /// reassembled before this is called.
    fn emf_plus_object(
        self,
        record_number: usize,
        object_id: u8,
        object: EmfPlusObjectData,
    ) -> Result<Self, PlayError> {
        let _ = (record_number, object_id, object);

        Ok(self)
    }

    record_handlers! {
        // Control records (2.3.3).
        emf_plus_header: EmfPlusHeader,
        emf_plus_end_of_file: EmfPlusEndOfFile,
        emf_plus_get_dc: EmfPlusGetDC,
        // Comment records (2.3.2).
        emf_plus_comment: EmfPlusComment,
        // Object records (2.3.5).
        emf_plus_serializable_object: EmfPlusSerializableObject,
        // Drawing records (2.3.4).
        emf_plus_clear: EmfPlusClear,
        emf_plus_draw_arc: EmfPlusDrawArc,
        emf_plus_draw_beziers: EmfPlusDrawBeziers,
        emf_plus_draw_closed_curve: EmfPlusDrawClosedCurve,
        emf_plus_draw_curve: EmfPlusDrawCurve,
        emf_plus_draw_driver_string: EmfPlusDrawDriverString,
        emf_plus_draw_ellipse: EmfPlusDrawEllipse,
        emf_plus_draw_image: EmfPlusDrawImage,
        emf_plus_draw_image_points: EmfPlusDrawImagePoints,
        emf_plus_draw_lines: EmfPlusDrawLines,
        emf_plus_draw_path: EmfPlusDrawPath,
        emf_plus_draw_pie: EmfPlusDrawPie,
        emf_plus_draw_rects: EmfPlusDrawRects,
        emf_plus_draw_string: EmfPlusDrawString,
        emf_plus_fill_closed_curve: EmfPlusFillClosedCurve,
        emf_plus_fill_ellipse: EmfPlusFillEllipse,
        emf_plus_fill_path: EmfPlusFillPath,
        emf_plus_fill_pie: EmfPlusFillPie,
        emf_plus_fill_polygon: EmfPlusFillPolygon,
        emf_plus_fill_rects: EmfPlusFillRects,
        emf_plus_fill_region: EmfPlusFillRegion,
        // Property records (2.3.6).
        emf_plus_set_anti_alias_mode: EmfPlusSetAntiAliasMode,
        emf_plus_set_compositing_mode: EmfPlusSetCompositingMode,
        emf_plus_set_compositing_quality: EmfPlusSetCompositingQuality,
        emf_plus_set_interpolation_mode: EmfPlusSetInterpolationMode,
        emf_plus_set_pixel_offset_mode: EmfPlusSetPixelOffsetMode,
        emf_plus_set_rendering_origin: EmfPlusSetRenderingOrigin,
        emf_plus_set_text_contrast: EmfPlusSetTextContrast,
        emf_plus_set_text_rendering_hint: EmfPlusSetTextRenderingHint,
        // State records (2.3.7).
        emf_plus_begin_container: EmfPlusBeginContainer,
        emf_plus_begin_container_no_params: EmfPlusBeginContainerNoParams,
        emf_plus_end_container: EmfPlusEndContainer,
        emf_plus_restore: EmfPlusRestore,
        emf_plus_save: EmfPlusSave,
        // Clipping records (2.3.1).
        emf_plus_offset_clip: EmfPlusOffsetClip,
        emf_plus_reset_clip: EmfPlusResetClip,
        emf_plus_set_clip_path: EmfPlusSetClipPath,
        emf_plus_set_clip_rect: EmfPlusSetClipRect,
        emf_plus_set_clip_region: EmfPlusSetClipRegion,
        // Terminal server records (2.3.8).
        emf_plus_set_ts_clip: EmfPlusSetTSClip,
        emf_plus_set_ts_graphics: EmfPlusSetTSGraphics,
        // Transform records (2.3.9).
        emf_plus_multiply_world_transform: EmfPlusMultiplyWorldTransform,
        emf_plus_reset_world_transform: EmfPlusResetWorldTransform,
        emf_plus_rotate_world_transform: EmfPlusRotateWorldTransform,
        emf_plus_scale_world_transform: EmfPlusScaleWorldTransform,
        emf_plus_set_page_transform: EmfPlusSetPageTransform,
        emf_plus_set_world_transform: EmfPlusSetWorldTransform,
        emf_plus_translate_world_transform: EmfPlusTranslateWorldTransform,
    }
}

/// Drives an [`EmfPlusPlayer`] over the EMF+ records of a metafile.
///
/// One dispatcher processes every EMR_COMMENT_EMFPLUS payload of a
/// metafile in order; it persists across those payloads because a
/// continued object can span several comment records and an object
/// created in one comment can be referenced by a later one.
#[derive(Clone, Debug, Default)]
pub struct EmfPlusDispatcher {
    assembler: EmfPlusObjectAssembler,
}

impl EmfPlusDispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses the EMF+ records of one EMR_COMMENT payload and folds the
    /// player through the matching [`EmfPlusPlayer`] methods. `data` is
    /// the raw comment payload including the leading `"EMF+"` identifier;
    /// a payload that does not carry that identifier leaves the player
    /// unchanged.
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn play_comment<P: EmfPlusPlayer>(
        &mut self,
        mut player: P,
        record_number: usize,
        data: &[u8],
    ) -> Result<P, PlayError> {
        if !is_emf_plus_comment(data) {
            return Ok(player);
        }

        let mut buf: &[u8] = &data[4..];

        while !buf.is_empty() {
            let (header, _) = EmfPlusRecordHeader::parse(&mut buf)?;
            let data_size = Size::from(header.data_size);
            let flags = header.flags;

            let Some(record_type) = RecordType::from_repr(header.record_type)
            else {
                // Unknown record type: skip its data and padding so a
                // single unknown record does not derail the stream.
                skip_record(&mut buf, &header)?;

                continue;
            };

            if record_type.is_reserved() {
                skip_record(&mut buf, &header)?;

                continue;
            }

            // Parses the matched record, logs it the way the EMF
            // converter logs each record, and folds the player through
            // the matching method. Defined here so it can reach the
            // per-iteration locals (`buf`, `header`, `data_size`, ...).
            macro_rules! deliver {
                ($record:ty, $method:ident) => {{
                    let record = <$record>::parse(
                        &mut buf,
                        record_type,
                        flags,
                        header.size,
                        data_size,
                    )?;

                    debug!(%record_number, ?record);
                    player = player.$method(record_number, record)?;
                }};
            }

            let mut end_of_file = false;

            match record_type {
                // Object records are delivered pre-assembled.
                RecordType::EmfPlusObject => {
                    let record = EmfPlusObject::parse(
                        &mut buf,
                        record_type,
                        flags,
                        header.size,
                        data_size,
                    )?;

                    if let Some((object_id, object)) =
                        self.assembler.push(&record)?
                    {
                        debug!(%record_number, %object_id, ?object);
                        player = player.emf_plus_object(
                            record_number,
                            object_id,
                            object,
                        )?;
                    }
                }
                RecordType::EmfPlusEndOfFile => {
                    deliver!(EmfPlusEndOfFile, emf_plus_end_of_file);
                    end_of_file = true;
                }
                RecordType::EmfPlusHeader => {
                    deliver!(EmfPlusHeader, emf_plus_header);
                }
                RecordType::EmfPlusGetDC => {
                    deliver!(EmfPlusGetDC, emf_plus_get_dc);
                }
                RecordType::EmfPlusComment => {
                    deliver!(EmfPlusComment, emf_plus_comment);
                }
                RecordType::EmfPlusSerializableObject => {
                    deliver!(
                        EmfPlusSerializableObject,
                        emf_plus_serializable_object
                    );
                }
                RecordType::EmfPlusClear => {
                    deliver!(EmfPlusClear, emf_plus_clear);
                }
                RecordType::EmfPlusDrawArc => {
                    deliver!(EmfPlusDrawArc, emf_plus_draw_arc);
                }
                RecordType::EmfPlusDrawBeziers => {
                    deliver!(EmfPlusDrawBeziers, emf_plus_draw_beziers);
                }
                RecordType::EmfPlusDrawClosedCurve => {
                    deliver!(
                        EmfPlusDrawClosedCurve,
                        emf_plus_draw_closed_curve
                    );
                }
                RecordType::EmfPlusDrawCurve => {
                    deliver!(EmfPlusDrawCurve, emf_plus_draw_curve);
                }
                RecordType::EmfPlusDrawDriverString => {
                    deliver!(
                        EmfPlusDrawDriverString,
                        emf_plus_draw_driver_string
                    );
                }
                RecordType::EmfPlusDrawEllipse => {
                    deliver!(EmfPlusDrawEllipse, emf_plus_draw_ellipse);
                }
                RecordType::EmfPlusDrawImage => {
                    deliver!(EmfPlusDrawImage, emf_plus_draw_image);
                }
                RecordType::EmfPlusDrawImagePoints => {
                    deliver!(
                        EmfPlusDrawImagePoints,
                        emf_plus_draw_image_points
                    );
                }
                RecordType::EmfPlusDrawLines => {
                    deliver!(EmfPlusDrawLines, emf_plus_draw_lines);
                }
                RecordType::EmfPlusDrawPath => {
                    deliver!(EmfPlusDrawPath, emf_plus_draw_path);
                }
                RecordType::EmfPlusDrawPie => {
                    deliver!(EmfPlusDrawPie, emf_plus_draw_pie);
                }
                RecordType::EmfPlusDrawRects => {
                    deliver!(EmfPlusDrawRects, emf_plus_draw_rects);
                }
                RecordType::EmfPlusDrawString => {
                    deliver!(EmfPlusDrawString, emf_plus_draw_string);
                }
                RecordType::EmfPlusFillClosedCurve => {
                    deliver!(
                        EmfPlusFillClosedCurve,
                        emf_plus_fill_closed_curve
                    );
                }
                RecordType::EmfPlusFillEllipse => {
                    deliver!(EmfPlusFillEllipse, emf_plus_fill_ellipse);
                }
                RecordType::EmfPlusFillPath => {
                    deliver!(EmfPlusFillPath, emf_plus_fill_path);
                }
                RecordType::EmfPlusFillPie => {
                    deliver!(EmfPlusFillPie, emf_plus_fill_pie);
                }
                RecordType::EmfPlusFillPolygon => {
                    deliver!(EmfPlusFillPolygon, emf_plus_fill_polygon);
                }
                RecordType::EmfPlusFillRects => {
                    deliver!(EmfPlusFillRects, emf_plus_fill_rects);
                }
                RecordType::EmfPlusFillRegion => {
                    deliver!(EmfPlusFillRegion, emf_plus_fill_region);
                }
                RecordType::EmfPlusSetAntiAliasMode => {
                    deliver!(
                        EmfPlusSetAntiAliasMode,
                        emf_plus_set_anti_alias_mode
                    );
                }
                RecordType::EmfPlusSetCompositingMode => {
                    deliver!(
                        EmfPlusSetCompositingMode,
                        emf_plus_set_compositing_mode
                    );
                }
                RecordType::EmfPlusSetCompositingQuality => {
                    deliver!(
                        EmfPlusSetCompositingQuality,
                        emf_plus_set_compositing_quality
                    );
                }
                RecordType::EmfPlusSetInterpolationMode => {
                    deliver!(
                        EmfPlusSetInterpolationMode,
                        emf_plus_set_interpolation_mode
                    );
                }
                RecordType::EmfPlusSetPixelOffsetMode => {
                    deliver!(
                        EmfPlusSetPixelOffsetMode,
                        emf_plus_set_pixel_offset_mode
                    );
                }
                RecordType::EmfPlusSetRenderingOrigin => {
                    deliver!(
                        EmfPlusSetRenderingOrigin,
                        emf_plus_set_rendering_origin
                    );
                }
                RecordType::EmfPlusSetTextContrast => {
                    deliver!(
                        EmfPlusSetTextContrast,
                        emf_plus_set_text_contrast
                    );
                }
                RecordType::EmfPlusSetTextRenderingHint => {
                    deliver!(
                        EmfPlusSetTextRenderingHint,
                        emf_plus_set_text_rendering_hint
                    );
                }
                RecordType::EmfPlusBeginContainer => {
                    deliver!(EmfPlusBeginContainer, emf_plus_begin_container);
                }
                RecordType::EmfPlusBeginContainerNoParams => {
                    deliver!(
                        EmfPlusBeginContainerNoParams,
                        emf_plus_begin_container_no_params
                    );
                }
                RecordType::EmfPlusEndContainer => {
                    deliver!(EmfPlusEndContainer, emf_plus_end_container);
                }
                RecordType::EmfPlusRestore => {
                    deliver!(EmfPlusRestore, emf_plus_restore);
                }
                RecordType::EmfPlusSave => {
                    deliver!(EmfPlusSave, emf_plus_save);
                }
                RecordType::EmfPlusOffsetClip => {
                    deliver!(EmfPlusOffsetClip, emf_plus_offset_clip);
                }
                RecordType::EmfPlusResetClip => {
                    deliver!(EmfPlusResetClip, emf_plus_reset_clip);
                }
                RecordType::EmfPlusSetClipPath => {
                    deliver!(EmfPlusSetClipPath, emf_plus_set_clip_path);
                }
                RecordType::EmfPlusSetClipRect => {
                    deliver!(EmfPlusSetClipRect, emf_plus_set_clip_rect);
                }
                RecordType::EmfPlusSetClipRegion => {
                    deliver!(EmfPlusSetClipRegion, emf_plus_set_clip_region);
                }
                RecordType::EmfPlusSetTSClip => {
                    deliver!(EmfPlusSetTSClip, emf_plus_set_ts_clip);
                }
                RecordType::EmfPlusSetTSGraphics => {
                    deliver!(EmfPlusSetTSGraphics, emf_plus_set_ts_graphics);
                }
                RecordType::EmfPlusMultiplyWorldTransform => {
                    deliver!(
                        EmfPlusMultiplyWorldTransform,
                        emf_plus_multiply_world_transform
                    );
                }
                RecordType::EmfPlusResetWorldTransform => {
                    deliver!(
                        EmfPlusResetWorldTransform,
                        emf_plus_reset_world_transform
                    );
                }
                RecordType::EmfPlusRotateWorldTransform => {
                    deliver!(
                        EmfPlusRotateWorldTransform,
                        emf_plus_rotate_world_transform
                    );
                }
                RecordType::EmfPlusScaleWorldTransform => {
                    deliver!(
                        EmfPlusScaleWorldTransform,
                        emf_plus_scale_world_transform
                    );
                }
                RecordType::EmfPlusSetPageTransform => {
                    deliver!(
                        EmfPlusSetPageTransform,
                        emf_plus_set_page_transform
                    );
                }
                RecordType::EmfPlusSetWorldTransform => {
                    deliver!(
                        EmfPlusSetWorldTransform,
                        emf_plus_set_world_transform
                    );
                }
                RecordType::EmfPlusTranslateWorldTransform => {
                    deliver!(
                        EmfPlusTranslateWorldTransform,
                        emf_plus_translate_world_transform
                    );
                }
                // Reserved record types are skipped above via
                // `is_reserved()`. Reaching here means that guard and this
                // arm have drifted apart; fail the record rather than
                // panicking on input a caller cannot control.
                RecordType::EmfPlusMultiFormatStart
                | RecordType::EmfPlusMultiFormatSection
                | RecordType::EmfPlusMultiFormatEnd
                | RecordType::EmfPlusStrokeFillPath => {
                    return Err(PlayError::invalid_record(alloc::format!(
                        "reserved EMF+ record type reached dispatch: {:#06X}",
                        header.record_type,
                    )));
                }
            }

            advance(&mut buf, header.padding_bytes())?;

            if end_of_file {
                break;
            }
        }

        Ok(player)
    }
}

/// Skips a record whose type is not dispatched, consuming its data and
/// alignment padding in one step.
fn skip_record(
    buf: &mut &[u8],
    header: &EmfPlusRecordHeader,
) -> Result<(), PlayError> {
    advance(buf, header.size as usize - EmfPlusRecordHeader::BYTE_SIZE)
}

/// Advances a byte cursor by `len`, failing when the record claims more
/// bytes than the comment payload holds.
fn advance(buf: &mut &[u8], len: usize) -> Result<(), PlayError> {
    *buf = buf
        .get(len..)
        .ok_or_else(|| PlayError::invalid_record("truncated EMF+ record"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use alloc::{string::String, vec, vec::Vec};

    use super::*;

    /// A player that records the tag of each EMF+ record it receives, to
    /// assert the dispatcher routes and orders records correctly.
    #[derive(Default)]
    struct Recorder {
        events: Vec<String>,
    }

    impl EmfPlusPlayer for Recorder {
        fn emf_plus_header(
            mut self,
            _record_number: usize,
            record: EmfPlusHeader,
        ) -> Result<Self, PlayError> {
            self.events.push(alloc::format!("header:dual={}", record.dual));

            Ok(self)
        }

        fn emf_plus_object(
            mut self,
            _record_number: usize,
            object_id: u8,
            _object: EmfPlusObjectData,
        ) -> Result<Self, PlayError> {
            self.events.push(alloc::format!("object:{object_id}"));

            Ok(self)
        }

        fn emf_plus_clear(
            mut self,
            _record_number: usize,
            _record: EmfPlusClear,
        ) -> Result<Self, PlayError> {
            self.events.push("clear".into());

            Ok(self)
        }

        fn emf_plus_end_of_file(
            mut self,
            _record_number: usize,
            _record: EmfPlusEndOfFile,
        ) -> Result<Self, PlayError> {
            self.events.push("end_of_file".into());

            Ok(self)
        }
    }

    fn record(record_type: u16, flags: u16, data: &[u8]) -> Vec<u8> {
        let data_size = u32::try_from(data.len()).unwrap();
        // Records are 32-bit aligned; DataSize covers the payload while
        // Size includes the trailing alignment padding.
        let mut payload = data.to_vec();
        while payload.len() % 4 != 0 {
            payload.push(0);
        }

        let size = u32::try_from(12 + payload.len()).unwrap();
        let mut out = vec![];
        out.extend(record_type.to_le_bytes());
        out.extend(flags.to_le_bytes());
        out.extend(size.to_le_bytes());
        out.extend(data_size.to_le_bytes());
        out.extend(payload);
        out
    }

    fn header_data() -> Vec<u8> {
        let mut data = vec![];
        data.extend(0xDBC0_1002_u32.to_le_bytes());
        data.extend(0_u32.to_le_bytes());
        data.extend(96_u32.to_le_bytes());
        data.extend(96_u32.to_le_bytes());
        data
    }

    /// A minimal EmfPlusImage carrying compressed (raw passthrough)
    /// content, easy to split into continued-object fragments.
    fn image_object(payload: &[u8]) -> Vec<u8> {
        let mut data = vec![];
        data.extend(0xDBC0_1002_u32.to_le_bytes());
        data.extend(1_u32.to_le_bytes()); // ImageDataTypeBitmap
        data.extend(0_i32.to_le_bytes()); // Width
        data.extend(0_i32.to_le_bytes()); // Height
        data.extend(0_i32.to_le_bytes()); // Stride
        data.extend(0_u32.to_le_bytes()); // PixelFormat
        data.extend(1_u32.to_le_bytes()); // BitmapDataTypeCompressed
        data.extend(payload);
        data
    }

    #[test]
    fn routes_records_and_stops_at_end_of_file() {
        let mut data = b"EMF+".to_vec();
        data.extend(record(0x4001, 0, &header_data()));
        data.extend(record(0x4009, 0, &0_u32.to_le_bytes())); // Clear
        data.extend(record(0x4002, 0, &[])); // EndOfFile
        // A record after EndOfFile must not be dispatched.
        data.extend(record(0x4009, 0, &0_u32.to_le_bytes()));

        let mut dispatcher = EmfPlusDispatcher::new();
        let player =
            dispatcher.play_comment(Recorder::default(), 7, &data).unwrap();

        assert_eq!(player.events, [
            "header:dual=false",
            "clear",
            "end_of_file",
        ]);
    }

    #[test]
    fn skips_unknown_record_types() {
        let mut data = b"EMF+".to_vec();
        data.extend(record(0x40FF, 0x1234, &[0_u8; 4])); // unknown
        data.extend(record(0x4009, 0, &0_u32.to_le_bytes())); // Clear

        let mut dispatcher = EmfPlusDispatcher::new();
        let player =
            dispatcher.play_comment(Recorder::default(), 1, &data).unwrap();

        assert_eq!(player.events, ["clear"]);
    }

    #[test]
    fn delivers_continued_objects_once_assembled() {
        let whole = image_object(b"0123456789");
        let total = u32::try_from(whole.len()).unwrap();
        let (first, second) = whole.split_at(whole.len() / 2);

        let mut fragment_1 = total.to_le_bytes().to_vec();
        fragment_1.extend_from_slice(first);
        let mut fragment_2 = total.to_le_bytes().to_vec();
        fragment_2.extend_from_slice(second);

        let mut data = b"EMF+".to_vec();
        // ObjectType image (5) in bits 8-14, ObjectId 4 in the low byte;
        // the C flag (0x8000) marks the non-final fragment.
        data.extend(record(0x4008, 0x8000 | (5 << 8) | 4, &fragment_1));
        data.extend(record(0x4008, (5 << 8) | 4, &fragment_2));

        let mut dispatcher = EmfPlusDispatcher::new();
        let player =
            dispatcher.play_comment(Recorder::default(), 1, &data).unwrap();

        assert_eq!(player.events, ["object:4"]);
    }

    #[test]
    fn ignores_non_emf_plus_comments() {
        let mut dispatcher = EmfPlusDispatcher::new();
        let player = dispatcher
            .play_comment(Recorder::default(), 1, b"GDIC\x00\x00\x00\x00")
            .unwrap();

        assert!(player.events.is_empty());
    }
}
