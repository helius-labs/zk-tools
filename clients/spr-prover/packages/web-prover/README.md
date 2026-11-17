# @heliuslabs/spr-web-prover

Local Groth16 proving in browser and Node workers. The core entries have no required protocol SDK or Solana dependencies; transaction building stays in your app.

**Experimental and unaudited.** Use test funds only.

## Quickstart: Node

This is the source preview from [zk-tools #12](https://github.com/helius-labs/zk-tools/pull/12).
The published `0.7.0-alpha` package does **not** include the `/node` or `/sdk`
entries shown here. Use the pinned checkout below until a new npm release is
announced. Source builds need Git, Node.js 24+, npm, and [Rust via rustup](https://rustup.rs/).

```sh
git clone https://github.com/helius-labs/zk-tools.git
cd zk-tools
git checkout de68f5204812e92de2c430b14acc37f1af3691ab
cd clients/spr-prover

rustup toolchain install nightly-2025-11-15 --profile minimal \
  --component rust-src --target wasm32-unknown-unknown
cargo +stable install wasm-bindgen-cli --version 0.2.128 --locked
npm ci
npm run build:prover
node examples/node/prove.ts
```

The example downloads the public 2×2 proving key, proves a synthetic request,
verifies it locally, and prints the backend, elapsed time and proof JSON. It
needs no wallet, RPC, API key or prover server, and submits no transaction.
The first build compiles two WASM runtimes; the first proof also downloads and
decodes its key. Later runs reuse `~/.cache/spr-prover`.

The [complete Node example](https://github.com/helius-labs/zk-tools/blob/sergei/web-prover-subproject/clients/spr-prover/examples/node/prove.ts)
uses the package as follows (the checkout supplies the request fixture):

```ts
import { readFile } from "node:fs/promises";
import { SprNodeProver, shapeByLabel } from "@heliuslabs/spr-web-prover/node";

const prover = new SprNodeProver({ threads: 4, gpu: "off" });
try {
  await prover.ensureKey(shapeByLabel("2x2"));
  const request = await readFile("fixtures/transfer-2x2.json", "utf8");
  const result = await prover.proveRequest(request);
  console.log(result);
} finally {
  prover.terminate();
}
```

Keep an instance alive to reuse its loaded key across proofs. `proveRequest()`
accepts the protocol's `/prove` JSON body and returns locally verified proof
JSON plus `proveMs`, `verifyMs` and the actual `backend`. Direct proving and the
typed SDK adapter need no `proverUrl`.

## Quickstart: browser

From the same `clients/spr-prover` directory, after the build above:

```sh
npm run keys --workspace @zolana/browser-example
npm run dev
```

Open <http://127.0.0.1:5178> and select **Generate proof**. The page supplies a
synthetic 2×2 request and displays the verified result. No wallet or RPC is
needed. The Vite server supplies the isolation headers and serves the demo key
locally; deployed apps download keys from the pinned CDN by default.

For your own app, use a bundler that emits `new URL(..., import.meta.url)` assets
and nested module workers. Vite and webpack 5 are tested. Copy the
[demo Vite configuration](https://github.com/helius-labs/zk-tools/blob/sergei/web-prover-subproject/clients/spr-prover/examples/browser/vite.config.ts),
including `worker.format: "es"` and the headers below. Released packages contain
the built WASM; consumers of those packages do not need Rust.

## Quickstart: SDK transactions

Use the [complete deposit/transfer/withdraw quickstart](https://github.com/helius-labs/privacy-ring-examples/blob/sergei/spr-prover-example/typescript-client/README.md#prove-locally-in-node)
for a funded localnet wallet and a stack running without a prover server.
It pins this source preview and `@heliuslabs/spr@0.9.0-alpha`; its
`examples/local_proving.ts` connects the shared transaction flow like this:

```ts
import { SprNodeProver, shapeByLabel } from "@heliuslabs/spr-web-prover/node";
import { createLocalProofService } from "@heliuslabs/spr-web-prover/sdk";
import { runDepositTransferWithdraw } from "./deposit_transfer_withdraw.js";

const prover = new SprNodeProver({ threads: 4, gpu: "off" });
try {
  await prover.ensureKey(shapeByLabel("1x2"));
  await runDepositTransferWithdraw({
    proofService: createLocalProofService(prover),
  });
} finally {
  prover.terminate();
}
```

In that flow, `createSprClient({ ..., proofDataSource: "client" })` fetches
Merkle data from Photon, and `LocalKeys.fromKeypair(keypair, proofService)`
uses the local service. Solana RPC and indexer requests still use the network.
The adapter handles serialization, source-key validation, proof decoding and
cancellation/deadlines. A browser can pass `SprWebProver` to the same adapter.
Only `/sdk` imports the optional protocol SDK peer.

## Compatibility and resources

- Node.js 24+; browser builds need HTTPS or localhost and the bundler support above.
- The bundled keys and `/sdk` adapter target `@heliuslabs/spr@0.9.0-alpha`.
  Confidential signer transfers and merges of up to 24 notes are supported;
  ring-rail proofs are unsupported. Each proof carries the source-key digest
  that the SDK checks against its deployment pins.
- Keys can be tens to hundreds of MB. A decoded disk entry takes roughly 3×
  its compressed download size; proving also needs substantial RAM. Start with
  1×2 or 2×2 and measure your target device before loading larger shapes.
- Browser caches are origin-scoped; Node caches live in `~/.cache/spr-prover`.
  Only public proving keys are persisted by the package.
- `/sdk` checks the SDK's compiled deployment keys. For authenticated runtime
  key rotations, use the fetch integration under **Runtime key updates**.

## Node.js

Node uses worker threads and the same WASM runtime. CPU proving needs no native
GPU dependency. In the source workspace, add the optional GPU binding with:

```sh
npm install --no-save --package-lock=false webgpu@0.6.2 --workspace @heliuslabs/spr-web-prover
```

Then choose `gpu: "auto"` or `"required"` when constructing the prover. GPU
availability depends on the host and drivers.

- `gpu`: `"auto"` (default), `"off"`, or `"required"` to reject startup without a GPU.
  Each result's `backend` reports actual execution, including CPU recovery after
  a GPU failure.
- `threads`: defaults to available processors, capped at 18; `0` uses single-thread WASM.
- `cacheDir`: defaults to `~/.cache/spr-prover`; `false` disables disk caching.
  Only public proving keys are persisted.
- `residentKeyBudgetBytes`: defaults to `0` (one loaded shape). Set, for example,
  `512 * 1024 ** 2` to retain multiple shapes with LRU eviction in one worker.

Use `prefetch({ shapes })` to prepare the disk cache, or `ensureKey(shape)` to
make a shape resident. Keep the instance alive between proofs and call
`terminate()` when finished. `loadedKeys()` lists resident shapes.

The resident budget estimates key state using the larger of 8× compressed bytes
and 3× decoded bytes, when available. It is capped at 1 GiB; proof buffers,
workers and GPU allocations need additional memory. Keys exceeding the budget
run alone. Cancellation of active work clears resident state; disk entries survive.

Benchmarks: [all 40 shapes](https://github.com/helius-labs/zk-tools/blob/sergei/web-prover-subproject/clients/spr-prover/benchmarks/node/2026-11-17-m5-pro.md) and
[alternating shapes](https://github.com/helius-labs/zk-tools/blob/sergei/web-prover-subproject/clients/spr-prover/benchmarks/node/2026-11-17-switching.md), with raw
samples and reproduction commands.

## Browser isolation and threads

For a thread pool (`threads` ≥ 1, defaulting to the machine’s processors up to 18), serve the page with:

```text
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

Use HTTPS (localhost is fine in development).

Set `threads: 0` when you cannot isolate (OAuth/dApp popups, Firefox, Safari, Firefox extension pages): a second runtime proves without SharedArrayBuffer. Slower, but it runs. Key decompress still fans out across ordinary workers.

In Chrome 137+, `Document-Isolation-Policy: isolate-and-credentialless` also enables `crossOriginIsolated` without classic COOP/COEP, so threaded proving can keep working with popups.

### Chrome extension

Prove from an extension page (not the service worker). For threaded proving, set isolation in the manifest and allow Wasm; or use `threads: 0`:

```json
"cross_origin_embedder_policy": { "value": "require-corp" },
"cross_origin_opener_policy": { "value": "same-origin" },
"content_security_policy": { "extension_pages": "script-src 'self' 'wasm-unsafe-eval'; object-src 'self'" }
```

## Browser API

In the browser demo, the request fixture is served at `/fixtures/transfer-2x2.json`:

```ts
import { SprWebProver, WasmProverError } from "@heliuslabs/spr-web-prover";

const prover = new SprWebProver({});

try {
  const requestJson = await (await fetch("/fixtures/transfer-2x2.json")).text();
  const { proof, proveMs, verifyMs } = await prover.proveRequest(requestJson);
} catch (error) {
  if (error instanceof WasmProverError) console.error(error.code);
} finally {
  prover.terminate();
}
```

If a GPU proof fails verification, the same request is retried on the CPU.

## Remote proving and fetch integration

For remote proofs, supply your deployment's RPC, indexer and prover endpoints
and a reviewed policy. This configuration is separate from the local SDK
quickstart above:

```ts
import { createSprClient } from "@heliuslabs/spr";
import { SignedTeePolicy } from "@heliuslabs/spr/client";

const remoteClient = await createSprClient({
  solanaRpcUrl,
  indexerUrl,
  proverUrl,
  proverTee: SignedTeePolicy.forNetwork("devnet"), // or "mainnet"
});
```

The SDK verifies the network policy with its pinned signing keys, refreshes it
with attestation, and refuses expired or invalid policies. Select the network
explicitly; changing a proxy URL does not change trust. Custom deployments may
supply a complete reviewed static policy with `teePolicyFromJson` instead.
The wallet example shows devnet and mainnet configuration.
`ZOLANA_PROGRAM_IDS` and `ZOLANA_PROTOCOL_VERSION` still describe protocol compatibility.

For the optional fetch integration, use a loopback URL and omit `proverTee`. Keep the explicit policy
on a separate client when using remote merges.

Pass `prover.createFetch()` as `fetch`, set `proofDataSource: "client"`, and use the same loopback `proverUrl` on the client (the SDK encrypts non-loopback prover calls for an enclave). Then prove through `LocalKeys.fromKeypair` or, in a wallet, `LocalKeys.fromDerivationSeed`. Paths under that URL’s `/prove` are answered locally; everything else is delegated. The URL identifies intercepted requests; no server needs to listen there. Prefer the typed `/sdk` adapter for the bundled keys.

## Self-host keys

```sh
npx --no-install spr-prover-keys --output public/keys --keys transfer_confidential_1_2.key,transfer_confidential_2_2.key
```

Pass `keyBaseUrl` (for example `"/keys"`). Names must appear in the pinned key set; files install as `<name>.zopro` and are size/SHA-256 checked. For a local mirror, set `ZOLANA_KEYS_BASE_URL` to a `file:///.../` directory.

One shape is resident by default; Node can retain more with its resident-key
budget. The first load decompresses points and caches a raw form in the browser
Cache API or Node disk cache; later loads are much faster. In the browser, pass
`persistStorage: true` to request durable storage. Downloads resume in 8 MB parts
after interruption.

## Supply a download catalog

`keyCatalog` accepts `/proving-keys` JSON (`keys[].artifacts[]`). Obtain
it through your application's chosen source, or bundle a snapshot:

```ts
const prover = new SprWebProver({
  keyCatalog: catalog,
});
```

The gnark source digest and zopro checksum and size must match the bundled pins.
Downloads use the complete zopro URL, including its prefix and query. HTTPS and
loopback HTTP are accepted. `keyCatalog` and `keyBaseUrl` are mutually exclusive;
a missing conversion fails without falling back to another source. The catalog
describes downloads, not server routes or supported circuits. Proofs still
report the gnark digest as `provingKeySha256`.

The prover never fetches a catalog or policy itself. Omit `keyCatalog` to use the
bundled URLs. See [UPSTREAM.md](https://github.com/helius-labs/zk-tools/blob/sergei/web-prover-subproject/clients/spr-prover/UPSTREAM.md) for manifest provenance and
sync instructions.

## Runtime key updates

`keySource` accepts an authenticated `ProvingKeySet` from protocol SDK
`0.9.0-alpha` or later. It can replace the bundled checksums for compatible
confidential signer transfers and merges; a raw `keyCatalog` cannot.

```ts
import { SignedTeePolicy, SprClient } from "@heliuslabs/spr/client";
import { SprWebProver } from "@heliuslabs/spr-web-prover";

const provingKeys = await new SprClient({
  proverUrl: "https://your-prover.example/v1/spr",
  proverTee: SignedTeePolicy.forNetwork("devnet"),
}).fetchProvingKeySet();
const proverUrl = "http://127.0.0.1:3001";
const prover = new SprWebProver({ proverUrl, keySource: provingKeys });
const client = new SprClient({
  solanaRpcUrl: "https://your-solana-rpc.example",
  indexerUrl: "https://your-photon.example",
  proverUrl,
  proofDataSource: "client",
  provingKeys,
  fetch: prover.createFetch(),
});
```

The prover must serve the runtime metadata and match the on-chain verifier.
One instance keeps one snapshot; download a fresh set and create new instances
between runs. Downloads, sizes, checksums and worker key identities come from
that snapshot. Runtime generations have separate caches and do not delete each
other; use `clearKeys()` on an obsolete instance to reclaim its files.
Use `prover.provingKeys()` for current file sizes; `ShapeKey.keyBytes` describes
the bundled key set.

`keySource`, `keyCatalog` and `keyBaseUrl` are mutually exclusive. A custom
`ProvingKeySource` is an explicit trust decision by the application: authenticate
and approve its metadata before passing it in. New proof-input layouts, circuits
or solver features still need an SDK/runtime update. Old proofs must be rebuilt
after their keys are replaced on chain.

## Prefetch

```ts
await prover.prefetch({
  shapes: [shapeByLabel("1x2"), shapeByLabel("2x2")],
  onProgress: ({ keyFile, stage, loaded, total }) => render(keyFile, stage, loaded / total),
});
```

`neededKeys(maxInputs, outputs, maxMergeInputs)` lists shapes for spends/merges up to those limits. `provingKeys()` / `clearKeys()` report and clear cached key bytes.

## Lifecycle

- The worker starts on first use. Work is serialized; loaded shapes follow the resident-key budget (one by default).
- `AbortSignal` cancels queued work; cancelling in-flight work tears down the runtime (it can restart via the retained factory).
- `terminate()` stops the worker and disables auto-restart.
- Failures surface as `WasmProverError` with a stable `code` (no witness or backend detail in the message).
