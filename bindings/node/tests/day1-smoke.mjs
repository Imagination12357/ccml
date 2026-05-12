import {
  createPhase1BindingSkeleton,
  phase1Contract
} from "../src/index.js";

const contract = phase1Contract();
const binding = createPhase1BindingSkeleton();

if (!binding.libraryPath) {
  throw new Error("libraryPath must be resolved");
}

for (const name of contract.requiredExports) {
  if (!binding.requiredExports.includes(name)) {
    throw new Error(`required export missing from skeleton contract: ${name}`);
  }
}

console.log("node day1 smoke: ok");
console.log(`library=${binding.libraryPath}`);
console.log(`required_exports=${contract.requiredExports.join(",")}`);
