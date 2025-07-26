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

Comprehensive test suite covering:

**Unit Tests (`tests/peeler_tests.rs`):**
- String unescaping functionality (normal and edge cases)
- ConfigMap processing logic
- File extension detection
- YAML serialization with pipe syntax
- File I/O operations

**Integration Tests (`tests/cli_tests.rs`):**
- CLI command execution (help, version, peel)
- File input/output handling
- Error conditions and edge cases
- Command-line argument parsing

**Edge Case Tests (`tests/edge_cases_tests.rs`):**
- Empty and malformed YAML files
- Non-ConfigMap YAML documents
- Large strings and complex escaping
- Unicode content and special characters
- Binary file handling

**Run tests:**
```bash
cargo test                    # All tests
cargo test --test peeler_tests    # Unit tests only
cargo test --test cli_tests       # CLI integration tests
cargo test --test edge_cases_tests # Edge case tests
```

**Sample data:** `tests/test_data/sample_configmap.yaml` contains example input for manual testing.

## Contribution

IMPORTANT: Please check same cases locally as CICD does, before commit any changes such as `cargo test --all --verbose`
