from __future__ import annotations

import os
import platform
from pathlib import Path
import sys

from hatchling.builders.hooks.plugin.interface import BuildHookInterface


class CustomBuildHook(BuildHookInterface):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)
        self._hidden_native_dirs: list[tuple[Path, Path]] = []

    def initialize(self, version: str, build_data: dict) -> None:
        if version != "standard":
            return

        tag = os.getenv("CCML_PY_WHEEL_TAG", default_wheel_tag())
        build_data["pure_python"] = False
        build_data["tag"] = tag
        self._hide_non_target_native_dirs(platform_dir_for_tag(tag))

    def finalize(self, version: str, build_data: dict, artifact_path: str) -> None:
        self._restore_hidden_native_dirs()

    def _hide_non_target_native_dirs(self, target_platform_dir: str) -> None:
        if not target_platform_dir:
            return

        native_dir = Path(self.root) / "src" / "ccml_py" / "native"
        if not native_dir.exists():
            return
        if not (native_dir / target_platform_dir).is_dir():
            raise RuntimeError(f"native prebuilt directory is missing: {target_platform_dir}")

        hidden_root = Path(self.root) / ".hatch-native-skip"
        hidden_root.mkdir(exist_ok=True)
        for child in native_dir.iterdir():
            if not child.is_dir() or child.name == target_platform_dir:
                continue
            hidden = hidden_root / child.name
            if hidden.exists():
                raise RuntimeError(f"temporary native directory already exists: {hidden}")
            child.rename(hidden)
            self._hidden_native_dirs.append((hidden, child))

    def _restore_hidden_native_dirs(self) -> None:
        while self._hidden_native_dirs:
            hidden, original = self._hidden_native_dirs.pop()
            if hidden.exists():
                hidden.rename(original)
        hidden_root = Path(self.root) / ".hatch-native-skip"
        if hidden_root.exists() and not any(hidden_root.iterdir()):
            hidden_root.rmdir()


def default_wheel_tag() -> str:
    machine = platform.machine().lower()

    if sys.platform.startswith("win"):
        if machine in {"amd64", "x86_64"}:
            return "py3-none-win_amd64"
        if machine in {"arm64", "aarch64"}:
            return "py3-none-win_arm64"

    if sys.platform == "darwin":
        if machine in {"arm64", "aarch64"}:
            return "py3-none-macosx_11_0_arm64"
        if machine in {"amd64", "x86_64"}:
            return "py3-none-macosx_10_12_x86_64"

    if sys.platform.startswith("linux"):
        if machine in {"amd64", "x86_64"}:
            return "py3-none-linux_x86_64"
        if machine in {"arm64", "aarch64"}:
            return "py3-none-linux_aarch64"

    return "py3-none-any"


def platform_dir_for_tag(tag: str) -> str:
    if tag.endswith("win_amd64"):
        return "win32-x64"
    if tag.endswith("win_arm64"):
        return "win32-arm64"
    if "macosx" in tag and tag.endswith("arm64"):
        return "darwin-arm64"
    if "macosx" in tag and tag.endswith("x86_64"):
        return "darwin-x64"
    if tag.endswith("manylinux_2_28_x86_64") or tag.endswith("linux_x86_64"):
        return "linux-x64"
    if tag.endswith("manylinux_2_28_aarch64") or tag.endswith("linux_aarch64"):
        return "linux-arm64"
    return ""
