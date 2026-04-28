# AGENTS.md

A Rust library for parsing EMF (Enhanced Metafile) binaries and converting them to SVG.
Conforms to the [MS-EMF specification](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/91c257d7-c39d-4a36-9b1f-63e3f73d30ca).

## Project Structure

The project is managed as a Cargo workspace with 3 crates.

```
emf-rs/
  Cargo.toml          # Workspace root (resolver = "2")
  core/               # emf-core: EMF parsing & SVG conversion library (no_std)
  cli/                # emf-cli: CLI tool (example usage of emf-core)
  wasm/               # emf-wasm: WASM bindings (no_std, wasm-bindgen)
  docker/             # Development Docker environment
  wasm/dist/          # wasm-pack build artifacts
```

### emf-core (Main Library)

- `#![no_std]` compatible. Uses the `alloc` crate.
- Re-exports the `embedded_io::Read` trait via `pub use` for I/O abstraction.
- Depends on wmf-core (v0.0.20, git dependency) for shared types (`RectL`, `PointL`, `ColorRef`, `Bitmap`, etc.) and WMF fallback conversion.
- Feature flags:
  - `svg` (enabled by default): SVG conversion (`SVGPlayer`)
  - `tracing` (enabled by default): Log output (replaced with no-op macros when disabled)

#### parser Module (`core/src/parser/`)

Handles binary parsing based on the MS-EMF specification.

- `enums/` — EMF enumeration constants (Section 2.1). 37 files. `RecordType`, `PenStyle`, `MapMode`, `GraphicsMode`, etc.
  - Defined with `#[repr(u32)]` or similar, bidirectional conversion via `strum::FromRepr`.
  - Automatic parsing implementation via the `impl_parser!()` macro.
- `objects/` — EMF object definitions (Section 2.2). 19 files. `Header`, `LogFont*`, `LogPen*`, `LogBrushEx`, `LogPalette`, `XForm`, `EmrText`, `RegionData`, etc.
- `records/` — EMF record types (Section 2.3). 12 categories, 100+ records:
  - `bitmap/` (9): `AlphaBlend`, `BitBlt`, `MaskBlt`, `PlgBlt`, `SetDIBitsToDevice`, `StretchBlt`, `StretchDIBits`, `TransparentBlt`
  - `clipping/` (6): `ExcludeClipRect`, `ExtSelectClipRgn`, `IntersectClipRect`, `OffsetClipRgn`, `SelectClipPath`, `SetMetaRgn`
  - `comment/`: `EMR_COMMENT`
  - `control/` (2): `EMR_HEADER`, `EMR_EOF`
  - `drawing/` (30+): `Arc`, `ArcTo`, `AngleArc`, `Chord`, `Ellipse`, `ExtTextOutA/W`, `FillPath`, `GradientFill`, `LineTo`, `Pie`, `PolyBezier*`, `PolyDraw*`, `Polygon*`, `Polyline*`, `Rectangle`, `RoundRect`, `StrokePath`, etc.
  - `escape/` (3): `DrawEscape`, `ExtEscape`, `NamedEscape`
  - `object_creation/` (9): `CreateBrushIndirect`, `CreateColorSpace*`, `CreateDIBPatternBrushPt`, `CreateMonoBrush`, `CreatePalette`, `CreatePen`, `ExtCreateFontIndirectW`, `ExtCreatePen`
  - `object_manipulation/` (8): `DeleteObject`, `SelectObject`, `SelectPalette`, `SetPaletteEntries`, etc.
  - `open_gl/` (2): `GlsBoundedRecord`, `GlsRecord`
  - `path_bracket/` (6): `AbortPath`, `BeginPath`, `CloseFigure`, `EndPath`, `FlattenPath`, `WidenPath`
  - `state/` (34): `MoveToEx`, `SaveDC`, `RestoreDC`, `SetBkColor`, `SetMapMode`, `SetTextAlign`, `SetWindowExtEx`, etc.
  - `transform/` (2): `ModifyWorldTransform`, `SetWorldTransform`
- `primitive/` — Primitive types: `Size` (byte count tracking), `Gamma`, `Adjustment`, etc.
- Key types: `ParseError`, `ReadError`
- Each record type has a `::parse(buf, record_type, size) -> Result<Self, ParseError>` static method.

