import { CcmlFfiError, createCcmlFfi } from "../src/index.js";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

function assert(condition, msg) {
  if (!condition) {
    throw new Error(msg);
  }
}

const ffi = await createCcmlFfi();
const here = path.dirname(fileURLToPath(import.meta.url));
const parityPath = path.resolve(here, "../../../tests/bindings/parity-smoke-cases.json");
const cases = JSON.parse(fs.readFileSync(parityPath, "utf8"));

const ok = ffi.toJson(cases.success_case.input);
assert(ok === cases.success_case.expected_json, `unexpected toJson success output: ${ok}`);

let errorSeen = false;
try {
  ffi.toJson(cases.error_case.input);
} catch (err) {
  errorSeen = true;
  assert(err instanceof CcmlFfiError, "error path must throw CcmlFfiError");
  assert(
    err.status === cases.error_case.expected_status,
    `parse error status must be ${cases.error_case.expected_status}, got ${err.status}`
  );
  assert(typeof err.message === "string" && err.message.length > 0, "error message must exist");
}
assert(errorSeen, "expected parse error was not raised");

const diags = ffi.diagnose(cases.warn_case.input);
assert(Array.isArray(diags), "diagnose must return array");
assert(
  diags.some(
    (d) =>
      d?.code === cases.warn_case.expected_warning_code &&
      d?.severity === cases.warn_case.expected_warning_severity
  ),
  `expected warning ${cases.warn_case.expected_warning_code}/${cases.warn_case.expected_warning_severity} not found`
);

console.log("node day2 smoke: ok");
