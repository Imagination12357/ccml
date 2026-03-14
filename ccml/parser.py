"""
CCML (Convenient Compatible Markup Language) → JSON parser
"""

import json
import sys
import uuid
from lark import Lark, Transformer, v_args


# ---------------------------------------------------------------------------
# 숫자 원본 보존
# ---------------------------------------------------------------------------

class RawNumber:
    """유저가 입력한 숫자 리터럴을 손대지 않고 그대로 보존."""
    def __init__(self, s: str):
        self.s = s


def _dumps_preserving_numbers(obj, **kwargs) -> str:
    placeholders = {}

    def _replace(o):
        if isinstance(o, RawNumber):
            key = f"__RN_{uuid.uuid4().hex}__"
            placeholders[key] = o.s
            return key
        if isinstance(o, dict):
            return {k: _replace(v) for k, v in o.items()}
        if isinstance(o, list):
            return [_replace(v) for v in o]
        return o

    replaced = _replace(obj)
    raw_json = json.dumps(replaced, ensure_ascii=False, **kwargs)

    for key, num_str in placeholders.items():
        raw_json = raw_json.replace(f'"{key}"', num_str)

    return raw_json


# ---------------------------------------------------------------------------
# 문법 정의
# ---------------------------------------------------------------------------

CCML_GRAMMAR = r"""
    start       : array
                | object
                | object_body

    object_body : pair*
    object      : "{" pair* "}"
    array       : "[" (value (_SEP value)*)? "]"

    pair        : key ":" value _SEP?

    key         : BARE_KEY
                | ESCAPED_STRING

    value       : ESCAPED_STRING
                | NUMBER
                | TRUE
                | FALSE
                | NULL
                | object
                | array

    ESCAPED_STRING : "\"" (/[^"\\]/ | /\\./)* "\""
    BARE_KEY       : /[^\s:"{}\[\]#,]+/
    NUMBER         : /-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?/
    TRUE           : "true"
    FALSE          : "false"
    NULL           : "null"

    _SEP           : /[\s,]+/

    COMMENT        : /#[^\n]*/
    %ignore COMMENT
    %ignore /\s+/
"""

# ---------------------------------------------------------------------------
# Transformer
# ---------------------------------------------------------------------------

@v_args(inline=True)
class CCMLTransformer(Transformer):

    def start(self, child):
        return child

    def object_body(self, *pairs):
        return {k: v for k, v in pairs}

    def object(self, *pairs):
        return {k: v for k, v in pairs}

    def array(self, *values):
        return list(values)

    def pair(self, key, value):
        return (key, value)

    def key(self, token):
        s = str(token)
        return json.loads(s) if s.startswith('"') else s

    def value(self, child):
        return child

    def ESCAPED_STRING(self, token):
        return json.loads(str(token))

    def NUMBER(self, token):
        return RawNumber(str(token))

    def TRUE(self, _):  return True
    def FALSE(self, _): return False
    def NULL(self, _):  return None
    def BARE_KEY(self, token): return str(token)


# ---------------------------------------------------------------------------
# 파서 인스턴스
# ---------------------------------------------------------------------------

_parser = Lark(CCML_GRAMMAR, parser="earley", start="start")

# ---------------------------------------------------------------------------
# 내부 API
# ---------------------------------------------------------------------------

def parse(text: str):
    """CCML 텍스트 → Python 객체 (숫자는 RawNumber로 보존)"""
    return CCMLTransformer().transform(_parser.parse(text))


def to_json(text: str, indent=None) -> str:
    """CCML 텍스트 → JSON 문자열"""
    return _dumps_preserving_numbers(parse(text), indent=indent)


# ---------------------------------------------------------------------------
# CLI 진입점
# ---------------------------------------------------------------------------

def main():
    if len(sys.argv) == 2:
        with open(sys.argv[1], encoding="utf-8") as f:
            src = f.read()
    else:
        src = sys.stdin.read()
    print(to_json(src, indent=2))