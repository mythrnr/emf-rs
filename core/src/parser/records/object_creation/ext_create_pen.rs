use crate::imports::*;

/// The EMR_EXTCREATEPEN record defines an extended logical pen for graphics
/// operations. An optional DIB can be specified to use as the line style.
///
/// The extended logical pen object defined by this record can be selected into
/// the playback device context by an EMR_SELECTOBJECT record, which specifies
/// the logical pen to use in subsequent graphics operations.
#[derive(Clone, Debug)]
pub struct EMR_EXTCREATEPEN {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_EXTCREATEPEN. This value is 0x0000005F.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// ihPen (4 bytes): An unsigned integer that specifies the index of the
    /// extended logical pen object in the EMF object table. This index MUST be
    /// saved so that this object can be reused or modified.
    pub ih_pen: u32,
    /// offBmi (4 bytes): An unsigned integer that specifies the offset from
    /// the start of this record to the DIB header if the record contains a
    /// DIB.
    pub off_bmi: u32,
    /// cbBmi (4 bytes): An unsigned integer that specifies the size of the DIB
    /// header if the record contains a DIB.
    pub cb_bmi: u32,
    /// offBits (4 bytes): An unsigned integer that specifies the offset from
    /// the start of this record to the DIB bits if the record contains a DIB.
    pub off_bits: u32,
    /// cbBits (4 bytes): An unsigned integer that specifies the size of the
    /// DIB bits if the record contains a DIB.
    pub cb_bits: u32,
    /// elp (variable): A LogPenEx object that specifies an extended logical
    /// pen with attributes including an optional line style array.
    pub elp: crate::parser::LogPenEx,
    /// BitmapBuffer (variable, optional): An array of bytes containing a
    /// packed DIB in the form of a DeviceIndependentBitmap object ([MS-WMF]
    /// section 2.2.2.9). It is not required to be contiguous with the fixed
    /// portion of this record.
    ///
    /// BmiSrc (variable): The DIB header, which is the DibHeaderInfo field of
    /// a DeviceIndependentBitmap object.
    pub bmi_src: Option<wmf_core::parser::BitmapInfoHeader>,
    /// BitsSrc (variable): The DIB bits, which is the aData field of a
    /// DeviceIndependentBitmap object.
    pub bits_src: Option<Vec<u8>>,
}

impl EMR_EXTCREATEPEN {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = ?record_type),
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
            crate::parser::RecordType::EMR_EXTCREATEPEN as u32,
        )?;

        let ih_pen = read_field(buf, &mut size)?;
        let off_bmi: u32 = read_field(buf, &mut size)?;
        let cb_bmi: u32 = read_field(buf, &mut size)?;
        let off_bits = read_field(buf, &mut size)?;
        let cb_bits: u32 = read_field(buf, &mut size)?;
        let elp = read_with(buf, &mut size, crate::parser::LogPenEx::parse)?;

        let (bmi_src, bits_src) = if off_bmi > 0 && cb_bmi > 0 {
            let undef_offset_bmi = size.checked_offset(off_bmi)?;
            let _ = read_bytes_field(buf, &mut size, undef_offset_bmi)?;
            let bmi_src = read_with(
                buf,
                &mut size,
                wmf_core::parser::BitmapInfoHeader::parse,
            )?;

            let undef_offset_bits = size.checked_offset(off_bits)?;
            let _ = read_bytes_field(buf, &mut size, undef_offset_bits)?;
            let bits_src = read_bytes_field(buf, &mut size, cb_bits as usize)?;

            (Some(bmi_src), Some(bits_src))
        } else {
            (None, None)
        };

        consume_remaining_bytes(buf, size.remaining_bytes())?;

        Ok(Self {
            record_type,
            size,
            ih_pen,
            off_bmi,
            cb_bmi,
            off_bits,
            cb_bits,
            elp,
            bmi_src,
            bits_src,
        })
    }
}
