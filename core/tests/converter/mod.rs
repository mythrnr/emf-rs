//! End-to-end checks for the one-shot conversion entry points against
//! synthesized metafile binaries: `convert_to_svg` for the built-in
//! players (including the automatic WMF fallback) and `convert` for
//! explicitly supplied players.

use emf_core::converter::ConvertError;

const EMR_EOF: u32 = 0x0000_000E;
const EMR_RECTANGLE: u32 = 0x0000_002B;

/// Builds a minimal 88-byte EMR_HEADER whose bounds and frame span
/// `width` x `height`, so each test case can focus on the records that
/// follow it. The Bytes field is patched by `emf_binary` once the total
/// metafile size is known.
fn emf_header(width: i32, height: i32) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(&0x0000_0001_u32.to_le_bytes()); // Type: EMR_HEADER
    data.extend_from_slice(&88_u32.to_le_bytes()); // Size

    // Bounds and Frame share the same extent here.
    for value in [0_i32, 0, width, height, 0, 0, width, height] {
        data.extend_from_slice(&value.to_le_bytes());
    }

    data.extend_from_slice(&0x464D_4520_u32.to_le_bytes()); // Signature " EMF"
    data.extend_from_slice(&0x0001_0000_u32.to_le_bytes()); // Version
    data.extend_from_slice(&0_u32.to_le_bytes()); // Bytes (patched later)
    data.extend_from_slice(&0_u32.to_le_bytes()); // Records
    data.extend_from_slice(&1_u16.to_le_bytes()); // Handles
    data.extend_from_slice(&0_u16.to_le_bytes()); // Reserved
    data.extend_from_slice(&0_u32.to_le_bytes()); // nDescription
    data.extend_from_slice(&0_u32.to_le_bytes()); // offDescription
    data.extend_from_slice(&0_u32.to_le_bytes()); // nPalEntries

    // Device and Millimeters describe the reference device; they do not
    // affect playback under the default MM_TEXT mapping mode.
    for value in [100_u32, 100, 100, 100] {
        data.extend_from_slice(&value.to_le_bytes());
    }

    data
}

/// Encodes a record as Type + Size + 4-byte params, with the size
/// derived from the parameter count so cases stay consistent.
fn record(record_type: u32, params: &[i32]) -> Vec<u8> {
    let size = u32::try_from(8 + params.len() * 4).expect("too many params");
    let mut data = Vec::new();
    data.extend_from_slice(&record_type.to_le_bytes());
    data.extend_from_slice(&size.to_le_bytes());

    for param in params {
        data.extend_from_slice(&param.to_le_bytes());
    }

    data
}

fn emf_binary(width: i32, height: i32, records: &[Vec<u8>]) -> Vec<u8> {
    let mut data = emf_header(width, height);

    for r in records {
        data.extend_from_slice(r);
    }

    // The header's Bytes field must reflect the total metafile size.
    let total = u32::try_from(data.len()).expect("EMF should fit u32");
    data[48..52].copy_from_slice(&total.to_le_bytes());
    data
}

/// Builds a minimal WMF binary (memory metafile, version 3) carrying a
/// window extent and EOF, to exercise the automatic WMF fallback.
fn wmf_binary() -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(&0x0001_u16.to_le_bytes()); // Type: memory
    data.extend_from_slice(&9_u16.to_le_bytes()); // HeaderSize in words
    data.extend_from_slice(&0x0300_u16.to_le_bytes()); // Version
    data.extend_from_slice(&0_u16.to_le_bytes()); // SizeLow
    data.extend_from_slice(&0_u16.to_le_bytes()); // SizeHigh
    data.extend_from_slice(&0_u16.to_le_bytes()); // NumberOfObjects
    data.extend_from_slice(&0_u32.to_le_bytes()); // MaxRecord
    data.extend_from_slice(&0_u16.to_le_bytes()); // NumberOfMembers

    // META_SETWINDOWEXT (Y then X on the wire), then META_EOF.
    let records: [(u16, &[i16]); 2] = [(0x020C, &[200, 320]), (0x0000, &[])];

    for (function, params) in records {
        let words = 3 + u32::try_from(params.len()).expect("too many params");
        data.extend_from_slice(&words.to_le_bytes());
        data.extend_from_slice(&function.to_le_bytes());

        for param in params {
            data.extend_from_slice(&param.to_le_bytes());
        }
    }

    data
}

#[test]
fn convert_to_svg_renders_records() {
    struct TestCase {
        desc: &'static str,
        width: i32,
        height: i32,
        records: Vec<Vec<u8>>,
        expected_svg: &'static str,
    }

    let cases = [
        TestCase {
            desc: "Empty metafile produces an empty SVG document",
            width: 100,
            height: 100,
            records: vec![record(EMR_EOF, &[0, 0])],
            expected_svg: r#"<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg"></svg>"#,
        },
        TestCase {
            desc: "Header bounds become the viewBox",
            width: 320,
            height: 200,
            records: vec![record(EMR_EOF, &[0, 0])],
            expected_svg: r#"<svg viewBox="0 0 320 200" xmlns="http://www.w3.org/2000/svg"></svg>"#,
        },
        TestCase {
            desc: "Rectangle is rendered with default pen and brush",
            width: 200,
            height: 200,
            records: vec![
                // EMR_RECTANGLE carries Left, Top, Right, Bottom.
                record(EMR_RECTANGLE, &[10, 20, 150, 120]),
                record(EMR_EOF, &[0, 0]),
            ],
            expected_svg: r##"<svg viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg"><rect fill="#000000" fill-rule="evenodd" height="100" id="elem1" stroke="#000000" stroke-dasharray="none" stroke-linecap="butt" stroke-linejoin="miter" stroke-opacity="1.00" stroke-width="1" width="140" x="10" y="20"></rect></svg>"##,
        },
    ];

    for (i, case) in cases.iter().enumerate() {
        let data = emf_binary(case.width, case.height, &case.records);
        let svg = emf_core::converter::convert_to_svg(data.as_slice())
            .unwrap_or_else(|err| {
                panic!("case {i}: {}: conversion failed: {err}", case.desc)
            });
        let svg_str = String::from_utf8(svg).expect("SVG output is not UTF-8");

        assert_eq!(
            svg_str.trim(),
            case.expected_svg.trim(),
            "case {i}: {}: SVG output does not match expected",
            case.desc,
        );
    }
}

