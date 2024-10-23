use crate::imports::*;

/// The EMR_SMALLTEXTOUT record outputs a string.
///
/// If ETO_SMALL_CHARS is set in the fuOptions field, TextString contains 8-bit
/// codes for characters, derived from the low bytes of Unicode UTF16-LE
/// character codes, in which the high byte is assumed to be 0.
///
/// If ETO_NO_RECT is set in the fuOptions field, the Bounds field is not
/// included in the record.
#[derive(Clone, Debug)]
pub struct EMR_SMALLTEXTOUT {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SMALLTEXTOUT. This value is 0x0000006C.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// x (4 bytes): A signed integer specifying the x-coordinate of where to
    /// place the string.
    pub x: i32,
    /// y (4 bytes): A signed integer specifying the y-coordinate of where to
    /// place the string.
    pub y: i32,
    /// cChars (4 bytes): An unsigned integer specifying the number of 16-bit
    /// characters in the string. The string is NOT null-terminated.
    pub c_chars: u32,
    /// fuOptions (4 bytes): An unsigned integer specifying the text output
    /// options to use. These options are specified by one or a combination of
    /// values from the ExtTextOutOptions enumeration.
    pub fu_options: BTreeSet<crate::parser::ExtTextOutOptions>,
    /// iGraphicsMode (4 bytes): An unsigned integer specifying the graphics
    /// mode, from the GraphicsMode enumeration.
    pub i_graphics_mode: crate::parser::GraphicsMode,
    /// exScale (4 bytes): A FLOAT value that specifies how much to scale the
    /// text in the x-direction.
    pub ex_scale: f32,
    /// eyScale (4 bytes): A FLOAT value that specifies how much to scale the
    /// text in the y-direction.
    pub ey_scale: f32,
    /// Bounds (16 bytes, optional): A RectL object ([MS-WMF] section 2.2.2.19)
    /// that specifies the bounding rectangle in logical units.
    pub bounds: Option<wmf_core::parser::RectL>,
    /// TextString (variable): A string that contains the text string to draw,
    /// in either 8-bit or 16-bit character codes, according to the value of
    /// the fuOptions field.
    pub text_string: String,
}

impl EMR_SMALLTEXTOUT {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use strum::IntoEnumIterator;

        if record_type != crate::parser::RecordType::EMR_SMALLTEXTOUT {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_SMALLTEXTOUT as u32,
                    record_type as u32
                ),
            });
        }

        let ((x, x_bytes), (y, y_bytes), (c_chars, c_chars_bytes)) = (
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );
        let (fu_options, fu_options_bytes) = {
            let (v, values_bytes) = crate::parser::read_u32_from_le_bytes(buf)?;

            (
                crate::parser::ExtTextOutOptions::iter()
                    .filter(|c| v & (*c as u32) == (*c as u32))
                    .collect::<BTreeSet<_>>(),
                values_bytes,
            )
        };

        size.consume(x_bytes + y_bytes + c_chars_bytes + fu_options_bytes);

        let (
            (i_graphics_mode, i_graphics_mode_bytes),
            (ex_scale, ex_scale_bytes),
            (ey_scale, ey_scale_bytes),
        ) = (
            crate::parser::GraphicsMode::parse(buf)?,
            crate::parser::read_f32_from_le_bytes(buf)?,
            crate::parser::read_f32_from_le_bytes(buf)?,
        );

        size.consume(i_graphics_mode_bytes + ex_scale_bytes + ey_scale_bytes);

        let bounds = if fu_options
            .contains(&crate::parser::ExtTextOutOptions::ETO_NO_RECT)
        {
            None
        } else {
            let (v, b) = wmf_core::parser::RectL::parse(buf)?;
            size.consume(b);

            Some(v)
        };

        let text_string = {
            let (bytes, read_bytes) =
                crate::parser::read_variable(buf, (c_chars * 2) as usize)?;

            size.consume(read_bytes);

            if fu_options
                .contains(&crate::parser::ExtTextOutOptions::ETO_SMALL_CHARS)
            {
                let mut entries = vec![];

                for mut v in bytes.chunks(2) {
                    let (value, _) =
                        crate::parser::read_u16_from_le_bytes(&mut v)?;

                    entries.push(value & 0x00FF);
                }

                String::from_utf16_lossy(&entries)
            } else {
                crate::parser::utf16le_bytes_to_string(&bytes)?
            }
        };

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self {
            record_type,
            size,
            x,
            y,
            c_chars,
            fu_options,
            i_graphics_mode,
            ex_scale,
            ey_scale,
            bounds,
            text_string,
        })
    }
}
