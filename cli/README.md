# emf-cli

A command-line tool for converting
[EMF (Enhanced Metafile)](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/91c257d7-c39d-4a36-9b1f-63e3f73d30ca)
binaries to SVG, built on
[`emf-core`](https://github.com/mythrnr/emf-rs/tree/master/core).

> **Note:** This project is a work in progress. Some EMF records are not yet
> fully implemented.

## Installation

Install from the repository:

```sh
cargo install --git https://github.com/mythrnr/emf-rs emf-cli
```

## Usage

```sh
emf-cli --input sample.emf --output out.svg
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

## License

This project is licensed under the
[MIT License](https://github.com/mythrnr/emf-rs/blob/master/LICENSE).
