/// The EMR_HEADER record is the starting point of an EMF metafile. It specifies
/// properties of the device on which the image in the metafile was recorded;
/// this information in the header record makes it possible for EMF metafiles to
/// be independent of any specific output device.
#[derive(Clone, Debug)]
pub struct EMR_HEADER {
    /// Type (4 bytes): An unsigned integer that identifies this record type as
    /// EMR_HEADER. This value is 0x00000001.
    pub record_type: crate::parser::RecordType,
    /// Size (4 bytes): An unsigned integer that specifies the size in bytes of
    /// this record in the metafile. This value MUST be a multiple of 4 bytes.
    pub size: crate::parser::Size,
    /// EmfHeader (80 bytes): A Header object, which contains information about
    /// the content and structure of the metafile.
    pub emf_header: crate::parser::Header,
    /// EmfHeaderRecordBuffer (variable, optional): An array of bytes that
    /// contains the remainder of the EMF header record.
    pub emf_header_record_buffer: Option<EmfHeaderRecordBuffer>,
}

impl EMR_HEADER {
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
        if record_type != crate::parser::RecordType::EMR_HEADER {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "record_type must be `{:#010X}`, but specified `{:#010X}`",
                    crate::parser::RecordType::EMR_HEADER as u32,
                    record_type as u32
                ),
            });
        }

        let (emf_header, emf_header_bytes) = crate::parser::Header::parse(buf)?;
        size.consume(emf_header_bytes);

        let (emf_header_record_buffer, size) =
            EmfHeaderRecordBuffer::parse(buf, &emf_header, size)?;

        crate::parser::records::consume_remaining_bytes(
            buf,
            size.remaining_bytes(),
        )?;

        Ok(Self { record_type, size, emf_header, emf_header_record_buffer })
    }
}

#[derive(Clone, Debug)]
pub enum EmfHeaderRecordBuffer {
    /// The EmfMetafileHeader record is the header record used in the original
    /// version of EMF metafiles.
    EmfMetafileHeader {
        /// EmfDescription (variable): A null-terminated Unicode UTF16-LE
        /// string of arbitrary length and content. Its location in the record
        /// and number of characters are specified by the offDescription and
        /// nDescription fields, respectively, in EmfHeader. If the value of
        /// either field is zero, no description string is present.
        emf_description: Option<String>,
    },
    /// The EmfMetafileHeaderExtension1 record is the header record used in the
    /// first extension to EMF metafiles. Following the EmfHeaderExtension1
    /// field, the remaining fields are optional and can be present in any
    /// order.
    EmfMetafileHeaderExtension1 {
        /// EmfHeaderExtension1 (12 bytes): A HeaderExtension1 object, which
        /// specifies additional information about the image in the metafile.
        emf_header_extension_1: crate::parser::HeaderExtension1,
        /// EmfDescription (variable): A null-terminated Unicode UTF16-LE
        /// string of arbitrary length and content. Its location in the record
        /// and number of characters are specified by the offDescription and
        /// nDescription fields, respectively, in EmfHeader. If the value of
        /// either field is zero, no description string is present.
        emf_description: Option<String>,
        /// EmfPixelFormat (40 bytes): A PixelFormatDescriptor object, which
        /// specifies the pixel format that was defined when the metafile was
        /// recorded. Its size and location in the record are specified by the
        /// cbPixelFormat and offPixelFormat fields, respectively, in
        /// EmfHeaderExtension1. If the value of either field is zero, no pixel
        /// format descriptor is present.
        emf_pixel_format: Option<crate::parser::PixelFormatDescriptor>,
    },
    EmfMetafileHeaderExtension2 {
        /// EmfHeaderExtension1 (12 bytes): A HeaderExtension1 object, which
        /// specifies additional information about the image in the metafile.
        emf_header_extension_1: crate::parser::HeaderExtension1,
        /// EmfHeaderExtension2 (8 bytes): A HeaderExtension2 object, which
        /// specifies additional information about the image in the metafile.
        emf_header_extension_2: crate::parser::HeaderExtension2,
        /// EmfDescription (variable): A null-terminated Unicode UTF16-LE
        /// string of arbitrary length and content. Its location in the record
        /// and number of characters are specified by the offDescription and
        /// nDescription fields, respectively, in EmfHeader. If the value of
        /// either field is zero, no description string is present.
        emf_description: Option<String>,
        /// EmfPixelFormat (40 bytes): A PixelFormatDescriptor object that
        /// specifies the last pixel format that was defined when the metafile
        /// was recorded. Its size and location in the record are specified by
        /// the cbPixelFormat and offPixelFormat fields, respectively, in
        /// EmfHeaderExtension1. If the value of either field is zero, no pixel
        /// format descriptor is present.
        emf_pixel_format: Option<crate::parser::PixelFormatDescriptor>,
    },
}

