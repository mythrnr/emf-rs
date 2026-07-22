//! Implementation of the definitions in Section 2.3 of the MS-EMFPLUS
//! specifications (EMF+ records).
//!
//! Every record parser has the shape
//! `parse(buf, record_type, flags, size, data_size)`, mirroring the
//! EMF record parsers: the consumer reads the record header
//! ([`EmfPlusRecordHeader`]), dispatches on its Type value, and hands
//! the header fields to the `parse` of the matching record type,
//! which validates the Type value and stores every header field
//! alongside the record-specific data. `data_size` doubles as the
//! consumption tracker (the role `size` plays for EMF records): every
//! read is bounded by the DataSize field, and the alignment padding
//! between DataSize and Size is skipped by the consumer via
//! [`EmfPlusRecordHeader::padding_bytes`]. Flag bits that the
//! specification names as sub-fields of Flags are decoded into typed
//! fields at parse time in addition to the raw `flags` value; the
//! numeric bit positions follow the specification diagrams as
//! corrected by the behavior of GDI+ output (see the per-record
//! definitions).

mod clipping;
mod comment;
mod control;
mod drawing;
mod object;
mod property;
mod state;
mod terminal_server;
mod transform;

pub use self::{
    clipping::*, comment::*, control::*, drawing::*, object::*, property::*,
    state::*, terminal_server::*, transform::*,
};

/// The 12-byte record header that begins every EMF+ record: the Type,
/// Flags, Size, and DataSize fields of the generic record structure
/// (MS-EMFPLUS 2.3).
///
/// The Type value is kept as the raw 16-bit integer so the consumer
/// can skip records whose type is unknown or reserved instead of
/// failing the whole stream, mirroring how the EMF converter loop
/// reads a record header and dispatches on the record type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EmfPlusRecordHeader {
    /// Type (2 bytes): An unsigned integer that identifies the record
    /// type.
    pub record_type: u16,
    /// Flags (2 bytes): An unsigned integer that contains information
    /// for some records on how the operation is to be performed and
    /// on the structure of the record.
    pub flags: u16,
    /// Size (4 bytes): An unsigned integer that defines the
    /// 32-bit-aligned size of the entire record in bytes, including
    /// the 12-byte record header and the record-specific data.
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that defines the
    /// 32-bit-aligned number of bytes of data in the RecordData field
    /// that follows. This number does not include the 12-byte record
    /// header.
    pub data_size: u32,
}

impl EmfPlusRecordHeader {
    /// Number of bytes of the record header itself.
    pub const BYTE_SIZE: usize = 12;

    /// Reads and validates one record header, returning the header
    /// and the number of bytes consumed (the `(value, consumed)`
    /// convention shared by the composite parsers).
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::read_field;

        let mut consumed = 0_usize;

        let record_type: u16 = read_field(buf, &mut consumed)?;
        let flags: u16 = read_field(buf, &mut consumed)?;
        let size: u32 = read_field(buf, &mut consumed)?;
        let data_size: u32 = read_field(buf, &mut consumed)?;

        // Size covers the whole record including this 12-byte header
        // and MUST be 32-bit aligned; DataSize covers the data that
        // follows the header. Rejecting inconsistent values up front
        // keeps every size computation of the consumer trivially
        // sound.
        crate::parser::Size::parse(size)?;

        if (size as usize) < Self::BYTE_SIZE || size % 4 != 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause:
                    alloc::format!("invalid EMF+ record size: {size:#010X}",)
                        .into(),
            });
        }

        if data_size as usize > size as usize - Self::BYTE_SIZE {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: alloc::format!(
                    "EMF+ record data size ({data_size:#010X}) exceeds record \
                     size ({size:#010X})",
                )
                .into(),
            });
        }

        Ok((Self { record_type, flags, size, data_size }, consumed))
    }

    /// Bytes between the end of the record-specific data (DataSize)
    /// and the end of the record (Size): the extra alignment padding
    /// the consumer skips before reading the next record header.
    pub fn padding_bytes(&self) -> usize {
        self.size as usize - Self::BYTE_SIZE - self.data_size as usize
    }
}

/// The S bit of record flags: a BrushId field carries an ARGB color
/// instead of an object table index.
pub(in crate::parser::emf_plus) const FLAG_S: u16 = 0x8000;
/// The C bit of record flags: coordinates are 16-bit integers instead
/// of 32-bit floating-point values.
pub(in crate::parser::emf_plus) const FLAG_C: u16 = 0x4000;
/// The P bit of record flags: point data is relative (EmfPlusPointR).
pub(in crate::parser::emf_plus) const FLAG_P: u16 = 0x0800;

/// Whether the single-bit `mask` is set in the record flags.
pub(in crate::parser::emf_plus) fn flag(flags: u16, mask: u16) -> bool {
    flags & mask != 0
}

/// The object table index carried in the low byte of record flags.
///
/// The specification bounds the value to the range [0, 63] for every
/// record. The parser enforces the bound only in EmfPlusObject, where
/// it selects the object table slot the parser itself populates; for
/// the records that merely reference an object, an out-of-range index
/// is kept as-is and fails the table lookup at playback instead of
/// discarding an otherwise well-formed record.
pub(in crate::parser::emf_plus) fn object_id(flags: u16) -> u8 {
    (flags & 0x00FF) as u8
}

