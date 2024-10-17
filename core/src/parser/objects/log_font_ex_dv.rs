/// The LogFontExDv object specifies the design vector for an extended logical
/// font.
#[derive(Clone, Debug)]
pub struct LogFontExDv {
    /// LogFontEx (348 bytes): A LogFontEx object that specifies the extended
    /// attributes of the logical font.
    pub log_font_ex: crate::parser::LogFontEx,
    /// DesignVector (variable): A DesignVector object. This field MUST NOT be
    /// longer than 72 bytes.
    ///
    /// A design vector SHOULD be specified only for a multiple master OpenType
    /// font.
    pub design_vector: crate::parser::DesignVector,
}

impl LogFontExDv {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (log_font_ex, log_font_ex_bytes),
            (design_vector, design_vector_bytes),
        ) = (
            crate::parser::LogFontEx::parse(buf)?,
            crate::parser::DesignVector::parse(buf)?,
        );

        if design_vector_bytes > 72 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: "Length of DesignVector must no be longer than 72 bytes"
                    .to_owned(),
            });
        }

        Ok((
            Self { log_font_ex, design_vector },
            log_font_ex_bytes + design_vector_bytes,
        ))
    }
}
