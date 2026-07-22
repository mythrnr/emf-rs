//! Parser for EMF+ (MS-EMFPLUS) data embedded in EMF metafiles.
//!
//! EMF+ records travel inside `EMR_COMMENT` records whose payload starts
//! with the comment identifier `"EMF+"` (0x2B464D45, EMR_COMMENT_EMFPLUS,
//! MS-EMF 2.3.3.2). This module parses that payload into the typed
//! records defined by the MS-EMFPLUS specification:
//!
//! - [`is_emf_plus_comment`] detects the identifier,
//! - [`EmfPlusRecordHeader`] reads the 12-byte header shared by all EMF+
//!   records; the consumer dispatches on its Type value, calls the `parse` of
//!   the matching record type with the header fields, and skips
//!   [`EmfPlusRecordHeader::padding_bytes`] afterwards. This mirrors the EMF
//!   record dispatch of the converter: there is no aggregate record enum, the
//!   `match` lives at the consumer,
//! - [`EmfPlusObjectAssembler`] reassembles continued [`EmfPlusObject`] records
//!   and produces typed [`objects::EmfPlusObjectData`].
//!
//! Unlike the EMF parser, none of these types are re-exported flat from
//! `crate::parser`; several EMF+ names (e.g. `HatchStyle`) would collide
//! with their EMF counterparts, so access always goes through
//! `crate::parser::emf_plus::*`.

pub mod enums;
pub mod objects;
pub mod records;

mod object_assembler;

pub use self::{
    enums::*, object_assembler::EmfPlusObjectAssembler, records::*,
};
use crate::imports::*;

/// The CommentIdentifier value that marks an EMR_COMMENT record as an
/// EMF+ comment (EMR_COMMENT_EMFPLUS, MS-EMF 2.3.3.2). The
/// little-endian bytes read as `"EMF+"`.
pub const EMF_PLUS_COMMENT_IDENTIFIER: u32 = 0x2B46_4D45;

/// Upper bound for count fields that drive per-element allocations
/// (points, rectangles, palette entries, blend positions, ...).
///
/// Mirrors `MAX_TOTAL_POINTS` of the EMF drawing records: the smallest
/// element is 2 bytes, so 16 Mi elements stay far below the 64 MiB
/// record bound while rejecting counts that would push
/// `Vec::with_capacity` into absurd allocations.
pub(crate) const MAX_ELEMENT_COUNT: u32 = 16 * 1024 * 1024;

/// Guards a count field read from EMF+ data against `MAX_ELEMENT_COUNT`
/// before it is used to size an allocation or drive a parse loop.
pub(crate) fn check_element_count(
    field: &'static str,
    count: u32,
) -> Result<(), crate::parser::ParseError> {
    crate::parser::ParseError::expect_le(field, count, MAX_ELEMENT_COUNT)
}

/// Reads a UTF-16LE string of `char_count` code units and advances the
/// tracker. Unpaired surrogates are replaced instead of failing the
/// parse: a damaged string should not discard the record around it.
pub(crate) fn read_utf16_field<R: crate::Read>(
    buf: &mut R,
    tracker: &mut impl crate::parser::ConsumeTracker,
    char_count: u32,
    field: &'static str,
) -> Result<String, crate::parser::ParseError> {
    check_element_count(field, char_count)?;

    let bytes = crate::parser::records::read_bytes_field(
        buf,
        tracker,
        (char_count as usize) * 2,
    )?;
    let units: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect();

    Ok(String::from_utf16_lossy(&units))
}

/// Whether an EMR_COMMENT payload carries EMF+ records.
pub fn is_emf_plus_comment(private_data: &[u8]) -> bool {
    private_data.len() >= 4
        && private_data[..4] == EMF_PLUS_COMMENT_IDENTIFIER.to_le_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier_check_matches_the_wire_bytes() {
        assert!(is_emf_plus_comment(b"EMF+ anything"));
        assert!(!is_emf_plus_comment(b"EMF"));
        assert!(!is_emf_plus_comment(b"GDIC"));
    }
}
