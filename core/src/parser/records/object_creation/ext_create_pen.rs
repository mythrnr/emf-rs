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
    pub bmi_src: wmf_core::parser::BitmapInfoHeader,
    /// BitsSrc (variable): The DIB bits, which is the aData field of a
    /// DeviceIndependentBitmap object.
    pub bits_src: Vec<u8>,
}

impl EMR_EXTCREATEPEN {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(record_type = %format!("{record_type:?}")),
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_type: crate::parser::RecordType,
        mut size: crate::parser::Size,
    ) -> Result<Self, crate::parser::ParseError> {
        if record_type != crate::parser::RecordType::EMR_EXTCREATEPEN {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_EXTCREATEPEN as u32,
                    record_type as u32
                ),
            });
        }

        let (
            (ih_pen, ih_pen_bytes),
            (off_bmi, off_bmi_bytes),
            (cb_bmi, cb_bmi_bytes),
            (off_bits, off_bits_bytes),
            (cb_bits, cb_bits_bytes),
            (elp, elp_bytes),
        ) = (
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::LogPenEx::parse(buf)?,
        );

        size.consume(
            ih_pen_bytes
                + off_bmi_bytes
                + cb_bmi_bytes
                + off_bits_bytes
                + cb_bits_bytes
                + elp_bytes,
        );

        let ((_, undef_space_bytes), (bmi_src, bmi_src_bytes)) = (
            crate::parser::read_variable(
                buf,
                off_bmi as usize - size.consumed_bytes(),
            )?,
            wmf_core::parser::BitmapInfoHeader::parse(buf)?,
        );

        size.consume(undef_space_bytes + bmi_src_bytes);

        let ((_, undef_space_bytes), (bits_src, bits_src_bytes)) = (
            crate::parser::read_variable(
                buf,
                off_bits as usize - size.consumed_bytes(),
            )?,
            crate::parser::read_variable(buf, cb_bits as usize)?,
        );

        size.consume(undef_space_bytes + bits_src_bytes);

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

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