impl EmfHeaderRecordBuffer {
    fn parse<R: std::io::Read>(
        buf: &mut R,
        emf_header: &crate::parser::Header,
        mut size: crate::parser::Size,
    ) -> Result<(Option<Self>, crate::parser::Size), crate::parser::ParseError>
    {
        // Valid header record size?
        if size.byte_count() < 88 {
            return Ok((None, size));
        }

        // Initialize HeaderSize to minimum
        // Set HeaderSize to header record size
        let mut header_size = size.byte_count() as u32;

        // Valid description values?
        if emf_header.off_description >= 88
            && (emf_header.off_description + emf_header.n_description * 2)
                <= size.byte_count() as u32
        {
            // Set HeaderSize to description offset
            header_size = emf_header.off_description;
        }

        // Header big enough to contain extension?
        if header_size < 100 {
            return Self::parse_as_emf_file_header(buf, emf_header, size);
        }

        let (emf_header_extension_1, emf_header_extension_1_bytes) =
            crate::parser::HeaderExtension1::parse(buf)?;
        size.consume(emf_header_extension_1_bytes);

        // Valid pixel format values?
        if emf_header_extension_1.off_pixel_format < 100
            || (emf_header_extension_1.off_pixel_format
                + emf_header_extension_1.cb_pixel_format
                > size.byte_count() as u32)
            // Pixel format before description?
            || emf_header_extension_1.off_pixel_format < header_size
        {
            return if header_size < 108 {
                Self::parse_as_emf_file_header_extension_1(
                    buf,
                    emf_header,
                    emf_header_extension_1,
                    size,
                )
            } else {
                Self::parse_as_emf_file_header_extension_2(
                    buf,
                    emf_header,
                    emf_header_extension_1,
                    size,
                )
            };
        }

        // Set HeaderSize to pixel format offset
        header_size = emf_header_extension_1.off_pixel_format;

        if header_size < 108 {
            Self::parse_as_emf_file_header_extension_1(
                buf,
                emf_header,
                emf_header_extension_1,
                size,
            )
        } else {
            Self::parse_as_emf_file_header_extension_2(
                buf,
                emf_header,
                emf_header_extension_1,
                size,
            )
        }
    }

    fn parse_as_emf_file_header<R: std::io::Read>(
        buf: &mut R,
        emf_header: &crate::parser::Header,
        mut size: crate::parser::Size,
    ) -> Result<(Option<Self>, crate::parser::Size), crate::parser::ParseError>
    {
        let description_exists =
            emf_header.off_description != 0 && emf_header.n_description > 0;

        if !description_exists {
            return Ok((
                Some(Self::EmfMetafileHeader { emf_description: None }),
                size,
            ));
        }

        let ((_, undefined_bytes), (b, description_bytes)) = (
            crate::parser::read_variable(
                buf,
                emf_header.off_description as usize - size.consumed_bytes(),
            )?,
            crate::parser::read_variable(
                buf,
                (emf_header.n_description * 2) as usize,
            )?,
        );

        size.consume(undefined_bytes + description_bytes);

        let emf_description = Some(crate::parser::utf16le_bytes_to_string(&b)?);

        Ok((Some(Self::EmfMetafileHeader { emf_description }), size))
    }

