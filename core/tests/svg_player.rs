//! End-to-end checks for `SVGPlayer` against representative EMF records.
//!
//! Each test seeds the playback context with `EMR_HEADER`, applies the
//! record under test, and asserts the generated SVG contains the expected
//! elements and attributes.
#![allow(clippy::cast_sign_loss)]

use emf_core::{
    converter::{Player, SVGPlayer},
    parser::{
        EMR_HEADER, EMR_RECTANGLE, FormatSignature, Header, RecordType, Size,
    },
};
use wmf_core::parser::{RectL, SizeL};

fn build_header(width: i32, height: i32) -> EMR_HEADER {
    EMR_HEADER {
        record_type: RecordType::EMR_HEADER,
        size: Size::from(0),
        emf_header: Header {
            bounds: RectL { left: 0, top: 0, right: width, bottom: height },
            frame: RectL { left: 0, top: 0, right: width, bottom: height },
            record_signature: FormatSignature::ENHMETA_SIGNATURE,
            version: 0x0001_0000,
            bytes: 0,
            records: 0,
            handles: 1,
            reserved: 0,
            n_description: 0,
            off_description: 0,
            n_pal_entries: 0,
            device: SizeL { cx: width as u32, cy: height as u32 },
            millimeters: SizeL { cx: width as u32, cy: height as u32 },
        },
        emf_header_record_buffer: None,
    }
}

fn render(player: SVGPlayer) -> String {
    let bytes = player.generate().expect("SVG generation failed");
    String::from_utf8(bytes).expect("SVG output is not UTF-8")
}

#[test]
fn rectangle_record_emits_rect_in_viewbox() {
    let player = SVGPlayer::new()
        .header(0, build_header(1024, 768))
        .expect("header should succeed");

    let player = player
        .rectangle(1, EMR_RECTANGLE {
            record_type: RecordType::EMR_RECTANGLE,
            size: Size::from(0),
            bx: RectL { left: 100, top: 50, right: 300, bottom: 200 },
        })
        .expect("rectangle should succeed");

    let svg = render(player);
    assert!(svg.contains("<svg"), "missing <svg> open tag: {svg}");
    assert!(
        svg.contains(r#"viewBox="0 0 1024 768""#),
        "viewBox does not match header bounds: {svg}",
    );
    assert!(
        svg.contains("<rect "),
        "expected <rect> element to be emitted: {svg}",
    );
    // Top-left (100, 50) with width 200 and height 150 must appear as attrs.
    assert!(svg.contains(r#"x="100""#), "rect x attr missing: {svg}");
    assert!(svg.contains(r#"y="50""#), "rect y attr missing: {svg}");
    assert!(svg.contains(r#"width="200""#), "rect width attr missing: {svg}");
    assert!(svg.contains(r#"height="150""#), "rect height attr missing: {svg}");
    assert!(svg.contains(r#"id="elem1""#), "elem id missing: {svg}");
}

#[test]
fn multiple_records_assign_unique_element_ids() {
    let player = SVGPlayer::new()
        .header(0, build_header(1024, 768))
        .expect("header should succeed");

    let player = player
        .rectangle(1, EMR_RECTANGLE {
            record_type: RecordType::EMR_RECTANGLE,
            size: Size::from(0),
            bx: RectL { left: 0, top: 0, right: 100, bottom: 100 },
        })
        .expect("first rectangle should succeed");

    let player = player
        .rectangle(2, EMR_RECTANGLE {
            record_type: RecordType::EMR_RECTANGLE,
            size: Size::from(0),
            bx: RectL { left: 200, top: 200, right: 400, bottom: 300 },
        })
        .expect("second rectangle should succeed");

    let svg = render(player);
    assert!(svg.contains(r#"id="elem1""#), "elem1 missing: {svg}");
    assert!(svg.contains(r#"id="elem2""#), "elem2 missing: {svg}");
    // Each rectangle is rendered at a distinct position.
    assert!(svg.contains(r#"x="0""#));
    assert!(svg.contains(r#"x="200""#));
}

#[test]
fn empty_metafile_emits_only_svg_skeleton() {
    let player = SVGPlayer::new()
        .header(0, build_header(640, 480))
        .expect("header should succeed");

    let svg = render(player);
    assert!(svg.starts_with("<svg"));
    assert!(svg.ends_with("</svg>"));
    assert!(svg.contains(r#"viewBox="0 0 640 480""#));
    // No drawing elements should appear in the body.
    assert!(!svg.contains("<rect"));
    assert!(!svg.contains("<polyline"));
    assert!(!svg.contains("<path"));
}
