# Rust STDF Wafer Parser

A high-performance Rust library for parsing Standard Test Data Format (STDF) files commonly used in semiconductor manufacturing. This library provides efficient parsing of wafer test data with Python bindings via PyO3.

## Features

- **Fast STDF Parsing**: Efficiently parse STDF files with Rust's performance
- **Wafer Map Generation**: Extract and visualize die-level test results
- **Polars Integration**: Convert parsed data to Polars DataFrames for data analysis
- **Python Bindings**: Use from Python via PyO3 for seamless integration
- **Memory Efficient**: Stream-based parsing for large STDF files

## Installation

### From Source (Rust)

```bash
cargo add stdf-wafer-parser
```

### Python Package

```bash
pip install stdf-wafer-parser
```

## Usage

### Rust

```rust
use stdf_wafer_parser::parse_stdf;

let data = parse_stdf("path/to/file.stdf")?;
println!("Parsed {} records", data.len());
```

### Python

```python
import stdf_wafer_parser

# Parse STDF file
df = stdf_wafer_parser.parse_to_dataframe("path/to/file.stdf")
print(df.head())
```

## PyO3 Integration

This library uses PyO3 to provide Python bindings for the Rust implementation. The Python API exposes:

- `parse_to_dataframe()`: Parse STDF files directly to Polars DataFrames
- `WaferData`: Python class for accessing parsed wafer information
- Efficient zero-copy data transfer between Rust and Python

## License

MIT License - See LICENSE file for details
