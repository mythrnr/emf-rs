/// The MapMode enumeration is used to define the unit of measure for
/// transforming page space units into device space units and for defining the
/// orientation of the drawing axes.
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
pub enum MapMode {
    /// Each logical unit is mapped to one device pixel. Positive x is to the
    /// right; positive y is down.
    MM_TEXT = 0x01,
    /// Each logical unit is mapped to 0.1 millimeter. Positive x is to the
    /// right; positive y is up.
    MM_LOMETRIC = 0x02,
    /// Each logical unit is mapped to 0.01 millimeter. Positive x is to the
    /// right; positive y is up.
    MM_HIMETRIC = 0x03,
    /// Each logical unit is mapped to 0.01 inch. Positive x is to the right;
    /// positive y is up.
    MM_LOENGLISH = 0x04,
    /// Each logical unit is mapped to 0.001 inch. Positive x is to the right;
    /// positive y is up.
    MM_HIENGLISH = 0x05,
    /// Each logical unit is mapped to one-twentieth of a printer's point
    /// (1/1440 inch, also called a "twip"). Positive x is to the right;
    /// positive y is up.
    MM_TWIPS = 0x06,
    /// Logical units are isotropic; that is, they are mapped to arbitrary
    /// units with equally scaled axes. Thus, one unit along the x-axis is
    /// equal to one unit along the y-axis. The EMR_SETWINDOWEXTEX and
    /// EMR_SETVIEWPORTEXTEX records are used to specify the units and the
    /// orientation of the axes.
    ///
    /// Adjustments MUST be made as necessary to ensure that the x and y units
    /// remain the same size. For example, when the window extent is set, the
    /// viewport MUST be adjusted to keep the units isotropic.
    MM_ISOTROPIC = 0x07,
    /// Logical units are anisotropic; that is, they are mapped to arbitrary
    /// units with arbitrarily scaled axes. The EMR_SETWINDOWEXTEX and
    /// EMR_SETVIEWPORTEXTEX records are used to specify the units,
    /// orientation, and scaling of the axes.
    MM_ANISOTROPIC = 0x08,
}

crate::parser::enums::impl_parser!(MapMode, u8);
