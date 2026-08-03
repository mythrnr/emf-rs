use crate::parser::emf_plus::objects::{EmfPlusARGB, EmfPlusGraphicsVersion};

/// The EmfPlusImageAttributes object specifies how bitmap image colors
/// are manipulated during rendering (MS-EMFPLUS 2.2.1.5).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmfPlusImageAttributes {
    /// Version (4 bytes): An EmfPlusGraphicsVersion object (section
    /// 2.2.2.19) that specifies the version of operating system
    /// graphics that was used to create this object.
    pub version: EmfPlusGraphicsVersion,
    /// Reserved1 (4 bytes): A field that is not used and MUST be
    /// ignored.
    pub reserved_1: u32,
    /// WrapMode (4 bytes): An unsigned integer that specifies how to
    /// handle edge conditions with a value from the WrapMode
    /// enumeration (section 2.1.1.33).
    pub wrap_mode: crate::parser::emf_plus::WrapMode,
    /// ClampColor (4 bytes): An EmfPlusARGB object (section 2.2.2.1)
    /// that specifies the edge color to use when the WrapMode value is
    /// WrapModeClamp. This color is visible when the source rectangle
    /// processed by an EmfPlusDrawImage record (section 2.3.4.8) is
    /// larger than the image itself.
    pub clamp_color: EmfPlusARGB,
    /// ObjectClamp (4 bytes): A signed integer that specifies the
    /// object clamping behavior. It is not used until this object is
    /// applied to an image being drawn. This value MUST be one of the
    /// values defined in the following table.
    ///
    /// | Value | Meaning |
    /// |---|---|
    /// | RectClamp (0x00000000) | The object is clamped to a rectangle. |
    /// | BitmapClamp (0x00000001) | The object is clamped to a bitmap. |
    pub object_clamp: i32,
    /// Reserved2 (4 bytes): A value that SHOULD be set to zero and
    /// MUST be ignored upon receipt.
    pub reserved_2: u32,
}

impl EmfPlusImageAttributes {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let version =
            read_with(buf, &mut consumed_bytes, EmfPlusGraphicsVersion::parse)?;
        let reserved_1: u32 = read_field(buf, &mut consumed_bytes)?;
        let wrap_mode = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::emf_plus::WrapMode::parse,
        )?;
        let clamp_color =
            read_with(buf, &mut consumed_bytes, EmfPlusARGB::parse)?;
        let object_clamp: i32 = read_field(buf, &mut consumed_bytes)?;
        let reserved_2: u32 = read_field(buf, &mut consumed_bytes)?;

        Ok((
            Self {
                version,
                reserved_1,
                wrap_mode,
                clamp_color,
                object_clamp,
                reserved_2,
            },
            consumed_bytes,
        ))
    }
}
