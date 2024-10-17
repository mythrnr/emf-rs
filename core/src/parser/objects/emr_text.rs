/// The EmrText object contains values for text output.
///
/// If the Options field of the EmrText object contains the ETO_PDY flag, then
/// this buffer contains twice as many values as there are characters in the
/// output string, one horizontal and one vertical offset for each, in that
/// order.
///
/// If ETO_RTLREADING is specified, characters are laid right to left instead of
/// left to right. No other options affect the interpretation of this field.
///
/// The size and encoding of the characters in the OutputString is determined by
/// the type of record that contains this object, as follows:
///
/// - EMR_EXTTEXTOUTA and EMR_POLYTEXTOUTA records: 8-bit ASCII characters.
/// - EMR_EXTTEXTOUTW and EMR_POLYTEXTOUTW records: 16-bit Unicode UTF16-LE
///   characters.
#[derive(Clone, Debug)]
pub struct EmrText {
    /// Reference (8 bytes): A PointL object ([MS-WMF] section 2.2.2.15) that
    /// specifies the coordinates of the reference point used to position the
    /// string. The reference point is defined by the last EMR_SETTEXTALIGN
    /// record. If no such record has been set, the default alignment is
    /// (TA_LEFT, TA_TOP), which is specified using TextAlignmentMode flags
    /// ([MS-WMF] section 2.1.2.3).
    pub reference: wmf_core::parser::PointL,
    /// Chars (4 bytes): An unsigned integer that specifies the number of
    /// characters in the string.
    pub chars: u32,
    /// offString (4 bytes): An unsigned integer that specifies the offset to
    /// the output string in bytes, from the start of the record in which this
    /// object is contained. This value is 8- or 16-bit aligned, according to
    /// the character format.
    pub off_string: u32,
    /// Options (4 bytes): An unsigned integer that specifies how to use the
    /// rectangle specified in the Rectangle field. This field can be a
    /// combination of more than one ExtTextOutOptions enumeration values.
    pub options: std::collections::BTreeSet<crate::parser::ExtTextOutOptions>,
    /// Rectangle (16 bytes, optional): A RectL object ([MS-WMF] section
    /// 2.2.2.19) that defines a clipping and/or opaquing rectangle in logical
    /// units. This rectangle is applied to the text output performed by the
    /// containing record.(In Windows implementations, this is the clipping
    /// and/or opaquing rectangle that is passed to GDI methods ExtTextOutA and
    /// ExtTextOutW.)
    pub rectangle: Option<wmf_core::parser::RectL>,
    /// offDx (4 bytes): An unsigned integer that specifies the offset to an
    /// intercharacter spacing array in bytes, from the start of the record in
    /// which this object is contained. This value is 32-bit aligned.
    pub off_dx: u32,
    /// UndefinedSpace1 (variable, optional): The number of unused bytes. The
    /// OutputString field is not required to follow immediately the preceding
    /// portion of this structure.
    _undefined_space1: Vec<u8>,
    /// StringBuffer (variable): The character string buffer.
    ///
    /// OutputString (variable): An array of characters that specify the string
    /// to output. The location of this field is specified by the value of
    /// offString in bytes from the start of this record. The number of
    /// characters is specified by the value of Chars.
    pub string_buffer: String,
    /// UndefinedSpace2 (variable, optional): The number of unused bytes. The
    /// OutputDx field is not required to follow immediately the preceding
    /// portion of this structure.
    _undefined_space2: Vec<u8>,
    /// DxBuffer (variable, optional): The character spacing buffer.
    ///
    /// OutputDx (variable): An array of 32-bit unsigned integers that specify
    /// the output spacing between the origins of adjacent character cells in
    /// logical units. The location of this field is specified by the value of
    /// offDx in bytes from the start of this record. If spacing is defined,
    /// this field contains the same number of values as characters in the
    /// output string.
    pub dx_buffer: Vec<u32>,
}

impl EmrText {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: &crate::parser::RecordType,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use strum::IntoEnumIterator;

