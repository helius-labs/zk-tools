import { init } from "../src/browser.js";

self.addEventListener(
  "message",
  async ({ data: { key, request, threads = 0 } }) => {
    let runtime;
    let prover;
    try {
      runtime = await init({ threads });
      prover = new runtime.SprProver(key);
      self.postMessage({ result: await prover.prove(request) });
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
