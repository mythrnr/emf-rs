use crate::imports::*;

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
    pub options: crate::parser::ExtTextOutOptionsFlags,
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
    /// StringBuffer (variable): The character string buffer.
    ///
    /// OutputString (variable): An array of characters that specify the string
    /// to output. The location of this field is specified by the value of
    /// offString in bytes from the start of this record. The number of
    /// characters is specified by the value of Chars.
    pub string_buffer: String,
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
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: &crate::parser::RecordType,
        offset: usize,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{
            check_total_points, read_bytes_field, read_field, read_with,
        };

        let mut consumed_bytes: usize = 0;
        let reference = read_with(
            buf,
            &mut consumed_bytes,
            wmf_core::parser::PointL::parse,
        )?;
        let chars: u32 = read_field(buf, &mut consumed_bytes)?;

        // `chars` is unbounded in the spec; cap it at 16 Mi before
        // it drives `read_bytes_field`'s `Vec::with_capacity(chars *
        // 2)` (UTF-16 path) and the dx_buffer pre-allocation below
        // (`length = chars * {1, 2}`).
        check_total_points(chars)?;

        let off_string: u32 = read_field(buf, &mut consumed_bytes)?;

        let options = crate::parser::ExtTextOutOptionsFlags::from_raw(
            read_field(buf, &mut consumed_bytes)?,
        );

        let rectangle = if (off_string as usize - offset) - consumed_bytes >= 20
        {
            Some(read_with(
                buf,
                &mut consumed_bytes,
                wmf_core::parser::RectL::parse,
            )?)
        } else {
            None
        };
        let off_dx: u32 = read_field(buf, &mut consumed_bytes)?;

        let undefined_space1_len =
            (off_string as usize - offset) - consumed_bytes;
        let _undefined_space1 =
            read_bytes_field(buf, &mut consumed_bytes, undefined_space1_len)?;

        let string_buffer = match record_type {
            crate::parser::RecordType::EMR_EXTTEXTOUTA
            | crate::parser::RecordType::EMR_POLYTEXTOUTA => {
                let buffer =
                    read_bytes_field(buf, &mut consumed_bytes, chars as usize)?;

                str::from_utf8(&buffer)
                    .map_err(|err| {
                        crate::parser::ParseError::UnexpectedPattern {
                            cause: err.to_string().into(),
                        }
                    })?
                    .to_string()
            }
            crate::parser::RecordType::EMR_EXTTEXTOUTW
            | crate::parser::RecordType::EMR_POLYTEXTOUTW => {
                // Multiply in usize so a crafted `chars` close to
                // u32::MAX cannot overflow before being passed to
                // `read_bytes_field`. usize is at least 32-bit on every
                // supported target so the conversion is lossless.
                let buffer = read_bytes_field(
                    buf,
                    &mut consumed_bytes,
                    (chars as usize) * 2,
                )?;

                crate::parser::utf16le_bytes_to_string(&buffer)?
            }
            _ => {
                return Err(crate::parser::ParseError::UnexpectedPattern {
                    cause: format!(
                        "EmrText object is expected to be included in \
                         following RecordType: `EMR_EXTTEXTOUTA`, \
                         `EMR_POLYTEXTOUTA`, `EMR_EXTTEXTOUTW`, \
                         `EMR_POLYTEXTOUTW`. But record_type is \
                         {record_type:?}."
                    )
                    .into(),
                });
            }
        };

        let undefined_space2_len = (off_dx as usize - offset) - consumed_bytes;
        let _undefined_space2 =
            read_bytes_field(buf, &mut consumed_bytes, undefined_space2_len)?;

        let dx_buffer = {
            // Compute in usize to avoid the u32-multiplication
            // overflow that would let a crafted `chars` near
            // MAX_TOTAL_POINTS wrap when ETO_PDY doubles the count.
            let length = (chars as usize)
                * if options.contains(crate::parser::ExtTextOutOptions::ETO_PDY)
                {
                    2
                } else {
                    1
                };

            let mut values: Vec<u32> = Vec::with_capacity(length);

            for _ in 0..length {
                values.push(read_field(buf, &mut consumed_bytes)?);
            }

            values
        };

        Ok((
            Self {
                reference,
                chars,
                off_string,
                options,
                rectangle,
                off_dx,
                string_buffer,
                dx_buffer,
            },
            consumed_bytes,
        ))
    }
}
