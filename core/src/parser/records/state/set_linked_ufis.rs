use crate::imports::*;

/// The EMR_SETLINKEDUFIS record sets the UniversalFontIds of linked fonts to
/// use during character lookup.
#[derive(Clone, Debug)]
pub struct EMR_SETLINKEDUFIS {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETLINKEDUFIS. This value is 0x00000077.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// uNumLinkedUFI (4 bytes): An unsigned integer specifying the number of
    /// UFIs to follow.
    pub u_num_linked_ufi: u32,
    /// ufis (variable): An array of uNumLinkedUFI elements of type
    /// UniversalFontId, which specifies the identifiers of the linked fonts.
    pub ufis: Vec<crate::parser::UniversalFontId>,
    /// Reserved (8 bytes): This field is reserved and MUST be ignored.
    pub reserved: [u8; 8],
}

impl EMR_SETLINKEDUFIS {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = ?record_type),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{
            check_total_points, consume_remaining_bytes, read_array_field,
            read_field, read_with,
        };

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_SETLINKEDUFIS as u32,
        )?;

        let u_num_linked_ufi: u32 = read_field(buf, &mut size)?;

        // `u_num_linked_ufi` is unbounded in the spec; cap it at the
        // same 16 Mi ceiling used for polygon points before it drives
        // `Vec::with_capacity`.
        check_total_points(u_num_linked_ufi)?;

        let ufis = {
            let mut entries = Vec::with_capacity(u_num_linked_ufi as usize);

            for _ in 0..u_num_linked_ufi {
                entries.push(read_with(
                    buf,
                    &mut size,
                    crate::parser::UniversalFontId::parse,
                )?);
            }

            entries
        };

        let reserved: [u8; 8] = read_array_field(buf, &mut size)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, u_num_linked_ufi, ufis, reserved })
    }
}
