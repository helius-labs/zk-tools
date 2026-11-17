import {
  init,
  type Runtime,
  type Proof,
  type ProverInputs,
} from "@heliuslabs/zopro-spr";
import type { ProofService, MergeInputs } from "@heliuslabs/spr/client";

async function prove(key: Uint8Array, inputs: ProverInputs): Promise<Proof> {
  const runtime: Runtime = await init({ threads: 4 });
  const prover = new runtime.SprProver(key);
  const service: ProofService = prover;
  // @ts-expect-error the local prover accepts the same typed inputs as the SDK, not JSON
  void service.prove("{}");
  try {
    const result: Proof = await service.prove(inputs);
    const verified: boolean = prover.verify(inputs, result);
    if (!verified) throw new Error("invalid proof");
    return result;
  } finally {
    prover.free();
    await runtime.close();
  }
}

void prove;
async function merge(
  service: ProofService,
  inputs: MergeInputs,
): Promise<Proof> {
  return service.proveMerge(inputs);
}
void merge;
// @ts-expect-error threads are numeric in both environments
void init({ threads: "4" });
