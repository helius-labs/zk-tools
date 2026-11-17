import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { test } from "node:test";
import {
  bigintToBytes,
  bytesToBigInt,
  poseidon,
} from "@heliuslabs/spr/internal";
import { benchmarkInputs } from "../bench/inputs.mjs";

const lock = JSON.parse(
  await readFile(
    new URL("../../../tools/spr/zopro-keys.lock", import.meta.url),
    "utf8",
  ),
);
const hash = (left, right) =>
  bytesToBigInt(
    poseidon([bigintToBytes(left, "left"), bigintToBytes(right, "right")]),
  );

test("benchmark inputs cover every catalog shape with distinct real notes and balanced real outputs", async () => {
  assert.equal(Object.keys(lock.keys).length, 41);
  for (const key of Object.keys(lock.keys)) {
    const match = /_(\d+)_(\d+)\.key$/u.exec(key);
    assert.ok(match, key);
    const inputCount = Number(match[1]);
    const outputCount = Number(match[2]);
    const request = await benchmarkInputs(key);
    const merge = key.startsWith("merge_");
    const payload = merge ? request : request.payload;
    const outputs = merge ? [payload.output] : payload.outputs;
    assert.equal(payload.inputs.length, inputCount, key);
    assert.equal(outputs.length, outputCount, key);
    assert.ok(
      payload.inputs.every(
        (input) => input.isDummy === 0n && input.circuit.amount > 0n,
      ),
      key,
    );
    assert.ok(
      outputs.every(
        (output) => output.isDummy === 0n && output.circuit.amount > 0n,
      ),
      key,
    );
    assert.equal(
      new Set(payload.inputs.map((input) => input.nullifier)).size,
      inputCount,
      key,
    );
    assert.equal(
      payload.inputs.reduce((sum, input) => sum + input.circuit.amount, 0n),
      outputs.reduce((sum, output) => sum + output.circuit.amount, 0n),
      key,
    );
    assert.notEqual(payload.publicInputHash, 0n, key);
    for (const input of payload.inputs) {
      const utxo = input.circuit;
      let root = bytesToBigInt(
        poseidon(
          [
            utxo.domain,
            7n,
            utxo.asset,
            utxo.amount,
            utxo.dataHash,
            hash(utxo.ringDataHash, utxo.ringProgramId),
            hash(utxo.owner, utxo.blinding),
          ].map((value) => bigintToBytes(value, "commitment")),
        ),
      );
      assert.equal(input.statePathElements.length, 32);
      assert.equal(input.nullifierLowPathElements.length, 40);
      for (const [level, sibling] of input.statePathElements.entries()) {
        root =
          ((input.statePathIndex >> BigInt(level)) & 1n) === 0n
            ? hash(root, sibling)
            : hash(sibling, root);
      }
      assert.equal(root, payload.treeSlots[0].utxoRoot, `${key} Merkle path`);
    }
  }
});

test("the generated 1x2 fixture retains known protocol commitment and blinding vectors", async () => {
  const fixture = JSON.parse(
    await readFile(
      new URL("../../../tools/spr/fixtures/transfer-1x2.json", import.meta.url),
      "utf8",
    ),
  );
  const { payload } = await benchmarkInputs("transfer_confidential_1_2.key");
  assert.equal(payload.privateTxHash, BigInt(fixture.privateTxHash));
  assert.equal(
    payload.inputs[0].nullifier,
    BigInt(fixture.inputs[0].nullifier),
  );
  assert.deepEqual(
    payload.outputs.map((output) => output.hash),
    fixture.outputs.map((output) => BigInt(output.hash)),
  );
});

test("unknown circuit shapes are rejected instead of omitted", async () => {
  await assert.rejects(
    benchmarkInputs("transfer_confidential_7_64.key"),
    /Unsupported benchmark key/,
  );
});