#### converter Module (`core/src/converter/`)

Converts parsed records into an output format.

- `player.rs` — `Player` trait: defines 100+ methods for processing each EMF record. `generate(self) -> Result<Vec<u8>, PlayError>` produces the final output.
- `mod.rs` — `EMFConverter<B, P, WP>`: accepts a buffer (`embedded_io::Read`), an EMF player (`Player`), and a WMF player (`wmf_core::converter::Player`), then runs the conversion. Automatically falls back to the WMF player if the input is a WMF file.
- `playback_device_context.rs` — `PlaybackDeviceContext`: manages the graphics environment and transformation matrix. `EmfObjectTable` (object storage/retrieval), `SelectedObject` (currently selected brush/font/pen, etc.), `GraphicsObject` (enum of all creatable object types).
- `svg/` — SVG conversion implementation (when the `svg` feature is enabled):
  - `mod.rs` — `SVGPlayer` struct. Implements the `Player` trait.
  - `svg_player.rs` — SVG generation logic for each record.
  - `node.rs` — SVG DOM node builder.
  - `util.rs` — Utilities for color conversion, coordinate transforms, etc.
- Error types: `ConvertError` (`ParseError`, `PlayError`, `WMFConvertError`, `IoError`), `PlayError` (`FailedGenerate`, `InvalidBrush`, `InvalidRecord`, `UnexpectedGraphicsObject`, `Unknown`)

### emf-cli

- Single binary consisting of `cli/src/main.rs` only.
- Argument parsing with `clap`:
  - `-i, --input`: EMF file path to convert
  - `-o, --output`: Destination SVG path (default: `output.svg`)
  - `-q, --quiet`: Suppress logs except errors
  - `--verbose`: Print debug logs
- Log control via `tracing-subscriber` with `EnvFilter`.

### emf-wasm

- Consists of `wasm/src/lib.rs` only.
- `#![no_std]`, `crate-type = ["cdylib"]`
- Functions exported via `#[wasm_bindgen]`:
  - `convertEmf2Svg(buf: &[u8]) -> Result<String, JsValue>` — EMF to SVG conversion
  - `setLogLevel(level: &str)` — Set log level (functional only when the `tracing` feature is enabled)
- Uses `console_error_panic_hook` for better panic messages in the browser.
- Pre-built artifacts in `wasm/dist/`

## Development Environment

### Required Tools

- Rust 1.88.0 (pinned via `rust-toolchain.toml`)
- Rust nightly (for rustfmt and cargo-udeps)
- Docker (for spell-check)

### Optional Tools

- `cargo-machete`, `cargo-udeps` (unused dependency detection)
- `wasm-pack` (WASM build)
- Yarn 1.22.22+ (running WASM demo)

Bulk install of tools:

```sh
make install-tools
```

## Build, Test & Quality Checks

### Key Make Targets

| Command | Description |
| --- | --- |
| `make check` | `cargo check --workspace --all-targets --all-features` |
| `make test` | `cargo test --workspace --all-targets` |
| `make fmt` | `cargo +nightly fmt --all` |
| `make lint` | `cargo clippy --workspace --all-targets --all-features -- --no-deps -D warnings` |
| `make udeps` | `cargo machete` && `cargo +nightly udeps --all-targets` |
| `make spell-check` | Run cSpell via Docker |
| `make ci-suite` | Run all of the above: `spell-check fix fmt lint udeps test` |
| `make wasm` | `wasm-pack build --out-dir dist --target web` |
| `make serve` | Start WASM demo at `localhost:8080` |

### CI (GitHub Actions)

`.github/workflows/ci.yaml` runs the following on PRs and pushes to master:

1. `cargo +nightly fmt --all` (format check)
2. `cargo clippy` (lint)
3. cSpell (spell check)
4. `cargo test` (unit tests)

## Coding Conventions

### Rust Style

- Edition 2024, MSRV 1.88.0
- Formatted according to `rustfmt.toml` (`cargo +nightly fmt`)
  - Line width: 80 characters (including comments)
  - Imports: grouped by `StdExternalCrate`, merged at `Crate` granularity
  - Uses nightly features (`unstable_features = true`)
