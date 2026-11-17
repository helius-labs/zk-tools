# Node prover

Run the TypeScript example with Node 24+ from the repository root:

```sh
npm ci
npm run build:prover
node examples/node/prove.ts
```

Preloads a key, proves a synthetic 2→2 request and prints the proof and timing.
First use downloads the public key; later runs reuse the disk cache. GPU is
used when available, with CPU fallback. No RPC or prover server is needed.

See the [Node options](../../packages/web-prover/README.md#nodejs) for threads,
GPU selection and keeping multiple shapes loaded.
