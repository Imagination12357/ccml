# CCML 1.0 Draft Specification

Status: Draft  
Version: 1.0.0-draft.1

## 1. Purpose
CCML is a human-friendly superset of JSON syntax that must remain fully representable as JSON data.

## 2. Compatibility Principle
- Any JSON document MUST be representable in CCML.
- Any CCML document MUST map to one valid JSON value.
- Features that cannot map cleanly to JSON are out of scope for 1.0.

## 3. Data Model
CCML maps to the JSON data model:
- object
- array
- string
- number
- boolean
- null

## 4. Syntax Summary (Draft EBNF)
```ebnf
start       = array | object | object_body ;
object_body = [ pair , { sep , pair } , [ "," ] ] ;
object      = "{" , object_body , "}" ;
array       = "[" , [ value , { sep , value } , [ "," ] ] , "]" ;
pair        = key , ":" , value ;
key         = bare_key | escaped_string ;
value       = escaped_string | number | "true" | "false" | "null" | object | array ;
sep         = layout_nonempty | layout , "," , layout ;
layout_nonempty = ( whitespace | comment ) , layout ;
layout      = { whitespace | comment } ;
```

## 5. Lexical Rules
- `escaped_string`: follows JSON string escaping rules.
  - Unescaped control characters `U+0000` through `U+001F` are invalid.
  - UTF-16 surrogate escapes MUST form a valid high-surrogate + low-surrogate pair.
- `number`: follows JSON number lexical form.
- `comment`: starts with `#` and continues to end of line.
- `whitespace`: allowed as token separator.
- `bare_key` (accepted class):
  - MUST match: `[A-Za-z0-9_.-]+`
  - MUST NOT match: `[.-]+` (to prevent punctuation-only typo-like keys)
  - MUST NOT contain reserved separators: whitespace, `:`, `{`, `}`, `[`, `]`, `,`, `#`, `"`
  - keys outside this class MUST be quoted (`escaped_string`)

## 6. Semantic Rules
1. Top-level object braces are optional when document is a sequence of key-value pairs.
2. Whitespace and comments are the primary separators between adjacent array values or object pairs.
3. A single comma MAY appear within a separator region, with optional whitespace/comments around it.
4. A separator region MUST NOT contain more than one comma, even when whitespace/comments occur between the commas.
5. After the final array value or object pair, the trailing separator region MAY contain whitespace/comments and at most one comma.
6. After the root value, only whitespace and comments are allowed before end of input.
7. Quoted keys MUST be used when key includes spaces or reserved separators (`:`, `{`, `}`, `[`, `]`, `,`, `#`).
8. A key in object context always maps to a JSON string key.
9. Bare keys are case-sensitive and preserve source text exactly (no unescaping).
10. Tokens `true`, `false`, `null`, or numeric-looking tokens used as keys are treated as string keys (for example, `true: 1` means key `"true"`).
11. If a bare key token `k` is valid by lexical rules, `k: v` and `"k": v` are semantically equivalent.
12. Duplicate object keys follow `keep-last` semantics in parse result and transcoded JSON output.
13. Implementations MUST emit a non-fatal warning diagnostic for every duplicate-key overwrite (`keep-last + warn`).
14. Tooling MAY provide a strict mode that upgrades duplicate-key warnings to errors.

## 7. Error Model (Draft)
Every parser error MUST include:
- error_code
- message
- line
- column

Draft error code set:
- `CCML1001` Unexpected token
- `CCML1002` Unterminated string
- `CCML1003` Invalid number format
- `CCML1004` Missing colon in pair
- `CCML1005` Mismatched closing delimiter
- `CCML1006` Invalid key token

Warning code set:
- `CCML2001` Duplicate key overwritten by keep-last policy

## 8. Conformance Requirements
- A conforming parser MUST accept all valid cases in `tests/conformance/valid`.
- A conforming parser MUST reject invalid cases in `tests/conformance/invalid` with positional diagnostics.
- A conforming transcoder MUST produce valid JSON text for all valid CCML inputs.
- Test vector file format MUST follow `tests/conformance/_schema/ccml-conformance-case.schema.json`.

### 8.1 Number Preservation Policy
- JSON transcode mode MUST preserve numeric raw lexeme text from source where valid (for example, `1e10` remains `1e10`).
- Native-object mode MAY expose parsed numeric helpers, but MUST preserve the original raw numeric lexeme alongside any derived numeric representation.

## 9. Non-Goals for 1.0
- include/import directives
- schema/type extension system
- comments preservation in output JSON

## 10. Accepted Decisions
1. Duplicate key policy: `keep-last + warn`.
2. Bare key class: `[A-Za-z0-9_.-]+` with punctuation-only keys (`[.-]+`) disallowed.

## 11. Remaining Open Decision Before RC
1. CLI output mode defaults (compact vs pretty)

## 12. Decision Analysis by 3C

3C = Convenient for humans / Compatible with JSON / Clear to parsers

### 12.1 Duplicate key policy
- `keep-last`
  - Convenient: high (common user expectation in config formats)
  - Compatible: high (matches behavior of many JSON parsers/object loaders)
  - Clear: medium (needs explicit spec text)
- `keep-first`
  - Convenient: low (surprising override behavior)
  - Compatible: medium (less common expectation)
  - Clear: medium
- `reject as error`
  - Convenient: medium-low
  - Compatible: low for existing JSON corpora that rely on duplicate keys
  - Clear: high

Accepted: `keep-last + warn`, with optional strict mode escalation.

### 12.2 Bare key character class
- permissive class
  - Convenient: high
  - Compatible: medium
  - Clear: low (tokenization edge cases increase)
- strict class
  - Convenient: medium
  - Compatible: high (quoted key escape hatch always available)
  - Clear: high

Accepted: `[A-Za-z0-9_.-]+` with punctuation-only keys disallowed (`[.-]+`).

### 12.3 CLI default output
- `pretty` default
  - Convenient: high for humans
  - Compatible: high (still valid JSON)
  - Clear: high (readable diagnostics and diffs)
- `compact` default
  - Convenient: medium
  - Compatible: high
  - Clear: medium

Draft recommendation: default `pretty`, optional `--compact`.
