# RSP (Raw String Peeler)

A Rust CLI tool that converts escaped strings embedded in YAML ConfigMaps into properly formatted multi-line strings using YAML's pipe (`|`) syntax.

## Installation

### From Source

1. Clone the repository:
```bash
git clone <repository-url>
cd rsp
```

2. Build the project:

```bash
# Install from source
cargo install --path .
```

3. The binary will be available at `target/release/rsp`

### Prerequisites

- Rust 1.70+ (uses Rust 2024 edition)
- Cargo package manager

## Get Started

### Basic Usage

Process a YAML ConfigMap file and output to stdout:
```bash
rsp input.yaml
```

Process a file and save to output file:
```bash
rsp input.yaml -o output.yaml
```

## What it does

RSP transforms hard-to-read escaped strings in Kubernetes ConfigMaps into human-readable format:

**Before:**
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: example-config
data:
  config.json: "{\"hello\":\"test\",\n  \"foo\":\"bar\"\n}"
```

**After:**
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: example-config
data:
  config.json: |
    {"hello":"test",
      "foo":"bar"
    }
```

## Supported file types

RSP automatically processes string values for keys ending with:
- `.yaml` or `.yml`
- `.json`
- `.toml`

## Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test --test peeler_tests     # Core functionality
cargo test --test cli_tests        # CLI integration  
cargo test --test edge_cases_tests # Edge cases and error handling
```

## CI/CD

The project uses GitHub Actions for continuous integration:

- **Build**: Compiles the project
- **Test**: Runs all test suites
- **Lint**: Checks formatting (rustfmt) and code quality (clippy)
- **Audit**: Scans for security vulnerabilities

## License

This project is open source. Please check the repository for license information.

## Contribution

We welcome contributions! Please follow these guidelines:

1. **Testing:** Run the full test suite before submitting changes:
   ```bash
   cargo test --all --verbose
   ```

2. **Code Quality:** Ensure your code passes all checks:
   ```bash
   cargo check
   cargo clippy
   ```

3. **Documentation:** Update documentation for any new features or changes

4. **Commit Messages:** Use clear, descriptive commit messages

5. **Pull Requests:** 
   - Create feature branches from the main branch
   - Include tests for new functionality
   - Ensure all CI checks pass

### Project Structure

- `src/main.rs` - Main CLI entry point
- `src/cli.rs` - Command-line interface
- `src/peeler.rs` - Core YAML processing logic
- `src/error.rs` - Error handling
- `tests/` - Comprehensive test suite
- `specs/README.md` - Detailed specifications

For more detailed development information, see `CLAUDE.md` in the project root.
