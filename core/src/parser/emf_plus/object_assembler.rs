//! Reassembly of continued EmfPlusObject records (MS-EMFPLUS 2.3.5.1).
//!
//! An object definition larger than 64 KB spans a series of
//! EmfPlusObject records. Every record of the series carries the C
//! flag except the final one, and the object data of every record in
//! the series (including the final one) starts with the 4-byte
//! TotalObjectSize field. This matches the behavior of GDI+ output and
//! of the LibreOffice reader; the specification text is ambiguous
//! about the final record but its C flag definition ("never set in the
//! final record") settles the termination rule.

use crate::{
    imports::*,
    parser::emf_plus::{objects::EmfPlusObjectData, records::EmfPlusObject},
};

/// A continued object under reassembly.
#[derive(Clone, Debug)]
struct PendingObject {
    object_type: crate::parser::emf_plus::ObjectType,
    object_id: u8,
    total_object_size: usize,
    data: Vec<u8>,
}

/// Combines EmfPlusObject records into typed object data, reassembling
/// continued objects along the way.
///
/// Feed every EmfPlusObject record to [`push`](Self::push) in stream
/// order; a returned pair is the object table index and the completed
/// object for that slot.
#[derive(Clone, Debug, Default)]
pub struct EmfPlusObjectAssembler {
    pending: Option<PendingObject>,
}

impl EmfPlusObjectAssembler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether a continued object is still waiting for its final
    /// fragment. If the stream ends in this state, the object is
    /// incomplete and its table slot keeps its previous content.
    pub fn is_pending(&self) -> bool {
        self.pending.is_some()
    }

    /// Consumes one EmfPlusObject record. Returns the completed typed
    /// object and its table index, or `None` when the record is a
    /// non-final fragment of a continued object.
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn push(
        &mut self,
        record: &EmfPlusObject,
    ) -> Result<Option<(u8, EmfPlusObjectData)>, crate::parser::ParseError>
    {
        if record.continuable {
            self.push_fragment(record)?;

            return Ok(None);
        }

        let Some(pending) = self.pending.take() else {
            // The common case: a self-contained object record.
            let object_data = EmfPlusObjectData::parse(
                record.object_type,
                &record.object_data,
            )?;

            return Ok(Some((record.object_id, object_data)));
        };

        if pending.object_type != record.object_type
            || pending.object_id != record.object_id
        {
            // The series was abandoned mid-way. The specification
            // requires the records of a continued object to be
            // contiguous, so drop the fragments and process the new
            // record on its own.
            warn!(
                object_type = ?pending.object_type,
                object_id = pending.object_id,
                "incomplete continued EMF+ object is discarded",
            );

            let object_data = EmfPlusObjectData::parse(
                record.object_type,
                &record.object_data,
            )?;

            return Ok(Some((record.object_id, object_data)));
        }

        // The final fragment also starts with TotalObjectSize.
        let mut data = pending.data;
        data.extend_from_slice(strip_total_object_size(&record.object_data)?.1);

        // A series that ends short of the declared size lost at least
        // one fragment; typing the partial buffer would hand a
        // silently corrupted object to the consumer.
        if data.len() < pending.total_object_size {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: alloc::format!(
                    "continued EMF+ object is incomplete: {} of {} bytes",
                    data.len(),
                    pending.total_object_size,
                )
                .into(),
            });
        }

        // Alignment padding of the fragment records can leave the
        // accumulated buffer slightly larger than the declared size.
        data.truncate(pending.total_object_size);

        let object_data = EmfPlusObjectData::parse(pending.object_type, &data)?;

        Ok(Some((pending.object_id, object_data)))
    }

    /// Accumulates a C-flagged fragment.
    fn push_fragment(
        &mut self,
        record: &EmfPlusObject,
    ) -> Result<(), crate::parser::ParseError> {
        let (total_object_size, fragment) =
            strip_total_object_size(&record.object_data)?;

        match &mut self.pending {
            Some(pending)
                if pending.object_type == record.object_type
                    && pending.object_id == record.object_id =>
            {
                pending.data.extend_from_slice(fragment);

                // The declared total bounds the accumulation even if a
                // crafted stream repeats fragments indefinitely. The
                // fragment payloads are exact (padding lives outside
                // DataSize), so only the trailing 32-bit alignment of
                // the final fragment can overshoot the total.
                crate::parser::ParseError::expect_le(
                    "accumulated continued object size",
                    pending.data.len() as u64,
                    (pending.total_object_size as u64).saturating_add(3),
                )?;

                Ok(())
            }
            Some(pending) => {
                warn!(
                    object_type = ?pending.object_type,
                    object_id = pending.object_id,
                    "incomplete continued EMF+ object is discarded",
                );

                *pending = PendingObject {
                    object_type: record.object_type,
                    object_id: record.object_id,
                    total_object_size,
                    data: fragment.to_vec(),
                };

                Ok(())
            }
            None => {
                self.pending = Some(PendingObject {
                    object_type: record.object_type,
                    object_id: record.object_id,
                    total_object_size,
                    data: fragment.to_vec(),
                });

                Ok(())
            }
        }
    }
}

