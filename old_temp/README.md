# CCML

**Convenient Compatible Markup Language** — a human-friendly superset of JSON.

CCML is designed to be easy to write by hand while remaining fully convertible to JSON. If you already know JSON, you already know most of CCML.

## Differences from JSON

| Feature | JSON | CCML |
|---|---|---|
| Separator | `,` (required) | whitespace (`,` optional) |
| Object keys | must be quoted | quotes optional |
| Root `{}` / `[]` | required | `{}` can be omitted |
| Comments | ✗ | `#` line comments |

## Examples

```ccml
# Server configuration
server: {
    host: "localhost"
    port: 8080
    tags: ["web" "api"]
}
debug: false
```

Converts to:

```json
{
  "server": {
    "host": "localhost",
    "port": 8080,
    "tags": ["web", "api"]
  },
  "debug": false
}
```

### Implicit root object

The top-level `{}` can be omitted — CCML treats a bare key-value file as an object by default:

```ccml
name: "Alice"
age: 30
```

### Commas are still allowed

```ccml
x: 1, y: 2, z: 3
```

### Keys with special characters use quotes

```ccml
"key:with:colon": "ok"
"key with spaces": true
```

## Installation

```bash
pip install ccml
```

## Usage

### As a library

```python
import ccml

# Parse to Python object
obj = ccml.parse('name: "Alice"\nage: 30')

# Convert to JSON string
json_str = ccml.to_json('name: "Alice"\nage: 30', indent=2)
```

### As a CLI

```bash
# From file
ccml config.ccml

# From stdin
cat config.ccml | ccml
```

## License

Apache 2.0