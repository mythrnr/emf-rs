/// The HeaderExtension2 object defines the second extension to the EMF metafile
/// header. It adds the ability to measure device surfaces in micrometers, which
/// enhances the resolution and scalability of EMF metafiles.
#[derive(Clone, Debug)]
pub struct HeaderExtension2 {
    /// MicrometersX (4 bytes): The 32-bit horizontal size of the display
    /// device for which the metafile image was generated, in micrometers.
    pub micrometers_x: u32,
    /// MicrometersY (4 bytes): The 32-bit vertical size of the display device
    /// for which the metafile image was generated, in micrometers.
    pub micrometers_y: u32,
}

impl HeaderExtension2 {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::read_field;

        let mut consumed_bytes: usize = 0;
        let micrometers_x = read_field(buf, &mut consumed_bytes)?;
        let micrometers_y = read_field(buf, &mut consumed_bytes)?;

        Ok((Self { micrometers_x, micrometers_y }, consumed_bytes))
    }
}
