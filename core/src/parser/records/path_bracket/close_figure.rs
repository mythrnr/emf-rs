/// This record closes the figure in path bracket construction.
///
/// Processing the EMR_CLOSEFIGURE record closes the figure by drawing a line
/// from the current drawing position to the first point of the figure, and then
/// it connects the lines by using the current line join. If the figure is
/// closed by processing an EMR_LINETO record instead of this record, the
/// current line cap is used to create the corner instead of the line join. The
/// line parameters are specified by the PenStyle field in the current LogPen
/// and LogPenEx objects.
///
/// The EMR_CLOSEFIGURE record SHOULD be used only if there is an open figure in
/// the path bracket. A figure in a path is open unless it is explicitly closed
/// by processing this record. A figure can be open even if the current point is
/// the same as the starting point.
///
/// After processing the EMR_CLOSEFIGURE record, adding a line or curve to the
/// path bracket starts a new figure.
#[derive(Clone, Debug)]
pub struct EMR_CLOSEFIGURE {
    /// Type (4 bytes): An unsigned integer that identifies this record type
    /// from the RecordType enumeration. It MUST be EMR_CLOSEFIGURE, which is
    /// 0x0000003D.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. For path bracket records, this value is
    /// 0x00000008.
    pub size: crate::parser::Size,
}

impl EMR_CLOSEFIGURE {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::consume_remaining_bytes;

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_CLOSEFIGURE as u32,
        )?;
        crate::parser::ParseError::expect_eq(
            "size field",
            size.byte_count() as u32,
            0x00000008_u32,
        )?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self { record_type, size })
    }
}
