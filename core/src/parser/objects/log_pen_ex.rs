use crate::imports::*;

/// The LogPen object defines the style, width, and color of a logical pen.
#[derive(Clone, Debug)]
pub struct LogPenEx {
    /// PenStyle (4 bytes): An unsigned integer that specifies the pen style.
    /// This value is defined from the PenStyle enumeration.
    ///
    /// The pen style is a combination of pen type, line style, line cap, and
    /// line join.
    pub pen_style: BTreeSet<crate::parser::PenStyle>,
    /// An unsigned integer that specifies the width of the line drawn by the
    /// pen.
    ///
    /// If the pen type in the PenStyle field is PS_GEOMETRIC, this value is
    /// the width in logical units; otherwise, the width is specified in device
    /// units. If the pen type in the PenStyle field is PS_COSMETIC, this value
    /// MUST be 0x00000001.
    pub width: u32,
    /// BrushStyle (4 bytes): An unsigned integer that specifies a brush style
    /// for the pen from the BrushStyle enumeration ([MS-WMF] section 2.1.1.4).
    ///
    /// If the pen type in the PenStyle field is PS_GEOMETRIC, this value is
    /// either BS_SOLID or BS_HATCHED. The value of this field can be BS_NULL,
    /// but only if the line style specified in PenStyle is PS_NULL. The
    /// BS_NULL style SHOULD be used to specify a brush that has no effect.
    ///
    /// ColorRef (4 bytes): A ColorRef object ([MS-WMF] section 2.2.2.8). The
    /// interpretation of this field depends on the BrushStyle value, as shown
    /// in the table later in this section.
    ///
    /// BrushHatch (4 bytes): The brush hatch pattern. The definition of this
    /// field depends on the BrushStyle value, as shown in the table later in
    /// this section.
    pub brush: LogPenExBrush,
    /// NumStyleEntries (4 bytes): The number of elements in the array
    /// specified in the StyleEntry field. This value SHOULD be zero if
    /// PenStyle does not specify PS_USERSTYLE.
    pub num_style_entries: u32,
    /// StyleEntry (variable, optional): An array of 32-bit unsigned integers
    /// that defines the lengths of dashes and gaps in the line drawn by this
    /// pen when the value of PenStyle is PS_USERSTYLE. The array contains the
    /// number of entries specified by NumStyleEntries, but it is used as if it
    /// repeated indefinitely.
    ///
    /// The first entry in the StyleEntry array specifies the length of the
    /// first dash. The second entry specifies the length of the first gap.
    /// Thereafter, lengths of dashes and gaps alternate.
    ///
    /// If the pen type in the PenStyle field is PS_GEOMETRIC, lengths are
    /// specified in logical units; otherwise, they are specified in device
    /// units.
    ///
    /// The LogPenEx object includes the specification of brush attributes, so
    /// it can be used to draw lines that consist of custom or predefined
    /// patterns. The following table shows the relationship between the
    /// BrushStyle, ColorRef, and BrushHatch fields in this object. Only
    /// supported brush styles are listed.
    pub style_entry: Vec<u32>,
}

impl LogPenEx {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use strum::IntoEnumIterator;

        let (pen_style, pen_style_bytes) = {
            let (v, values_bytes) = crate::parser::read_u32_from_le_bytes(buf)?;

            (
                crate::parser::PenStyle::iter()
                    .filter(|c| v & (*c as u32) == (*c as u32))
                    .collect::<BTreeSet<_>>(),
                values_bytes,
            )
        };
        let (
            (width, width_bytes),
            (brush, brush_bytes),
            (num_style_entries, num_style_entries_bytes),
        ) = (
            crate::parser::read_u32_from_le_bytes(buf)?,
            LogPenExBrush::parse(buf, &pen_style)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        let (style_entry, style_entry_bytes) = {
            let mut entries = vec![];
            let mut bytes = 0;

            for _ in 0..num_style_entries {
                let (v, b) = crate::parser::read_u32_from_le_bytes(buf)?;

                entries.push(v);
                bytes += b;
            }

            (entries, bytes)
        };

        Ok((
            Self { pen_style, width, brush, num_style_entries, style_entry },
            pen_style_bytes
                + width_bytes
                + brush_bytes
                + num_style_entries_bytes
                + style_entry_bytes,
        ))
    }

    pub fn black_pen() -> Self {
        Self {
            pen_style: BTreeSet::from_iter([crate::parser::PenStyle::PS_SOLID]),
            width: 1,
            brush: LogPenExBrush::Solid {
                color_ref: wmf_core::parser::ColorRef::black(),
            },
            num_style_entries: 0,
            style_entry: vec![],
        }
    }

    pub fn white_pen() -> Self {
        Self {
            pen_style: BTreeSet::from_iter([crate::parser::PenStyle::PS_SOLID]),
            width: 1,
            brush: LogPenExBrush::Solid {
                color_ref: wmf_core::parser::ColorRef::white(),
            },
            num_style_entries: 0,
            style_entry: vec![],
        }
    }