    fn parse_as_emf_file_header_extension_1<R: std::io::Read>(
        buf: &mut R,
        emf_header: &crate::parser::Header,
        emf_header_extension_1: crate::parser::HeaderExtension1,
        mut size: crate::parser::Size,
    ) -> Result<(Option<Self>, crate::parser::Size), crate::parser::ParseError>
    {
        let description_exists =
            emf_header.off_description != 0 && emf_header.n_description > 0;

        let emf_description = if description_exists {
            let ((_, undefined_bytes), (b, description_bytes)) = (
                crate::parser::read_variable(
                    buf,
                    emf_header.off_description as usize - size.consumed_bytes(),
                )?,
                crate::parser::read_variable(
                    buf,
                    (emf_header.n_description * 2) as usize,
                )?,
            );

            size.consume(undefined_bytes + description_bytes);

            Some(crate::parser::utf16le_bytes_to_string(&b)?)
        } else {
            None
        };

        let pixel_format_exists = emf_header_extension_1.cb_pixel_format > 0
            && emf_header_extension_1.off_pixel_format > 0;

        let emf_pixel_format = if pixel_format_exists {
            let ((_, undefined_bytes), (pixel_format, pixel_format_bytes)) = (
                crate::parser::read_variable(
                    buf,
                    emf_header_extension_1.off_pixel_format as usize
                        - size.consumed_bytes(),
                )?,
                crate::parser::PixelFormatDescriptor::parse(buf)?,
            );

            size.consume(undefined_bytes + pixel_format_bytes);

            Some(pixel_format)
        } else {
            None
        };

        Ok((
            Some(Self::EmfMetafileHeaderExtension1 {
                emf_header_extension_1,
                emf_description,
                emf_pixel_format,
            }),
            size,
        ))
    }

    fn parse_as_emf_file_header_extension_2<R: std::io::Read>(
        buf: &mut R,
        emf_header: &crate::parser::Header,
        emf_header_extension_1: crate::parser::HeaderExtension1,
        mut size: crate::parser::Size,
    ) -> Result<(Option<Self>, crate::parser::Size), crate::parser::ParseError>
    {
        let (emf_header_extension_2, emf_header_extension_2_bytes) =
            crate::parser::HeaderExtension2::parse(buf)?;
        size.consume(emf_header_extension_2_bytes);

        let description_exists =
            emf_header.off_description != 0 && emf_header.n_description > 0;

        let emf_description = if description_exists {
            let ((_, undefined_bytes), (b, description_bytes)) = (
                crate::parser::read_variable(
                    buf,
                    emf_header.off_description as usize - size.consumed_bytes(),
                )?,
                crate::parser::read_variable(
                    buf,
                    (emf_header.n_description * 2) as usize,
                )?,
            );

            size.consume(undefined_bytes + description_bytes);

            Some(crate::parser::utf16le_bytes_to_string(&b)?)
        } else {
            None
        };

        let pixel_format_exists = emf_header_extension_1.cb_pixel_format > 0
            && emf_header_extension_1.off_pixel_format > 0;

        let emf_pixel_format = if pixel_format_exists {
            let ((_, undefined_bytes), (pixel_format, pixel_format_bytes)) = (
                crate::parser::read_variable(
                    buf,
                    emf_header_extension_1.off_pixel_format as usize
                        - size.consumed_bytes(),
                )?,
                crate::parser::PixelFormatDescriptor::parse(buf)?,
            );

            size.consume(undefined_bytes + pixel_format_bytes);

            Some(pixel_format)
        } else {
            None
        };

        Ok((
            Some(Self::EmfMetafileHeaderExtension2 {
                emf_header_extension_1,
                emf_header_extension_2,
                emf_description,
                emf_pixel_format,
            }),
            size,
        ))
    }
}
