from __future__ import annotations

import os
import platform
import sys

from hatchling.builders.hooks.plugin.interface import BuildHookInterface


class CustomBuildHook(BuildHookInterface):
    def initialize(self, version: str, build_data: dict) -> None:
        build_data["pure_python"] = False
        build_data["tag"] = os.getenv("CCML_PY_WHEEL_TAG", default_wheel_tag())


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