/// The UnitType carried in the low byte of EmfPlusBeginContainer /
/// EmfPlusSetPageTransform record flags.
pub(in crate::parser::emf_plus) fn page_unit_from_flags(
    flags: u16,
) -> Result<crate::parser::emf_plus::UnitType, crate::parser::ParseError> {
    let raw = u32::from(flags & 0x00FF);

    crate::parser::emf_plus::UnitType::from_repr(raw).ok_or_else(|| {
        crate::parser::ParseError::UnexpectedEnumValue {
            cause: alloc::format!("unexpected value as UnitType: {raw:#04X}")
                .into(),
        }
    })
}

/// The CombineMode carried in bits 8-11 of clipping record flags.
pub(in crate::parser::emf_plus) fn combine_mode(
    flags: u16,
) -> Result<crate::parser::emf_plus::CombineMode, crate::parser::ParseError> {
    let raw = u32::from((flags >> 8) & 0x000F);

    crate::parser::emf_plus::CombineMode::from_repr(raw).ok_or_else(|| {
        crate::parser::ParseError::UnexpectedEnumValue {
            cause: alloc::format!(
                "unexpected value as CombineMode: {raw:#03X}",
            )
            .into(),
        }
    })
}

/// A BrushId field of a fill record: the S flag selects between an
/// object table index and a literal ARGB color.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EmfPlusBrushIdOrColor {
    /// The index of an EmfPlusBrush object in the EMF+ Object Table.
    BrushId(u32),
    /// A literal color.
    Color(crate::parser::emf_plus::objects::EmfPlusARGB),
}

impl EmfPlusBrushIdOrColor {
    pub(in crate::parser::emf_plus) fn parse<R: crate::Read>(
        buf: &mut R,
        tracker: &mut impl crate::parser::ConsumeTracker,
        flags: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::read_field;

        let raw: u32 = read_field(buf, tracker)?;

        if flag(flags, FLAG_S) {
            Ok(Self::Color(
                crate::parser::emf_plus::objects::EmfPlusARGB::from_u32(raw),
            ))
        } else {
            Ok(Self::BrushId(raw))
        }
    }
}

/// Discards whatever is left of the record's DataSize. Every record
/// parser calls this last so unknown trailing fields or padding cannot
/// desynchronize the stream framing.
pub(in crate::parser::emf_plus) fn consume_remaining<R: crate::Read>(
    buf: &mut R,
    size: &mut crate::parser::Size,
) -> Result<(), crate::parser::ParseError> {
    let remaining = size.remaining_bytes();

    crate::parser::records::discard_bytes_field(buf, size, remaining)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imports::*;

    fn header_bytes(record_type: u16, size: u32, data_size: u32) -> Vec<u8> {
        let mut data = vec![];
        data.extend(record_type.to_le_bytes());
        data.extend(0_u16.to_le_bytes());
        data.extend(size.to_le_bytes());
        data.extend(data_size.to_le_bytes());
        data
    }

    #[test]
    fn parses_a_record_header() {
        let data = header_bytes(0x4009, 16, 4);
        let mut buf: &[u8] = &data;

        let (header, consumed) = EmfPlusRecordHeader::parse(&mut buf).unwrap();

        assert_eq!(consumed, EmfPlusRecordHeader::BYTE_SIZE);
        assert_eq!(header.record_type, 0x4009);
        assert_eq!(header.size, 16);
        assert_eq!(header.data_size, 4);
        assert_eq!(header.padding_bytes(), 0);
    }

    #[test]
    fn reports_alignment_padding() {
        let data = header_bytes(0x4008, 20, 5);
        let mut buf: &[u8] = &data;

        let (header, _) = EmfPlusRecordHeader::parse(&mut buf).unwrap();

        assert_eq!(header.padding_bytes(), 3);
    }

    #[test]
    fn rejects_truncated_headers() {
        let data = header_bytes(0x4009, 16, 4);
        let mut buf: &[u8] = &data[..data.len() - 1];

        assert!(EmfPlusRecordHeader::parse(&mut buf).is_err());
    }

    #[test]
    fn rejects_size_smaller_than_the_header() {
        let data = header_bytes(0x4009, 8, 0);
        let mut buf: &[u8] = &data;

        assert!(EmfPlusRecordHeader::parse(&mut buf).is_err());
    }

    #[test]
    fn rejects_unaligned_sizes() {
        let data = header_bytes(0x4009, 13, 1);
        let mut buf: &[u8] = &data;

        assert!(EmfPlusRecordHeader::parse(&mut buf).is_err());
    }

    #[test]
    fn rejects_data_size_exceeding_the_record() {
        let data = header_bytes(0x4009, 16, 8);
        let mut buf: &[u8] = &data;

        assert!(EmfPlusRecordHeader::parse(&mut buf).is_err());
    }

    #[test]
    fn rejects_oversized_records() {
        let data = header_bytes(0x4009, crate::parser::MAX_RECORD_BYTES + 4, 0);
        let mut buf: &[u8] = &data;

        assert!(EmfPlusRecordHeader::parse(&mut buf).is_err());
    }
}
