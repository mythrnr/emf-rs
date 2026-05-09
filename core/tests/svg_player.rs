//! End-to-end checks for `SVGPlayer` against representative EMF records.
//!
//! Each test seeds the playback context with `EMR_HEADER`, applies the
//! record under test, and asserts the generated SVG contains the expected
//! elements and attributes.
#![allow(clippy::cast_sign_loss)]

use emf_core::{
    converter::{Player, SVGPlayer},
    parser::{
        EMR_BEGINPATH, EMR_ENDPATH, EMR_FILLPATH, EMR_HEADER, EMR_MOVETOEX,
        EMR_POLYBEZIER, EMR_POLYBEZIERTO, EMR_POLYPOLYGON16,
        EMR_POLYPOLYLINE16, EMR_RECTANGLE, EMR_SETMAPMODE,
        EMR_SETVIEWPORTEXTEX, EMR_SETWINDOWEXTEX, FormatSignature, Header,
        MapMode, RecordType, Size,
    },
};
use wmf_core::parser::{PointL, PointS, RectL, SizeL};

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
fn poly_bezier_path_starts_with_moveto() {
    // Regression: EMR_POLYBEZIER inside a path bracket used to emit
    // path data starting with `C` because the moveto for the first
    // point was suppressed. The first aPoints entry is the curve's
    // starting point, so the SVG path must begin with `M`.
    let bounds = RectL { left: 0, top: 0, right: 200, bottom: 200 };
    let player = SVGPlayer::new()
        .header(0, build_header(1024, 768))
        .expect("header should succeed")
        .begin_path(1, EMR_BEGINPATH {
            record_type: RecordType::EMR_BEGINPATH,
            size: Size::from(8),
        })
        .expect("begin_path should succeed")
        .poly_bezier(2, EMR_POLYBEZIER {
            record_type: RecordType::EMR_POLYBEZIER,
            size: Size::from(0),
            bounds: bounds.clone(),
            count: 4,
            a_points: vec![
                PointL { x: 10, y: 20 },
                PointL { x: 30, y: 40 },
                PointL { x: 50, y: 60 },
                PointL { x: 70, y: 80 },
            ],
        })
        .expect("poly_bezier should succeed")
        .end_path(3, EMR_ENDPATH {
            record_type: RecordType::EMR_ENDPATH,
            size: Size::from(8),
        })
        .expect("end_path should succeed")
        .fill_path(4, EMR_FILLPATH {
            record_type: RecordType::EMR_FILLPATH,
            size: Size::from(0),
            bounds,
        })
        .expect("fill_path should succeed");

    let svg = render(player);

    assert!(
        svg.contains(r#"d="M 10 20 C 30 40 50 60 70 80""#),
        "path data must start with moveto to the first aPoints entry: {svg}",
    );
}

#[test]
fn poly_bezier_to_seeds_moveto_when_path_empty() {
    // Regression: EMR_POLYBEZIERTO records whose curves originate from
    // the current drawing position must seed the SVG path with M when
    // the shared path buffer is still empty (e.g. PolyBezierTo is the
    // first command after BeginPath without an intervening MoveToEx).
    let bounds = RectL { left: 0, top: 0, right: 200, bottom: 200 };
    let player = SVGPlayer::new()
        .header(0, build_header(1024, 768))
        .expect("header should succeed")
        .move_to_ex(1, EMR_MOVETOEX {
            record_type: RecordType::EMR_MOVETOEX,
            size: Size::from(0),
            offset: PointL { x: 5, y: 6 },
        })
        .expect("move_to_ex should succeed")
        .begin_path(2, EMR_BEGINPATH {
            record_type: RecordType::EMR_BEGINPATH,
            size: Size::from(8),
        })
        .expect("begin_path should succeed")
        .poly_bezier_to(3, EMR_POLYBEZIERTO {
            record_type: RecordType::EMR_POLYBEZIERTO,
            size: Size::from(0),
            bounds: bounds.clone(),
            count: 3,
            a_points: vec![
                PointL { x: 10, y: 20 },
                PointL { x: 30, y: 40 },
                PointL { x: 50, y: 60 },
            ],
        })
        .expect("poly_bezier_to should succeed")
        .end_path(4, EMR_ENDPATH {
            record_type: RecordType::EMR_ENDPATH,
            size: Size::from(8),
        })
        .expect("end_path should succeed")
        .fill_path(5, EMR_FILLPATH {
            record_type: RecordType::EMR_FILLPATH,
            size: Size::from(0),
            bounds,
        })
        .expect("fill_path should succeed");

    let svg = render(player);

    assert!(
        svg.contains(r#"d="M 5 6 C 10 20 30 40 50 60""#),
        "path data must start with moveto from the current drawing position: \
         {svg}",
    );
}

#[test]
fn header_does_not_offset_viewport_origin() {
    // Regression: when bounds had a non-zero top-left (e.g. (33, 211)),
    // the header used to seed the viewport/window origins from bounds,
    // which subsequent SETVIEWPORTEXTEX / SETWINDOWEXTEX records then
    // baked into every transformed point. The bounds determine the
    // SVG viewBox; the transform must keep the EMF default of
    // viewport/window origin (0, 0).
    let header = EMR_HEADER {
        record_type: RecordType::EMR_HEADER,
        size: Size::from(0),
        emf_header: Header {
            bounds: RectL { left: 33, top: 211, right: 233, bottom: 294 },
            frame: RectL { left: 0, top: 0, right: 200, bottom: 83 },
            record_signature: FormatSignature::ENHMETA_SIGNATURE,
            version: 0x0001_0000,
            bytes: 0,
            records: 0,
            handles: 1,
            reserved: 0,
            n_description: 0,
            off_description: 0,
            n_pal_entries: 0,
            device: SizeL { cx: 4800, cy: 6400 },
            millimeters: SizeL { cx: 203, cy: 271 },
        },
        emf_header_record_buffer: None,
    };

    let player = SVGPlayer::new()
        .header(0, header)
        .expect("header should succeed")
        .set_map_mode(1, EMR_SETMAPMODE {
            record_type: RecordType::EMR_SETMAPMODE,
            size: Size::from(0),
            map_mode: MapMode::MM_ANISOTROPIC,
        })
        .expect("set_map_mode should succeed")
        .set_viewport_ext_ex(2, EMR_SETVIEWPORTEXTEX {
            record_type: RecordType::EMR_SETVIEWPORTEXTEX,
            size: Size::from(0),
            extent: SizeL { cx: 200, cy: 100 },
        })
        .expect("set_viewport_ext_ex should succeed")
        .set_window_ext_ex(3, EMR_SETWINDOWEXTEX {
            record_type: RecordType::EMR_SETWINDOWEXTEX,
            size: Size::from(0),
            extent: SizeL { cx: 100, cy: 50 },
        })
        .expect("set_window_ext_ex should succeed")
        // Logical (10, 10) should map to device (10 * 200/100, 10 *
        // 100/50) = (20, 20). If header pre-loaded the viewport origin
        // with bounds.top-left, the result would shift by (33, 211)
        // and land at (53, 231), which is past the viewBox top.
        .rectangle(4, EMR_RECTANGLE {
            record_type: RecordType::EMR_RECTANGLE,
            size: Size::from(0),
            bx: RectL { left: 10, top: 10, right: 20, bottom: 20 },
        })
        .expect("rectangle should succeed");

    let svg = render(player);

    assert!(
        svg.contains(r#"viewBox="33 211 200 83""#),
        "viewBox should still reflect the EMF bounds: {svg}",
    );
    assert!(
        svg.contains(r#"x="20""#) && svg.contains(r#"y="20""#),
        "rectangle must transform without the bounds offset: {svg}",
    );
}

#[test]
fn poly_polyline_16_assigns_suffixed_ids_per_subpath() {
    // Regression: each sub-polyline used to receive the same
    // `elem{record_number}` id, producing duplicate-id SVG. The first
    // sub-shape keeps the bare `elem{N}` id; later sub-shapes get
    // `elem{N}-1`, `elem{N}-2`, ... so the SVG id-uniqueness rule
    // holds while the per-record prefix stays addressable.
    let player = SVGPlayer::new()
        .header(0, build_header(1024, 768))
        .expect("header should succeed")
        .poly_polyline_16(7, EMR_POLYPOLYLINE16 {
            record_type: RecordType::EMR_POLYPOLYLINE16,
            size: Size::from(0),
            bounds: RectL { left: 0, top: 0, right: 100, bottom: 100 },
            number_of_polylines: 3,
            count: 6,
            polyline_point_count: vec![2, 2, 2],
            a_points: vec![
                PointS { x: 0, y: 0 },
                PointS { x: 10, y: 10 },
                PointS { x: 20, y: 20 },
                PointS { x: 30, y: 30 },
                PointS { x: 40, y: 40 },
                PointS { x: 50, y: 50 },
            ],
        })
        .expect("poly_polyline_16 should succeed");

    let svg = render(player);

    assert_eq!(
        svg.matches(r#"id="elem7""#).count(),
        1,
        "elem7 must appear exactly once: {svg}",
    );
    assert_eq!(
        svg.matches(r#"id="elem7-1""#).count(),
        1,
        "elem7-1 must appear exactly once: {svg}",
    );
    assert_eq!(
        svg.matches(r#"id="elem7-2""#).count(),
        1,
        "elem7-2 must appear exactly once: {svg}",
    );
    assert_eq!(
        svg.matches("<path ").count(),
        3,
        "all sub-polylines should still be emitted: {svg}",
    );
}

#[test]
fn poly_polygon_16_assigns_suffixed_ids_per_subpolygon() {
    // Regression: poly_polygon_16 emitted a `<polygon>` per sub-shape
    // each with the same id. Multi-shape records must give the second
    // and later sub-shapes a numeric suffix so SVG ids stay unique.
    let player = SVGPlayer::new()
        .header(0, build_header(1024, 768))
        .expect("header should succeed")
        .poly_polygon_16(11, EMR_POLYPOLYGON16 {
            record_type: RecordType::EMR_POLYPOLYGON16,
            size: Size::from(0),
            bounds: RectL { left: 0, top: 0, right: 100, bottom: 100 },
            number_of_polygons: 2,
            count: 6,
            polygon_point_count: vec![3, 3],
            a_points: vec![
                PointS { x: 0, y: 0 },
                PointS { x: 10, y: 0 },
                PointS { x: 5, y: 10 },
                PointS { x: 20, y: 20 },
                PointS { x: 30, y: 20 },
                PointS { x: 25, y: 30 },
            ],
        })
        .expect("poly_polygon_16 should succeed");

    let svg = render(player);

    assert_eq!(
        svg.matches(r#"id="elem11""#).count(),
        1,
        "elem11 must appear exactly once: {svg}",
    );
    assert_eq!(
        svg.matches(r#"id="elem11-1""#).count(),
        1,
        "elem11-1 must appear exactly once: {svg}",
    );
    assert_eq!(
        svg.matches("<polygon ").count(),
        2,
        "both sub-polygons should still be emitted: {svg}",
    );
}

#[test]
fn poly_polyline_16_keeps_bare_id_when_single_subpath() {
    // When the record collapses to a single sub-shape no suffix is
    // appended; the bare `elem{N}` id is kept so element-level
    // addressing matches the simple-record case.
    let player = SVGPlayer::new()
        .header(0, build_header(1024, 768))
        .expect("header should succeed")
        .poly_polyline_16(13, EMR_POLYPOLYLINE16 {
            record_type: RecordType::EMR_POLYPOLYLINE16,
            size: Size::from(0),
            bounds: RectL { left: 0, top: 0, right: 100, bottom: 100 },
            number_of_polylines: 1,
            count: 2,
            polyline_point_count: vec![2],
            a_points: vec![PointS { x: 0, y: 0 }, PointS { x: 10, y: 10 }],
        })
        .expect("poly_polyline_16 should succeed");

    let svg = render(player);

    assert!(
        svg.contains(r"<path ") && svg.contains(r#"id="elem13""#),
        "single sub-shape should carry the bare id directly: {svg}",
    );
    assert!(
        !svg.contains(r#"id="elem13-"#),
        "single sub-shape must not produce suffixed ids: {svg}",
    );
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
