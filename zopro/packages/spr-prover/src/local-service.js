import { ClientError } from "@heliuslabs/spr/client";
import { PROVING_KEY_SHA256S } from "@heliuslabs/spr/internal";
import productionCatalog from "./proving-keys.json" with { type: "json" };

function request(context) {
  if (
    context?.timeoutMs !== undefined &&
    (!Number.isSafeInteger(context.timeoutMs) || context.timeoutMs <= 0)
  ) {
    throw new ClientError("CLIENT_INVALID_CONTEXT", {
      details: { method: "prove", field: "timeoutMs" },
    });
  }
  const deadline =
    context?.timeoutMs === undefined
      ? Infinity
      : performance.now() + context.timeoutMs;
  const controller = new AbortController();
  let timedOut = false;
  const aborted = () => controller.abort();
  context?.signal?.addEventListener("abort", aborted, { once: true });
  const timer =
    context?.timeoutMs === undefined
      ? undefined
      : setTimeout(() => {
          timedOut = true;
          controller.abort();
        }, context.timeoutMs);
  return {
    signal: controller.signal,
    check() {
      if (context?.signal?.aborted)
        throw new ClientError("CLIENT_ABORTED", {
          details: { method: "prove" },
        });
      if (timedOut || performance.now() >= deadline)
        throw new ClientError("CLIENT_TIMEOUT", {
          details: { method: "prove", retryable: true },
        });
    },
    context() {
      this.check();
      return {
        signal: controller.signal,
        ...(deadline === Infinity
          ? {}
          : {
              timeoutMs: Math.max(1, Math.ceil(deadline - performance.now())),
            }),
      };
    },
    cleanup() {
      clearTimeout(timer);
      context?.signal?.removeEventListener("abort", aborted);
    },
  };
}

export function localProofServiceFactory(
  initialize,
  storeFactory,
  {
    catalog = productionCatalog,
    pins = PROVING_KEY_SHA256S,
    fetch = (...args) => globalThis.fetch(...args),
  } = {},
) {
  let claimed = false;
  return async function createLocalProofService(options = {}) {
    if (!options || typeof options !== "object" || Array.isArray(options))
      throw new TypeError("Local proof service options must be an object");
    const threads = options.threads === undefined ? 0 : options.threads;
    if (!Number.isInteger(threads) || threads < 0 || threads > 64)
      throw new RangeError("threads must be an integer between 0 and 64");
    if (
      options.keyDirectory !== undefined &&
      (typeof options.keyDirectory !== "string" ||
        !options.keyDirectory ||
        options.keyDirectory.includes("\0"))
    )
      throw new TypeError("keyDirectory must be a nonempty directory path");
    if (claimed)
      throw new Error(
        "A local proof service already owns this worker's runtime; use a new worker for another service",
      );
    for (const [name, artifact] of Object.entries(catalog.keys)) {
      if (pins[name] !== artifact.keySha256)
        throw new Error(
          `The local key catalog does not match the SDK deployment: ${name}`,
        );
    }
    const keySet = new URL(catalog.baseUrl).pathname
      .split("/")
      .filter(Boolean)
      .at(-1);
    const store = await storeFactory(options, keySet);
    if (claimed) {
      await store.close?.();
      throw new Error(
        "A local proof service already owns this worker's runtime; use a new worker for another service",
      );
    }
    claimed = true;
    let runtime;
    try {
      runtime = await initialize({ threads });
    } catch (error) {
      await store.close?.();
      throw error;
    }
    let loaded;
    let queue = Promise.resolve();
    let closed = false;
    let closing;

    async function checked(bytes, artifact) {
      if (
        bytes.byteLength !== artifact.size ||
        (await store.sha256(bytes)) !== artifact.sha256
      ) {
        throw new Error(
          `Proving key size or SHA-256 mismatch: ${artifact.file}`,
        );
      }
      return bytes;
    }

    async function load(name, operation) {
      operation?.check();
      if (loaded?.name === name) return loaded.prover;
      const artifact = Object.hasOwn(catalog.keys, name)
        ? catalog.keys[name]
        : undefined;
      if (!artifact) throw new Error(`No pinned local proving key for ${name}`);
      const url = new URL(artifact.file, catalog.baseUrl).href;
      let bytes = await store.read(artifact, url);
      if (bytes === undefined) {
        const timeout = AbortSignal.timeout(120_000);
        let response;
        try {
          response = await fetch(url, {
            signal: operation
              ? AbortSignal.any([operation.signal, timeout])
              : timeout,
          });
          if (!response.ok)
            throw new Error(
              `Could not download ${artifact.file}: HTTP ${response.status}`,
            );
          bytes = await checked(
            new Uint8Array(await response.arrayBuffer()),
            artifact,
          );
        } catch (error) {
          operation?.check();
          throw error;
        }
        await store.write(artifact, bytes, url);
      } else {
        await checked(bytes, artifact);
      }
      operation?.check();
      loaded?.prover.free();
      loaded = undefined;
      const prover = new runtime.SprProver(bytes);
      if (prover.digest() !== artifact.keySha256) {
        prover.free();
        throw new Error(`Source proving key digest mismatch: ${name}`);
      }
      const shape = /_(\d+)_(\d+)\.key$/u.exec(name);
      if (
        !shape ||
        prover.inputs() !== Number(shape[1]) ||
        prover.outputs() !== Number(shape[2])
      ) {
        prover.free();
        throw new Error(`Decoded proving key shape mismatch: ${name}`);
      }
      loaded = { name, prover };
      operation?.check();
      return prover;
    }

    function enqueue(run) {
      if (closed)
        return Promise.reject(new Error("The local proof service is closed"));
      const result = queue.then(run);
      queue = result.then(
        () => undefined,
        () => undefined,
      );
      return result;
    }

    function prove(name, method, inputs, context) {
      let operation;
      try {
        operation = request(context);
        operation.check();
      } catch (error) {
        operation?.cleanup();
        return Promise.reject(error);
      }
      return enqueue(async () => {
        const prover = await load(name, operation);
        const proof = await prover[method](inputs, operation.context());
        operation.check();
        return proof;
      }).finally(() => operation.cleanup());
    }

    return {
      preload(keyName) {
        return enqueue(async () => {
          await load(keyName);
        });
      },
      async prove(inputs, context) {
        if (inputs.circuit !== "transfer")
          throw new ClientError("CLIENT_PROVER_INPUT");
        return prove(
          `transfer_confidential_${inputs.payload.inputs.length}_${inputs.payload.outputs.length}.key`,
          "prove",
          inputs,
          context,
        );
      },
      async proveMerge(inputs, context) {
        if (inputs.ringProgramId !== 0n)
          throw new ClientError("CLIENT_PROVER_INPUT");
        return prove(
          `merge_${inputs.inputs.length}_1.key`,
          "proveMerge",
          inputs,
          context,
        );
      },
      close() {
        closed = true;
        closing ??= queue.then(async () => {
          const owned = loaded;
          loaded = undefined;
          try {
            owned?.prover.free();
          } finally {
            try {
              await runtime.close();
            } finally {
              await store.close?.();
            }
          }
        });
        return closing;
      },
    };
  };
}
