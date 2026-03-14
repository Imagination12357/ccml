"""
CCML 파서 테스트
실행: pytest test_ccml.py
"""

import json
import pytest
from ccml import parse, to_json


# ---------------------------------------------------------------------------
# 헬퍼
# ---------------------------------------------------------------------------

def j(text: str):
    """to_json() → json.loads() 로 Python 객체 반환"""
    return json.loads(to_json(text))

def jraw(text: str):
    """JSON 문자열 자체 반환 (숫자 리터럴 원본 보존 확인용)"""
    return to_json(text)


# ---------------------------------------------------------------------------
# 기본 object
# ---------------------------------------------------------------------------

def test_implicit_root_object():
    assert j('name: "Alice"\nage: 30') == {"name": "Alice", "age": 30}

def test_explicit_object():
    assert j('{ foo: "bar" baz: true }') == {"foo": "bar", "baz": True}

def test_comma_separator():
    assert j('x: 1, y: 2') == {"x": 1, "y": 2}

def test_nested_object():
    assert j('person: {\n    name: "Bob"\n    age: 25\n}') == {
        "person": {"name": "Bob", "age": 25}
    }

# ---------------------------------------------------------------------------
# array
# ---------------------------------------------------------------------------

def test_string_array():
    assert j('["a" "b" "c"]') == ["a", "b", "c"]

def test_root_number_array():
    assert j('[1 2 3]') == [1, 2, 3]

def test_nested_array():
    assert j('tags: ["web" "api"]') == {"tags": ["web", "api"]}

def test_empty_array():
    assert j('val: []') == {"val": []}

def test_nested_nested_array():
    assert j('[[1 2] [3 4]]') == [[1, 2], [3, 4]]

# ---------------------------------------------------------------------------
# 키
# ---------------------------------------------------------------------------

def test_bare_key_number():
    assert j('123: "num"') == {"123": "num"}

def test_bare_key_reserved_word():
    assert j('true: "val"') == {"true": "val"}

def test_quoted_key_with_colon():
    assert j('"key:with:colon": "ok"') == {"key:with:colon": "ok"}

def test_quoted_key_with_spaces():
    assert j('"key with spaces": true') == {"key with spaces": True}

# ---------------------------------------------------------------------------
# 값
# ---------------------------------------------------------------------------

def test_null_and_false():
    assert j('a: null\nb: false') == {"a": None, "b": False}

def test_true():
    assert j('flag: true') == {"flag": True}

# ---------------------------------------------------------------------------
# 숫자 리터럴 원본 보존
# ---------------------------------------------------------------------------

def test_integer_preserved():
    assert jraw('n: 42') == '{"n": 42}'

def test_float_preserved():
    assert jraw('pi: 3.14') == '{"pi": 3.14}'

def test_exponent_preserved():
    assert jraw('big: 1e10') == '{"big": 1e10}'

def test_negative_preserved():
    assert jraw('val: -0.5') == '{"val": -0.5}'

# ---------------------------------------------------------------------------
# 주석
# ---------------------------------------------------------------------------

def test_line_comment():
    assert j('# 주석\nname: "Carol"') == {"name": "Carol"}

def test_inline_comment():
    assert j('name: "Carol"  # 인라인') == {"name": "Carol"}

# ---------------------------------------------------------------------------
# 빈 입력
# ---------------------------------------------------------------------------

def test_empty_object():
    assert j('{}') == {}

def test_empty_file():
    assert j('') == {}

# ---------------------------------------------------------------------------
# 복합
# ---------------------------------------------------------------------------

def test_complex_nested():
    src = """
# 서버 설정
server: {
    host: "localhost"
    port: 8080
    tags: ["web" "api"]
}
debug: false
"""
    assert j(src) == {
        "server": {
            "host": "localhost",
            "port": 8080,
            "tags": ["web", "api"],
        },
        "debug": False,
    }