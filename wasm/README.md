# Image to ASCII Art Converter (WASM/CLI)

This directory contains the core Rust logic for the image-to-ASCII-art conversion, which is compiled into both a WebAssembly (WASM) module for the web interface and a standalone command-line interface (CLI) tool.

## Features

-   **Dual-Purpose:** Serves as the engine for both the web application and a native CLI.
-   **High Performance:** Written in Rust and optimized for speed, with parallel processing via Rayon.
-   **Edge Detection:** Uses a Sobel filter to detect edges and represent them with specific characters.
-   **Color and Grayscale:** Preserves the original image's color and maps grayscale values to a character set.
-   **Configurable:** Allows customization of downsampling rate, edge detection threshold, and ASCII character sets.

## CLI Usage

To use the command-line tool, you must have Rust and Cargo installed.

### Building the CLI

Build the optimized release version of the CLI:

```bash
cargo build --release --features=cli
```

The executable will be located at `target/release/image2ascii-cli`.

### Running the CLI

Execute the CLI with the path to your input image.

```bash
./target/release/image2ascii-cli -i /path/to/your/image.jpg
```

### CLI Options

| Flag                     | Argument              | Description                                                              | Default      |
| ------------------------ | --------------------- | ------------------------------------------------------------------------ | ------------ |
| `-i`, `--input`          | `<PATH>`              | Path to the input image file.                                            | (Required)   |
| `-d`, `--downsample-rate`| `<RATE>`              | Downsample rate for the image. Higher values mean smaller output.        | `8`          |
| `-e`, `--edge-sobel-threshold` | `<THRESHOLD>`         | Sobel edge detection threshold.                                          | `50`         |
| `--ascii-chars-edge`     | `<CHARS>`             | ASCII characters for edges, from dark to light.                          | `" -/|\\"`   |
| `--ascii-chars-gray`     | `<CHARS>`             | ASCII characters for grayscale, from dark to light.                      | `"@?OPoc:. "` |

## Benchmarking

The project includes a benchmark suite using `criterion` to measure the performance of the core image processing functions.

To run the benchmarks:

```bash
cargo bench
```

The results, including an HTML report, will be saved in the `target/criterion` directory.
