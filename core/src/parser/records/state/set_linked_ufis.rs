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
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_SETLINKEDUFIS {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SETLINKEDUFIS as u32,
                    record_type as u32
                ),
            });
        }

        let (u_num_linked_ufi, u_num_linked_ufi_bytes) =
            crate::parser::read_u32_from_le_bytes(buf)?;

        size.consume(u_num_linked_ufi_bytes);

        let ufis = {
            let mut entries = vec![];

            for _ in 0..u_num_linked_ufi {
                let (v, b) = crate::parser::UniversalFontId::parse(buf)?;

                entries.push(v);
                size.consume(b);
            }

            entries
        };

        let (reserved, reserved_bytes) = crate::parser::read::<_, 8>(buf)?;

        size.consume(reserved_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, u_num_linked_ufi, ufis, reserved })
    }
}
