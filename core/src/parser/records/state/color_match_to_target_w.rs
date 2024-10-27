use crate::imports::*;

/// The EMR_COLORMATCHTOTARGETW record specifies whether to perform color
/// matching with a color profile that is specified in a file with a name
/// consisting of Unicode characters.
///
/// Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, and Windows NT 4.0 do not
/// support EMR_COLORMATCHTOTARGETW.
///
/// An EMR_COLORMATCHTOTARGETW record can be used to control whether to apply
/// the current color transform to subsequent graphics operations. If the
/// dwAction field value is CS_ENABLE, color mapping is enabled, and the current
/// color transform SHOULD be applied. If dwAction is set to CS_DISABLE, the
/// color transform SHOULD NOT be applied.
///
/// Before applying the current color transform, WCS SHOULD be enabled in the
/// playback device context. On Windows NT 4.0, Windows 2000, Windows XP, and
/// Windows Server 2003, WCS is not enabled in the playback device context
/// before applying the current color transform.
///
/// While color mapping to the target is enabled by a dwAction value of
/// CS_ENABLE, changes to the color space or color gamut mapping are not
/// applied. However, those changes MUST take effect when color mapping to the
/// target is disabled.
///
/// The dwAction field SHOULD NOT be set to CS_DELETE_TRANSFORM unless color
/// management has already been enabled with an EMR_SETICMMODE record.
#[derive(Clone, Debug)]
pub struct EMR_COLORMATCHTOTARGETW {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_COLORMATCHTOTARGETW. This value is 0x00000079.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// dwAction (4 bytes): An unsigned integer that specifies a value from the
    /// ColorSpace enumeration.
    pub dw_action: crate::parser::ColorSpace,
    /// dwFlags (4 bytes): An unsigned integer that specifies a value from the
    /// ColorMatchToTarget enumeration.
    pub dw_flags: crate::parser::ColorMatchToTarget,
    /// cbName (4 bytes): An unsigned integer that specifies the number of
    /// bytes in the Unicode UTF16-LE name of the target color profile.
    pub cb_name: u32,
    /// cbData (4 bytes): An unsigned integer that specifies the size of the
    /// raw data of the target color profile in the Data field.
    pub cb_data: u32,
    /// Data (variable): An array of size (cbName + cbData) bytes, which
    /// specifies the UTF16-LE name and raw data of the target color profile.
    pub data: Vec<u8>,
}

impl EMR_COLORMATCHTOTARGETW {
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
        if record_type != crate::parser::RecordType::EMR_COLORMATCHTOTARGETW {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_COLORMATCHTOTARGETW as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (dw_action, dw_action_bytes),
            (dw_flags, dw_flags_bytes),
            (cb_name, cb_name_bytes),
            (cb_data, cb_data_bytes),
        ) = (
            crate::parser::ColorSpace::parse(buf)?,
            crate::parser::ColorMatchToTarget::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        size.consume(
            dw_action_bytes + dw_flags_bytes + cb_name_bytes + cb_data_bytes,
        );

        let (data, data_bytes) =
            crate::parser::read_variable(buf, (cb_name + cb_data) as usize)?;

        size.consume(data_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self {
            record_type,
            size,
            dw_action,
            dw_flags,
            cb_name,
            cb_data,
            data,
        })
    }
}
