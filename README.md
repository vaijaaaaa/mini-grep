# minigrep

A small command-line text search tool written in Rust.

This project searches for a query string inside a text file and prints matching lines.

## Features

- Search lines by query text
- Case-sensitive search by default
- Optional case-insensitive search using `IGNORE_CASE`
- Clear argument validation and usage message
- Unit tests for parsing and search behavior

## Requirements

- Rust toolchain (stable)
- Cargo

## Build

```bash
cargo build
```

## Run

```bash
cargo run <query> <file_path>
```

Example:

```bash
cargo run body poem.txt
```

## Case-Insensitive Mode

Enable case-insensitive matching by setting the environment variable `IGNORE_CASE`.

PowerShell:

```powershell
$env:IGNORE_CASE='1'
cargo run "i'm" poem.txt
```

To disable it in the same session:

```powershell
Remove-Item Env:IGNORE_CASE -ErrorAction SilentlyContinue
```

## Usage

If arguments are invalid, the program shows:

```text
Usage: minigrep <query> <file_path>
```

## Test

```bash
cargo test
```

## Project Structure

```text
src/main.rs   # CLI entrypoint and top-level error handling
src/lib.rs    # Config parsing, search logic, and tests
poem.txt      # Sample input file
```
