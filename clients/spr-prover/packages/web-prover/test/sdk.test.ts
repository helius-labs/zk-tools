import { describe, expect, it, vi } from "vitest";
import type { MergeInputs, ProverInputs, TransferInput, TransferOutput } from "@heliuslabs/spr/client";
import { asField } from "@heliuslabs/spr/internal";
import { createLocalProofService } from "../src/sdk.js";

const zero = asField(0n);
const circuit = { domain: zero, owner: zero, asset: zero, amount: zero, blinding: zero, dataHash: zero, ringDataHash: zero, ringProgramId: zero };
const input: TransferInput = {
  circuit, isDummy: zero, statePathElements: [], statePathIndex: zero,
  nullifierLowValue: zero, nullifierNextValue: zero, nullifierLowPathElements: [],
  nullifierLowPathIndex: zero, treeSlot: zero, nullifier: zero,
  ownerPublicKeyHash: zero, nullifierSecret: zero,
};
const output: TransferOutput = { circuit, isDummy: zero, hash: zero, ownerPublicKeyHash: zero, nullifierPublicKey: zero };
const transfer: ProverInputs = { circuit: "transfer", payload: {
  inputs: [input], outputs: [output, output], treeSlots: [], outputTreeId: zero,
  externalDataHash: zero, privateTxHash: zero, blindingSeed: zero, publicAssets: [],
  publicAmounts: [], ringProgramId: zero, signerPublicKeyHashes: [], inputFlags: zero,
  publishedOutputOwnerPublicKeyHashes: [], cacheTreeId: zero, cacheReadHashChain: zero,
  cacheReadHashes: [], cacheIsCached: [], cacheReadIndex: [], publicInputHash: zero,
} };
const merge: MergeInputs = {
  inputs: Array.from({ length: 8 }, () => input), output, treeSlots: [], outputTreeId: zero,
  mintChunks: [zero, zero], outputCiphertextSalt: zero, ownerPublicKeyHash: zero,
  userNullifierPublicKey: zero, userNullifierSecret: zero, externalDataHash: zero,
  privateTxHash: zero, allowDummyInputs: zero, publicInputHash: zero,
  outputRingDataHash: zero, ringProgramId: zero,
};
const invalidProof = { proof: JSON.stringify({ provingKeySha256: "ff".repeat(32) }), proveMs: 0, verifyMs: 0 };

describe("optional typed SDK service", () => {
  it("rejects aborted and invalid contexts before invoking the worker", async () => {
    const proveRequest = vi.fn(async () => invalidProof);
    const service = createLocalProofService({ proveRequest });
    await expect(service.prove(transfer, { signal: AbortSignal.abort() })).rejects.toMatchObject({ code: "CLIENT_ABORTED" });
    await expect(service.prove(transfer, { timeoutMs: 0 })).rejects.toMatchObject({ code: "CLIENT_INVALID_CONTEXT" });
    expect(proveRequest).not.toHaveBeenCalled();
  });

  it("propagates deadlines to the worker and retains SDK timeout errors", async () => {
    const proveRequest = vi.fn((_body: string, signal?: AbortSignal) => new Promise<typeof invalidProof>((_resolve, reject) => {
      signal?.addEventListener("abort", () => reject(signal.reason), { once: true });
    }));
    await expect(createLocalProofService({ proveRequest }).prove(transfer, { timeoutMs: 5 })).rejects.toMatchObject({ code: "CLIENT_TIMEOUT" });
    expect(proveRequest).toHaveBeenCalledOnce();
    expect(proveRequest.mock.calls[0]?.[1]?.aborted).toBe(true);
  });

  it.each(["transfer", "merge"] as const)("validates the SDK's %s source-key digest", async (kind) => {
    const proveRequest = vi.fn(async () => invalidProof);
    const service = createLocalProofService({ proveRequest });
    const result = kind === "transfer" ? service.prove(transfer) : service.proveMerge(merge);
    await expect(result).rejects.toMatchObject({ code: "CLIENT_PROVING_KEY_MISMATCH" });
    expect(proveRequest).toHaveBeenCalledOnce();
  });
});
