# Node prover

Run with Node 24+ from `clients/spr-prover`, after installing the
[pinned Rust/WASM toolchain](../../packages/web-prover/README.md#quickstart-node):

```sh
npm ci
npm run build:prover
node examples/node/prove.ts
```

Preloads a key, proves a synthetic 2→2 request and prints the proof and timing.
First use downloads the public key; later runs reuse the disk cache. The example
uses four CPU threads and terminates its worker when finished. No wallet, RPC or
prover server is needed, and no transaction is submitted.

This preview's `/node` entry is not in the published `0.7.0-alpha` package.
Use the source checkout in the quickstart until a new npm release is available.

See the [Node options](../../packages/web-prover/README.md#nodejs) for threads,
GPU selection and keeping multiple shapes loaded.
