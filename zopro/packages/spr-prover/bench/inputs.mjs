import { readFile } from "node:fs/promises";
import { initializePoseidon } from "@heliuslabs/spr";
import {
  NullifierKey,
  outputBlindingSeed,
  privateTxBlinding,
  transactOutputBlinding,
} from "@heliuslabs/spr/keypair";
import {
  BN254_MODULUS,
  bigintToBytes,
  bytesToBigInt,
  emptyCachedInputs,
  mergeOutputBlinding,
  mergeOutputKey,
  mergeOutputPads,
  mergePrivateTxBlinding,
  mergePublicInputs,
  poseidon,
  privateTxHash,
  resolvedPublicInputHash,
  transferPublicInputHash,
} from "@heliuslabs/spr/internal";

const readJson = async (path) =>
  JSON.parse(await readFile(new URL(path, import.meta.url), "utf8"));
const manifest = await readJson("../../../tools/spr/zopro-keys.lock");
const transferFixture = await readJson(
  "../../../tools/spr/fixtures/transfer-1x2.json",
);
const mergeFixture = await readJson(
  "../../../tools/spr/fixtures/merge-8x1.json",
);
const supported = new Set(Object.keys(manifest.keys));
const fieldBytes = (value) => bigintToBytes(value, "benchmark field");
const hash = (...values) => bytesToBigInt(poseidon(values.map(fieldBytes)));
let initialized;

function circuit(owner, asset, amount, blinding) {
  return {
    domain: 3n,
    owner,
    asset,
    amount,
    blinding,
    dataHash: 0n,
    ringDataHash: 0n,
    ringProgramId: 0n,
  };
}

function commitment(utxo, treeId) {
  return hash(
    utxo.domain,
    BigInt(treeId),
    utxo.asset,
    utxo.amount,
    utxo.dataHash,
    hash(utxo.ringDataHash, utxo.ringProgramId),
    hash(utxo.owner, utxo.blinding),
  );
}

function merkleTree(leaves) {
  const zeroes = [0n];
  const levels = [leaves];
  for (let level = 0; level < 32; level++) {
    const nodes = levels.at(-1);
    const zero = zeroes.at(-1);
    zeroes.push(hash(zero, zero));
    levels.push(
      Array.from({ length: Math.ceil(nodes.length / 2) }, (_, index) =>
        hash(nodes[index * 2] ?? zero, nodes[index * 2 + 1] ?? zero),
      ),
    );
  }
  return {
    root: levels.at(-1)[0],
    path(index) {
      return levels
        .slice(0, 32)
        .map((nodes, level) => nodes[(index >>> level) ^ 1] ?? zeroes[level]);
    },
  };
}

function spendInputs(count, ownerPublicKeyHash, nullifierSecret, asset) {
  const nullifierPublicKey = hash(nullifierSecret);
  const owner = hash(ownerPublicKeyHash, nullifierPublicKey);
  const utxos = Array.from({ length: count }, (_, index) =>
    circuit(owner, asset, 100n + BigInt(index), 15n + BigInt(index)),
  );
  const hashes = utxos.map((utxo) => commitment(utxo, 7));
  const tree = merkleTree(hashes);
  const empty = transferFixture.inputs[0];
  const inputs = utxos.map((utxo, index) => ({
    circuit: utxo,
    isDummy: 0n,
    statePathElements: tree.path(index),
    statePathIndex: BigInt(index),
    nullifierLowValue: BigInt(empty.nullifierLowValue),
    nullifierNextValue: BigInt(empty.nullifierNextValue),
    nullifierLowPathElements: empty.nullifierLowPathElements.map(BigInt),
    nullifierLowPathIndex: BigInt(empty.nullifierLowPathIndex),
    treeSlot: 0n,
    nullifier: hash(hashes[index], utxo.blinding, nullifierSecret),
    ownerPublicKeyHash,
    nullifierSecret,
  }));
  const nullifiers = inputs.map((input) => input.nullifier);
  if (
    new Set(nullifiers).size !== count ||
    inputs.some(
      (input) =>
        input.nullifier <= input.nullifierLowValue ||
        input.nullifier >= input.nullifierNextValue,
    )
  ) {
    throw new Error(
      "The benchmark inputs need distinct nullifiers in the empty tree's range",
    );
  }
  const treeSlots = [
    {
      id: 7n,
      utxoRoot: tree.root,
      nullifierRoot: BigInt(transferFixture.treeSlots[0].nullifierRoot),
    },
    ...Array.from({ length: 4 }, () => ({
      id: 0n,
      utxoRoot: 0n,
      nullifierRoot: 0n,
    })),
  ];
  return {
    inputs,
    hashes,
    nullifiers,
    treeSlots,
    owner,
    nullifierPublicKey,
    amount: utxos.reduce((total, utxo) => total + utxo.amount, 0n),
  };
}

const publicTreeSlots = (slots) =>
  slots.map((slot) => ({
    id: Number(slot.id),
    utxoRoot: fieldBytes(slot.utxoRoot),
    nullifierRoot: fieldBytes(slot.nullifierRoot),
  }));

