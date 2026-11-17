function check(condition, message) {
  if (!condition) throw new Error(message);
}

export async function checkProofs(init, key, request, threads, keyDigest) {
  const runtime = await init({ threads });
  let prover;
  try {
    prover = new runtime.SprProver(key);
    check(prover.digest() === keyDigest, "source key digest");
    check(prover.inputs() === 2 && prover.outputs() === 2, "loaded 2x2 shape");
    const first = await prover.prove(request);
    const second = await prover.prove(request);
    check(prover.verify(request, first.proof), "first proof verifies");
    check(prover.verify(request, second.proof), "second proof verifies");
    const a = JSON.parse(first.proof);
    const b = JSON.parse(second.proof);
    for (const coordinate of ["ar", "bs", "krs"]) {
      check(
        JSON.stringify(a[coordinate]) !== JSON.stringify(b[coordinate]),
        `randomized ${coordinate}`,
      );
    }
    const changed = JSON.parse(request);
    changed.publicInputHash = "0x01";
    check(
      !prover.verify(JSON.stringify(changed), first.proof),
      "changed public input rejected",
    );
    check(
      JSON.parse(first.proof).provingKeySha256 === keyDigest,
      "proof reports source key digest",
    );
    let rejected = false;
    try {
      await prover.prove("{}");
    } catch {
      rejected = true;
    }
    check(rejected, "malformed request rejected");
    return { threads, proveMs: second.proveMs };
  } finally {
    prover?.free();
    await runtime.close();
  }
}
