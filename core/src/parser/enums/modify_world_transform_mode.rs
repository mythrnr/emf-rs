/// The ModifyWorldTransformMode enumeration defines modes for changing the
/// world-space to page-space transform that is currently defined in the
/// playback device context.
///
/// The transform data is specified as an XForm object.
/// For more information concerning transforms and coordinate spaces, see
/// [MSDN-WRLDPGSPC].
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
pub enum ModifyWorldTransformMode {
    /// Reset the current transform using the identity matrix. In this mode,
    /// the specified transform data is ignored.
    MWT_IDENTITY = 0x01,
    /// Multiply the current transform. In this mode, the specified transform
    /// data is the left multiplicand, and the current transform is the right
    /// multiplicand.
    MWT_LEFTMULTIPLY = 0x02,
    /// Multiply the current transform. In this mode, the specified transform
    /// data is the right multiplicand, and the current transform is the left
    /// multiplicand.
    MWT_RIGHTMULTIPLY = 0x03,
    /// Set the current transform to the specified transform data.
    MWT_SET = 0x04,
}

crate::parser::enums::impl_parser!(ModifyWorldTransformMode, u8);
