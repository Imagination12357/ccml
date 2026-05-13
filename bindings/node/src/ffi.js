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
  const OutCStringPtr = koffi.out(koffi.pointer("char", 2));
  const ccmlToJson = lib.func("ccml_to_json", "int32_t", [
    "str",
    "uintptr_t",
    OutCStringPtr,
    OutCStringPtr
  ]);
  const ccmlDiagnose = lib.func("ccml_diagnose", "int32_t", [
    "str",
    "uintptr_t",
    OutCStringPtr,
    OutCStringPtr
  ]);

  function toJson(input) {
    const bytes = Buffer.from(input, "utf8");
    const outJson = [""];
    const outErr = [""];
    const status = ccmlToJson(input, bytes.length, outJson, outErr);
    if (status !== CCML_STATUS.OK) {
      throw new CcmlFfiError(status, decodeErrorPayload(outErr[0]));
    }
    return outJson[0] ?? "";
  }

  function diagnose(input) {
    const bytes = Buffer.from(input, "utf8");
    const outDiag = [""];
    const outErr = [""];
    const status = ccmlDiagnose(input, bytes.length, outDiag, outErr);
    if (status !== CCML_STATUS.OK) {
      throw new CcmlFfiError(status, decodeErrorPayload(outErr[0]));
    }
    return normalizeDiagnostics(JSON.parse(outDiag[0] ?? "[]"));
  }

  return {
    toJson,
    diagnose
  };
}
