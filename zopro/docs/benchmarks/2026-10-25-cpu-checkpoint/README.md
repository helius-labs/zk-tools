# CPU prover checkpoint — October 25, 2026

This checkpoint adds CPU basis/Euler images, streamed solver-image handling, bounded solver scheduling, SIMD arithmetic, rerandomized proof assembly and memory-aware buffer reclamation. The tables below describe the accepted implementation and its measured configurations. The original objective of a tenfold latency improvement than every supplied M5 CPU reference has not been reached.

Gnark and Aeglos provide the reference implementations and independent verification used in this work. The implementations make different preparation, arithmetic and memory tradeoffs. These results describe the versions, fixtures and machines below; they are not a general ranking of the projects.

## Local measurements

Ryzen 9 9950X, 16 physical cores / 32 hardware threads, 32 workers. Each circuit uses nine rotated/reversed process blocks with 31 measured requests after three priming requests. These are the accepted candidate observations from its fixed qualification against retained and freshly built controls. The timer covers fresh witness solving, blinding, proving, SDK verification and serialization; image loading is excluded. Peak RSS includes loading.

| Circuit | Zopro ms | Peak RSS MiB | Supplied M5 CPU ms | CPU reference/Zopro | Supplied M5 Metal ms | Metal reference/Zopro |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 9.004 | 124.75 | 37 | 4.11 × | 24 | 2.67 × |
| 2x2 | 11.957 | 280.25 | 59 | 4.93 × | 30 | 2.51 × |
| 4x2 | 16.462 | 318.25 | 109 | 6.62 × | 48 | 2.92 × |
| 8x2 | 24.910 | 513.75 | 189 | 7.59 × | 74 | 2.97 × |
| 12x2 | 35.222 | 733.75 | 284 | 8.06 × | 101 | 2.87 × |
| 16x2 | 41.136 | 851.25 | 335 | 8.14 × | 124 | 3.01 × |
| 24x2 | 68.512 | 1306.57 | 528 | 7.71 × | 176 | 2.57 × |
| merge-24x1 | 67.657 | 1188.30 | 507 | 7.49 × | 175 | 2.59 × |
| merge-54x1 | 174.479 | 1899.12 | — | — | — | — |

The M5 numbers are supplied historical references, rather than new same-machine runs. Differences include hardware and software configuration. No M5 reference was supplied for 54-to-1.

## AWS comparison with Aeglos

Dedicated c7a.24xlarge with 96 physical cores, a 30 GiB container and a Zen 4-targeted build. Each of the nine circuits runs at 16, 32, 64 and 96 workers in three rotating/reversing fresh-process repetitions. Each process measures 31 requests for 1x2 or 17 for other circuits after three priming requests. Requests include witness solving, fresh randomness, proving, verification and serialization; key loading is excluded. The 16-worker AWS cases are unpinned.

