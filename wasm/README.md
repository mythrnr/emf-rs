# emf-wasm

WebAssembly bindings for converting
[EMF (Enhanced Metafile)](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/91c257d7-c39d-4a36-9b1f-63e3f73d30ca)
binaries to SVG in the browser, built on
[`emf-core`](https://github.com/mythrnr/emf-rs/tree/master/core) with
[`wasm-pack`](https://github.com/rustwasm/wasm-pack).

> **Note:** This project is a work in progress. Some EMF records are not yet
> fully implemented.

## Feature Flags

| Feature                    | Default | Description                                       |
| -------------------------- | ------- | ------------------------------------------------- |
| `console_error_panic_hook` | Yes     | Reports panics via `console.error`                |
| `tracing`                  | Yes     | Enables `setLogLevel` and browser-console logging |

## Installation

Pre-built artifacts (`emf_wasm_bg.wasm`, `emf_wasm.js`, `emf_wasm.d.ts`) are
attached to each release on the
[GitHub Releases](https://github.com/mythrnr/emf-rs/releases) page in two
variants:

- `emf-wasm-<version>.tar.gz` — full build with `tracing` enabled; pair with
  `setLogLevel` for browser-console logging.
- `emf-wasm-minimal-<version>.tar.gz` — built without the `tracing` feature.
  `setLogLevel` becomes a no-op, but the bundle is noticeably smaller because
  the `tracing-wasm` dependency is dropped entirely.

To build from source instead:

```sh
make wasm          # full build -> wasm/dist
make wasm-minimal  # without tracing -> wasm/dist-minimal
```

## Usage

```html
<script type="module">
import init, { convertEmf2Svg, setLogLevel } from "./emf_wasm.js";

async function run() {
  await init();
  setLogLevel("info");

  const bytes = new Uint8Array(await file.arrayBuffer());
  const svg = convertEmf2Svg(bytes);

  document.getElementById("output").innerHTML = svg;
}

run();
</script>
```

To build and run the demo locally:

```sh
make serve
# Open http://localhost:8080
```

## API

- `convertEmf2Svg(buf: Uint8Array): string` - Converts EMF binary data to an
  SVG string. Falls back to WMF parsing automatically when the input is a WMF
  file.
- `setLogLevel(level: "trace" | "debug" | "info" | "warn" | "error")` - Sets
  the log level (default: `info`).
  - **Note:** `trace` and `debug` levels are very slow to execute.
  - If the `tracing` feature is disabled, `setLogLevel` has no effect.

## License

This project is licensed under the
[MIT License](https://github.com/mythrnr/emf-rs/blob/master/LICENSE).
