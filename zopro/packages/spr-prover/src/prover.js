import {
  ClientError,
  mergeProverRequestBody,
  parseProof,
  proverRequestBody,
} from "@heliuslabs/spr/client";

// The SDK stores -A for Solana's pairing instruction; the kernel verifies A.
const BN254_BASE_MODULUS =
  21888242871839275222246405745257275088696311157297823662689037894645226208583n;

function complete(body) {
  if (
    body.userNullifierSecret === null ||
    body.inputs.some((input) => input.nullifierSecret === null)
  ) {
    throw new ClientError("CLIENT_MISSING_NULLIFIER_SECRET");
  }
  return JSON.stringify(body);
}

function contextCheck(context) {
  const timeout = context?.timeoutMs;
  if (
    timeout !== undefined &&
    (!Number.isSafeInteger(timeout) || timeout <= 0)
  ) {
    throw new ClientError("CLIENT_INVALID_CONTEXT", {
      details: { method: "prove", field: "timeoutMs" },
    });
  }
  const deadline =
    timeout === undefined ? Infinity : performance.now() + timeout;
  return () => {
    if (context?.signal?.aborted)
      throw new ClientError("CLIENT_ABORTED", { details: { method: "prove" } });
    if (performance.now() >= deadline)
      throw new ClientError("CLIENT_TIMEOUT", {
        details: { method: "prove", retryable: true },
      });
  };
}

function point(bytes, length) {
  if (!(bytes instanceof Uint8Array) || bytes.length !== length)
    throw new TypeError(`Expected ${length} proof bytes`);
  const result = [];
  for (let offset = 0; offset < length; offset += 32) {
    result.push(
      `0x${Array.from(bytes.subarray(offset, offset + 32), (byte) => byte.toString(16).padStart(2, "0")).join("")}`,
    );
  }
  return result;
}

function kernelProof(proof) {
  const [x, yHex] = point(proof.a, 64);
  const y = BigInt(yHex);
  if (y >= BN254_BASE_MODULUS)
    throw new RangeError("Invalid proof.a coordinate");
  const b = point(proof.b, 128);
  return JSON.stringify({
    ar: [
      x,
      `0x${(y === 0n ? 0n : BN254_BASE_MODULUS - y).toString(16).padStart(64, "0")}`,
    ],
    bs: [b.slice(0, 2), b.slice(2)],
    krs: point(proof.c, 64),
  });
}

export function proverClass(KernelProver, isClosed = () => false) {
  return class SprProver {
    #kernel;
    #pending = 0;

    constructor(key) {
      if (isClosed()) throw new Error("The runtime is closed");
      this.#kernel = new KernelProver(key);
    }

    #loaded() {
      if (isClosed()) throw new Error("The runtime is closed");
      if (!this.#kernel) throw new Error("The prover is freed");
      return this.#kernel;
    }

    async #prove(body, context) {
      const check = contextCheck(context);
      check();
      const kernel = this.#loaded();
      this.#pending++;
      try {
        const result = await kernel.prove(complete(body));
        check();
        return parseProof(JSON.parse(result.proof));
      } finally {
        this.#pending--;
      }
    }

    async prove(inputs, context) {
      return this.#prove(proverRequestBody(inputs), context);
    }

    async proveMerge(inputs, context) {
      return this.#prove(mergeProverRequestBody(inputs), context);
    }

    verify(inputs, proof) {
      return this.#loaded().verify(
        complete(proverRequestBody(inputs)),
        kernelProof(proof),
      );
    }

    verifyMerge(inputs, proof) {
      return this.#loaded().verify(
        complete(mergeProverRequestBody(inputs)),
        kernelProof(proof),
      );
    }

    name() {
      return this.#loaded().name();
    }
    digest() {
      return this.#loaded().digest();
    }
    inputs() {
      return this.#loaded().inputs();
    }
    outputs() {
      return this.#loaded().outputs();
    }
    export_raw() {
      return this.#loaded().export_raw();
    }

    free() {
      if (this.#pending)
        throw new Error("Finish pending proofs before freeing the prover");
      this.#kernel?.free();
      this.#kernel = undefined;
    }
  };
}
