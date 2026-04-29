use crate::imports::*;

/// The EMR_EOF record contains arbitrary private data.
#[derive(Clone, Debug)]
pub struct EMR_EOF {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_EOF. This value is 0x0000000E.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// nPalEntries (4 bytes): An unsigned integer that specifies the number of
    /// palette entries.
    pub n_pal_entries: u32,
    /// offPalEntries (4 bytes): An unsigned integer that specifies the offset
    /// to the palette entries from the start of this record.
    pub off_pal_entries: u32,
    /// PaletteBuffer (variable, optional): An array of bytes that contains
    /// palette data, which is not required to be contiguous with the
    /// fixed-length portion of the EMR_EOF record. Thus, fields in this buffer
    /// that are labeled "UndefinedSpace" are optional and MUST be ignored.
    ///
    /// PaletteEntries (variable): An array of LogPaletteEntry objects that
    /// specifies the palette data.
    pub palette_buffer: Vec<crate::parser::LogPaletteEntry>,
    /// SizeLast (4 bytes): An unsigned integer that MUST be the same as Size
    /// and MUST be the last field of the record and hence the metafile.
    /// LogPaletteEntry objects, if they exist, MUST precede this field.
    pub size_last: u32,
}

impl EMR_EOF {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{
            check_total_points, consume_remaining_bytes, discard_bytes_field,
            read_field, read_with,
        };

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_EOF as u32,
        )?;

        let n_pal_entries: u32 = read_field(buf, &mut size)?;
        let off_pal_entries: u32 = read_field(buf, &mut size)?;

        // `n_pal_entries` is unbounded in the spec; cap it before
        // driving `Vec::with_capacity`.
        check_total_points(n_pal_entries)?;

        let palette_buffer = if off_pal_entries > 0 {
            let undef_offset = size.checked_offset(off_pal_entries)?;
            discard_bytes_field(buf, &mut size, undef_offset)?;

            let palette_buffer = {
                let mut entries = Vec::with_capacity(n_pal_entries as usize);

                for _ in 0..n_pal_entries {
                    entries.push(read_with(
                        buf,
                        &mut size,
                        crate::parser::LogPaletteEntry::parse,
                    )?);
                }

                entries
            };

            let trailing = size.remaining_bytes() - 4;
            discard_bytes_field(buf, &mut size, trailing)?;

            palette_buffer
        } else {
            vec![]
        };

        let size_last = read_field(buf, &mut size)?;

        // if size.byte_count() as u32 != size_last {
        //     warn!(
        //         size = %size.byte_count(),
        //         %size_last,
        //         "size and size_last must be same value",
        //     );
        // }

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self {
            record_type,
            size,
            n_pal_entries,
            off_pal_entries,
            palette_buffer,
            size_last,
        })
    }
}
