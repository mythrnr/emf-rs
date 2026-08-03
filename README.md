# emf-rs

A Rust library for parsing [EMF (Enhanced Metafile)](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/91c257d7-c39d-4a36-9b1f-63e3f73d30ca) binaries and converting them to SVG.

> **Note:** This project is a work in progress. Some EMF records are not yet fully implemented.

## Features

- Parses EMF binary format according to the MS-EMF specification
- Converts EMF records to SVG output
- Automatic WMF fallback: delegates to [wmf-rs](https://github.com/mythrnr/wmf-rs) when the input is a WMF file
- `no_std` compatible (uses `alloc`)
- Works in WebAssembly environments via `emf-wasm` (release builds run
  through `wasm-opt -Oz`)
- Extensible conversion via the `Player` trait

## Installation

```sh
cargo add emf-core
```

[`wmf-core`](https://crates.io/crates/wmf-core) is re-exported as
`emf_core::wmf_core` for the WMF fallback path, so a direct dependency
on it is not needed.

### Feature Flags

| Feature | Default | Description |
| --- | --- | --- |
| `svg` | Yes | Enables SVG conversion (`SVGPlayer`) |
| `tracing` | Yes | Enables log output via the `tracing` crate |

To use with minimal dependencies:

```sh
cargo add emf-core --no-default-features
```

## Usage

### As a Rust Library

```rust
use std::fs;

fn main() {
    let emf_data = fs::read("input.emf").expect("failed to read file");

    match emf_core::converter::convert_to_svg(emf_data.as_slice()) {
        Ok(svg_bytes) => {
            let svg = String::from_utf8_lossy(&svg_bytes);
            println!("{svg}");
        }
        Err(err) => {
            eprintln!("conversion failed: {err}");
        }
    }
}
```

When the input turns out to be a WMF file rather than EMF, conversion
falls back to the SVG player of `wmf-core` automatically.

### Custom Player

The conversion process is abstracted through the `Player` trait.
You can implement your own `Player` to produce output formats other than
SVG, and pass the implementation to `emf_core::converter::convert`
together with a WMF player for the fallback path:

```rust
use emf_core::converter::{Player, PlayError};
use emf_core::parser::*;

struct MyPlayer { /* ... */ }

impl Player for MyPlayer {
    fn generate(self) -> Result<Vec<u8>, PlayError> {
        // Produce your output format here
        todo!()
    }

    // Implement all required record handler methods...
    // See `emf_core::converter::Player` for the full list.
    # fn bit_blt(self, _: usize, _: EMR_BITBLT) -> Result<Self, PlayError> { Ok(self) }
    // ...
}
```

### As a CLI Tool

The `emf-cli` crate provides a command-line converter:

```sh
cargo run --package emf-cli -- --input sample.emf --output out.svg
```

```text
Usage: emf-cli [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>    The EMF file path to convert to SVG
  -o, --output <OUTPUT>  The destination file path to save converted SVG [default: output.svg]
  -q, --quiet            Omit logs except error log
      --verbose          Print debug logs
  -h, --help             Print help
  -V, --version          Print version
```

### As WASM in the Browser

The `emf-wasm` crate provides WebAssembly bindings built with `wasm-pack`.

```html
<script type="module">
import init, { convertEmf2Svg, setLogLevel } from "./emf_wasm.js";

async function run() {
  await init();
  setLogLevel("info");

  document.getElementById("input").addEventListener("change", () => {
    const input = document.getElementById("input");
    const files = input.files;

    if (files === null || files.length === 0) {
      return;
    }

    const fileReader = new FileReader();

    fileReader.onload = function (e) {
      const bytes = new Uint8Array(e.target.result);
      const svg = convertEmf2Svg(bytes);

      document.getElementById("output").innerHTML = svg;
    };

    fileReader.readAsArrayBuffer(files[0]);
  });
}

run();
</script>
```

To build and run the WASM demo locally:

```sh
make serve
# Open http://localhost:8080
```

Pre-built artifacts (`emf_wasm_bg.wasm`, `emf_wasm.js`, `emf_wasm.d.ts`) are
attached to each release on the
[GitHub Releases](https://github.com/mythrnr/emf-rs/releases) page in two
variants:

- `emf-wasm-<version>.tar.gz` — full build with `tracing` enabled; pair with
  `setLogLevel` for browser-console logging.
- `emf-wasm-minimal-<version>.tar.gz` — built without the `tracing` feature.
  `setLogLevel` becomes a no-op, but the bundle is noticeably smaller because
  the `tracing-wasm` dependency is dropped entirely.

#### WASM API

- `convertEmf2Svg(buf: Uint8Array): string` - Converts EMF binary data to an SVG string. Falls back to WMF parsing automatically when the input is a WMF file.
- `setLogLevel(level: "trace" | "debug" | "info" | "warn" | "error")` - Sets the log level (default: `info`).
  - **Note:** `trace` and `debug` levels are very slow to execute.
  - If the `tracing` feature is disabled, `setLogLevel` has no effect.

## Crate Overview

| Crate | Description |
| --- | --- |
| `emf-core` | Core library: EMF parser and SVG converter (`no_std`) |
| `emf-cli` | CLI tool for EMF to SVG conversion |
| `emf-wasm` | WASM bindings for browser usage (`no_std`) |

## Requirements (for Development)

- Rust 1.88.0+
- Rust nightly toolchain (for `rustfmt` and `cargo-udeps`)
- Docker (for spell-check)
- [wasm-pack](https://github.com/rustwasm/wasm-pack) (for WASM builds)
- Yarn 1.22.22+ (to run the WASM demo)

Optional tools can be installed with:

```sh
make install-tools
```

## Releasing

Direct pushes to `master` are forbidden, so a release is driven by a
version-bump PR:

1. Bump the version:

   ```sh
   make release version=<x.y.z>
   ```

   This creates a `release/<x.y.z>` branch from `master`, updates
   `[workspace.package].version` and dependent version requirements via
   `cargo release version`, then refreshes `Cargo.lock`. Nothing is
   committed, tagged, or pushed.

2. Commit the result and open a PR. Merging it to `master` is the release
   trigger.

3. On the merge, `tag-release.yaml` creates the matching `<version>` git
   tag and invokes the release workflow, which verifies that the version
   equals the workspace version, publishes the WASM bundles as GitHub
   Releases assets, and then publishes `emf-core` to crates.io through
   Trusted Publishing.

To re-run a release whose tag already exists, dispatch the "Release"
workflow manually from the Actions tab with the version as input.

## License

This project is licensed under the [MIT License](LICENSE).

Portions of the API documentation are adapted from the
[MS-EMF](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/91c257d7-c39d-4a36-9b1f-63e3f73d30ca)
and
[MS-EMFPLUS](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emfplus/5f92c789-64f2-46b5-9ed4-15a9bb0946c6)
Open Specifications documentation, © Microsoft Corporation, and are used under
the Intellectual Property Rights Notice for Open Specifications Documentation.
The MS-EMF and MS-EMFPLUS specifications are covered by the
[Microsoft Open Specification Promise](https://go.microsoft.com/fwlink/?LinkId=214445).