- clippy: `all` + `pedantic` enabled at `warn` level
  - Allowed at workspace level: `doc_markdown`, `module_name_repetitions`, `must_use_candidate`
  - Allowed at crate level: `cast_possible_truncation`, `cast_possible_wrap`, `cast_precision_loss`, `cast_sign_loss`, `enum_variant_names`, `missing_errors_doc`, `missing_panics_doc`, `similar_names`, `too_many_lines`, `unreadable_literal`, `upper_case_acronyms`, `used_underscore_binding`, `wildcard_imports`
  - `non_camel_case_types` and `non_snake_case` are allowed (to match specification record names)
- Error definitions: uses the `snafu` crate
- Logging: uses the `tracing` crate; can be disabled via feature flag
- EMF record type names follow the specification in `UPPER_SNAKE_CASE`
- All comments in source files (`.rs`, `.toml`, shell scripts, etc.) MUST
  be written in English. This rule has no exceptions: do not use any
  language other than English, even temporarily during refactors or while
  adding new code, and do not mix languages within the same comment block.
  Inline doc strings, `//` line comments, and `/* */` block comments all
  fall under this rule
- Log messages emitted via `tracing` MUST be in English
- Error messages produced by `snafu` (or any other path) MUST be in English
- Identifier names follow Rust conventions in English (record type names
  follow the MS-EMF specification as noted above)

### EditorConfig

- UTF-8, LF line endings
- Indentation: 4 spaces (Rust), 2 spaces (HTML, JSON, TOML, YAML, Shell), tabs (Makefile)
- Trailing whitespace trimmed (except Markdown)
- Final newline inserted

### Spell Check

- Custom dictionary defined in `.vscode/cspell.json`
- Contains 200+ domain-specific terms from the EMF specification
- When adding new EMF terms, add them to the `words` list in `cspell.json`

## Testing

- Integration tests in `core/tests/svg_player.rs`
  - `test_svgplayer_polyline()`, `test_svgplayer_rectangle()`, `test_svgplayer_ext_text_out_a()`
  - Verifies that SVG output contains expected elements and attributes
- Run tests: `make test` or `cargo test --workspace --all-targets`

## Architecture

### Player Pattern

EMF record processing is abstracted via the `Player` trait.
`EMFConverter` sequentially parses records from a binary stream and calls the corresponding `Player` methods.
To add a new output format, implement the `Player` trait.

`EMFConverter<B, P, WP>` takes 3 type parameters:
- `B`: Buffer (implements `embedded_io::Read`)
- `P`: EMF player (implements `emf_core::converter::Player`)
- `WP`: WMF player (implements `wmf_core::converter::Player`, for WMF fallback)

### WMF Fallback

`EMFConverter::run()` validates the input file signature. If the file is WMF rather than EMF, it automatically delegates processing to the WMF player. This means the caller does not need to determine the file format in advance.

### no_std Support

Both `emf-core` and `emf-wasm` run under `#![no_std]`.
They use the `alloc` crate (`Vec`, `String`, `BTreeMap`, etc.) instead of `std`.
I/O is abstracted via `embedded_io::Read`.

### Binary Parsing

All data is read in little-endian byte order.
Each record type has a `::parse(buf, record_type, size) -> Result<Self, ParseError>` static method that sequentially reads data from the buffer.

### Dependency Graph

```
emf-core (no_std)
├── embedded-io    (I/O trait)
├── paste          (macros)
├── snafu          (error handling)
├── strum          (enum conversions)
├── wmf-core       (WMF shared types & fallback conversion)
└── tracing        (logging, optional)

emf-cli
├── clap               (CLI parsing)
├── tracing            (logging)
├── tracing-subscriber (log formatting)
├── emf-core           (svg, tracing features)
└── wmf-core           (svg, tracing features)

emf-wasm (no_std, cdylib)
├── wasm-bindgen              (WASM bindings)
├── tracing-wasm              (WASM logging, optional)
├── console_error_panic_hook  (panic message improvement, optional)
├── emf-core                  (svg feature)
└── wmf-core                  (svg feature)
```

## Branching & Releases

- Main branch: `master`
- Dependabot: weekly updates for `cargo` and `github-actions` (targeting `master`)
- Release: `make release version=<tag>` creates and pushes a git tag
