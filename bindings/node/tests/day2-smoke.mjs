import { CcmlFfiError, createCcmlFfi } from "../src/index.js";

function assert(condition, msg) {
  if (!condition) {
    throw new Error(msg);
  }
}

const ffi = await createCcmlFfi();

const ok = ffi.toJson('name: "Alice"\nage: 30');
assert(ok === '{"age":30,"name":"Alice"}', `unexpected toJson success output: ${ok}`);

let errorSeen = false;
try {
  ffi.toJson('name "Alice"');
} catch (err) {
  errorSeen = true;
  assert(err instanceof CcmlFfiError, "error path must throw CcmlFfiError");
  assert(err.status === 3, `parse error status must be 3, got ${err.status}`);
  assert(typeof err.message === "string" && err.message.length > 0, "error message must exist");
}
assert(errorSeen, "expected parse error was not raised");

const diags = ffi.diagnose("x: 1\nx: 2");
assert(Array.isArray(diags), "diagnose must return array");
assert(
  diags.some((d) => d?.code === "CCML2001" && d?.severity === "warning"),
  "expected CCML2001 warning not found"
);

console.log("node day2 smoke: ok");
