/// The ObjectType enumeration defines types of graphics objects that can
/// be created and used in graphics operations (MS-EMFPLUS 2.1.1.22).
///
/// Graphics objects are specified by EmfPlusObject records.
///
/// In an EmfPlusObject record the value travels in bits 8-14 of the
/// record flags.
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
pub enum ObjectType {
    /// The object is not a valid object.
    ObjectTypeInvalid = 0x00000000,
    /// An EmfPlusBrush object. Brush objects fill graphics regions.
    /// (2.2.1.1)
    ObjectTypeBrush = 0x00000001,
    /// An EmfPlusPen object. Pen objects draw graphics lines. (2.2.1.7)
    ObjectTypePen = 0x00000002,
    /// An EmfPlusPath object. Path objects specify sequences of lines,
    /// curves, and shapes. (2.2.1.6)
    ObjectTypePath = 0x00000003,
    /// An EmfPlusRegion object. Region objects specify areas of the
    /// output surface. (2.2.1.8)
    ObjectTypeRegion = 0x00000004,
    /// An EmfPlusImage object. Image objects encapsulate bitmaps and
    /// metafiles. (2.2.1.4)
    ObjectTypeImage = 0x00000005,
    /// An EmfPlusFont object. Font objects specify font properties,
    /// including typeface style, em size, and font family. (2.2.1.3)
    ObjectTypeFont = 0x00000006,
    /// An EmfPlusStringFormat object. String format objects specify text
    /// layout, including alignment, orientation, tab stops, clipping,
    /// and digit substitution for languages that do not use Western
    /// European digits. (2.2.1.9)
    ObjectTypeStringFormat = 0x00000007,
    /// An EmfPlusImageAttributes object. Image attribute objects specify
    /// operations on pixels during image rendering, including color
    /// adjustment, grayscale adjustment, gamma correction, and color
    /// mapping. (2.2.1.5)
    ObjectTypeImageAttributes = 0x00000008,
    /// An EmfPlusCustomLineCap object. Custom line cap objects specify
    /// shapes to draw at the ends of a graphics line, including squares,
    /// circles, and diamonds. (2.2.1.2)
    ObjectTypeCustomLineCap = 0x00000009,
}

crate::parser::enums::impl_parser!(ObjectType, u32);
