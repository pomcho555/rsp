# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

RSP (Raw String Peeler) is a Rust CLI tool that converts escaped strings embedded in YAML ConfigMaps into properly formatted multi-line strings using YAML's pipe (`|`) syntax. It transforms hard-to-read escaped JSON/YAML/TOML strings into human-readable format.

## Commands

**Build the project:**
```bash
cargo build
```

**Run the application:**
```bash
cargo run
```

**Run tests:**
```bash
cargo test
```

**Build for release:**
```bash
cargo build --release
```

**Check code without building:**
```bash
cargo check
```

## Architecture

- **Entry point:** `src/main.rs` - Main CLI entry point
- **CLI module:** `src/cli.rs` - Command-line interface using clap
- **Core logic:** `src/peeler.rs` - YAML parsing and string processing
- **Error handling:** `src/error.rs` - Custom error types using thiserror
- **Specification:** `specs/README.md` - Contains detailed requirements and expected behavior
- **Package configuration:** `Cargo.toml` - Uses Rust 2024 edition with dependencies: clap, serde, serde_yaml, anyhow, thiserror

## Key functionality

1. Parses YAML files containing Kubernetes ConfigMaps
2. Detects escaped string values in the `data` section for keys ending with `.yaml`, `.yml`, `.json`, or `.toml`
3. Converts escaped strings to proper YAML multi-line format using pipe (`|`) syntax
4. Outputs formatted YAML to stdout or specified file

## Testing

Test files are located in `tests/test_data/` directory. Use sample ConfigMaps to verify functionality.