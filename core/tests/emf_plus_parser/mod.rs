//! Integration tests for the EMF+ parser: a synthetic stream in the
//! shape produced by GDI+ (header, object, drawing records, EOF) is
//! parsed end-to-end through the public API, including reassembly of
//! continued objects. The record loop mirrors the dispatch the
//! converter performs: read the record header, match on the Type
//! value, and hand the header fields to the record parser.

use emf_core::parser::emf_plus::{
    self, EmfPlusDrawImage, EmfPlusEndOfFile, EmfPlusHeader, EmfPlusObject,
    EmfPlusObjectAssembler, EmfPlusRecordHeader, RecordType, objects,
};

fn emf_plus_record(record_type: u16, flags: u16, data: &[u8]) -> Vec<u8> {
    let mut padded = data.to_vec();
    while padded.len() % 4 != 0 {
        padded.push(0);
    }

    let size = u32::try_from(12 + padded.len()).expect("record should fit");
    let data_size = u32::try_from(data.len()).expect("data should fit");
    let mut record = Vec::with_capacity(size as usize);
    record.extend_from_slice(&record_type.to_le_bytes());
    record.extend_from_slice(&flags.to_le_bytes());
    record.extend_from_slice(&size.to_le_bytes());
    record.extend_from_slice(&data_size.to_le_bytes());
    record.extend_from_slice(&padded);
    record
}

fn header_record() -> Vec<u8> {
    let mut header = Vec::new();
    header.extend_from_slice(&0xDBC0_1002_u32.to_le_bytes());
    header.extend_from_slice(&0_u32.to_le_bytes());
    header.extend_from_slice(&96_u32.to_le_bytes());
    header.extend_from_slice(&96_u32.to_le_bytes());
    emf_plus_record(0x4001, 0, &header)
}

fn bitmap_image_object_data() -> Vec<u8> {
    let mut image = Vec::new();
    image.extend_from_slice(&0xDBC0_1002_u32.to_le_bytes());
    image.extend_from_slice(&1_u32.to_le_bytes()); // ImageDataTypeBitmap
    image.extend_from_slice(&2_i32.to_le_bytes()); // Width
    image.extend_from_slice(&1_i32.to_le_bytes()); // Height
    image.extend_from_slice(&8_i32.to_le_bytes()); // Stride
    image.extend_from_slice(&0x0026_200A_u32.to_le_bytes()); // 32bppARGB
    image.extend_from_slice(&0_u32.to_le_bytes()); // BitmapDataTypePixel
    image.extend_from_slice(&[0, 0, 255, 255, 0, 255, 0, 255]);
    image
}

/// Reads and dispatches one record header the way the converter loop
/// does, returning the typed record type and the parse inputs.
fn read_header(
    buf: &mut &[u8],
) -> (RecordType, u16, u32, emf_core::parser::Size, usize) {
    let (header, consumed) =
        EmfPlusRecordHeader::parse(buf).expect("record header should parse");

    assert_eq!(consumed, EmfPlusRecordHeader::BYTE_SIZE);

    let record_type = RecordType::from_repr(header.record_type)
        .expect("record type should be known");

    (
        record_type,
        header.flags,
        header.size,
        emf_core::parser::Size::from(header.data_size),
        header.padding_bytes(),
    )
}

