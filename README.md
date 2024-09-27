# emf-rs

Library to parse EMF and convert to SVG (WIP).

## crates

- `emf-core`
  - `parser` module ... Parsing according to [MS-EMF](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/91c257d7-c39d-4a36-9b1f-63e3f73d30ca) specifications.
  - `converter` module ... Converting parsed records to SVG.
- `emf-cli` ... An example runner for `emf-core`.

## Requirements

- Rust 1.81.0+ (For Development)
- [wasm-pack](https://github.com/rustwasm/wasm-pack)
- Yarn 1.22.22+ (To run example)

### Optionals

- Docker
- [cargo-machete](https://github.com/bnjbvr/cargo-machete)
- [cargo-udeps](https://github.com/est31/cargo-udeps)

## Installation

```toml
[dependencies]
emf-core = { git = "https://github.com/mythrnr/emf-rs.git", tag = "0.0.1", package = "emf-core" }
```

## Examples

### Run as CLI

More details, see `emf-cli` crate.

```bash
$ cargo run --package emf-cli -- --help
Usage: emf-cli [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>    The EMF file path to convert to SVG
  -o, --output <OUTPUT>  The destination file path to save converted SVG [default: output.svg]
  -q, --quiet            Omit logs except error log
      --verbose          Print debug logs
  -h, --help             Print help
  -V, --version          Print version
```

### Run as WASM on browser

- Run example in http://localhost:8080

```bash
make serve
```

- Enable to set log level by running `setLogLevel(level: "trace" | "debug" | "info" | "warn" | "error")`
  - Default is `info` level.
  - **NOTE: trace and debug levels are very slow to execute.**

```html
<script type="module">
import init, { convertEmf2Svg, setLogLevel } from './emf_wasm.js';

async function run() {
  await init();
  setLogLevel('debug');

  document.getElementById('input').addEventListener('change', () => {
    const input = document.getElementById('input');
    const files = input.files;

    if (files === null || files.length === 0) {
      return;
    }

    const fileReader = new FileReader();

    fileReader.onload = function (e) {
      const bytes = new Uint8Array(e.target.result);
      const output = convertEmf2Svg(bytes);

      document.getElementById('output').innerHTML = output;
    };

    fileReader.readAsArrayBuffer(files[0]);
  });
}

run();
</script>
```
