# Input-pack locality in complete G2 queries

Only **6.61%** of primary G2 eight-point groups are consecutive and aligned to an eight-point source block. The corresponding grouped 2×2 value is **6.94%**. A representation change cannot assume that ordinary streaming batches map directly to one contiguous block.

Exactly two source blocks cover **80.68% / 81.26%** of groups. Including single-block groups, the totals are **87.29% / 88.20%**. This identifies a concrete next question: can two block loads plus a lane permutation beat the current input transpose when irregular groups, routing, radix conversion, and grouped-point preparation are included? **No native fast path, key-format change, timing gain, or adoption is established.**

## Observation and correctness

The diagnostic observes the accepted native borrowed-G2 caller during four complete queries: two real retained point/scalar populations at one and 32 workers. Every output matches its existing pinned independent Arkworks complete-MSM oracle; all point inputs remain unchanged. Normalized private source-index transcripts are byte-identical across worker counts for each population. These are two distinct input populations observed twice, not four independent witness populations or proof verifications.

The observer records source ordinals at the existing batch entry, under a scoped root slice established by the original Phi2 window/part caller. Every ordinal must belong to that exact live root and original part. Component labels follow the original high/low feeds. Grouping, scalar decomposition, point arithmetic, queue order, batch sizes, and scheduling rules remain original.

| Fixture | Original input points | Actual caller root points | Root population | Ordinary batches | Full eight-point groups | Affine pairs | Scalar-tail pairs |
|---|---:|---:|---|---:|---:|---:|---:|
| 1×2 B2 | 30,089 | 30,089 | original input array | 2,605 | 64,541 | 516,476 | 148 |
| 2×2 B2 | 52,274 | 28,852 | proof-local grouped array | 2,688 | 66,884 | 535,224 | 152 |

Each query has 22 scoped window/part records. The two narrow top-window records have no borrowed batches; the remaining 20 contain the ordinary caller observations. Scalar tails are separate from complete eight-entry groups. Primary's 2,605 batches and 516,476 affine pairs agree with the earlier independent operation-census totals. These are logical operation counts, not cycles or physical traffic.

## Input classes

The following four classes are disjoint and cover every full group. A source block is the logical interval `[8k,8k+8)`, relative to the actual root array. It is not an observed cache line or virtual-address alignment.

| Class | Primary groups | Primary share | Grouped 2×2 groups | Grouped 2×2 share |
|---|---:|---:|---:|---:|
| Aligned consecutive | 4,263 | 6.6051% | 4,645 | 6.9449% |
| Consecutive, crossing two blocks | 30,349 | 47.0228% | 32,536 | 48.6454% |
| Single-block permutation | 3 | 0.0046% | 0 | 0.0000% |
| General gather | 29,926 | 46.3674% | 29,703 | 44.4097% |

| Distinct source blocks per full group | Primary count | Grouped 2×2 count |
|---|---:|---:|
| 1 | 4,266 | 4,645 |
| 2 | 52,072 | 54,347 |
| 3 | 3,324 | 3,045 |
| 4 | 932 | 933 |
| 5 | 1,006 | 996 |
| 6 | 1,294 | 1,244 |
| 7 | 1,162 | 1,188 |
| 8 | 485 | 486 |

The block histogram overlaps the class view: many general-gather groups still use only two blocks. Overall consecutive groups account for 53.63% / 55.59%. Consecutive edges are 384,046 of 451,787 and 401,505 of 468,188 possible edges. No scalar-tail entry is padded into these full-group counts, and no unfavorable class is excluded.

## Representation decision and limits

The unimplemented blocked layout would store each of the sixteen coordinate words of eight points contiguously. Its point payload remains 128 bytes per point, with at most seven padding points (896 bytes) per separately stored query. This is a storage formula, not a converted image, measured allocation, or minimum-RAM claim. It requires neither an expanded radix-52 key nor a second retained point table.

A one-block direct-load shortcut alone is uncommon on these fixtures. A worthwhile native screen must cover two-block permutation and the remaining irregular gathers, including dispatch and radix conversion. It must preserve original input order and complete caller behavior rather than benchmark only the favorable contiguous class. The source-block counts do not establish fewer instructions, loads, cache misses, or proof latency.

