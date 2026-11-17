import { sdkInputs, sdkMergeInputs } from "./sdk-inputs.js";

function check(condition, message) {
  if (!condition) throw new Error(message);
}

export async function checkProofs(
  init,
  key,
  request,
  threads,
  keyDigest,
  merge = false,
) {
  const runtime = await init({ threads });
  let prover;
  try {
    prover = new runtime.SprProver(key);
    check(prover.digest() === keyDigest, "source key digest");
    check(
      prover.inputs() === (merge ? 8 : 2) &&
        prover.outputs() === (merge ? 1 : 2),
      "loaded shape",
    );
    const inputs = merge ? sdkMergeInputs(request) : sdkInputs(request);
    const prove = merge
      ? prover.proveMerge.bind(prover)
      : prover.prove.bind(prover);
    const verify = merge
      ? prover.verifyMerge.bind(prover)
      : prover.verify.bind(prover);
    const first = await prove(inputs);
    const started = performance.now();
    const second = await prove(inputs);
    const proveMs = performance.now() - started;
    check(verify(inputs, first), "first proof verifies");
    check(verify(inputs, second), "second proof verifies");
    check(
      first.a instanceof Uint8Array && first.a.length === 64,
      "SDK a bytes",
    );
    check(
      first.b instanceof Uint8Array && first.b.length === 128,
      "SDK b bytes",
    );
    check(
      first.c instanceof Uint8Array && first.c.length === 64,
      "SDK c bytes",
    );
    for (const coordinate of ["a", "b", "c"]) {
      check(
        first[coordinate].some(
          (byte, index) => byte !== second[coordinate][index],
        ),
        `randomized ${coordinate}`,
      );
    }
    const changed = merge
      ? { ...inputs, publicInputHash: 1n }
      : { ...inputs, payload: { ...inputs.payload, publicInputHash: 1n } };
    check(!verify(changed, first), "changed public input rejected");
    let rejected = false;
    try {
      await prove("{}");
    } catch {
      rejected = true;
    }
    check(rejected, "malformed request rejected");
    return { threads, proveMs };
  } finally {
    prover?.free();
    await runtime.close();
  }
}
