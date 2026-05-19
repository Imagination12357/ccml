import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { CcmlFfiError, createCcmlFfi } from "../src/index.js";

function assert(condition, msg) {
  if (!condition) {
    throw new Error(msg);
  }
}

const here = path.dirname(fileURLToPath(import.meta.url));
const parityPath = path.resolve(here, "../../../tests/bindings/parity-smoke-cases.json");
const cases = JSON.parse(fs.readFileSync(parityPath, "utf8"));

const ffi = await createCcmlFfi();
const cycles = 150;

for (let i = 0; i < cycles; i += 1) {
  const ok = ffi.toJson(cases.success_case.input);
  assert(ok === cases.success_case.expected_json, `success output drift at cycle ${i}`);

  let errSeen = false;
  try {
    ffi.toJson(cases.error_case.input);
  } catch (err) {
    errSeen = true;
    assert(err instanceof CcmlFfiError, `error type mismatch at cycle ${i}`);
    assert(err.status === cases.error_case.expected_status, `error status drift at cycle ${i}`);
  }
  assert(errSeen, `expected parse error missing at cycle ${i}`);

  const diags = ffi.diagnose(cases.warn_case.input);
  assert(Array.isArray(diags), `diagnostics must be array at cycle ${i}`);
  assert(
    diags.some(
      (d) =>
        d?.code === cases.warn_case.expected_warning_code &&
        d?.severity === cases.warn_case.expected_warning_severity
    ),
    `warning drift at cycle ${i}`
  );
}

console.log(`node day1 regression: ok (${cycles} mixed cycles)`);
