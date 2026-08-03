/// The HatchStyle enumeration defines hatch patterns used by graphics
/// brushes (MS-EMFPLUS 2.1.1.13). A hatch pattern consists of a solid
/// background color and lines drawn over the background.
///
/// Graphics brushes are specified by EmfPlusBrush objects.
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
pub enum HatchStyle {
    /// Equally spaced horizontal lines.
    HatchStyleHorizontal = 0x00000000,
    /// Equally spaced vertical lines.
    HatchStyleVertical = 0x00000001,
    /// Lines on a diagonal from upper left to lower right.
    HatchStyleForwardDiagonal = 0x00000002,
    /// Lines on a diagonal from upper right to lower left.
    HatchStyleBackwardDiagonal = 0x00000003,
    /// Crossing horizontal and vertical lines.
    HatchStyleLargeGrid = 0x00000004,
    /// Crossing forward diagonal and backward diagonal lines with
    /// anti-aliasing.
    HatchStyleDiagonalCross = 0x00000005,
    /// A 5-percent hatch, which is the ratio of foreground color to
    /// background color equal to 5:100.
    HatchStyle05Percent = 0x00000006,
    /// A 10-percent hatch, which is the ratio of foreground color to
    /// background color equal to 10:100.
    HatchStyle10Percent = 0x00000007,
    /// A 20-percent hatch, which is the ratio of foreground color to
    /// background color equal to 20:100.
    HatchStyle20Percent = 0x00000008,
    /// A 25-percent hatch, which is the ratio of foreground color to
    /// background color equal to 25:100.
    HatchStyle25Percent = 0x00000009,
    /// A 30-percent hatch, which is the ratio of foreground color to
    /// background color equal to 30:100.
    HatchStyle30Percent = 0x0000000A,
    /// A 40-percent hatch, which is the ratio of foreground color to
    /// background color equal to 40:100.
    HatchStyle40Percent = 0x0000000B,
    /// A 50-percent hatch, which is the ratio of foreground color to
    /// background color equal to 50:100.
    HatchStyle50Percent = 0x0000000C,
    /// A 60-percent hatch, which is the ratio of foreground color to
    /// background color equal to 60:100.
    HatchStyle60Percent = 0x0000000D,
    /// A 70-percent hatch, which is the ratio of foreground color to
    /// background color equal to 70:100.
    HatchStyle70Percent = 0x0000000E,
    /// A 75-percent hatch, which is the ratio of foreground color to
    /// background color equal to 75:100.
    HatchStyle75Percent = 0x0000000F,
    /// An 80-percent hatch, which is the ratio of foreground color to
    /// background color equal to 80:100.
    HatchStyle80Percent = 0x00000010,
    /// A 90-percent hatch, which is the ratio of foreground color to
    /// background color equal to 90:100.
    HatchStyle90Percent = 0x00000011,
    /// Diagonal lines that slant to the right from top to bottom points
    /// with no anti-aliasing. They are spaced 50 percent further apart
    /// than lines in the HatchStyleForwardDiagonal pattern.
    HatchStyleLightDownwardDiagonal = 0x00000012,
    /// Diagonal lines that slant to the left from top to bottom points
    /// with no anti-aliasing. They are spaced 50 percent further apart
    /// than lines in the HatchStyleBackwardDiagonal pattern.
    HatchStyleLightUpwardDiagonal = 0x00000013,
    /// Diagonal lines that slant to the right from top to bottom points
    /// with no anti-aliasing. They are spaced 50 percent closer and are
    /// twice the width of lines in the HatchStyleForwardDiagonal
    /// pattern.
    HatchStyleDarkDownwardDiagonal = 0x00000014,
    /// Diagonal lines that slant to the left from top to bottom points
    /// with no anti-aliasing. They are spaced 50 percent closer and are
    /// twice the width of lines in the HatchStyleBackwardDiagonal
    /// pattern.
    HatchStyleDarkUpwardDiagonal = 0x00000015,
    /// Diagonal lines that slant to the right from top to bottom points
    /// with no anti-aliasing. They have the same spacing between lines
    /// in HatchStyleWideDownwardDiagonal pattern and
    /// HatchStyleForwardDiagonal pattern, but
    /// HatchStyleWideDownwardDiagonal has the triple line width of
    /// HatchStyleForwardDiagonal.
    HatchStyleWideDownwardDiagonal = 0x00000016,
    /// Diagonal lines that slant to the left from top to bottom points
    /// with no anti-aliasing. They have the same spacing between lines
    /// in HatchStyleWideUpwardDiagonal pattern and
    /// HatchStyleBackwardDiagonal pattern, but
    /// HatchStyleWideUpwardDiagonal has the triple line width of
    /// HatchStyleWideUpwardDiagonal.
    HatchStyleWideUpwardDiagonal = 0x00000017,
    /// Vertical lines that are spaced 50 percent closer together than
    /// lines in the HatchStyleVertical pattern.
    HatchStyleLightVertical = 0x00000018,
    /// Horizontal lines that are spaced 50 percent closer than lines in
    /// the HatchStyleHorizontal pattern.
    HatchStyleLightHorizontal = 0x00000019,
    /// Vertical lines that are spaced 75 percent closer than lines in
    /// the HatchStyleVertical pattern; or 25 percent closer than lines
    /// in the HatchStyleLightVertical pattern.
    HatchStyleNarrowVertical = 0x0000001A,
    /// Horizontal lines that are spaced 75 percent closer than lines in
    /// the HatchStyleHorizontal pattern; or 25 percent closer than lines
    /// in the HatchStyleLightHorizontal pattern.
    HatchStyleNarrowHorizontal = 0x0000001B,
    /// Lines that are spaced 50 percent closer than lines in the
    /// HatchStyleVertical pattern.
    HatchStyleDarkVertical = 0x0000001C,
    /// Lines that are spaced 50 percent closer than lines in the
    /// HatchStyleHorizontal pattern.
    HatchStyleDarkHorizontal = 0x0000001D,
    /// Dashed diagonal lines that slant to the right from top to bottom
    /// points.
    HatchStyleDashedDownwardDiagonal = 0x0000001E,
    /// Dashed diagonal lines that slant to the left from top to bottom
    /// points.
    HatchStyleDashedUpwardDiagonal = 0x0000001F,
    /// Dashed horizontal lines.
    HatchStyleDashedHorizontal = 0x00000020,
    /// Dashed vertical lines.
    HatchStyleDashedVertical = 0x00000021,
    /// A pattern of lines that has the appearance of confetti.
    HatchStyleSmallConfetti = 0x00000022,
    /// A pattern of lines that has the appearance of confetti and is
    /// composed of larger pieces than the HatchStyleSmallConfetti
    /// pattern.
    HatchStyleLargeConfetti = 0x00000023,
    /// Horizontal lines that are composed of zigzags.
    HatchStyleZigZag = 0x00000024,
    /// Horizontal lines that are composed of tildes.
    HatchStyleWave = 0x00000025,
    /// A pattern of lines that has the appearance of layered bricks that
    /// slant to the left from top to bottom points.
    HatchStyleDiagonalBrick = 0x00000026,
    /// A pattern of lines that has the appearance of horizontally
    /// layered bricks.
    HatchStyleHorizontalBrick = 0x00000027,
    /// A pattern of lines that has the appearance of a woven material.
    HatchStyleWeave = 0x00000028,
    /// A pattern of lines that has the appearance of a plaid material.
    HatchStylePlaid = 0x00000029,
    /// A pattern of lines that has the appearance of divots.
    HatchStyleDivot = 0x0000002A,
    /// Crossing horizontal and vertical lines, each of which is composed
    /// of dots.
    HatchStyleDottedGrid = 0x0000002B,
    /// Crossing forward and backward diagonal lines, each of which is
    /// composed of dots.
    HatchStyleDottedDiamond = 0x0000002C,
    /// A pattern of lines that has the appearance of diagonally layered
    /// shingles that slant to the right from top to bottom points.
    HatchStyleShingle = 0x0000002D,
    /// A pattern of lines that has the appearance of a trellis.
    HatchStyleTrellis = 0x0000002E,
    /// A pattern of lines that has the appearance of spheres laid
    /// adjacent to each other.
    HatchStyleSphere = 0x0000002F,
    /// Crossing horizontal and vertical lines that are spaced 50 percent
    /// closer together than HatchStyleLargeGrid.
    HatchStyleSmallGrid = 0x00000030,
    /// A pattern of lines that has the appearance of a checkerboard.
    HatchStyleSmallCheckerBoard = 0x00000031,
    /// A pattern of lines that has the appearance of a checkerboard,
    /// with squares that are twice the size of the squares in the
    /// HatchStyleSmallCheckerBoard pattern.
    HatchStyleLargeCheckerBoard = 0x00000032,
    /// Crossing forward and backward diagonal lines; the lines are not
    /// anti-aliased.
    HatchStyleOutlinedDiamond = 0x00000033,
    /// A pattern of lines that has the appearance of a checkerboard
    /// placed diagonally.
    HatchStyleSolidDiamond = 0x00000034,
}

crate::parser::enums::impl_parser!(HatchStyle, u32);

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn covers_the_full_range_without_gaps() {
        assert_eq!(HatchStyle::iter().count(), 0x35);
        for raw in 0..=0x34_u32 {
            assert!(HatchStyle::from_repr(raw).is_some(), "{raw:#010X}");
        }
    }
}
