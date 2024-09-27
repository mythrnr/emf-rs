/// The GradientFill enumeration defines the modes for gradient fill operations.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u32)]
pub enum GradientFill {
    /// Color interpolation along a gradient from the left to the right edges
    /// of a rectangle.
    GRADIENT_FILL_RECT_H = 0x00000000,
    /// Color interpolation along a gradient from the top to the bottom edges
    /// of a rectangle.
    GRADIENT_FILL_RECT_V = 0x00000001,
    /// Color interpolation between vertexes of a triangle.
    GRADIENT_FILL_TRIANGLE = 0x00000002,
}

crate::parser::constants::impl_parser!(GradientFill, u32);
