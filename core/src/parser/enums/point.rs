/// The Point enumeration is used to specify how a point is to be used in a
/// drawing call.
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
pub enum Point {
    /// A PT_LINETO or PT_BEZIERTO type can be combined with this value by
    /// using the bitwise operator OR to indicate that the corresponding point
    /// is the last point in a figure and the figure is closed.
    ///
    /// The current position is set to the ending point of the closing line.
    PT_CLOSEFIGURE = 0x01,
    /// Specifies that a line is to be drawn from the current position to this
    /// point, which then becomes the new current position.
    PT_LINETO = 0x02,
    /// Specifies that this point is a control point or ending point for a
    /// Bezier curve.
    ///
    /// PT_BEZIERTO types always occur in sets of three. The current position
    /// defines the starting point for the Bezier curve. The first two
    /// PT_BEZIERTO points are the control points, and the third PT_BEZIERTO
    /// point is the ending point. The ending point becomes the new current
    /// position. If there are not three consecutive PT_BEZIERTO points, an
    /// error results.
    PT_BEZIERTO = 0x04,
    /// Specifies that this point starts a disjoint figure. This point becomes
    /// the new current position.
    PT_MOVETO = 0x06,
}

crate::parser::enums::impl_parser!(Point, u8);
