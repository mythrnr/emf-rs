/// The PathPointType enumeration defines types of points on a graphics
/// path (MS-EMFPLUS 2.1.1.23).
///
/// Graphics path point types are specified by EmfPlusPathPointType
/// objects.
///
/// The value occupies the low 4 bits of an EmfPlusPathPointType object;
/// the high 4 bits carry PathPointTypeFlags.
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
#[repr(u8)]
pub enum PathPointType {
    /// The point is the starting point of a path.
    PathPointTypeStart = 0x00,
    /// The point is one of the two endpoints of a line.
    PathPointTypeLine = 0x01,
    /// The point is an endpoint or control point of a cubic Bezier
    /// curve. The value 0x02 is not defined by the specification.
    PathPointTypeBezier = 0x03,
}
