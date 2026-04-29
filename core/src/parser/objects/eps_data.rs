use crate::imports::*;

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
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_bytes_field, read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let size_data: u32 = read_field(buf, &mut consumed_bytes)?;
        let version = read_field(buf, &mut consumed_bytes)?;

        crate::parser::ParseError::expect_eq(
            "version (EpsData)",
            version,
            0x00000001_u32,
        )?;

        let points = {
            let mut points = vec![];

            for _ in 0..3 {
                points.push(read_with(
                    buf,
                    &mut consumed_bytes,
                    crate::parser::Point28_4::parse,
                )?);
            }

            let points: [_; 3] =
                points.try_into().expect("should be converted");

            points
        };

        let post_script_data =
            read_bytes_field(buf, &mut consumed_bytes, size_data as usize)?;

        Ok((
            Self { size_data, version, points, post_script_data },
            consumed_bytes,
        ))
    }
}
