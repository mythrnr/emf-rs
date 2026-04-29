use crate::imports::*;

/// The EMR_SETTEXTALIGN record specifies text alignment for text drawing.
///
/// The EMR_SMALLTEXTOUT, EMR_EXTTEXTOUTA, and EMR_EXTTEXTOUTW records use text
/// alignment values to position a string of text on the output medium. The
/// values specify the relationship between a reference point and a rectangle
/// that bounds the text. The reference point is either the current drawing
/// position or a point passed to a text output record.
///
/// The rectangle that bounds the text is formed by the character cells in the
/// text string.
#[derive(Clone, Debug)]
pub struct EMR_SETTEXTALIGN {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_SETTEXTALIGN. This value is 0x00000016.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size of this
    /// record in bytes.
    pub size: crate::parser::Size,
    /// TextAlignmentMode (4 bytes): An unsigned integer that specifies text
    /// alignment by using a mask of text alignment flags. These are either
    /// TextAlignmentMode flags ([MS-WMF] section 2.1.2.3) for text with a
    /// horizontal baseline, or VerticalTextAlignmentMode flags ([MS-WMF]
    /// section 2.1.2.4) for text with a vertical baseline. Only one value can
    /// be chosen from those that affect horizontal and vertical alignment.
    pub text_alignment_mode: u32,
}

impl EMR_SETTEXTALIGN {
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
        use crate::parser::records::{consume_remaining_bytes, read_field};

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_SETTEXTALIGN as u32,
        )?;

        let text_alignment_mode = read_field(buf, &mut size)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size, text_alignment_mode })
    }

    pub fn horizontal_baseline(
        &self,
    ) -> BTreeSet<wmf_core::parser::TextAlignmentMode> {
        use strum::IntoEnumIterator;

        wmf_core::parser::TextAlignmentMode::iter()
            .filter(|c| {
                self.text_alignment_mode as u16 & (*c as u16) == (*c as u16)
            })
            .collect()
    }

    pub fn vertical_baseline(
        &self,
    ) -> BTreeSet<wmf_core::parser::VerticalTextAlignmentMode> {
        use strum::IntoEnumIterator;

        wmf_core::parser::VerticalTextAlignmentMode::iter()
            .filter(|c| {
                self.text_alignment_mode as u16 & (*c as u16) == (*c as u16)
            })
            .collect()
    }
}
