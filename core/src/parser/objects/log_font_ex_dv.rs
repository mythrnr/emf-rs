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
        use crate::parser::records::read_with;

        let mut consumed_bytes: usize = 0;
        let log_font_ex = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::LogFontEx::parse,
        )?;

        // The DesignVector sub-record must fit within 72 bytes; track its
        // contribution separately so that bound can be enforced after
        // the read.
        let before_design_vector = consumed_bytes;
        let design_vector = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::DesignVector::parse,
        )?;
        let design_vector_bytes = consumed_bytes - before_design_vector;

        crate::parser::ParseError::expect_le(
            "design_vector size",
            design_vector_bytes,
            72_usize,
        )?;

        Ok((Self { log_font_ex, design_vector }, consumed_bytes))
    }
}

impl From<crate::parser::LogFontPanose> for LogFontExDv {
    fn from(v: crate::parser::LogFontPanose) -> Self {
        Self {
            log_font_ex: crate::parser::LogFontEx {
                log_font: v.log_font,
                full_name: v.full_name,
                style: v.style,
                script: String::new(),
            },
            design_vector: crate::parser::DesignVector {
                signature: 0x08007664,
                num_axes: 0,
                values: vec![],
            },
        }
    }
}