/// Splits the leading TotalObjectSize field off a continued-object
/// fragment and validates it against the allocation bound.
fn strip_total_object_size(
    object_data: &[u8],
) -> Result<(usize, &[u8]), crate::parser::ParseError> {
    let Some(prefix) = object_data.get(..4) else {
        return Err(crate::parser::ParseError::UnexpectedPattern {
            cause: Cow::from(
                "continued EMF+ object record has no room for TotalObjectSize",
            ),
        });
    };

    let total =
        u32::from_le_bytes([prefix[0], prefix[1], prefix[2], prefix[3]]);

    crate::parser::ParseError::expect_le(
        "TotalObjectSize",
        total,
        crate::parser::MAX_RECORD_BYTES,
    )?;

    Ok((total as usize, &object_data[4..]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::emf_plus::{ObjectType, objects};

    /// A minimal EmfPlusImage carrying compressed (raw passthrough)
    /// content, easy to split into fragments.
    fn image_object_bytes(payload: &[u8]) -> Vec<u8> {
        let mut data = vec![];
        data.extend(0xDBC0_1002_u32.to_le_bytes());
        data.extend(1_u32.to_le_bytes()); // ImageDataTypeBitmap
        data.extend(0_i32.to_le_bytes());
        data.extend(0_i32.to_le_bytes());
        data.extend(0_i32.to_le_bytes());
        data.extend(0_u32.to_le_bytes());
        data.extend(1_u32.to_le_bytes()); // BitmapDataTypeCompressed
        data.extend(payload);
        data
    }

    fn object_record(continuable: bool, object_data: Vec<u8>) -> EmfPlusObject {
        let object_id = 3_u8;
        let flags = if continuable { 0x8000_u16 } else { 0 }
            | (ObjectType::ObjectTypeImage as u16) << 8
            | u16::from(object_id);

        EmfPlusObject {
            record_type: crate::parser::emf_plus::RecordType::EmfPlusObject,
            flags,
            continuable,
            object_type: ObjectType::ObjectTypeImage,
            object_id,
            size: u32::try_from(object_data.len()).unwrap() + 12,
            data_size: crate::parser::Size::from(
                u32::try_from(object_data.len()).unwrap(),
            ),
            object_data,
        }
    }

    #[test]
    fn passes_through_self_contained_objects() {
        let mut assembler = EmfPlusObjectAssembler::new();
        let record = object_record(false, image_object_bytes(b"PNG!"));

        let (id, object_data) = assembler.push(&record).unwrap().unwrap();

        assert_eq!(id, 3);
        assert!(!assembler.is_pending());

        let objects::EmfPlusObjectData::Image(image) = object_data else {
            panic!("expected image object data");
        };
        let objects::EmfPlusImageData::Bitmap(bitmap) = image.image_data else {
            panic!("expected bitmap image data");
        };
        let objects::EmfPlusBitmapContent::Compressed {
            compressed_image_data,
            ..
        } = bitmap.bitmap_data
        else {
            panic!("expected compressed content");
        };
        assert_eq!(compressed_image_data, b"PNG!");
    }

    #[test]
    fn reassembles_continued_objects() {
        let whole = image_object_bytes(b"0123456789");
        let total = whole.len() as u32;
        let (first, second) = whole.split_at(whole.len() / 2);

        let mut fragment_1 = total.to_le_bytes().to_vec();
        fragment_1.extend_from_slice(first);
        let mut fragment_2 = total.to_le_bytes().to_vec();
        fragment_2.extend_from_slice(second);

        let mut assembler = EmfPlusObjectAssembler::new();

        assert!(
            assembler.push(&object_record(true, fragment_1)).unwrap().is_none()
        );
        assert!(assembler.is_pending());

        let (id, object_data) =
            assembler.push(&object_record(false, fragment_2)).unwrap().unwrap();

        assert_eq!(id, 3);
        assert!(!assembler.is_pending());

        let objects::EmfPlusObjectData::Image(image) = object_data else {
            panic!("expected image object data");
        };
        let objects::EmfPlusImageData::Bitmap(bitmap) = image.image_data else {
            panic!("expected bitmap image data");
        };
        let objects::EmfPlusBitmapContent::Compressed {
            compressed_image_data,
            ..
        } = bitmap.bitmap_data
        else {
            panic!("expected compressed content");
        };
        assert_eq!(compressed_image_data, b"0123456789");
    }

    #[test]
    fn discards_pending_series_on_unrelated_record() {
        let whole = image_object_bytes(b"abcdef");
        let total = whole.len() as u32;
        let mut fragment = total.to_le_bytes().to_vec();
        fragment.extend_from_slice(&whole[..3]);

        let mut assembler = EmfPlusObjectAssembler::new();
        assembler.push(&object_record(true, fragment)).unwrap();

        // A self-contained record for a different slot arrives before
        // the series completes.
        let mut other = object_record(false, image_object_bytes(b"x"));
        other.object_id = 9;

        let (id, _) = assembler.push(&other).unwrap().unwrap();

        assert_eq!(id, 9);
        assert!(!assembler.is_pending());
    }

    #[test]
    fn rejects_truncated_continued_object() {
        let whole = image_object_bytes(b"0123456789");
        let total = u32::try_from(whole.len()).unwrap();
        let half = whole.len() / 2;

        let mut fragment_1 = total.to_le_bytes().to_vec();
        fragment_1.extend_from_slice(&whole[..half]);
        // The final fragment is 3 bytes short of the declared total.
        let mut fragment_2 = total.to_le_bytes().to_vec();
        fragment_2.extend_from_slice(&whole[half..whole.len() - 3]);

        let mut assembler = EmfPlusObjectAssembler::new();
        assembler.push(&object_record(true, fragment_1)).unwrap();

        assert!(assembler.push(&object_record(false, fragment_2)).is_err());
    }

    #[test]
    fn rejects_fragment_without_total_object_size() {
        let mut assembler = EmfPlusObjectAssembler::new();
        let record = object_record(true, vec![0x01, 0x02]);

        assert!(assembler.push(&record).is_err());
    }
}
