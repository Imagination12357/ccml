import { CCML_STATUS } from "./contract.js";
import { resolveLibraryPath } from "./loader.js";

export class CcmlFfiError extends Error {
  constructor(status, payload) {
    const body = payload && typeof payload === "object" ? payload : {};
    const message = typeof body.message === "string" ? body.message : `ccml ffi call failed (status=${status})`;
    super(message);
    this.name = "CcmlFfiError";
    this.status = status;
    this.payload = body;
  }
}

function decodeErrorPayload(errJson) {
  if (!errJson) {
    return {};
  }
  try {
    const parsed = JSON.parse(errJson);
    return parsed && typeof parsed === "object" ? parsed : { message: errJson };
  } catch {
    return { message: errJson };
  }
}

function normalizeDiagnostics(payload) {
  if (!Array.isArray(payload)) {
    throw new Error("diagnostics payload is not a list");
  }
  return payload;
}

export async function createCcmlFfi() {
  let koffi;
  try {
    ({ default: koffi } = await import("koffi"));
  } catch (err) {
    throw new Error(
      `Failed to load Node FFI dependency 'koffi'. Run 'npm install' in bindings/node. (${err.message})`
    );
  }

  const lib = koffi.load(resolveLibraryPath());
  const OpaquePtr = koffi.pointer(koffi.opaque());
  const OutOwnedCStringPtr = koffi.out(koffi.pointer(OpaquePtr));
  const ccmlToJson = lib.func("ccml_to_json", "int32_t", [
    "void *",
    "uintptr_t",
    OutOwnedCStringPtr,
    OutOwnedCStringPtr
  ]);
  const ccmlDiagnose = lib.func("ccml_diagnose", "int32_t", [
    "void *",
    "uintptr_t",
    OutOwnedCStringPtr,
    OutOwnedCStringPtr
  ]);
  const ccmlFree = lib.func("ccml_free", "void", [OpaquePtr]);

  function consumeOwnedCString(slot) {
    const ptr = slot[0];
    slot[0] = null;
    if (!ptr) {
      return "";
    }
    const decoded = koffi.decode(ptr, "char", -1);
    ccmlFree(ptr);
    return decoded ?? "";
  }

  function releaseOwnedCString(slot) {
    const ptr = slot[0];
    slot[0] = null;
    if (ptr) {
      ccmlFree(ptr);
    }
  }

  function toJson(input) {
    const bytes = Buffer.from(input, "utf8");
    const outJson = [null];
    const outErr = [null];
    try {
      const status = ccmlToJson(bytes, bytes.length, outJson, outErr);
      if (status !== CCML_STATUS.OK) {
        throw new CcmlFfiError(status, decodeErrorPayload(consumeOwnedCString(outErr)));
      }
      return consumeOwnedCString(outJson);
    } finally {
      releaseOwnedCString(outJson);
      releaseOwnedCString(outErr);
    }
  }

  function diagnose(input) {
    const bytes = Buffer.from(input, "utf8");
    const outDiag = [null];
    const outErr = [null];
    try {
      const status = ccmlDiagnose(bytes, bytes.length, outDiag, outErr);
      if (status !== CCML_STATUS.OK) {
        throw new CcmlFfiError(status, decodeErrorPayload(consumeOwnedCString(outErr)));
      }
      return normalizeDiagnostics(JSON.parse(consumeOwnedCString(outDiag) || "[]"));
    } finally {
      releaseOwnedCString(outDiag);
      releaseOwnedCString(outErr);
    }
  }

  return {
    toJson,
    diagnose
  };
}
