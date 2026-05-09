/// The Panose object describes the PANOSE font-classification values for a
/// TrueType font. These characteristics are used to associate the font with
/// other fonts of similar appearance but different names.
#[derive(Clone, Debug)]
pub struct Panose {
    /// FamilyType (1 byte): An unsigned integer that specifies the family
    /// type. This value is in the FamilyType enumeration table.
    pub family_type: crate::parser::FamilyType,
    /// SerifStyle (1 byte): An unsigned integer that specifies the serif
    /// style. This value is in the SerifType enumeration table.
    pub serif_style: crate::parser::SerifType,
    /// Weight (1 byte): An unsigned integer that specifies the weight of the
    /// font. This value is in the Weight enumeration table.
    pub weight: crate::parser::Weight,
    /// Proportion (1 byte): An unsigned integer that specifies the proportion
    /// of the font. This value is in the Proportion enumeration table.
    pub proportion: crate::parser::Proportion,
    /// Contrast (1 byte): An unsigned integer that specifies the contrast of
    /// the font. This value is in the Contrast enumeration table.
    pub contrast: crate::parser::Contrast,
    /// StrokeVariation (1 byte): An unsigned integer that specifies the stroke
    /// variation for the font. This value is in the StrokeVariation
    /// enumeration table.
    pub stroke_variation: crate::parser::StrokeVariation,
    /// ArmStyle (1 byte): An unsigned integer that specifies the arm style of
    /// the font. This value is in the ArmStyle enumeration table.
    pub arm_style: crate::parser::ArmStyle,
    /// Letterform (1 byte): An unsigned integer that specifies the letterform
    /// of the font. This value is in the Letterform enumeration table.
    pub letterform: crate::parser::Letterform,
    /// Midline (1 byte): An unsigned integer that specifies the midline of the
    /// font. This value is in the MidLine enumeration table.
    pub midline: crate::parser::MidLine,
    /// XHeight (1 byte): An unsigned integer that specifies the x height of
    /// the font. This value is in the XHeight enumeration table.
    pub x_height: crate::parser::XHeight,
}

impl Panose {
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
        let family_type = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::FamilyType::parse,
        )?;
        let serif_style = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::SerifType::parse,
        )?;
        let weight =
            read_with(buf, &mut consumed_bytes, crate::parser::Weight::parse)?;
        let proportion = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::Proportion::parse,
        )?;
        let contrast = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::Contrast::parse,
        )?;
        let stroke_variation = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::StrokeVariation::parse,
        )?;
        let arm_style = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::ArmStyle::parse,
        )?;
        let letterform = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::Letterform::parse,
        )?;
        let midline =
            read_with(buf, &mut consumed_bytes, crate::parser::MidLine::parse)?;
        let x_height =
            read_with(buf, &mut consumed_bytes, crate::parser::XHeight::parse)?;

        Ok((
            Self {
                family_type,
                serif_style,
                weight,
                proportion,
                contrast,
                stroke_variation,
                arm_style,
                letterform,
                midline,
                x_height,
            },
            consumed_bytes,
        ))
    }
}
