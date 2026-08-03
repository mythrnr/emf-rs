use crate::imports::*;

/// The EmfPlusSetTSClip record specifies clipping areas in the
/// graphics device context for a terminal server.
///
/// The compression scheme for data in this record uses the following
/// algorithm. Each point of each rectangle is encoded in either a
/// single byte or 2 bytes. If the point is encoded in a single byte,
/// the high bit (0x80) of the byte MUST be set, and the value is a
/// signed number represented by the lower 7 bits. If the high bit is
/// not set, then the value is encoded in 2 bytes, with the high-order
/// byte encoded in the 7 lower bits of the first byte, and the
/// low-order byte value encoded in the second byte.
///
/// Each point is encoded as the difference between the point in the
/// current rectangle and the point in the previous rectangle. The
/// bottom point of the rectangle is encoded as the difference between
/// the bottom coordinate and the top coordinate on the current
/// rectangle.
#[derive(Clone, Debug)]
pub struct EmfPlusSetTSClip {
    /// Type (2 bytes): An unsigned integer that identifies this record
    /// type as EmfPlusSetTSClip from the RecordType enumeration. The
    /// value MUST be 0x403A.
    pub record_type: crate::parser::emf_plus::RecordType,
    /// Flags (2 bytes): An unsigned integer that provides information
    /// about how the operation is to be performed, and about the
    /// structure of the record.
    pub flags: u16,
    /// C (1 bit): The format of the rectangle data in the Rects field.
    /// If set, each rectangle is defined in 4 bytes. If clear, each
    /// rectangle is defined in 8 bytes.
    ///
    /// The C bit is the 0x8000 bit of the record flags.
    pub compressed: bool,
    /// NumRects (15 bits): The number of rectangles that are defined
    /// in the Rect field.
    ///
    /// NumRects occupies bits 0-14 of the record flags.
    pub num_rects: u16,
    /// Size (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes in the entire record, including
    /// the 12-byte record header and record-specific data. The
    /// computation of this value is determined by the C bit in the
    /// Flags field, as shown in the following table.
    ///
    /// If the C bit value is 0, NumRects rectangles, consisting of
    /// 8 bytes each, are defined in the Rects field, and Size is
    /// computed as follows:
    ///
    /// Size = (NumRects * 0x00000008) + 0x0000000C
    ///
    /// If the C bit value is 1, NumRects rectangles, consisting of
    /// 4 bytes each, are defined in the Rects field, and Size is
    /// computed as follows:
    ///
    /// Size = (NumRects * 0x00000004) + 0x0000000C
    pub size: u32,
    /// DataSize (4 bytes): An unsigned integer that specifies the
    /// 32-bit-aligned number of bytes of record-specific data that
    /// follows. The computation of this value is determined by the C
    /// bit in the Flags field, as shown in the following table.
    ///
    /// If the C bit value is 0, NumRects rectangles, consisting of
    /// 8 bytes each, are defined in the Rects field, and DataSize is
    /// computed as follows:
    ///
    /// DataSize = NumRects * 0x00000008
    ///
    /// If the C bit value is 1, NumRects rectangles, consisting of
    /// 4 bytes each, are defined in the Rects field, and DataSize is
    /// computed as follows:
    ///
    /// DataSize = NumRects * 0x00000004
    pub data_size: crate::parser::Size,
    /// Rects (variable): An array of NumRects rectangles that define
    /// clipping areas. The format of this data is determined by the C
    /// bit in the Flags field.
    ///
    /// The data is kept undecoded: the compression scheme description
    /// in the specification does not reconcile with the fixed
    /// 4-bytes-per-rectangle size formula of the same section, and no
    /// mainstream implementation (GDI+, LibreOffice, Wine) writes or
    /// decodes this record, so there is no behavior to validate a
    /// decoder against.
    pub rects_data: Vec<u8>,
}

impl EmfPlusSetTSClip {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = ?record_type),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::emf_plus::RecordType,
        flags: u16,
        size: u32,
        mut data_size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::read_bytes_field;

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u16,
            crate::parser::emf_plus::RecordType::EmfPlusSetTSClip as u16,
        )?;

        let remaining = data_size.remaining_bytes();
        let rects_data = read_bytes_field(buf, &mut data_size, remaining)?;

        Ok(Self {
            record_type,
            flags,
            compressed: flags & 0x8000 != 0,
            num_rects: flags & 0x7FFF,
            size,
            data_size,
            rects_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_compressed_rectangle_data() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusSetTSClip::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusSetTSClip,
            0x8002,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert_eq!(
            record.record_type,
            crate::parser::emf_plus::RecordType::EmfPlusSetTSClip,
        );
        assert_eq!(record.flags, 0x8002);
        assert_eq!(record.size, 20);
        assert!(record.compressed);
        assert_eq!(record.num_rects, 2);
        assert_eq!(record.rects_data, data);
    }

    #[test]
    fn parses_uncompressed_rectangle_data() {
        let data = [0_u8; 8];
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);
        let record = EmfPlusSetTSClip::parse(
            &mut buf,
            crate::parser::emf_plus::RecordType::EmfPlusSetTSClip,
            0x0001,
            u32::try_from(data.len()).unwrap() + 12,
            data_size,
        )
        .unwrap();

        assert!(!record.compressed);
        assert_eq!(record.num_rects, 1);
        assert_eq!(record.rects_data, data);
    }

    #[test]
    fn rejects_wrong_record_type() {
        let data = [0_u8; 8];
        let mut buf: &[u8] = &data;
        let data_size = crate::parser::Size::from(data.len() as u32);

        assert!(
            EmfPlusSetTSClip::parse(
                &mut buf,
                crate::parser::emf_plus::RecordType::EmfPlusSetTSGraphics,
                0x0001,
                u32::try_from(data.len()).unwrap() + 12,
                data_size,
            )
            .is_err()
        );
    }
}
