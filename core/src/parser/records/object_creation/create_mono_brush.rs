use crate::imports::*;

/// The EMR_CREATEMONOBRUSH record defines a monochrome pattern brush for
/// graphics operations. The pattern is specified by a monochrome DIB.
///
/// The monochrome pattern brush object defined by this record can be selected
/// into the playback device context by an EMR_SELECTOBJECT record, which
/// specifies the pattern brush to use in subsequent graphics operations.
#[derive(Clone, Debug)]
pub struct EMR_CREATEMONOBRUSH {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_CREATEMONOBRUSH. This value is 0x0000005D.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// ihBrush (4 bytes): An unsigned integer that specifies the index of the
    /// monochrome pattern brush object in the EMF object table. This index
    /// MUST be saved so that this object can be reused or modified.
    pub ih_brush: u32,
    /// Usage (4 bytes): An unsigned integer that specifies how to interpret
    /// values in the color table in the DIB header. This value is in the
    /// DIBColors enumeration.
    pub usage: crate::parser::DIBColors,
    /// offBmi (4 bytes): An unsigned integer that specifies the offset from
    /// the start of this record to the DIB header.
    pub off_bmi: u32,
    /// cbBmi (4 bytes): An unsigned integer that specifies the size of the DIB
    /// header.
    pub cb_bmi: u32,
    /// offBits (4 bytes): An unsigned integer that specifies the offset from
    /// the start of this record to the DIB bits.
    pub off_bits: u32,
    /// cbBits (4 bytes): An unsigned integer that specifies the size of the
    /// DIB bits.
    pub cb_bits: u32,
    /// BitmapBuffer (variable): A buffer containing a packed DIB in the form
    /// of a monochrome DeviceIndependentBitmap object ([MS-WMF] section
    /// 2.2.2.9). It is not required to be contiguous with the fixed portion of
    /// this record.
    ///
    /// BmiSrc (variable): The DIB header, which is the DibHeaderInfo field of
    /// a DeviceIndependentBitmap object.
    pub bmi_src: wmf_core::parser::BitmapInfoHeader,
    /// BitsSrc (variable): The DIB bits, which is the aData field of a
    /// DeviceIndependentBitmap object.
    pub bits_src: Vec<u8>,
}

impl EMR_CREATEMONOBRUSH {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{
            consume_remaining_bytes, read_bytes_field, read_field, read_with,
        };

        crate::parser::ParseError::expect_eq(
            "record_type",
            record_type as u32,
            crate::parser::RecordType::EMR_CREATEMONOBRUSH as u32,
        )?;

        let ih_brush = read_field(buf, &mut size)?;
        let usage = read_with(buf, &mut size, crate::parser::DIBColors::parse)?;
        let off_bmi = read_field(buf, &mut size)?;
        let cb_bmi = read_field(buf, &mut size)?;
        let off_bits = read_field(buf, &mut size)?;
        let cb_bits: u32 = read_field(buf, &mut size)?;

        let undef_space_bmi = size.checked_offset(off_bmi)?;
        let _ = read_bytes_field(buf, &mut size, undef_space_bmi)?;
        let bmi_src = read_with(
            buf,
            &mut size,
            wmf_core::parser::BitmapInfoHeader::parse,
        )?;

        let undef_space_bits = size.checked_offset(off_bits)?;
        let _ = read_bytes_field(buf, &mut size, undef_space_bits)?;
        let bits_src = read_bytes_field(buf, &mut size, cb_bits as usize)?;

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self {
            record_type,
            size,
            ih_brush,
            usage,
            off_bmi,
            cb_bmi,
            off_bits,
            cb_bits,
            bmi_src,
            bits_src,
        })
    }
}
