use crate::{
    imports::*,
    parser::emf_plus::objects::{
        EmfPlusBrush, EmfPlusCustomLineCap, EmfPlusFont, EmfPlusImage,
        EmfPlusImageAttributes, EmfPlusPath, EmfPlusPen, EmfPlusRegion,
        EmfPlusStringFormat,
    },
};

/// The ObjectData field of an EmfPlusObject record (MS-EMFPLUS
/// 2.3.5.1), typed by the ObjectType value of the record flags.
///
/// ObjectData (variable): An array of bytes that contains data for the
/// type of object specified in the Flags field. The content and format
/// of the data can be different for each object type.
///
/// ObjectType (7 bits): The type of object to be created by this
/// record, from the ObjectType enumeration.
#[derive(Clone, Debug, PartialEq)]
pub enum EmfPlusObjectData {
    Brush(EmfPlusBrush),
    // Boxed: the pen carries two optional custom line caps and an
    // embedded brush, which makes it several times larger than the
    // other variants.
    Pen(Box<EmfPlusPen>),
    Path(EmfPlusPath),
    Region(EmfPlusRegion),
    Image(EmfPlusImage),
    Font(EmfPlusFont),
    StringFormat(EmfPlusStringFormat),
    ImageAttributes(EmfPlusImageAttributes),
    CustomLineCap(EmfPlusCustomLineCap),
}

impl EmfPlusObjectData {
    /// Types a complete object data buffer. For continued objects this
    /// runs on the reassembled buffer, not on the individual record
    /// fragments.
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(object_type = ?object_type),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse(
        object_type: crate::parser::emf_plus::ObjectType,
        data: &[u8],
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::emf_plus::ObjectType;

        let buf = &mut &data[..];
        let available = data.len();

        let object_data = match object_type {
            ObjectType::ObjectTypeInvalid => {
                return Err(crate::parser::ParseError::UnexpectedPattern {
                    cause: Cow::from(
                        "EmfPlusObject record carries ObjectTypeInvalid",
                    ),
                });
            }
            ObjectType::ObjectTypeBrush => {
                Self::Brush(EmfPlusBrush::parse(buf, available)?.0)
            }
            ObjectType::ObjectTypePen => {
                Self::Pen(Box::new(EmfPlusPen::parse(buf, available)?.0))
            }
            ObjectType::ObjectTypePath => {
                Self::Path(EmfPlusPath::parse(buf)?.0)
            }
            ObjectType::ObjectTypeRegion => {
                Self::Region(EmfPlusRegion::parse(buf)?.0)
            }
            ObjectType::ObjectTypeImage => {
                Self::Image(EmfPlusImage::parse(buf, available)?.0)
            }
            ObjectType::ObjectTypeFont => {
                Self::Font(EmfPlusFont::parse(buf)?.0)
            }
            ObjectType::ObjectTypeStringFormat => {
                Self::StringFormat(EmfPlusStringFormat::parse(buf)?.0)
            }
            ObjectType::ObjectTypeImageAttributes => {
                Self::ImageAttributes(EmfPlusImageAttributes::parse(buf)?.0)
            }
            ObjectType::ObjectTypeCustomLineCap => {
                Self::CustomLineCap(EmfPlusCustomLineCap::parse(buf)?.0)
            }
        };

        Ok(object_data)
    }
}