The 2×2 root is formed by per-proof scalar grouping. A fixed proving-key transpose does not automatically transpose that witness-dependent array. A complete design must emit grouped points into the new representation without retaining a duplicate, or charge conversion and lifetime overlap. Loader/image compatibility, arbitrary point APIs, portable access, and GPU paths also require design. No key-format rewrite follows from this census alone.

The observer allocates private index transcripts and scoped/result metadata. The four saved JSON transcripts total 11,889,930 bytes on disk; duplicate worker-count observations retain identical content. This is diagnostic storage, not proposed prover workspace or a measured memory peak. Input-dependent ordinal lists remain private; published summaries contain counts and hashes. Reproducing the independent classification needs those local files or regenerated traces from authorized fixtures.

## Validation and provenance

Two new focused tests pass once. They check exact ordinal/component capture, nested scope restoration, intentional panic cleanup, and parallel/reused scopes in 1/4/16/32-worker pools. The other 125 core tests are filtered and inherited; field and prover suites are not rerun. All 17 field source files stay exact. The 177-file snapshot has 172 byte-exact originals and four reversible diagnostic parent edits; all 176 original hashes recover.

Coherent native libraries and proc macros are frozen. The actual direct-link receipt includes `-O`, `target-cpu=native`, `lto=fat`, and `codegen-units=1`. Each real query uses 512 KiB worker stacks, CPU 2 at one worker and CPUs 0–31 at 32 workers, with individual affinity receipts and a post-query check. There are no primes, latency clocks, PMU panels, CPU profiles, SDK requests, or new proofs.

Independent Rust review imports no observer or classifier code. It checks **259 pins and eight successful workflow commands**, protocols and stream hashes, source recovery, the two focused tests, optimized links, worker identity, fixture/oracle binding, and every private ordinal. A separate set-based classifier reproduces every per-component class, histogram, edge, tail, and aggregate count: 262,850 full groups and 2,103,400 affine pairs across all four runs. Cross-worker transcript hashes match exactly. Review succeeds on its first execution; no successful test, query, or capture is repeated.

Production and the accepted SDK remain unchanged. The canonical proof ledger remains 29,953. No new proof/Gnark, AWS/Aeglos, phone/ARM/TEE, huge-circuit, pressure/swap, minimum-RAM, or concurrent-throughput qualification is claimed.

## Accepted CPU and Metal comparisons, inherited

These are accepted 32-worker local full-proof results and supplied historical M5 references, not timings of this diagnostic. [Accepted source manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json). Different machines do not establish equal hardware efficiency. Latest separate full-SDK evidence remains [finalization/drain composition](../2026-11-05-finalize-drain-bundle176/README.md), retaining its primary gain and failed full gate.

| Shape | Accepted local ms | M5 CPU ms | CPU/local | M5 Metal ms | Metal/local | CPU10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1×2 | 7.928660 | 37 | 4.667× | 24 | 3.027× | 3.7 |
| 2×2 | 11.151932 | 59 | 5.291× | 30 | 2.690× | 5.9 |
| 4×2 | 16.086386 | 109 | 6.776× | 48 | 2.984× | 10.9 |
| 8×2 | 24.179453 | 189 | 7.817× | 74 | 3.060× | 18.9 |
| 12×2 | 33.884455 | 284 | 8.381× | 101 | 2.981× | 28.4 |
| 16×2 | 40.077381 | 335 | 8.359× | 124 | 3.094× | 33.5 |
| 24×2 | 65.850777 | 528 | 8.018× | 176 | 2.673× | 52.8 |
| merge24×1 | 65.060937 | 507 | 7.793× | 175 | 2.690× | 50.7 |

Merge54×1 remains 161.883899 ms without a supplied M5 reference. The all-eight CPU10× goal remains active and unmet, **0/8**.

## Artifacts

[Design](artifacts/design.md), [all summaries](artifacts/analysis.json), [independent review](artifacts/review.json), [observer](artifacts/locality.rs), [source edits](artifacts/observer-edits.json), [driver and classifier](artifacts/driver.rs), [reproduction](artifacts/reproduce.md), [source archive](artifacts/source.tar.gz), [exact receipts](artifacts/receipts.json.gz), [checksums](SHA256SUMS).

The source archive is extracted and checked against all 177 member hashes. The gzip JSON receipt archive preserves exact UTF-8 member bytes, lengths, and SHA256s. Executables, libraries, private input-dependent ordinal transcripts, point/scalar/key/image/witness arrays, and raw perf data are excluded.