#[test]
fn convert_to_svg_rejects_broken_input() {
    enum Expected {
        // The input is recognized as EMF but its records are malformed.
        ParseError,
        // The input is not recognized as EMF, so conversion falls back
        // to WMF, which rejects it as well.
        WmfConvertError,
    }

    struct TestCase {
        desc: &'static str,
        data: Vec<u8>,
        expected: Expected,
    }

    let cases = [
        TestCase {
            desc: "Empty input",
            data: Vec::new(),
            expected: Expected::WmfConvertError,
        },
        TestCase {
            desc: "Input shorter than the 4-byte record type field",
            data: vec![0x01, 0x00],
            expected: Expected::WmfConvertError,
        },
        TestCase {
            desc: "First record is not EMR_HEADER",
            data: emf_binary(100, 100, &[])[88..].to_vec(),
            expected: Expected::WmfConvertError,
        },
        TestCase {
            desc: "Truncated EMR_HEADER",
            data: emf_header(100, 100)[..40].to_vec(),
            expected: Expected::ParseError,
        },
        TestCase {
            desc: "Header without any record",
            data: emf_binary(100, 100, &[]),
            expected: Expected::ParseError,
        },
        TestCase {
            desc: "Unknown record type after the header",
            data: emf_binary(100, 100, &[
                record(0x7FFF_FFFF, &[]),
                record(EMR_EOF, &[0, 0]),
            ]),
            expected: Expected::ParseError,
        },
        TestCase {
            desc: "Record payload shorter than the declared size",
            data: emf_binary(100, 100, &[{
                // Declares a 24-byte rectangle record but omits the box.
                let mut r = Vec::new();
                r.extend_from_slice(&EMR_RECTANGLE.to_le_bytes());
                r.extend_from_slice(&24_u32.to_le_bytes());
                r
            }]),
            expected: Expected::ParseError,
        },
    ];

    for (i, case) in cases.iter().enumerate() {
        let result = emf_core::converter::convert_to_svg(case.data.as_slice());

        match case.expected {
            Expected::ParseError => assert!(
                matches!(result, Err(ConvertError::ParseError { .. })),
                "case {i}: {}: expected ParseError, got {result:?}",
                case.desc,
            ),
            Expected::WmfConvertError => assert!(
                matches!(result, Err(ConvertError::WMFConvertError { .. })),
                "case {i}: {}: expected WMFConvertError, got {result:?}",
                case.desc,
            ),
        }
    }
}

#[test]
fn convert_to_svg_falls_back_to_wmf() {
    let data = wmf_binary();

    let svg = emf_core::converter::convert_to_svg(data.as_slice())
        .expect("WMF input should fall back to the WMF player");
    let svg_str = String::from_utf8(svg).expect("SVG output is not UTF-8");

    assert_eq!(
        svg_str.trim(),
        r#"<svg viewBox="0 0 320 200" xmlns="http://www.w3.org/2000/svg"></svg>"#,
        "WMF fallback SVG output does not match expected",
    );
}

#[test]
fn convert_drives_explicit_players() {
    // `convert` is the generic entry point behind `convert_to_svg`;
    // drive it with explicitly constructed players to cover the
    // three-argument signature.
    let data = emf_binary(100, 100, &[
        record(EMR_RECTANGLE, &[10, 20, 150, 120]),
        record(EMR_EOF, &[0, 0]),
    ]);

    let output = emf_core::converter::convert(
        data.as_slice(),
        emf_core::converter::SVGPlayer::new(),
        emf_core::wmf_core::converter::SVGPlayer::new(),
    )
    .expect("conversion failed");

    let svg = String::from_utf8(output).expect("SVG output is not UTF-8");
    assert!(svg.contains("<rect "), "expected a rect element: {svg}");
}

/// A player that relies on the default no-op record handlers of
/// [`emf_core::converter::Player`], overriding only the records it
/// observes.
struct RecordingPlayer {
    events: Vec<String>,
}

impl emf_core::converter::Player for RecordingPlayer {
    fn generate(self) -> Result<Vec<u8>, emf_core::converter::PlayError> {
        Ok(self.events.join(",").into_bytes())
    }

    fn rectangle(
        mut self,
        record_number: usize,
        _record: emf_core::parser::EMR_RECTANGLE,
    ) -> Result<Self, emf_core::converter::PlayError> {
        self.events.push(format!("rectangle:{record_number}"));

        Ok(self)
    }
}

#[test]
fn convert_drives_player_default_handlers() {
    // EMR_HEADER and EMR_EOF flow through the default no-op handlers;
    // only the overridden `rectangle` observes its record.
    let data = emf_binary(100, 100, &[
        record(EMR_RECTANGLE, &[10, 20, 150, 120]),
        record(EMR_EOF, &[0, 0]),
    ]);

    let output = emf_core::converter::convert(
        data.as_slice(),
        RecordingPlayer { events: Vec::new() },
        emf_core::wmf_core::converter::SVGPlayer::new(),
    )
    .expect("conversion failed");

    assert_eq!(
        String::from_utf8(output).expect("output is not UTF-8"),
        "rectangle:1",
    );
}
