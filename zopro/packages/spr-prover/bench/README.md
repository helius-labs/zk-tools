# Node WASM prover benchmark

From `zopro/packages/spr-prover`, build the package and run:

```sh
npm ci
npm run build
npm run bench -- --output results.csv
```

The default matrix covers **every key in `tools/spr/zopro-keys.lock`**:
38 confidential transfer input/output shapes and the 8-, 24- and 52-input
merge circuits. Each runs at **1, 2, 4, 8 and 18 threads**: 205 cases.
One thread means the threaded runtime with a one-worker Rayon pool.

Each case starts a fresh Node process, initializes its own pool, loads one
checked key, warms up once, then measures three proofs. Fixtures use real
positive-value inputs and outputs in every slot, unique nullifiers, valid
state Merkle paths and a balanced asset total. The transfers and merges
use the same typed SDK API as applications. No prover server is involved.

```sh
# Inspect all 205 cases without downloading keys or starting WASM.
npm run bench -- --list

# A quick proof check across all five requested thread counts.
npm run bench -- --circuits 1x2 --rounds 1 --warmup 0 --output smoke.csv

# Selected transfer and merge shapes; keep only two thread counts.
npm run bench -- --circuits 1x4,3x8,49x2,merge-8x1 --threads 4,18 --output selected.csv

# More measured samples, with newline-delimited JSON output.
npm run bench -- --rounds 10 --format jsonl --output results.jsonl
```

Times are milliseconds. `key_read_ms` includes disk/download and checksum
verification; `init_ms` covers WASM and pool initialization; `key_load_ms`
measures the key constructor. `prove_*_ms` measures the complete public
proving call, including witness construction, serialization, the proof,
its internal verification and SDK decoding. An additional explicit
verification is timed as `verify_median_ms`. Warmup proofs are excluded.
`peak_rss_mb` is the isolated process's peak resident memory, including
its worker threads, source key bytes and WASM memory.

The runner prints Node/CPU/platform metadata and progress to stderr, with
CSV or JSONL records on stdout (or `--output`). Keys download once into
`~/.cache/zopro-wasm/<key-set>/`; `--keys-dir` or `ZOPRO_KEYS_DIR` selects
another directory. Size, artifact SHA-256 and source gnark key digest must
match the repository's benchmark catalog. This catalog is an explicit
benchmark key set, independent of a client's deployed key set.

The full matrix loads several large keys; the largest shapes need several
GB of memory. Cases run sequentially. `--timeout-ms` bounds each isolated
case (default 900000). A failure produces an error row and a nonzero exit
status; `--fail-fast` stops at the first failure. Unknown shapes are errors,
so a missing fixture cannot silently reduce matrix coverage.
