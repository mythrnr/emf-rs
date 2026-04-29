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
        use strum::IntoEnumIterator;

        use crate::parser::records::{
            consume_remaining_bytes, read_bytes_field, read_field, read_with,
        };

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_SMALLTEXTOUT as u32,
        )?;

        let x = read_field(buf, &mut size)?;
        let y = read_field(buf, &mut size)?;
        let c_chars: u32 = read_field(buf, &mut size)?;
        let fu_options = {
            let v: u32 = read_field(buf, &mut size)?;

            crate::parser::ExtTextOutOptions::iter()
                .filter(|c| v & (*c as u32) == (*c as u32))
                .collect::<BTreeSet<_>>()
        };

        let i_graphics_mode =
            read_with(buf, &mut size, crate::parser::GraphicsMode::parse)?;
        let ex_scale = read_field(buf, &mut size)?;
        let ey_scale = read_field(buf, &mut size)?;

        let bounds = if fu_options
            .contains(&crate::parser::ExtTextOutOptions::ETO_NO_RECT)
        {
            None
        } else {
            Some(read_with(buf, &mut size, wmf_core::parser::RectL::parse)?)
        };

        let text_string = {
            // Multiply in usize so a crafted `c_chars` close to
            // u32::MAX cannot overflow before being passed to
            // `read_bytes_field`.
            let bytes =
                read_bytes_field(buf, &mut size, (c_chars as usize) * 2)?;

            if fu_options
                .contains(&crate::parser::ExtTextOutOptions::ETO_SMALL_CHARS)
            {
                let mut entries = vec![];

                for mut v in bytes.chunks(2) {
                    let (value, _) =
                        <u16 as crate::parser::ReadLeField>::read_le(&mut v)?;

                    entries.push(value & 0x00FF);
                }

                String::from_utf16_lossy(&entries)
            } else {
                crate::parser::utf16le_bytes_to_string(&bytes)?
            }
        };

        consume_remaining_bytes(buf, size.remaining_bytes())?;

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