Aeglos is pinned to [0290613](https://github.com/helius-labs/aeglos/tree/0290613e8eeca17a4ce88f6f2d726e8e06a5385d), with Gnark 0.16.3 and gnark-crypto 0.21.0. Sorted is its default tested engine. Windowed is a benchmark constructor configuration that disables shifted bases. Both configurations are shown. Zopro uses prepared Euler images, while Aeglos performs its native preparation before warm measurements. Preparation time is excluded from these latency tables, and whole-process memory includes it.

Each cell is the median of three process medians at that engine’s fastest tested worker count. Selecting different worker counts is descriptive; [the data](measurements.json) also includes every same-worker configuration, process median and sample, plus the preceding Zopro control.

| Circuit | Zopro ms (workers) | Aeglos Sorted ms (workers) | Aeglos Windowed ms (workers) |
|---|---:|---:|---:|
| 1x2 | 13.627 (32) | 7.966 (96) | 26.809 (32) |
| 2x2 | 17.555 (32) | 12.084 (96) | 40.452 (64) |
| 4x2 | 21.588 (64) | 18.565 (96) | 62.196 (64) |
| 8x2 | 28.298 (64) | 31.418 (96) | 102.447 (64) |
| 12x2 | 35.988 (64) | 49.276 (96) | 163.502 (32) |
| 16x2 | 40.158 (64) | 57.150 (96) | 178.162 (32) |
| 24x2 | 57.526 (64) | 87.765 (96) | 264.164 (64) |
| merge-24x1 | 62.197 (64) | 85.384 (96) | 254.971 (64) |
| merge-54x1 | 129.985 (64) | 173.566 (96) | 418.382 (96) |

Aeglos Sorted has lower measured latency for 1x2, 2x2 and 4x2. Zopro has lower measured latency for the other six circuits. For 1x2, Zopro’s best tested result is 13.627 ms at 32 workers, compared with 7.966 ms for Aeglos Sorted at 96 workers. This remaining scaling difference is an optimization target. These observations do not identify an exclusive cause.

| Circuit | Zopro peak RSS MiB | Aeglos Sorted peak RSS MiB | Aeglos Windowed peak RSS MiB |
|---|---:|---:|---:|
| 1x2 | 114.94 | 464.63 | 377.70 |
| 2x2 | 225.95 | 815.14 | 716.75 |
| 4x2 | 268.29 | 1365.40 | 1156.75 |
| 8x2 | 450.89 | 2459.89 | 1926.00 |
| 12x2 | 640.97 | 3642.78 | 2240.62 |
| 16x2 | 777.53 | 4606.06 | 2754.74 |
| 24x2 | 1208.00 | 6769.57 | 4123.26 |
| merge-24x1 | 1109.62 | 6585.76 | 3936.81 |
| merge-54x1 | 1787.33 | 13099.78 | 7169.03 |

RSS is the maximum whole-process value across three processes at each selected worker count. It includes loading, preparation and priming; it is not incremental proof workspace or minimum required RAM. The native preparation budget and whole-process memory have different scopes. Seven remote processes recorded a combined 162,767 microseconds of CPU throttling; no result was discarded or rerun because of it.

| Circuit | AWS Zopro ms | M5 CPU reference/Zopro | M5 Metal reference/Zopro |
|---|---:|---:|---:|
| 1x2 | 13.627 | 2.72 × | 1.76 × |
| 2x2 | 17.555 | 3.36 × | 1.71 × |
| 4x2 | 21.588 | 5.05 × | 2.22 × |
| 8x2 | 28.298 | 6.68 × | 2.62 × |
| 12x2 | 35.988 | 7.89 × | 2.81 × |
| 16x2 | 40.158 | 8.34 × | 3.09 × |
| 24x2 | 57.526 | 9.18 × | 3.06 × |
| merge-24x1 | 62.197 | 8.15 × | 2.81 × |

## Validation and limits

Native candidate and disabled builds pass 33 field, 104 core, two raw-image and six cubic tests each. The local fixed qualification contains 9,340 SDK requests and 310 distinct independently Gnark-verified exports. The AWS matrix and its preflight contain 9,456 SDK requests and 468 distinct independently verified exports. Every timed SDK call verifies its generated proof. Exported proof/public hashes, image/source identity and fixture public inputs were checked; verifying the same export remotely and locally is counted once.

Further qualification covers portable fallback proofs, concurrent callers, cold and warm memory-pressure transitions and the 210,853,022-constraint forester 4999 circuit. In the fixed large-circuit comparison, the accepted implementation’s two input variants measured 153.525 and 154.558 seconds by pooled medians under a 95 GiB cgroup cap, with approximately 94.69 GiB process peak RSS and no swap or OOM. Those finite checks do not establish minimum physical RAM, phone feasibility for this circuit or performance under arbitrary memory limits. No comparable Aeglos proving time for this large circuit is asserted here.

CPU sampling, setup-time experiments and unadopted prototypes are excluded from the latency tables. The checkpoint does not establish TEE overhead, production tail latency, concurrent throughput or physical-phone performance. Aeglos and Gnark may perform differently with other versions, workloads and preparation policies.

## Reproducing the Zopro request boundary

From the zopro directory on an AVX-512 IFMA-capable Linux build host:

```sh
source docs/benchmarks/2026-10-25-cpu-checkpoint/environment.sh
cargo build --locked --release -p zopro-bench --example cpu-sdk --example cpu-euler
target/release/examples/cpu-euler prepare /path/to/raw.image /path/to/euler.image
RAYON_NUM_THREADS=32 target/release/examples/cpu-sdk /path/to/euler.image tools/zolana/fixtures/transfer-1x2.json 31 /path/to/proof-output
```

Use matching trusted key images and fixtures. The preparation output must not already exist. Image preparation is a separate operation and can require substantial disk and RAM for large keys. For an AWS-targeted build replace `target-cpu=native` with `target-cpu=znver4`. The sample driver uses the same allocator and SDK boundary, excludes three priming requests and exports one final proof after timing. Reproduce the recorded process order and worker matrix for a comparison; a single invocation is a smoke test.

[Measurements](measurements.json), [source hashes and flags](source-sha256.json), [validation record](validation.json), [runtime environment](environment.sh). Keys, binaries and raw profiling captures remain in the local development archive.
