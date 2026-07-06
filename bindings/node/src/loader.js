import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { FFI_OVERRIDE_ENV, PHASE1_EXPORTS } from "./contract.js";

function repoRoot() {
  const here = fileURLToPath(new URL(".", import.meta.url));
  return path.resolve(here, "../../../");
}

function packageRoot() {
  const here = fileURLToPath(new URL(".", import.meta.url));
  return path.resolve(here, "../");
}

function platformTag() {
  const archAliases = {
    x64: "x64",
    arm64: "arm64"
  };
  return `${process.platform}-${archAliases[process.arch] ?? process.arch}`;
}

function libraryNames() {
  if (process.platform === "win32") {
    return ["ccml_ffi.dll"];
  }
  if (process.platform === "darwin") {
    return ["libccml_ffi.dylib"];
  }
  return ["libccml_ffi.so"];
}

function packageLibraryCandidates() {
  const native = path.join(packageRoot(), "native");
  return [
    ...libraryNames().map((name) => path.join(native, platformTag(), name)),
    ...libraryNames().map((name) => path.join(native, name))
  ];
}

function candidateLibraries() {
  const target = path.join(repoRoot(), "core", "rust", "target");
  return [
    ...packageLibraryCandidates(),
    path.join(target, "debug", "ccml_ffi.dll"),
    path.join(target, "release", "ccml_ffi.dll"),
    path.join(target, "debug", "libccml_ffi.so"),
    path.join(target, "release", "libccml_ffi.so"),
    path.join(target, "debug", "libccml_ffi.dylib"),
    path.join(target, "release", "libccml_ffi.dylib")
  ];
}

export function resolveLibraryPath() {
  const override = process.env[FFI_OVERRIDE_ENV];
  if (override) {
    if (!fs.existsSync(override)) {
      throw new Error(`${FFI_OVERRIDE_ENV} is set but file does not exist: ${override}`);
    }
    return override;
  }

  for (const candidate of candidateLibraries()) {
    if (fs.existsSync(candidate)) {
      return candidate;
    }
  }

  throw new Error(
    [
      "Could not find ccml-ffi dynamic library.",
      "Install a package with a bundled native library, build it first (e.g. `cargo build -p ccml-ffi`), or set CCML_FFI_LIB.",
      "Looked in:",
      ...candidateLibraries().map((p) => `- ${p}`)
    ].join("\n")
  );
}

export function phase1Contract() {
  return {
    requiredExports: [...PHASE1_EXPORTS],
    ffiOverrideEnv: FFI_OVERRIDE_ENV
  };
}

export function createPhase1BindingSkeleton() {
  return {
    libraryPath: resolveLibraryPath(),
    // Day 1 scope: contract + load path only. Symbol binding is added on Day 2.
    requiredExports: [...PHASE1_EXPORTS]
  };
}
