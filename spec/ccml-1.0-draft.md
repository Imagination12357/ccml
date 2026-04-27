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
object_body = { pair } ;
object      = "{" , { pair } , "}" ;
array       = "[" , [ value , { sep , value } ] , "]" ;
pair        = key , ":" , value , [ sep ] ;
key         = bare_key | escaped_string ;
value       = escaped_string | number | "true" | "false" | "null" | object | array ;
sep         = whitespace_or_comma ;
```

## 5. Lexical Rules
- `escaped_string`: JSON string escaping 규칙을 따른다.
- `number`: JSON number lexical form을 따른다.
- `bare_key`: 공백/구조 문자를 제외한 키 토큰.
- `comment`: `#`로 시작해 줄 끝까지.
- whitespace는 토큰 구분자로 허용한다.

## 6. Semantic Rules
1. Top-level object braces are optional when document is a sequence of key-value pairs.
2. Commas are optional separators; whitespace-only separation is valid.
3. Quoted keys MUST be used when key includes spaces or reserved separators (`:`, `{`, `}`, `[`, `]`, `,`, `#`).
4. Duplicate object keys are currently undefined behavior in draft and MUST be finalized before RC.

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

## 8. Conformance Requirements
- A conforming parser MUST accept all valid cases in `tests/conformance/valid`.
- A conforming parser MUST reject invalid cases in `tests/conformance/invalid` with positional diagnostics.
- A conforming transcoder MUST produce valid JSON text for all valid CCML inputs.

## 9. Non-Goals for 1.0
- include/import directives
- schema/type extension system
- comments preservation in output JSON

## 10. Open Decisions Before RC
1. Duplicate key policy:
   - keep-last
   - keep-first
   - reject as error
2. Strict vs permissive bare key character class
3. CLI output mode defaults (compact vs pretty)