    pub fn null_pen() -> Self {
        Self {
            pen_style: BTreeSet::from_iter([crate::parser::PenStyle::PS_NULL]),
            width: 0,
            brush: LogPenExBrush::Null,
            num_style_entries: 0,
            style_entry: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub enum LogPenExBrush {
    Solid {
        /// A ColorRef object that specifies the color of lines drawn by the
        /// pen.
        color_ref: wmf_core::parser::ColorRef,
    },
    Null,
    Hatched {
        /// A ColorRef object that specifies the foreground color of the hatch
        /// pattern.
        color_ref: wmf_core::parser::ColorRef,
        /// A value from the HatchStyle enumeration that specifies the
        /// orientation of lines used to create the hatch. If PS_GEOMETRIC is
        /// not set in the PenStyle field, this field MUST be either
        /// HS_SOLIDTEXTCLR (`0x0008`) or HS_SOLIDBKCLR (`0x000A`).
        brush_hatch: crate::parser::HatchStyle,
    },
    Pattern {
        /// The low-order 16-bits is a value from the ColorUsage enumeration
        /// ([MS-WMF] section 2.1.1.6).
        color_usage: wmf_core::parser::ColorUsage,
    },
    DIBPattern {
        /// The low-order 16 bits is a value from the ColorUsage enumeration.
        color_usage: wmf_core::parser::ColorUsage,
    },
    DIBPatternPT {
        /// The low-order word is be a value from the ColorUsage enumeration.
        color_usage: wmf_core::parser::ColorUsage,
    },
}

impl LogPenExBrush {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn parse<R: crate::Read>(
        buf: &mut R,
        pen_style: &BTreeSet<crate::parser::PenStyle>,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (brush_style, brush_style_bytes),
            (color, color_bytes),
            (brush_hatch, brush_hatch_bytes),
        ) = (
            wmf_core::parser::BrushStyle::parse(buf)?,
            crate::parser::read::<_, 4>(buf)?,
            crate::parser::read::<_, 4>(buf)?,
        );

        let v = match brush_style {
            wmf_core::parser::BrushStyle::BS_SOLID => {
                let mut b = &color[..];
                let (color_ref, _) = wmf_core::parser::ColorRef::parse(&mut b)?;

                Self::Solid { color_ref }
            }
            wmf_core::parser::BrushStyle::BS_NULL => Self::Null,
            wmf_core::parser::BrushStyle::BS_HATCHED => {
                let mut b = &color[..];
                let (color_ref, _) = wmf_core::parser::ColorRef::parse(&mut b)?;
                let mut b = &brush_hatch[..];
                let (brush_hatch, _) =
                    crate::parser::HatchStyle::parse(&mut b)?;

                if !pen_style.contains(&crate::parser::PenStyle::PS_GEOMETRIC)
                    && !matches!(
                        brush_hatch,
                        crate::parser::HatchStyle::HS_SOLIDTEXTCLR
                            | crate::parser::HatchStyle::HS_SOLIDBKCLR
                    )
                {
                    return Err(crate::parser::ParseError::NotSupported {
                        cause: format!(
                            "If PS_GEOMETRIC is not set in the PenStyle \
                             field, this field MUST be either HS_SOLIDTEXTCLR \
                             (0x0008) or HS_SOLIDBKCLR (0x000A). but \
                             HatchStyle is {brush_hatch:?}"
                        ),
                    });
                }

                Self::Hatched { color_ref, brush_hatch }
            }
            wmf_core::parser::BrushStyle::BS_PATTERN => {
                let mut b = &color[2..];
                let (color_usage, _) =
                    wmf_core::parser::ColorUsage::parse(&mut b)?;

                Self::Pattern { color_usage }
            }
            wmf_core::parser::BrushStyle::BS_DIBPATTERN => {
                let mut b = &color[2..];
                let (color_usage, _) =
                    wmf_core::parser::ColorUsage::parse(&mut b)?;

                Self::DIBPattern { color_usage }
            }
            wmf_core::parser::BrushStyle::BS_DIBPATTERNPT => {
                let mut b = &color[2..];
                let (color_usage, _) =
                    wmf_core::parser::ColorUsage::parse(&mut b)?;

                Self::DIBPatternPT { color_usage }
            }
            _ => {
                return Err(crate::parser::ParseError::NotSupported {
                    cause: format!("Unsupported BrushStyle {brush_style:?}"),
                });
            }
        };

        Ok((v, brush_style_bytes + color_bytes + brush_hatch_bytes))
    }
}

impl From<crate::parser::LogPen> for LogPenEx {
    fn from(v: crate::parser::LogPen) -> Self {
        Self {
            pen_style: BTreeSet::from_iter([v.pen_style]),
            width: v.width.x.abs() as u32,
            brush: LogPenExBrush::Solid { color_ref: v.color_ref },
            num_style_entries: 0,
            style_entry: vec![],
        }
    }
}
