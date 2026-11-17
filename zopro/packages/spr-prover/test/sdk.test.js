import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { test } from "node:test";
import { parseProof } from "@heliuslabs/spr/client";
import { proverClass } from "../src/prover.js";
import { sdkInputs, sdkMergeInputs } from "./sdk-inputs.js";

const fixture = (name) =>
  readFile(
    new URL(
      `../../../crates/zopro-spr/tests/fixtures/${name}`,
      import.meta.url,
    ),
    "utf8",
  );
const request = await fixture("transfer-2x2.request.json");
const merge = await fixture("merge-24x1.request.json");
const response = JSON.parse(await fixture("transfer-2x2.response.json"));
const inputs = sdkInputs(request);

// Compare semantic field values; the SDK writes short canonical hex strings.
const normalized = (value) =>
  JSON.stringify(value, (key, item) =>
    typeof item === "string" && item.startsWith("0x")
      ? BigInt(item).toString(16)
      : item,
  );
const sameBody = (actual, expected) =>
  assert.deepEqual(
    JSON.parse(normalized(JSON.parse(actual))),
    JSON.parse(normalized(JSON.parse(expected))),
  );

function mockKernel() {
  const calls = [];
  const SprProver = proverClass(
    class {
      async prove(body) {
        calls.push(body);
        return { proof: JSON.stringify(response.proof), proveMs: 1 };
      }
      verify(body, proof) {
        calls.push(body);
        assert.deepEqual(JSON.parse(proof), response.proof);
        return true;
      }
      free() {}
    },
  );
  return { prover: new SprProver(new Uint8Array()), calls };
}

test("prove accepts server SDK inputs and returns exactly the SDK Proof including negated A", async () => {
  const { prover, calls } = mockKernel();
  try {
    const proof = await prover.prove(inputs);
    assert.deepEqual(proof, parseProof(response));
    assert.deepEqual(Object.keys(proof), ["a", "b", "c"]);
    sameBody(calls[0], request);
    assert.equal(prover.verify(inputs, proof), true);
  } finally {
    prover.free();
  }
});

test("proveMerge uses the server SDK's MergeInputs and Proof contract", async () => {
  const { prover, calls } = mockKernel();
  try {
    const inputs = sdkMergeInputs(merge);
    const proof = await prover.proveMerge(inputs);
    assert.deepEqual(proof, parseProof(response));
    sameBody(calls[0], merge);
    assert.equal(prover.verifyMerge(inputs, proof), true);
  } finally {
    prover.free();
  }
});

test("missing secrets, invalid context and pre-aborted calls never reach WASM", async () => {
  const { prover, calls } = mockKernel();
  try {
    const missing = {
      ...inputs,
      payload: {
        ...inputs.payload,
        inputs: inputs.payload.inputs.map((input) => ({
          ...input,
          nullifierSecret: undefined,
        })),
      },
    };
    await assert.rejects(prover.prove(missing), {
      code: "CLIENT_MISSING_NULLIFIER_SECRET",
    });
    const missingMerge = {
      ...sdkMergeInputs(merge),
      userNullifierSecret: undefined,
    };
    await assert.rejects(prover.proveMerge(missingMerge), {
      code: "CLIENT_MISSING_NULLIFIER_SECRET",
    });
    await assert.rejects(prover.prove(inputs, { timeoutMs: 0 }), {
      code: "CLIENT_INVALID_CONTEXT",
    });
    await assert.rejects(
      prover.prove(inputs, { signal: AbortSignal.abort() }),
      { code: "CLIENT_ABORTED" },
    );
    assert.deepEqual(calls, []);
  } finally {
    prover.free();
  }
});

test("a timed-out proof is discarded and a freed prover cannot be reused", async () => {
  const SprProver = proverClass(
    class {
      async prove() {
        await new Promise((resolve) => setTimeout(resolve, 10));
        return { proof: JSON.stringify(response.proof) };
      }
      free() {}
    },
  );
  const prover = new SprProver(new Uint8Array());
  await assert.rejects(prover.prove(inputs, { timeoutMs: 1 }), {
    code: "CLIENT_TIMEOUT",
  });
  prover.free();
  await assert.rejects(prover.prove(inputs), /freed/);
});

test("cancelling a pending proof discards it without freeing live WASM state", async () => {
  let finish;
  const SprProver = proverClass(
    class {
      prove() {
        return new Promise((resolve) => {
          finish = resolve;
        });
      }
      free() {}
    },
  );
  const prover = new SprProver(new Uint8Array());
  const controller = new AbortController();
  const proof = prover.prove(inputs, { signal: controller.signal });
  assert.throws(() => prover.free(), /pending proofs/);
  controller.abort();
  finish({ proof: JSON.stringify(response.proof) });
  await assert.rejects(proof, { code: "CLIENT_ABORTED" });
  prover.free();
});
