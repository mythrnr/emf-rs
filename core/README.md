# emf-core

A Rust library for parsing
[EMF (Enhanced Metafile)](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/91c257d7-c39d-4a36-9b1f-63e3f73d30ca)
binaries and converting them to SVG.

> **Note:** This project is a work in progress. Some EMF records are not yet
> fully implemented.

## Features

- Parses the EMF binary format according to the MS-EMF specification
- Parses EMF+ ([MS-EMFPLUS](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emfplus/5f92c789-64f2-46b5-9ed4-15a9bb0946c6))
  records embedded in EMF comment records
- Converts EMF records to SVG output
- Automatic WMF fallback: delegates to
  [`wmf-core`](https://crates.io/crates/wmf-core) when the input is a WMF file
- `no_std` compatible (uses `alloc`)
- Extensible conversion via the `Player` trait

## Installation

```sh
cargo add emf-core
```

[`wmf-core`](https://crates.io/crates/wmf-core) is re-exported as
`emf_core::wmf_core` for the WMF fallback path, so a direct dependency
on it is not needed.

### Feature Flags

| Feature   | Default | Description                                |
| --------- | ------- | ------------------------------------------ |
| `svg`     | Yes     | Enables SVG conversion (`SVGPlayer`)       |
| `tracing` | Yes     | Enables log output via the `tracing` crate |

To use with minimal dependencies:

```sh
cargo add emf-core --no-default-features
```

## Usage

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

## License

This project is licensed under the
[MIT License](https://github.com/mythrnr/emf-rs/blob/master/LICENSE).

Portions of the API documentation are adapted from the
[MS-EMF](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/91c257d7-c39d-4a36-9b1f-63e3f73d30ca)
and
[MS-EMFPLUS](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emfplus/5f92c789-64f2-46b5-9ed4-15a9bb0946c6)
Open Specifications documentation, © Microsoft Corporation, and are used under
the Intellectual Property Rights Notice for Open Specifications Documentation.
The MS-EMF and MS-EMFPLUS specifications are covered by the
[Microsoft Open Specification Promise](https://go.microsoft.com/fwlink/?LinkId=214445).
