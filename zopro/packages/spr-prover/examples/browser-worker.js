import { init } from "@heliuslabs/zopro-spr";

self.addEventListener(
  "message",
  async ({ data: { key, inputs, threads = 0 } }) => {
    let runtime;
    let prover;
    try {
      runtime = await init({ threads });
      prover = new runtime.SprProver(key);
      const proof =
        "circuit" in inputs
          ? await prover.prove(inputs)
          : await prover.proveMerge(inputs);
      self.postMessage({ proof });
    } catch (error) {
      self.postMessage({ error: String(error) });
    } finally {
      prover?.free();
      await runtime?.close();
      self.close();
    }
  },
  { once: true },
);
