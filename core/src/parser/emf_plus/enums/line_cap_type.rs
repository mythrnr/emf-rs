/// The LineCapType enumeration defines types of line caps to use at the
/// ends of lines that are drawn with graphics pens (MS-EMFPLUS 2.1.1.18).
///
/// Graphics line caps are specified by EmfPlusPen objects.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    strum::FromRepr,
    strum::EnumIter,
)]
#[repr(u32)]
pub enum LineCapType {
    /// A squared-off line cap. The end of the line MUST be the last
    /// point in the line.
    LineCapTypeFlat = 0x00000000,
    /// A square line cap. The center of the square MUST be located at
    /// the last point in the line. The width of the square is the line
    /// width.
    LineCapTypeSquare = 0x00000001,
    /// A circular line cap. The center of the circle MUST be located at
    /// the last point in the line. The diameter of the circle is the
    /// line width.
    LineCapTypeRound = 0x00000002,
    /// A triangular line cap. The base of the triangle MUST be located
    /// at the last point in the line. The base of the triangle is the
    /// line width.
    LineCapTypeTriangle = 0x00000003,
    /// The line end is not anchored.
    LineCapTypeNoAnchor = 0x00000010,
    /// The line end is anchored with a square line cap. The center of
    /// the square MUST be located at the last point in the line. The
    /// height and width of the square are the line width.
    LineCapTypeSquareAnchor = 0x00000011,
    /// The line end is anchored with a circular line cap. The center of
    /// the circle MUST be located at the last point in the line. The
    /// circle SHOULD be wider than the line.
    LineCapTypeRoundAnchor = 0x00000012,
    /// The line end is anchored with a diamond-shaped line cap, which
    /// is a square turned at 45 degrees. The center of the diamond MUST
    /// be located at the last point in the line. The diamond SHOULD be
    /// wider than the line.
    LineCapTypeDiamondAnchor = 0x00000013,
    /// The line end is anchored with an arrowhead shape. The arrowhead
    /// point MUST be located at the last point in the line. The
    /// arrowhead SHOULD be wider than the line.
    LineCapTypeArrowAnchor = 0x00000014,
    /// Mask used to check whether a line cap is an anchor cap.
    LineCapTypeAnchorMask = 0x000000F0,
    /// A custom line cap.
    LineCapTypeCustom = 0x000000FF,
}

crate::parser::enums::impl_parser!(LineCapType, u32);
