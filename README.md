# emf-rs

A Rust library for parsing [EMF (Enhanced Metafile)](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/91c257d7-c39d-4a36-9b1f-63e3f73d30ca) binaries and converting them to SVG.

> **Note:** This project is a work in progress. Some EMF records are not yet fully implemented.

## Features

- Parses EMF binary format according to the MS-EMF specification
- Converts EMF records to SVG output
- Automatic WMF fallback: delegates to [wmf-rs](https://github.com/mythrnr/wmf-rs) when the input is a WMF file
- `no_std` compatible (uses `alloc`)
- Works in WebAssembly environments via `emf-wasm`
- Extensible conversion via the `Player` trait

## Installation

Add `emf-core` to your `Cargo.toml`:

```toml
[dependencies]
emf-core = { git = "https://github.com/mythrnr/emf-rs.git", tag = "0.0.1", package = "emf-core" }
```

Because `emf-core` may delegate WMF inputs to a WMF player, applications that
want to enable the fallback also need `wmf-core`:

```toml
[dependencies]
wmf-core = { git = "https://github.com/mythrnr/wmf-rs.git", tag = "0.1.0", package = "wmf-core" }
```

### Feature Flags

| Feature | Default | Description |
| --- | --- | --- |
| `svg` | Yes | Enables SVG conversion (`SVGPlayer`) |
| `tracing` | Yes | Enables log output via the `tracing` crate |

To use with minimal dependencies:

```toml
[dependencies]
emf-core = { git = "https://github.com/mythrnr/emf-rs.git", tag = "0.0.1", package = "emf-core", default-features = false }
```

## Usage

### As a Rust Library

`EMFConverter` takes both an EMF player and a WMF player. The WMF player is
only invoked when the input file turns out to be WMF rather than EMF, so you
can reuse the SVG player implementations shipped by each crate:

```rust
use std::fs;

fn main() {
    let emf_data = fs::read("input.emf").expect("failed to read file");

    let emf_player = emf_core::converter::SVGPlayer::new();
    let wmf_player = wmf_core::converter::SVGPlayer::new();
    let converter = emf_core::converter::EMFConverter::new(
        emf_data.as_slice(),
        emf_player,
        wmf_player,
    );

    match converter.run() {
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

### Custom Player

The conversion process is abstracted through the `Player` trait.
You can implement your own `Player` to produce output formats other than SVG:

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

```
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

## License

This project is licensed under the [MIT License](LICENSE).
