# CCML Conformance Test Vector Schema

## 1. Purpose
Define one shared test-case format that every implementation (Rust core, Python binding, Node binding, WASM) must consume.

## 2. File Location Convention
- Schema file:
  - `tests/conformance/_schema/ccml-conformance-case.schema.json`
- Case files:
  - `tests/conformance/valid/*.json`
  - `tests/conformance/invalid/*.json`
  - `tests/conformance/warn/*.json`

## 3. Common Fields
Every case file MUST contain:
- `schema_version`:
  - currently `"1.0.0"`
- `id`:
  - stable unique id (for example `valid.basic_object.001`)
- `category`:
  - one of `valid`, `invalid`, `warn`
- `description`:
  - short human-readable explanation
- `input`:
  - raw CCML source text
- `expect`:
  - expected result object

## 4. Expected Result Shapes

### 4.1 Success
Used for `valid` and `warn`.
- `status`: `ok` or `ok_with_warnings`
- `output_json`: expected JSON string output (canonical comparison target)
- `warnings`: optional list (required for `ok_with_warnings`)

Warning entry fields:
- `code` (for example `CCML2001`)
- `severity` (`warning`)
- `message_contains` (substring match target)

### 4.2 Error
Used for `invalid`.
- `status`: `error`
- `error` object with:
  - `code`
  - `line`
  - `column`
  - optional `message_contains`

## 5. Comparison Rules
1. `output_json` comparison:
- parse both actual and expected JSON strings
- compare parsed JSON values for semantic equality

2. Warnings:
- implementation may emit extra non-breaking details
- minimum required warnings in vector MUST be present

3. Errors:
- `code`, `line`, `column` MUST match exactly
- `message_contains` uses substring match

## 6. Naming Rules
- File names SHOULD start with numeric order and short title:
  - `001-basic-object.json`
  - `002-array-whitespace-separator.json`

## 7. Evolution Policy
- Any backward-incompatible schema change MUST bump `schema_version`.
- New optional fields MAY be added without version bump.