        let (
            (reference, reference_bytes),
            (chars, chars_bytes),
            (off_string, off_string_bytes),
        ) = (
            wmf_core::parser::PointL::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        let (options, options_bytes) = {
            let (v, options_bytes) =
                crate::parser::read_u32_from_le_bytes(buf)?;

            (
                crate::parser::ExtTextOutOptions::iter()
                    .filter(|o| v & (*o as u32) == (*o as u32))
                    .collect::<std::collections::BTreeSet<_>>(),
                options_bytes,
            )
        };
        let mut consumed_bytes =
            reference_bytes + chars_bytes + off_string_bytes + options_bytes;

        let (rectangle, rectangle_bytes) =
            if (off_string as usize) - consumed_bytes >= 20 {
                let (rect, rect_bytes) = wmf_core::parser::RectL::parse(buf)?;
                (Some(rect), rect_bytes)
            } else {
                (None, 0)
            };
        let (off_dx, off_dx_bytes) =
            crate::parser::read_u32_from_le_bytes(buf)?;

        consumed_bytes += rectangle_bytes + off_dx_bytes;

        let (_undefined_space1, _undefined_space1_bytes) =
            crate::parser::read_variable(
                buf,
                (off_string as usize) - consumed_bytes,
            )?;

        consumed_bytes += _undefined_space1_bytes;

        let (string_buffer, string_buffer_bytes) = match record_type {
            crate::parser::RecordType::EMR_EXTTEXTOUTA
            | crate::parser::RecordType::EMR_POLYTEXTOUTA => {
                let (buffer, buffer_bytes) =
                    crate::parser::read_variable(buf, chars as usize)?;
                let string_buffer = std::str::from_utf8(&buffer)
                    .map_err(|err| {
                        crate::parser::ParseError::UnexpectedPattern {
                            cause: err.to_string(),
                        }
                    })?
                    .to_string();

                (string_buffer, buffer_bytes)
            }
            crate::parser::RecordType::EMR_EXTTEXTOUTW
            | crate::parser::RecordType::EMR_POLYTEXTOUTW => {
                let (buffer, buffer_bytes) =
                    crate::parser::read_variable(buf, (2 * chars) as usize)?;
                let buffer: Vec<u16> = buffer
                    .chunks_exact(2)
                    .map(|chunk| {
                        u16::from_le_bytes(
                            chunk.try_into().expect("should be success"),
                        )
                    })
                    .collect();
                let string_buffer =
                    String::from_utf16(&buffer).map_err(|err| {
                        crate::parser::ParseError::UnexpectedPattern {
                            cause: err.to_string(),
                        }
                    })?;

                (string_buffer, buffer_bytes)
            }
            _ => {
                return Err(crate::parser::ParseError::UnexpectedPattern {
                    cause: format!(
                        "EmrText object is expected to be included in \
                         following RecordType: `EMR_EXTTEXTOUTA`, \
                         `EMR_POLYTEXTOUTA`, `EMR_EXTTEXTOUTW`, \
                         `EMR_POLYTEXTOUTW`. But record_type is \
                         {record_type:?}."
                    ),
                });
            }
        };

        consumed_bytes += string_buffer_bytes;

        let (_undefined_space2, _undefined_space2_bytes) =
            crate::parser::read_variable(
                buf,
                (off_dx as usize) - consumed_bytes,
            )?;

        consumed_bytes += _undefined_space2_bytes;

        let (dx_buffer, dx_buffer_bytes) = {
            let length = chars
                * if options
                    .contains(&crate::parser::ExtTextOutOptions::ETO_PDY)
                {
                    2
                } else {
                    1
                };

            let mut values = vec![];
            let mut values_bytes = 0;

            for _ in 0..length {
                let (v, v_bytes) = crate::parser::read_u32_from_le_bytes(buf)?;

                values.push(v);
                values_bytes += v_bytes;
            }

            (values, values_bytes)
        };

        consumed_bytes += dx_buffer_bytes;

        Ok((
            Self {
                reference,
                chars,
                off_string,
                options,
                rectangle,
                off_dx,
                _undefined_space1,
                string_buffer,
                _undefined_space2,
                dx_buffer,
            },
            consumed_bytes,
        ))
    }
}