function transferInputs(inputCount, outputCount) {
  const ownerPublicKeyHash = BigInt(transferFixture.inputs[0].ownerPkHash);
  const nullifierSecret = BigInt(transferFixture.inputs[0].nullifierSecret);
  const spent = spendInputs(
    inputCount,
    ownerPublicKeyHash,
    nullifierSecret,
    7n,
  );
  const firstNullifier = fieldBytes(spent.nullifiers[0]);
  const blindingSeed = 800n;
  const outputSeed = outputBlindingSeed(
    firstNullifier,
    fieldBytes(blindingSeed),
  );
  const outputs = Array.from({ length: outputCount }, (_, index) => {
    const amount =
      spent.amount / BigInt(outputCount) +
      (BigInt(index) < spent.amount % BigInt(outputCount) ? 1n : 0n);
    const blinding = bytesToBigInt(
      transactOutputBlinding(firstNullifier, outputSeed, index),
    );
    const utxo = circuit(spent.owner, 7n, amount, blinding);
    return {
      circuit: utxo,
      isDummy: 0n,
      hash: commitment(utxo, 11),
      ownerPublicKeyHash,
      nullifierPublicKey: spent.nullifierPublicKey,
    };
  });
  const cached = emptyCachedInputs(inputCount);
  const payload = {
    inputs: spent.inputs,
    outputs,
    treeSlots: spent.treeSlots,
    outputTreeId: 11n,
    externalDataHash: 300n,
    privateTxHash: bytesToBigInt(
      privateTxHash({
        inputHashes: spent.hashes.map(fieldBytes),
        outputHashes: outputs.map((output) => fieldBytes(output.hash)),
        blinding: privateTxBlinding(firstNullifier, fieldBytes(blindingSeed)),
      }),
    ),
    blindingSeed,
    publicAssets: [0n, 0n, 0n],
    publicAmounts: [0n, 0n, 0n],
    ringProgramId: 0n,
    signerPublicKeyHashes: [
      ownerPublicKeyHash,
      ...Array(Math.min(inputCount, 60 - inputCount)).fill(0n),
    ],
    inputFlags: 1n,
    publishedOutputOwnerPublicKeyHashes: outputs.map(() => ownerPublicKeyHash),
    ...cached,
  };
  payload.publicInputHash = transferPublicInputHash({
    ...payload,
    nullifiers: spent.nullifiers,
    outputHashes: outputs.map((output) => output.hash),
    outputTreeId: 11,
    treeSlots: publicTreeSlots(spent.treeSlots),
    publicSlots: [0n, 0n, 0n, 0n, 0n, 0n],
  });
  return { circuit: "transfer", payload };
}

function mergeInputs(inputCount) {
  const ownerPublicKeyHash = BigInt(mergeFixture.ownerPkHash);
  const userNullifierSecret = BigInt(mergeFixture.userNullifierSecret);
  const mintChunks = mergeFixture.mintChunks.map(BigInt);
  const asset = hash(...mintChunks);
  const spent = spendInputs(
    inputCount,
    ownerPublicKeyHash,
    userNullifierSecret,
    asset,
  );
  const firstNullifier = fieldBytes(spent.nullifiers[0]);
  const key = NullifierKey.fromSecret(
    bigintToBytes(userNullifierSecret, "benchmark nullifier secret", 31),
  );
  try {
    const blinding = bytesToBigInt(mergeOutputBlinding(key, firstNullifier));
    const utxo = circuit(spent.owner, asset, spent.amount, blinding);
    const output = {
      circuit: utxo,
      isDummy: 0n,
      hash: commitment(utxo, 11),
      ownerPublicKeyHash,
      nullifierPublicKey: spent.nullifierPublicKey,
    };
    const outputCiphertextSalt = BigInt(mergeFixture.outputCiphertextSalt);
    const pads = mergeOutputPads(
      mergeOutputKey(key, firstNullifier),
      bigintToBytes(outputCiphertextSalt, "benchmark ciphertext salt", 16),
    );
    const ciphertext = [spent.amount, ...mintChunks].map(
      (value, index) => (value + pads[index]) % BN254_MODULUS,
    );
    const inputs = {
      inputs: spent.inputs,
      output,
      treeSlots: spent.treeSlots,
      outputTreeId: 11n,
      mintChunks,
      outputCiphertextSalt,
      ownerPublicKeyHash,
      userNullifierPublicKey: spent.nullifierPublicKey,
      userNullifierSecret,
      externalDataHash: BigInt(mergeFixture.externalDataHash),
      privateTxHash: bytesToBigInt(
        privateTxHash({
          inputHashes: spent.hashes.map(fieldBytes),
          outputHashes: [fieldBytes(output.hash)],
          blinding: mergePrivateTxBlinding(key, firstNullifier),
        }),
      ),
      allowDummyInputs: 1n,
      outputRingDataHash: 0n,
      ringProgramId: 0n,
    };
    inputs.publicInputHash = resolvedPublicInputHash(
      mergePublicInputs({
        nullifiers: spent.nullifiers,
        outputHash: output.hash,
        outputTreeId: 11,
        privateTxHash: inputs.privateTxHash,
        externalDataHash: inputs.externalDataHash,
        binding: {
          kind: "default",
          ownerPublicKeyHash,
          nullifierPublicKey: spent.nullifierPublicKey,
          outputCiphertextSalt,
          outputCiphertext: ciphertext,
        },
      }),
      publicTreeSlots(spent.treeSlots),
    );
    return inputs;
  } finally {
    key.destroy();
  }
}

/** Deterministic, all-real notes for every key shipped in the benchmark catalog. */
export async function benchmarkInputs(keyName) {
  if (!supported.has(keyName))
    throw new Error(`Unsupported benchmark key: ${keyName}`);
  initialized ??= initializePoseidon();
  await initialized;
  const transfer = /^transfer_confidential_(\d+)_(\d+)\.key$/u.exec(keyName);
  if (transfer) return transferInputs(Number(transfer[1]), Number(transfer[2]));
  const merge = /^merge_(\d+)_1\.key$/u.exec(keyName);
  if (merge) return mergeInputs(Number(merge[1]));
  throw new Error(`The benchmark needs an input generator for ${keyName}`);
}
