from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "src"))

from ccml_py import CcmlFfiError, diagnose, to_json


def test_to_json_success() -> None:
    out = to_json('name: "Alice"\nage: 30')
    assert out == '{"age":30,"name":"Alice"}'


def test_to_json_error() -> None:
    try:
        to_json("name \"Alice\"")
        raise AssertionError("expected CcmlFfiError")
    except CcmlFfiError as exc:
        assert exc.status == 3  # ParseError
        assert str(exc)


def test_duplicate_warning_propagation() -> None:
    diags = diagnose("x: 1\nx: 2")
    assert any(d.get("code") == "CCML2001" and d.get("severity") == "warning" for d in diags)


if __name__ == "__main__":
    test_to_json_success()
    test_to_json_error()
    test_duplicate_warning_propagation()
    print("python smoke: ok")
