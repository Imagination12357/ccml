import fs from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

const here = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(here, "..", "..");
const parityPath = path.resolve(repoRoot, "tests", "bindings", "parity-smoke-cases.json");
const cases = JSON.parse(fs.readFileSync(parityPath, "utf8"));

const wasmEntry = process.env.WASM_JS_ENTRY
  ? path.resolve(process.env.WASM_JS_ENTRY)
  : path.resolve(repoRoot, "core", "rust", "target", "wasm-smoke", "ccml_wasm.js");

if (!fs.existsSync(wasmEntry)) {
  throw new Error(`WASM JS entry not found: ${wasmEntry}`);
}

const wasmModule = await import(pathToFileURL(wasmEntry).href);

assert(typeof wasmModule.ccml_to_json_wasm === "function", "missing export: ccml_to_json_wasm");
assert(typeof wasmModule.ccml_diagnose_wasm === "function", "missing export: ccml_diagnose_wasm");

const ok = wasmModule.ccml_to_json_wasm(cases.success_case.input);
assert(ok === cases.success_case.expected_json, `unexpected wasm to_json output: ${ok}`);

let parseErrorSeen = false;
try {
  wasmModule.ccml_to_json_wasm(cases.error_case.input);
} catch (err) {
  parseErrorSeen = true;
  const msg = String(err?.message ?? err);
  assert(msg.includes("ccml parse error"), `unexpected wasm error message: ${msg}`);
}
assert(parseErrorSeen, "expected wasm parse error was not raised");

const diagText = wasmModule.ccml_diagnose_wasm(cases.warn_case.input);
const diagnostics = JSON.parse(diagText);
assert(Array.isArray(diagnostics), "wasm diagnose output must be array json");
assert(
  diagnostics.some(
    (d) =>
      d?.code === cases.warn_case.expected_warning_code &&
      d?.severity === cases.warn_case.expected_warning_severity
  ),
  `expected warning ${cases.warn_case.expected_warning_code}/${cases.warn_case.expected_warning_severity} not found`
);

console.log("wasm runtime smoke: ok");