#[test]
fn parses_a_synthetic_bitmap_stream_end_to_end() {
    let mut data = b"EMF+".to_vec();
    data.extend(header_record());
    data.extend(emf_plus_record(0x4008, 0x0501, &bitmap_image_object_data()));

    let mut draw_image = Vec::new();
    draw_image.extend_from_slice(&u32::MAX.to_le_bytes());
    draw_image.extend_from_slice(&2_u32.to_le_bytes()); // UnitTypePixel
    for value in [0.0_f32, 0.0, 2.0, 1.0, 10.0, 20.0, 40.0, 20.0] {
        draw_image.extend_from_slice(&value.to_le_bytes());
    }
    data.extend(emf_plus_record(0x401A, 0x0001, &draw_image));
    data.extend(emf_plus_record(0x4002, 0, &[]));

    assert!(emf_plus::is_emf_plus_comment(&data));
    let mut buf: &[u8] = &data[4..];

    let (record_type, flags, size, data_size, padding) = read_header(&mut buf);
    assert_eq!(record_type, RecordType::EmfPlusHeader);
    let header =
        EmfPlusHeader::parse(&mut buf, record_type, flags, size, data_size)
            .expect("header record should parse");
    buf = &buf[padding..];

    assert!(!header.dual);
    assert_eq!(header.logical_dpi_x, 96);

    let (record_type, flags, size, data_size, padding) = read_header(&mut buf);
    assert_eq!(record_type, RecordType::EmfPlusObject);
    let object =
        EmfPlusObject::parse(&mut buf, record_type, flags, size, data_size)
            .expect("object record should parse");
    buf = &buf[padding..];

    let mut assembler = EmfPlusObjectAssembler::new();
    let (object_id, object_data) = assembler
        .push(&object)
        .expect("object should parse")
        .expect("object should complete immediately");

    assert_eq!(object_id, 1);

    let objects::EmfPlusObjectData::Image(image) = object_data else {
        panic!("expected an image object");
    };
    let objects::EmfPlusImageData::Bitmap(bitmap) = image.image_data else {
        panic!("expected a bitmap image");
    };
    assert_eq!((bitmap.width, bitmap.height, bitmap.stride), (2, 1, 8));

    let objects::EmfPlusBitmapContent::Pixel {
        pixel_format,
        palette,
        pixel_data,
    } = bitmap.bitmap_data
    else {
        panic!("expected raw pixel content");
    };
    assert_eq!(pixel_format, emf_plus::PixelFormat::PixelFormat32bppARGB,);
    assert!(palette.is_none());
    assert_eq!(pixel_data, vec![0, 0, 255, 255, 0, 255, 0, 255]);

    let (record_type, flags, size, data_size, padding) = read_header(&mut buf);
    assert_eq!(record_type, RecordType::EmfPlusDrawImage);
    let draw =
        EmfPlusDrawImage::parse(&mut buf, record_type, flags, size, data_size)
            .expect("draw image record should parse");
    buf = &buf[padding..];

    assert_eq!(draw.object_id, 1);
    assert_eq!(draw.src_unit, emf_plus::UnitType::UnitTypePixel);
    assert_eq!(draw.src_rect, objects::EmfPlusRectF {
        x: 0.0,
        y: 0.0,
        width: 2.0,
        height: 1.0
    },);
    assert_eq!(draw.rect_data.as_rect_f(), objects::EmfPlusRectF {
        x: 10.0,
        y: 20.0,
        width: 40.0,
        height: 20.0
    },);

    let (record_type, flags, size, data_size, padding) = read_header(&mut buf);
    assert_eq!(record_type, RecordType::EmfPlusEndOfFile);
    EmfPlusEndOfFile::parse(&mut buf, record_type, flags, size, data_size)
        .expect("end of file record should parse");
    buf = &buf[padding..];

    assert!(buf.is_empty());
}

#[test]
fn reassembles_a_continued_object_across_records() {
    let whole = bitmap_image_object_data();
    let total = u32::try_from(whole.len()).expect("object should fit");
    let (first, second) = whole.split_at(whole.len() / 2);

    let mut fragment_1 = total.to_le_bytes().to_vec();
    fragment_1.extend_from_slice(first);
    let mut fragment_2 = total.to_le_bytes().to_vec();
    fragment_2.extend_from_slice(second);

    let mut data = b"EMF+".to_vec();
    data.extend(header_record());
    // The C bit (0x8000) marks the non-final fragment.
    data.extend(emf_plus_record(0x4008, 0x8000 | 0x0502, &fragment_1));
    data.extend(emf_plus_record(0x4008, 0x0502, &fragment_2));
    data.extend(emf_plus_record(0x4002, 0, &[]));

    let mut buf: &[u8] = &data[4..];
    let mut assembler = EmfPlusObjectAssembler::new();
    let mut completed = vec![];

    while !buf.is_empty() {
        let (record_type, flags, size, data_size, padding) =
            read_header(&mut buf);

        match record_type {
            RecordType::EmfPlusObject => {
                let object = EmfPlusObject::parse(
                    &mut buf,
                    record_type,
                    flags,
                    size,
                    data_size,
                )
                .expect("object record should parse");

                if let Some(done) =
                    assembler.push(&object).expect("fragments should assemble")
                {
                    completed.push(done);
                }
            }
            RecordType::EmfPlusHeader => {
                EmfPlusHeader::parse(
                    &mut buf,
                    record_type,
                    flags,
                    size,
                    data_size,
                )
                .expect("header record should parse");
            }
            RecordType::EmfPlusEndOfFile => {
                EmfPlusEndOfFile::parse(
                    &mut buf,
                    record_type,
                    flags,
                    size,
                    data_size,
                )
                .expect("end of file record should parse");
            }
            other => panic!("unexpected record type: {other:?}"),
        }

        buf = &buf[padding..];
    }

    assert_eq!(completed.len(), 1);
    assert!(!assembler.is_pending());

    let (object_id, object_data) = completed.remove(0);
    assert_eq!(object_id, 2);
    assert!(matches!(object_data, objects::EmfPlusObjectData::Image(_),));
}

#[test]
fn detects_non_emf_plus_comments() {
    // EMR_COMMENT public data uses other identifiers (e.g. "GDIC").
    assert!(!emf_plus::is_emf_plus_comment(b"GDIC\x01\x00\x00\x00"));
    assert!(emf_plus::is_emf_plus_comment(b"EMF+"));
}
