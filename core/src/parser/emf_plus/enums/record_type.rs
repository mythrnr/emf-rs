/// The RecordType enumeration defines record types used in EMF+
/// metafiles (MS-EMFPLUS 2.1.1.1).
///
/// Values are 16-bit; every EMF+ record starts with this type followed by
/// the 16-bit Flags field.
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
#[repr(u16)]
pub enum RecordType {
    /// This record specifies the start of EMF+ data in the metafile. It
    /// MUST be embedded in the first EMF record after the EMF Header
    /// record. (2.3.3.3)
    EmfPlusHeader = 0x4001,
    /// This record specifies the end of EMF+ data in the metafile.
    /// (2.3.3.1)
    EmfPlusEndOfFile = 0x4002,
    /// This record specifies arbitrary private data. (2.3.2.1)
    EmfPlusComment = 0x4003,
    /// This record specifies that subsequent EMF records ([MS-EMF]
    /// section 2.3) encountered in the metafile SHOULD be processed. EMF
    /// records cease being processed when the next EMF+ record is
    /// encountered. (2.3.3.2)
    EmfPlusGetDC = 0x4004,
    /// This record is reserved and MUST NOT be used.
    EmfPlusMultiFormatStart = 0x4005,
    /// This record is reserved and MUST NOT be used.
    EmfPlusMultiFormatSection = 0x4006,
    /// This record is reserved and MUST NOT be used.
    EmfPlusMultiFormatEnd = 0x4007,
    /// This record specifies an object for use in graphics operations.
    /// (2.3.5.1)
    EmfPlusObject = 0x4008,
    /// This record clears the output coordinate space and initializes
    /// it with a specified background color and transparency. (2.3.4.1)
    EmfPlusClear = 0x4009,
    /// This record defines how to fill the interiors of a series of
    /// rectangles, using a specified brush. (2.3.4.20)
    EmfPlusFillRects = 0x400A,
    /// This record defines the pen strokes for drawing a series of
    /// rectangles. (2.3.4.13)
    EmfPlusDrawRects = 0x400B,
    /// This record defines the data to fill the interior of a polygon,
    /// using a specified brush. (2.3.4.19)
    EmfPlusFillPolygon = 0x400C,
    /// This record defines the pen strokes for drawing a series of
    /// connected lines. (2.3.4.10)
    EmfPlusDrawLines = 0x400D,
    /// This record defines how to fill the interiors of an ellipse,
    /// using a specified brush. (2.3.4.16)
    EmfPlusFillEllipse = 0x400E,
    /// This record defines the pen strokes for drawing an ellipse.
    /// (2.3.4.7)
    EmfPlusDrawEllipse = 0x400F,
    /// This record defines how to fill a section of an interior section
    /// of an ellipse using a specified brush. (2.3.4.18)
    EmfPlusFillPie = 0x4010,
    /// This record defines pen strokes for drawing a section of an
    /// ellipse. (2.3.4.12)
    EmfPlusDrawPie = 0x4011,
    /// The record defines pen strokes for drawing an arc of an ellipse.
    /// (2.3.4.2)
    EmfPlusDrawArc = 0x4012,
    /// This record defines how to fill the interior of a region using a
    /// specified brush. (2.3.4.21)
    EmfPlusFillRegion = 0x4013,
    /// The record defines how to fill the interiors of the figures
    /// defined in a graphics path with a specified brush. A path is an
    /// object that defines an arbitrary sequence of lines, curves, and
    /// shapes. (2.3.4.17)
    EmfPlusFillPath = 0x4014,
    /// The record defines the pen strokes to draw the figures in a
    /// graphics path. A path is an object that defines an arbitrary
    /// sequence of lines, curves, and shapes. (2.3.4.11)
    EmfPlusDrawPath = 0x4015,
    /// This record defines how to fill the interior of a closed cardinal
    /// spline using a specified brush. (2.3.4.15)
    EmfPlusFillClosedCurve = 0x4016,
    /// This record defines the pen and strokes for drawing a closed
    /// cardinal spline. (2.3.4.4)
    EmfPlusDrawClosedCurve = 0x4017,
    /// This record defines the pen strokes for drawing a cardinal
    /// spline. (2.3.4.5)
    EmfPlusDrawCurve = 0x4018,
    /// This record defines the pen strokes for drawing a Bezier spline.
    /// (2.3.4.3)
    EmfPlusDrawBeziers = 0x4019,
    /// This record defines a scaled EmfPlusImage object. An image can
    /// consist of either bitmap or metafile data. (2.3.4.8)
    EmfPlusDrawImage = 0x401A,
    /// This record defines a scaled EmfPlusImage object inside a
    /// parallelogram. An image can consist of either bitmap or metafile
    /// data. (2.3.4.9)
    EmfPlusDrawImagePoints = 0x401B,
    /// This record defines a text string based on a font, a layout
    /// rectangle, and a format. (2.3.4.14)
    EmfPlusDrawString = 0x401C,
    /// This record sets the origin of rendering to the specified
    /// horizontal and vertical coordinates. This applies to hatch
    /// brushes and to 8 and 16 bits per pixel dither patterns. (2.3.6.6)
    EmfPlusSetRenderingOrigin = 0x401D,
    /// This record defines whether to enable or disable text
    /// anti-aliasing. Text anti-aliasing is a method of making lines and
    /// edges of character glyphs appear smoother when drawn on an output
    /// surface. (2.3.6.1)
    EmfPlusSetAntiAliasMode = 0x401E,
    /// This record defines the process used for rendering text.
    /// (2.3.6.8)
    EmfPlusSetTextRenderingHint = 0x401F,
    /// This record sets text contrast according to the specified text
    /// gamma value. (2.3.6.7)
    EmfPlusSetTextContrast = 0x4020,
    /// This record defines the interpolation mode of an object according
    /// to the specified type of image filtering. The interpolation mode
    /// influences how scaling (stretching and shrinking) is performed.
    /// (2.3.6.4)
    EmfPlusSetInterpolationMode = 0x4021,
    /// This record defines the pixel offset mode according to the
    /// specified pixel centering value. (2.3.6.5)
    EmfPlusSetPixelOffsetMode = 0x4022,
    /// This record defines the compositing mode according to the state
    /// of alpha blending, which specifies how source colors are combined
    /// with background colors. (2.3.6.2)
    EmfPlusSetCompositingMode = 0x4023,
    /// This record defines the compositing quality, which describes the
    /// desired level of quality for creating composite images from
    /// multiple objects. (2.3.6.3)
    EmfPlusSetCompositingQuality = 0x4024,
    /// This record saves the graphics state, identified by a specified
    /// index, on a stack of saved graphics states. Each stack index is
    /// associated with a particular saved state, and the index is used
    /// by an EmfPlusRestore record to restore the state. (2.3.7.5)
    EmfPlusSave = 0x4025,
    /// This record restores the graphics state, identified by a
    /// specified index, from a stack of saved graphics states. Each
    /// stack index is associated with a particular saved state, and the
    /// index is defined by an EmfPlusSave record to save the state.
    /// (2.3.7.4)
    EmfPlusRestore = 0x4026,
    /// This record opens a new graphics state container and specifies a
    /// transform for it. Graphics containers are used to retain elements
    /// of the graphics state. (2.3.7.1)
    EmfPlusBeginContainer = 0x4027,
    /// This record opens a new graphics state container. (2.3.7.2)
    EmfPlusBeginContainerNoParams = 0x4028,
    /// This record closes a graphics state container that was previously
    /// opened by a begin container operation. (2.3.7.3)
    EmfPlusEndContainer = 0x4029,
    /// This record defines the current world space transform in the
    /// playback device context, according to a specified transform
    /// matrix. (2.3.9.6)
    EmfPlusSetWorldTransform = 0x402A,
    /// This record resets the current world space transform to the
    /// identify matrix. (2.3.9.5)
    EmfPlusResetWorldTransform = 0x402B,
    /// This record multiplies the current world space by a specified
    /// transform matrix. (2.3.9.1)
    EmfPlusMultiplyWorldTransform = 0x402C,
    /// This record applies a translation transform to the current world
    /// space by specified horizontal and vertical distances. (2.3.9.7)
    EmfPlusTranslateWorldTransform = 0x402D,
    /// This record applies a scaling transform to the current world
    /// space by specified horizontal and vertical scale factors.
    /// (2.3.9.4)
    EmfPlusScaleWorldTransform = 0x402E,
    /// This record rotates the current world space by a specified angle.
    /// (2.3.9.3)
    EmfPlusRotateWorldTransform = 0x402F,
    /// This record specifies extra scaling factors for the current world
    /// space transform. (2.3.9.2)
    EmfPlusSetPageTransform = 0x4030,
    /// This record resets the current clipping region for the world
    /// space to infinity. (2.3.1.2)
    EmfPlusResetClip = 0x4031,
    /// This record combines the current clipping region with a
    /// rectangle. (2.3.1.4)
    EmfPlusSetClipRect = 0x4032,
    /// This record combines the current clipping region with a graphics
    /// path. (2.3.1.3)
    EmfPlusSetClipPath = 0x4033,
    /// This record combines the current clipping region with another
    /// graphics region. (2.3.1.5)
    EmfPlusSetClipRegion = 0x4034,
    /// This record applies a translation transform on the current
    /// clipping region of the world space. (2.3.1.1)
    EmfPlusOffsetClip = 0x4035,
    /// This record specifies text output with character positions.
    /// (2.3.4.6)
    EmfPlusDrawDriverString = 0x4036,
    /// This record closes any open figures in a path, strokes the
    /// outline of the path by using the current pen, and fills its
    /// interior by using the current brush.
    ///
    /// Listed in the RecordType enumeration but has no record definition
    /// in MS-EMFPLUS 2.3; treated as unsupported.
    EmfPlusStrokeFillPath = 0x4037,
    /// This record defines an image effects parameter block that has
    /// been serialized into a data buffer. (2.3.5.2)
    EmfPlusSerializableObject = 0x4038,
    /// This record specifies the state of a graphics device context for
    /// a terminal server. (2.3.8.2)
    EmfPlusSetTSGraphics = 0x4039,
    /// This record specifies clipping areas in the graphics device
    /// context for a terminal server. (2.3.8.1)
    EmfPlusSetTSClip = 0x403A,
}

crate::parser::enums::impl_parser!(RecordType, u16);

impl RecordType {
    /// Record types that are defined in the RecordType enumeration but
    /// have no record layout in MS-EMFPLUS 2.3. Their payload can only
    /// be skipped.
    pub fn is_reserved(self) -> bool {
        matches!(
            self,
            Self::EmfPlusMultiFormatStart
                | Self::EmfPlusMultiFormatSection
                | Self::EmfPlusMultiFormatEnd
                | Self::EmfPlusStrokeFillPath
        )
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn roundtrip_all_variants() {
        for v in RecordType::iter() {
            assert_eq!(RecordType::from_repr(v as u16), Some(v));
        }
    }

    #[test]
    fn covers_the_full_record_type_range() {
        // 0x4001..=0x403A with no gaps.
        for raw in 0x4001..=0x403A_u16 {
            assert!(RecordType::from_repr(raw).is_some(), "{raw:#06X}");
        }
        assert!(RecordType::from_repr(0x4000).is_none());
        assert!(RecordType::from_repr(0x403B).is_none());
    }
}
