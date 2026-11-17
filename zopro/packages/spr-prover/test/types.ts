import { init, type Runtime, type ProofResult } from "@heliuslabs/zopro-spr";

async function prove(key: Uint8Array, request: string): Promise<ProofResult> {
  const runtime: Runtime = await init({ threads: 4 });
  const prover = new runtime.SprProver(key);
  try {
    const result = await prover.prove(request);
    const verified: boolean = prover.verify(request, result.proof);
    if (!verified) throw new Error("invalid proof");
    return result;
  } finally {
    prover.free();
    await runtime.close();
  }
}

void prove;
// @ts-expect-error threads are numeric in both environments
void init({ threads: "4" });
