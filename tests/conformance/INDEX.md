# CCML Conformance Vector Index

## Summary
- Total cases: 64
- valid: 36
- invalid: 13
- warn: 15

## Schema
- Case schema: `tests/conformance/_schema/ccml-conformance-case.schema.json`
- Spec reference: `spec/conformance-test-vector-schema.md`

## Coverage Classes
1. Bare key rules
- leading digit/dot/minus mixed forms
- punctuation-only rejection (`.` / `--`)
- keyword-like and numeric-looking keys

2. Separators and layout
- whitespace separators
- comma separators
- mixed separators
- implicit and explicit root object

3. Comments
- line comments
- inline comments
- comment interaction with parsing failures

4. Nested structures
- nested object + array
- root array of objects
- empty object/array

5. Value lexemes
- booleans, null
- negative and exponent numbers
- escaped strings
- unicode escape decoding
- uppercase exponent and negative-zero numeric forms

6. Diagnostics
- missing colon
- missing value
- unclosed delimiters
- trailing token
- unterminated string
- invalid number format
- newline boundary failures after comments/pairs

7. Duplicate key policy
- default behavior: keep-last
- warning required: `CCML2001`
- equivalence across quoted/bare key forms
- replacement across scalar/array/object/null->boolean transitions
- mixed stress with comments and escaped string values

## Notes
- Current invalid-case line/column positions are draft expectations for parser bring-up and may be adjusted when Rust diagnostics stabilize.
