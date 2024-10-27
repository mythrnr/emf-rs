use crate::imports::*;

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
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
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

impl From<crate::parser::LogFontPanose> for LogFontExDv {
    fn from(v: crate::parser::LogFontPanose) -> Self {
        Self {
            log_font_ex: crate::parser::LogFontEx {
                log_font: v.log_font,
                full_name: v.full_name,
                style: v.style,
                script: "".to_owned(),
            },
            design_vector: crate::parser::DesignVector {
                signature: 0x08007664,
                num_axes: 0,
                values: vec![],
            },
        }
    }
}
