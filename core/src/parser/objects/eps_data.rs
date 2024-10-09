/// The EpsData object is a container for EPS data.
#[derive(Clone, Debug)]
pub struct EpsData {
    /// SizeData (4 bytes): An unsigned integer that specifies the total size
    /// of this object in bytes.
    pub size_data: u32,
    /// Version (4 bytes): An unsigned integer that specifies the PostScript
    /// language level. This value is 0x00000001.
    pub version: u32,
    /// Points (24 bytes): An array of three Point28_4 objects that defines the
    /// coordinates of the output parallelogram using 28.4 bit FIX notation.
    ///
    /// The upper-left corner of the parallelogram is the first point in this
    /// array, the upper-right corner is the second point, and the lower-left
    /// corner is the third point. The lower-right corner of the parallelogram
    /// is computed from the first three points (A, B, and C) by treating them
    /// as vectors.
    ///
    /// ```
    /// D = B + C A
    /// ```
    pub points: [crate::parser::Point28_4; 3],
    /// PostScriptData (variable): An array of bytes of PostScript data. The
    /// length of this array can be computed from the SizeData field. This data
    /// MAY be used to render an image. Windows does not parse the PostScript
    /// data in an EpsData object; the data is handed off to the graphics
    /// printer driver if the driver supports PostScript printing.
    pub post_script_data: Vec<u8>,
}

impl EpsData {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let ((size_data, size_data_bytes), (version, version_bytes)) = (
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );

        if version != 0x00000001 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "version field in EpsData must be `0x00000001`, but \
                     parsed value is {version:#010X}"
                ),
            });
        }

        let (points, points_bytes) = {
            let mut points = vec![];
            let mut points_bytes = 0;

            for _ in 0..3 {
                let (p, b) = crate::parser::Point28_4::parse(buf)?;
                points.push(p);
                points_bytes += b;
            }

            let points: [_; 3] =
                points.try_into().expect("should be converted");

            (points, points_bytes)
        };

        let (post_script_data, post_script_data_bytes) =
            crate::parser::read_variable(buf, size_data as usize)?;

        Ok((
            Self { size_data, version, points, post_script_data },
            size_data_bytes
                + version_bytes
                + points_bytes
                + post_script_data_bytes,
        ))
    }
}
