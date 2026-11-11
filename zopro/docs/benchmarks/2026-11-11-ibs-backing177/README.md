# Same-byte SDK backing: matched operation and fetch profiles

The retained slow OFF copy again takes longer than its two byte-identical peers.
Its G2 op-cache miss fraction is higher, while sampled operation waits and
completed fetch latency are lower. This makes instruction delivery a concrete
lead for the backing-associated slowdown. It does not establish a cache-index
mechanism, an exclusive bottleneck or a production optimization.

All 18 profile-quality checks and six observer checks pass. All 27 fresh exports
independently pass Gnark verification. No candidate is adopted and no earlier
performance gate is reopened.

## Matched results

Ryzen 9950X, 32 fixed workers on CPUs 0–31. Three blocks include plain, operation
IBS and fetch IBS processes for each of the three predetermined OFF copies from
the [G1-only study](../2026-11-11-g1-offset177/README.md). Each process measures
61 complete SDK requests after three primes. Values below are medians of three
process metrics, with no confidence interval or adoption claim.

| OFF copy | Plain proof ms | CPU ms/proof | Plain lifetime peak MiB |
| --- | ---: | ---: | ---: |
| 0 | 7.7122 | 175.50 | 117.50 |
| 1 | 8.7256 | 194.24 | 119.50 |
| 2 | 7.7410 | 176.35 | 116.25 |

Lifetime peak includes loading and priming. This shorter diagnostic is not a
replacement for the accepted full-screen latency or memory comparison. The
maximum of three peaks is not minimum required RAM. All plain major-fault
counters are zero; minor faults remain recorded in the full result.

| G2 borrowed apply observation | Copy 0 | Copy 1 | Copy 2 |
| --- | ---: | ---: | ---: |
| Retired-operation samples/process | 31,403 | 31,451 | 31,346 |
| Tag to completion, mean cycles | 39.22 | 32.15 | 39.37 |
| Completion to retirement, mean cycles | 23.93 | 18.81 | 23.97 |
| L1 misses / load operations | 10.29% | 10.31% | 10.49% |
| L2 misses / load operations | 0.70% | 0.81% | 0.82% |
| RAM demand samples, all three profiles | 0 | 0 | 1 |
| Fetch samples/process | 3,125 | 3,077 | 3,083 |
| Completed fetch fraction | 99.81% | 99.81% | 99.74% |
| Completed fetch latency, mean cycles | 1.824 | 1.068 | 1.824 |
| Instruction-cache miss fraction | 11.86% | 6.47% | 11.77% |
| Op-cache miss fraction | 41.95% | 59.28% | 41.26% |

G1 provides a useful comparison: its tag-to-completion means stay near
46.13–46.20 cycles and op-cache miss fractions near 7.60–8.60%. The G2 difference
is also visible within IFMA instruction classes, not only in the aggregate mix.
One G2 operation and one G1 operation across the entire campaign have imprecise
addresses and are excluded by the inherited latency logic; neither group has
invalid or saturated latency pairs. Full per-block and instruction-class data
are retained.

These means include queueing, operand availability and other effects. Lower
operation waits in the slower copy can be consistent with a less well-fed
execution pipeline; they do not mean the arithmetic executes intrinsically
faster. Completed fetch latency excludes decode and is not total frontend delay.
Miss fractions from sampled fetches cannot be converted into removable wall time.
Selection bias and speculation remain limitations of IBS.

## Page identities and qualification

Read-only censuses run before the first SDK and after the last. All 11,382
file-offset page identities across the nine retained REF/OFF/ON files match
between endpoints. Hashes and device/inode/mode identities also match. Each
census independently checks same-inode alias mappings. There is no page eviction,
migration, locking, executable rewrite or newly chosen fast copy.

Endpoint equality does not prove the absence of intermediate migration and does
not inspect SDK private/COW data pages. It does show that the retained backing
frames did not differ at these two observations.

Operation and fetch periods are both 524,288. FIFO controls isolate warm request
loops. Every raw/script identity join and register cross-check passes, with zero
lost records, one expected user PID, checked executable MMAP and fixed worker
affinity. All exact SDK operation lookups resolve. The total is 1,704,369 IBS
samples. Median instrumented/plain latency factors are 1.0211–1.0294 for operation
sampling and 1.0007–1.0081 for fetch sampling, inside the declared factor-1.05
guard. This guard supports interpretation without proving observer equivalence.

There are 1,728 SDK calls including primes, 1,647 timed samples and 27 distinct
164-byte exports. Proof/public/image bindings, verifier receipts and freshness
pass; the canonical ledger advances **52,945 → 52,972**. All capture precedes
verification and decoding; no owned work overlaps a recording.

## Interpretation and next experiment

AMD's Zen 5 optimization guide describes a physically tagged op cache, restricted
transitions from instruction-cache mode, and different delivery rates for decoded
versus cached instructions. The retained primary document is
`/home/ubuntu/src/zopro-perf-work/vnni13-screen175/sources/58455_1.00.pdf`,
section 2.9.1, SHA256
`d32a1c0166578fc8008ecdfe0af5aa1b93d185e9999ac54897fdea5da9f2b7fc`.
Those properties make the measured pattern plausible; they do not supply the
unknown physical indexing function or prove it causes this particular difference.

The next source investigation should target G2's instruction working set and
long straight-line regions, preserving arithmetic and bounded scratch. Inspect
the exact linked code before choosing the change. This should not repeat the
rejected [fused-product outlining](../2026-10-29-g2-fused-outline175/README.md) or
[CIOS outer-loop](../2026-11-07-cios-outer-loop176/README.md) implementations, nor
choose page colors or the fastest executable copy. A distinct implementation
needs correctness, full-proof timing and memory evidence before adoption.

The accepted full-screen local 1x2 remains **7.800 ms / 131.75 MiB peak**. The
[latest complete CPU/Metal comparison](../2026-11-11-g1-offset177/README.md)
retains all eight supplied shapes; the CPU 10× goal is still unmet for all eight.
No AWS, fresh Aeglos, mobile, huge-circuit or pressure measurement was added.

## Evidence and recovery

[Summary](artifacts/summary.json), [protocol](artifacts/protocol.md), frozen plans,
Rust helpers, all SDK/verifier logs and exported proofs are saved locally. The
compressed complete result preserves every profile group and resource row.
Raw perf data, multi-gigabyte decoded streams, physical addresses and disassembly
remain under `/home/ubuntu/src/zopro-perf-work/ibs-backing177/`, with hashes in
the archived receipts and result.

Capture completed once. Offline decoding initially stopped because files had
been transferred to ubuntu while perf was still invoked as root. A separate
offline plan reads them as their owning user. The initial closeout reader also
expected one mapping for the file; IBS records writable/read-only mappings too.
The corrected reader selects the executable mapping explicitly. Original plans,
sources and failures remain retained. No capture, successful verifier, sample,
threshold or decoder arithmetic was changed or repeated.

Production remains unchanged. All work is committed locally; no push or registry
publication is approved.
