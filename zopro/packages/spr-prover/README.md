# @heliuslabs/zopro-spr

The `zopro-spr` CPU WASM runtime in one package for Node 24+ and browsers.
Both environments use the same local `ProofService` API. Conditional exports
select the worker host: Node's `worker_threads` or browser module workers.
The threaded hosts share one WASM binary. Proofs are verified before returning.

The prover implements `ProofService` from `@heliuslabs/spr` 0.9.0-alpha:

```ts
prove(inputs: ProverInputs, context?: RequestContext): Promise<Proof>;
proveMerge(inputs: MergeInputs, context?: RequestContext): Promise<Proof>;
```

Inputs are the SDK's typed objects, including their `bigint` fields. The
returned `Proof` is the same as a server proof: `a` and `c` are 64-byte
`Uint8Array`s and `b` is 128 bytes. The SDK's serializer and proof decoder
handle the WASM boundary, including negating A for Solana's pairing verifier.

## Local proof service

```ts
import { LocalKeys } from "@heliuslabs/spr/client";
import { createLocalProofService } from "@heliuslabs/zopro-spr";

const proofs = await createLocalProofService({ threads: 4 });
try {
  await proofs.preload("transfer_confidential_1_2.key");
  const keys = LocalKeys.fromKeypair(shieldedKeypair, proofs);
  // Pass keys to the SDK transaction flow.
} finally {
  await proofs.close();
}
```

Set the SDK client's `proofDataSource: "client"` so it assembles Merkle data
before `LocalKeys` calls this proof service.

The service selects the key for each confidential transfer or merge shape,
checks its source digest against the SDK's production pins, downloads and
checks the artifact, and caches it. Calls are serialized and one decoded
key is resident at a time. `preload(keyName)` completes download, validation
and WASM key decoding during setup. Proofs with that shape reuse the resident
key; changing shape loads another key.

Node uses its platform cache directory; `keyDirectory` overrides it. Browsers
use Cache API storage, with a bounded in-memory fallback. The default is
`threads: 0`; use 1–64 for a Rayon pool. Browser calls run in a dedicated
module worker, with cross-origin isolation for a threaded pool.

One service owns a worker's runtime. Finish calls and close the service before
the worker exits; initialize a new worker for another service after closing.
Proof counters and application HTTP policy belong in the caller's test code.

## Explicit key buffers

Use `init` and `SprProver` when the caller manages its own checked key bytes:

```js
import { init } from "@heliuslabs/zopro-spr";

const runtime = await init({ threads: 4 });
let prover;
try {
  prover = new runtime.SprProver(keyBytes); // Uint8Array of a checked .zopro key
  const proof = await prover.prove(inputs); // SDK ProverInputs -> SDK Proof
  const valid = prover.verify(inputs, proof);
} finally {
  prover?.free();
  await runtime.close();
}
```

The same prover can be passed directly as the `ProofService` argument to
`LocalKeys.fromKeypair(keypair, prover)`. A loaded key serves its own circuit
and shape. Load the appropriate checksum-verified key for the inputs to prove.
`verifyMerge(inputs, proof)` is the merge counterpart of `verify`.

Use `threads: 0` (the default) for the single-threaded build, or 1–64 for a
Rayon pool. Initialize once per JavaScript worker with a fixed thread count.
Free loaded provers before calling `close()`, which terminates the pool; to
initialize again after closing or a startup failure, start a new worker.
Finish pending proofs before freeing a prover or closing its runtime. Request
contexts reject pre-aborted calls and discard results after cancellation or a
timeout; CPU work already executing cannot be interrupted by the calling
thread. Host the runtime in a worker when immediate cancellation is needed.

The caller provides keys and SDK inputs. Check `.zopro` bytes against a trusted
deployment's size and SHA-256 before loading them, and check `prover.digest()`
against its source gnark key pin. Transaction construction, RPC and indexer
access remain in the protocol SDK. See [zopro](../../README.md) for supported
circuits and the kernel's experimental status.

## Node

Read the WASM and keys from disk; no bundler or prover server is required.
The runtime uses Node's cryptographic randomness. A CLI may prove directly;
server applications should host it in a worker because key loading and some
proving work block the calling thread.

After building, run the [CLI example](examples/prove.mjs):

```sh
npm run example -- /path/transfer_confidential_2_2.zopro /path/inputs.mjs 4
```

The input module default-exports the SDK `ProverInputs` or `MergeInputs`
object built by your application. Bigints stay as bigints; applications do
not encode prover request JSON or decode server proof JSON themselves.

## Browser

Call the same API inside a dedicated module worker. Use a browser bundler
such as Vite, which selects the browser export and emits the module workers
and WASM assets. The worker hosts its Rayon workers itself; proving never
moves to a remote server. Install the `@heliuslabs/spr` peer dependency in
the application too.

The [browser worker example](examples/browser-worker.js) takes checked key
bytes and typed SDK inputs. From a Vite application with that worker:

```js
const worker = new Worker(new URL("./browser-worker.js", import.meta.url), {
  type: "module",
});
worker.onmessage = ({ data }) => console.log(data);
worker.postMessage({ key: keyBytes, inputs, threads: 4 });
```

For `threads` greater than zero, use HTTPS (or localhost) and serve the page
and worker assets with:

```text
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

Without isolation, explicitly select `threads: 0`. Both builds require WASM
SIMD: Chrome 91+, Firefox 89+, Safari/iOS 16.4+. The tests exercise Chromium,
Firefox and Playwright WebKit; WebKit does not substitute for testing Safari
or physical iOS devices. Keep keys and WASM within the target device's memory
budget; a larger input shape needs more memory.

## Build and test

The [Node benchmark](bench/README.md) covers all 41 supported transfer and
merge circuit shapes at 1, 2, 4, 8 and 18 threads, using verified local
proofs and a fresh process per case. Run `npm run bench -- --list` to inspect
the full 205-case matrix, or `npm run bench -- --output results.csv` to run it.

From this directory, install Node 24+, Rust `nightly-2025-11-15` with
`rust-src` and `wasm32-unknown-unknown`, and `wasm-bindgen-cli` 0.2.128:

```sh
npm ci
npm run build
npm test
npm run test:types
npm run test:proof
npx playwright install chromium firefox webkit
npm run test:browser
npm run test:package
npm pack
```

`npm pack` builds both runtimes from this checkout before including them in
the tarball. It does not publish the package. Consumers install that tarball
without Rust. The build replaces wasm-bindgen-rayon's generated browser helper
import with the appropriate worker host; its import shape is checked before
rewriting. Neither runtime changes the Rust kernel or its randomness source.

Proof tests download the checksum-pinned 2x2 transfer and 8-input merge keys
from `tools/spr/zopro-keys.lock` once into ignored `test/keys/`. Transfers
prove twice with 0, 1, 2 and 4 threads; merges prove twice with 4 threads. Tests
verify each result, check that all three proof points differ, reject a changed
public input and malformed inputs, and close the runtime. Browser tests
also cover missing isolation and missing cryptographic randomness. Codec tests
compare serialized transfer and merge inputs with the protocol fixtures and
check exact compatibility with the SDK's server proof decoder.
