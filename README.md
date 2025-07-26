# RSP (Raw String Peeler)

A Rust CLI tool that converts escaped strings embedded in YAML ConfigMaps into properly formatted multi-line strings using YAML's pipe (`|`) syntax.

## Installation

```bash
cargo build --release
```

## Usage

```bash
# Process a YAML file and output to stdout
rsp peel configmap.yaml

# Process a YAML file and save to a file
rsp peel configmap.yaml -o output.yaml
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

## License

MIT