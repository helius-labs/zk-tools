# @heliuslabs/zopro-spr

The `zopro-spr` CPU WASM runtime in one package for Node 24+ and browsers.
Both environments use the same `init` and `SprProver` API. Conditional exports
select the worker host: Node's `worker_threads` or browser module workers.
The threaded hosts share one WASM binary. Proofs are verified before returning.

```js
import { init } from "@heliuslabs/zopro-spr";

const runtime = await init({ threads: 4 });
let prover;
try {
  prover = new runtime.SprProver(keyBytes); // Uint8Array of a checked .zopro key
  const { proof, proveMs, verifyMs } = await prover.prove(requestJson);
  // proof is the protocol prover's JSON response, including provingKeySha256.
} finally {
  prover?.free();
  await runtime.close();
}
```

Use `threads: 0` (the default) for the single-threaded build, or 1–64 for a
Rayon pool. Initialize once per JavaScript worker with a fixed thread count.
Free loaded provers before calling `close()`, which terminates the pool; to
initialize again after closing or a startup failure, start a new worker.
Finish pending proofs before freeing a prover or closing its runtime.

The caller provides keys and requests. Check `.zopro` bytes against a trusted
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
npm run example -- /path/transfer_confidential_2_2.zopro /path/request.json 4
```

## Browser

Call the same API inside a dedicated module worker. A browser bundler selects
the package's default export; a static ESM deployment can import
`./src/browser.js` from the extracted package. Serve its `src/` and `runtime/`
directories together, with `application/wasm` for `.wasm` files. The worker
hosts its Rayon workers itself; proving never moves to a remote server.

The [browser worker example](examples/browser-worker.js) takes checked key
bytes and a request. From a page served beside the extracted package:

```js
const worker = new Worker("./examples/browser-worker.js", { type: "module" });
worker.onmessage = ({ data }) => console.log(data);
worker.postMessage({ key: keyBytes, request: requestJson, threads: 4 });
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

Proof tests download the checksum-pinned 2x2 key from `tools/spr/zopro-keys.lock`
once into ignored `test/keys/`. They prove twice with 0, 1, 2 and 4 threads,
verify each result, check that all three proof points differ, reject a changed
public input and a malformed request, and close the runtime. Browser tests
also cover missing isolation and missing cryptographic randomness.
