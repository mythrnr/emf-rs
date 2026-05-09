//! Implementation of the definitions in Section 2.3.6 of the EMF
//! specifications.

mod draw_escape;
mod ext_escape;
mod named_escape;

pub use self::{draw_escape::*, ext_escape::*, named_escape::*};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{RecordType, Size};

    /// Returns a `Size` whose `byte_count` matches the full record
    /// length and whose consumed counter is already advanced past the
    /// 8-byte `Type` + `Size` header (matching the converter's actual
    /// flow).
    fn record_size(byte_count: u32) -> Size {
        let mut size = Size::from(byte_count);
        size.consume(8);
        size
    }

    #[test]
    fn draw_escape_empty_payload_parses() {
        // i_escape = NEWFRAME (0x0001 as u16); cj_in = 0; no Data.
        // Total record bytes = 8 (header) + 2 + 4 = 14.
        let mut buf: &[u8] = &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let result = EMR_DRAWESCAPE::parse(
            &mut buf,
            RecordType::EMR_DRAWESCAPE,
            record_size(14 + 2),
        );
        assert!(result.is_ok(), "{:?}", result.err());
        let r = result.unwrap();
        assert_eq!(r.cj_in, 0);
        assert!(r.data.is_empty());
    }

    #[test]
    fn ext_escape_empty_payload_parses() {
        let mut buf: &[u8] = &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let result = EMR_EXTESCAPE::parse(
            &mut buf,
            RecordType::EMR_EXTESCAPE,
            record_size(14 + 2),
        );
        assert!(result.is_ok(), "{:?}", result.err());
        let r = result.unwrap();
        assert_eq!(r.cj_in, 0);
        assert!(r.data.is_empty());
    }

    #[test]
    fn named_escape_minimal_driver_name_parses() {
        // i_escape = NEWFRAME; cj_driver = 2 (single null terminator);
        // cj_in = 0; driver_name = "" (just the null terminator);
        // no Data.
        let mut buf: &[u8] = &[
            0x01, 0x00, // i_escape
            0x02, 0x00, 0x00, 0x00, // cj_driver = 2
            0x00, 0x00, 0x00, 0x00, // cj_in = 0
            0x00, // driver_name terminator (null)
        ];
        let result = EMR_NAMEDESCAPE::parse(
            &mut buf,
            RecordType::EMR_NAMEDESCAPE,
            record_size(8 + 2 + 4 + 4 + 1),
        );
        assert!(result.is_ok(), "{:?}", result.err());
        let r = result.unwrap();
        assert_eq!(r.cj_in, 0);
        assert!(r.data.is_empty());
        assert_eq!(r.driver_name, "");
    }
}
