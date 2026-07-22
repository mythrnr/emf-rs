/// The EmfPlusGraphicsVersion object specifies the version of operating
/// system graphics that is used to create an EMF+ metafile
/// (MS-EMFPLUS 2.2.2.19).
///
/// MetafileSignature (20 bits): A value that identifies the type of
/// metafile. The value for an EMF+ metafile is 0xDBC01.
///
/// GraphicsVersion (12 bits): The version of operating system
/// graphics. This value is defined in the GraphicsVersion enumeration
/// (section 2.1.1.12).
///
/// Graphics versions are vendor-extensible; however, to ensure
/// inter-operability, any such extension MUST be implemented in both
/// clients and servers of EMF+ metafiles.
///
/// Both wire fields are packed into the stored 32-bit value; they are
/// exposed via `metafile_signature()` and `graphics_version()`.
///
/// The raw value is preserved: the signature is validated strictly only
/// where the specification makes the whole stream unusable without it
/// (the EmfPlusHeader record); objects embed this value merely as
/// provenance, and rejecting an odd value there would discard an
/// otherwise parsable object.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EmfPlusGraphicsVersion(u32);

impl EmfPlusGraphicsVersion {
    /// The value the 20-bit MetafileSignature field MUST carry.
    pub const METAFILE_SIGNATURE: u32 = 0xDBC01;

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (value, consumed_bytes) =
            <u32 as crate::parser::ReadLeField>::read_le(buf)?;

        Ok((Self(value), consumed_bytes))
    }

    /// The raw 32-bit value.
    pub fn raw(self) -> u32 {
        self.0
    }

    /// The 20-bit metafile signature (high bits).
    pub fn metafile_signature(self) -> u32 {
        self.0 >> 12
    }

    /// The GraphicsVersion enumeration value (low 12 bits), when it is
    /// a defined variant.
    pub fn graphics_version(
        self,
    ) -> Option<crate::parser::emf_plus::GraphicsVersion> {
        crate::parser::emf_plus::GraphicsVersion::from_repr(
            (self.0 & 0x0FFF) as u16,
        )
    }

    /// Validates the metafile signature. Called by the EmfPlusHeader
    /// record parser, where the specification mandates the check.
    pub fn validate_signature(self) -> Result<(), crate::parser::ParseError> {
        crate::parser::ParseError::expect_eq(
            "MetafileSignature",
            self.metafile_signature(),
            Self::METAFILE_SIGNATURE,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decomposes_signature_and_version() {
        // 0xDBC01 << 12 | 2 = GDI+ 1.1.
        let mut buf: &[u8] = &0xDBC0_1002_u32.to_le_bytes();
        let (v, c) = EmfPlusGraphicsVersion::parse(&mut buf).unwrap();

        assert_eq!(c, 4);
        assert_eq!(v.metafile_signature(), 0xDBC01);
        assert_eq!(
            v.graphics_version(),
            Some(crate::parser::emf_plus::GraphicsVersion::GraphicsVersion1_1),
        );
        assert!(v.validate_signature().is_ok());
    }

    #[test]
    fn rejects_wrong_signature_only_on_validation() {
        let mut buf: &[u8] = &0x0000_0002_u32.to_le_bytes();
        let (v, _) = EmfPlusGraphicsVersion::parse(&mut buf).unwrap();

        assert!(v.validate_signature().is_err());
    }
}
