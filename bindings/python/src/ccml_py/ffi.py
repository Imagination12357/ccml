from __future__ import annotations

import ctypes
import json
import os
import platform
import sys
from pathlib import Path
from typing import Any


STATUS_OK = 0


class CcmlFfiError(RuntimeError):
    def __init__(self, status: int, payload: dict[str, Any] | None) -> None:
        self.status = status
        self.payload = payload or {}
        message = self.payload.get("message", f"ccml ffi call failed (status={status})")
        super().__init__(message)


def _repo_root() -> Path:
    return Path(__file__).resolve().parents[4]


def _platform_tag() -> str:
    system = sys.platform
    if system.startswith("win"):
        os_name = "win32"
    elif system == "darwin":
        os_name = "darwin"
    elif system.startswith("linux"):
        os_name = "linux"
    else:
        os_name = system

    machine = platform.machine().lower()
    arch_aliases = {
        "amd64": "x64",
        "x86_64": "x64",
        "aarch64": "arm64",
        "arm64": "arm64",
    }
    arch = arch_aliases.get(machine, machine)
    return f"{os_name}-{arch}"


def _library_names() -> list[str]:
    if sys.platform.startswith("win"):
        return ["ccml_ffi.dll"]
    if sys.platform == "darwin":
        return ["libccml_ffi.dylib"]
    return ["libccml_ffi.so"]


def _package_library_candidates() -> list[Path]:
    native_dir = Path(__file__).resolve().parent / "native"
    platform_dir = native_dir / _platform_tag()
    return [platform_dir / name for name in _library_names()] + [native_dir / name for name in _library_names()]


def _default_library_candidates() -> list[Path]:
    root = _repo_root()
    target = root / "core" / "rust" / "target"
    source_candidates = [
        target / "debug" / "ccml_ffi.dll",
        target / "release" / "ccml_ffi.dll",
        target / "debug" / "libccml_ffi.so",
        target / "release" / "libccml_ffi.so",
        target / "debug" / "libccml_ffi.dylib",
        target / "release" / "libccml_ffi.dylib",
    ]
    return _package_library_candidates() + source_candidates


def _load_library() -> ctypes.CDLL:
    override = os.getenv("CCML_FFI_LIB")
    if override:
        lib_path = Path(override)
        if not lib_path.exists():
            raise FileNotFoundError(f"CCML_FFI_LIB does not exist: {lib_path}")
        return ctypes.CDLL(str(lib_path))

    for candidate in _default_library_candidates():
        if candidate.exists():
            return ctypes.CDLL(str(candidate))

    looked = "\n".join(str(p) for p in _default_library_candidates())
    raise FileNotFoundError(
        "Could not find ccml-ffi dynamic library. "
        "Install a package with a bundled native library, build it first "
        "(e.g. cargo build -p ccml-ffi), or set CCML_FFI_LIB.\n"
        f"Looked in:\n{looked}"
    )


_LIB = _load_library()

_LIB.ccml_to_json.argtypes = [
    ctypes.POINTER(ctypes.c_ubyte),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_void_p),
    ctypes.POINTER(ctypes.c_void_p),
]
_LIB.ccml_to_json.restype = ctypes.c_int32

_LIB.ccml_diagnose.argtypes = [
    ctypes.POINTER(ctypes.c_ubyte),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_void_p),
    ctypes.POINTER(ctypes.c_void_p),
]
_LIB.ccml_diagnose.restype = ctypes.c_int32

_LIB.ccml_free.argtypes = [ctypes.c_void_p]
_LIB.ccml_free.restype = None


def _bytes_ptr(data: bytes) -> tuple[ctypes.Array[ctypes.c_ubyte], ctypes.POINTER(ctypes.c_ubyte)]:
    arr = (ctypes.c_ubyte * len(data)).from_buffer_copy(data)
    return arr, ctypes.cast(arr, ctypes.POINTER(ctypes.c_ubyte))


def _ptr_to_str(ptr: ctypes.c_void_p) -> str:
    if not ptr:
        return ""
    value = ctypes.cast(ptr, ctypes.c_char_p).value
    if value is None:
        return ""
    return value.decode("utf-8")


def _decode_error_payload(err_json: str) -> dict[str, Any]:
    if not err_json:
        return {}
    try:
        parsed = json.loads(err_json)
    except json.JSONDecodeError:
        return {"message": err_json}
    if isinstance(parsed, dict):
        return parsed
    return {"message": err_json}


def to_json(text: str) -> str:
    data = text.encode("utf-8")
    out_json = ctypes.c_void_p()
    out_err = ctypes.c_void_p()
    buf, ptr = _bytes_ptr(data)
    _ = buf

    try:
        status = _LIB.ccml_to_json(ptr, len(data), ctypes.byref(out_json), ctypes.byref(out_err))
        if status != STATUS_OK:
            err_payload = _decode_error_payload(_ptr_to_str(out_err))
            raise CcmlFfiError(status, err_payload)
        return _ptr_to_str(out_json)
    finally:
        if out_json.value:
            _LIB.ccml_free(out_json)
        if out_err.value:
            _LIB.ccml_free(out_err)


def diagnose(text: str) -> list[dict[str, Any]]:
    data = text.encode("utf-8")
    out_diag = ctypes.c_void_p()
    out_err = ctypes.c_void_p()
    buf, ptr = _bytes_ptr(data)
    _ = buf

    try:
        status = _LIB.ccml_diagnose(ptr, len(data), ctypes.byref(out_diag), ctypes.byref(out_err))
        if status != STATUS_OK:
            err_payload = _decode_error_payload(_ptr_to_str(out_err))
            raise CcmlFfiError(status, err_payload)
        payload = _ptr_to_str(out_diag)
        parsed = json.loads(payload)
        if not isinstance(parsed, list):
            raise RuntimeError("diagnostics payload is not a list")
        return parsed
    finally:
        if out_diag.value:
            _LIB.ccml_free(out_diag)
        if out_err.value:
            _LIB.ccml_free(out_err)
