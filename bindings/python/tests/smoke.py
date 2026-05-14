from __future__ import annotations

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "src"))

from ccml_py import CcmlFfiError, diagnose, to_json

PARITY_CASES = json.loads(
    (Path(__file__).resolve().parents[3] / "tests" / "bindings" / "parity-smoke-cases.json").read_text(
        encoding="utf-8"
    )
)


def test_to_json_success() -> None:
    case = PARITY_CASES["success_case"]
    out = to_json(case["input"])
    assert out == case["expected_json"]


def test_to_json_error() -> None:
    case = PARITY_CASES["error_case"]
    try:
        to_json(case["input"])
        raise AssertionError("expected CcmlFfiError")
    except CcmlFfiError as exc:
        assert exc.status == case["expected_status"]
        assert str(exc)


def test_duplicate_warning_propagation() -> None:
    case = PARITY_CASES["warn_case"]
    diags = diagnose(case["input"])
    assert any(
        d.get("code") == case["expected_warning_code"]
        and d.get("severity") == case["expected_warning_severity"]
        for d in diags
    )


if __name__ == "__main__":
    test_to_json_success()
    test_to_json_error()
    test_duplicate_warning_propagation()
    print("python smoke: ok")
