# Clean FFT compatibility and memory qualification

The exact clean FFT candidate passes the declared portable, fallback, small-stack, concurrent-call and constrained-memory cases. Forester 250 and 4999 complete under a 95 GiB hard limit with swap disabled. These observations establish feasibility in the tested scenarios; they do not establish minimum RAM or a matched speed comparison. This resource study does not adopt source.

The [latency study](../2026-10-27-clean-fft-leaves/README.md) retains its failed full gate: uncertain primary improvement and an 8×2 candidate/fresh nonregression upper bound above 1.02. All calibration and aggregate checks pass, with original-eight paired gains of 0.856% and 0.419% against accepted and fresh controls. Adoption requires a separate engineering decision accepting the remaining uncertainty. The all-eight M5 CPU 10× goal remains unmet, 0/8.

## Sources and validation

Native resource drivers link the measured clean FFT libraries (SDK SHA-256 `14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a`) and accepted libraries (SDK prefix `824164`). Native arithmetic is not rebuilt. The legacy resource label `composed` denotes the clean FFT candidate. A fresh portable x86-64 build without IFMA or assembly passes 11 field, 99 core and eight prover tests. The native study's 116 fresh / 119 candidate core tests and two raw / six cubic tests are inherited receipts.

The portable build initially failed at its final SDK link because the source snapshot excludes the standard CLI files. A link-only repair used the pinned production CLI and the already-built libraries. Successful suites and builds were not repeated. The original failure, repair and earlier helper preparation failures are retained in the archive; no qualification request had started when they occurred.

Rust xtask runs frozen cases serially. Two controller checks cover memory-event parsing and a nonfaulting mincore residency probe. Every request, verifier, source/library/input pin and lifecycle receipt remains in the archive. No unrelated owned workload overlaps qualification requests. All 171 accepted production files remain unchanged. Physical phones, ARM and TEE environments are untested.

## Functional and concurrent cases

All 37 functional processes pass: nine portable standard shapes; portable raw and preparation-denied primary/54 cases; and native combinations of one/four workers, 512 KiB/1 MiB requested worker stacks, primary/54, and basis/raw/preparation-denied inputs. Each process makes three primes and one fresh internally verified request, then independently verifies its final export. `RUST_MIN_STACK` requests a stack size; no watermark is measured. Raw and denied cases exercise the existing fallback.

Portable standard-image diagnostics below are one request after three primes at four workers. They are correctness/resource observations without acceptance confidence intervals; RSS includes loading and primes.

|Shape|Observed request ms|Process peak RSS MiB|Independent proof|
|---|---:|---:|---|
|1x2|175.818|33.00|pass|
|2x2|269.207|55.38|pass|
|4x2|454.006|100.70|pass|
|8x2|810.890|197.11|pass|
|12x2|1134.411|292.23|pass|
|16x2|1413.787|354.99|pass|
|24x2|2080.929|563.27|pass|
|merge-24x1|1999.612|543.52|pass|
|merge-54x1|4236.431|1170.14|pass|

Four concurrent processes compare accepted and clean libraries for primary and 54. Four callers share one immutable mapped image and the existing 32-worker pool, making two requests each; all 32 exports independently verify. Assertions observe 32 workers, four callers and main while active, then 32 workers and main afterward. These checks establish concurrency correctness, not throughput or scheduling fairness.

|Variant|Shape|Callers / proofs|Process peak RSS MiB|
|---|---|---:|---:|
|accepted|1x2|4 / 8|121.50|
|accepted|54→1|4 / 8|3412.02|
|clean|1x2|4 / 8|118.75|
|clean|54→1|4 / 8|3317.62|

## Primary memory limits

Cold cases start entirely under 192 MiB with swap disabled. A bounded copy of the owned primary image inside the cgroup charges its new file pages there. Both allocator policies perform three primes and one request. Warm cases start at 512 MiB, prime three times, retain and touch 32 MiB of ballast, then lower `MemoryHigh` to 128 MiB and `MemoryMax` to 192 MiB before two further requests. Every case exits normally with exact limits and zero max/OOM/kill events.

|Case|Allocator|Cgroup lifetime peak MiB|High / max events|Independent exports|
|---|---|---:|---:|---:|
|cold / accepted|default|63.11|0 / 0|1|
|cold / accepted|retained|62.01|0 / 0|1|
|cold / composed|default|60.55|0 / 0|1|
|cold / composed|retained|62.64|0 / 0|1|
|warm / accepted|default|101.54|0 / 0|2|
|warm / accepted|retained|103.34|0 / 0|2|
|warm / composed|default|104.77|0 / 0|2|
|warm / composed|retained|104.20|0 / 0|2|

Small-case high-limit events total zero: the soft threshold did not trigger reclaim. Cgroup peaks include the child controller, copied image pages, SDK and warm ballast; they differ from standalone RSS and do not establish minimum RAM.

## Forester 250 and 4999

The original pinned solver images are unchanged. Before each launch, a bounded streaming hash verifies the image, then fsync/POSIX_FADV_DONTNEED acts only on that owned image. A PROT_NONE mincore probe confirms zero residency without faulting pages. No global cache drop, key conversion, image copy or larger retry occurs. One cold and one warm core-prover call use different original witnesses; proofs and exact public bindings independently verify.

|Batch|Load s|Cold proof s|Warm proof s|Process peak RSS GiB|Cgroup peak GiB|High / max events|OOM / kill|
|---|---:|---:|---:|---:|---:|---:|---:|
|250|4.200|3.203698|2.994999|11.849|11.895|0 / 0|0 / 0|
|4999|26.131|160.741298|149.119809|94.692|95.000|105 / 114373|0 / 0|

Both large cases use 95 GiB hard limits and `memory.swap.max=0`, with zero cgroup swap usage. Max/high event counters are not page counts or removable wall time. These observations do not establish a large-circuit speed gain or minimum RAM; no matched accepted/Aeglos/Gnark timing or lower-cap search occurs.

## Counts and scope

All 216 SDK calls and four Forester core calls complete. All 85 distinct new exports independently verify, with 164-byte proofs, exact public bindings and freshness against 11,575 prior identities. The ledger reaches 11,660, with 170 new proof/public members. Copies and reverification are not counted twice. Frozen helpers stream image hashes before and after execution; closing rehashes remaining inputs and exports. Owned cgroup services and runtime overrides are removed; no cloud resource is created.

The [integration review](../2026-10-27-clean-fft-leaves/artifacts/integration-review.md) describes unchanged allocation geometry. These scenarios do not establish universal memory bounds, physical phone/ARM/TEE behavior, 193 GiB key feasibility or giant concurrent-proof feasibility. No lookup table, key copy or duplicate execution plan is added to the prover. Small image copies belong to the qualification harness.

The parent full performance gate remains false. A separate engineering decision may weigh aggregate and larger-case benefits against primary and 8×2 uncertainty. This report does not adopt code or claim new AWS/M5/Aeglos performance. The all-eight CPU 10× goal remains active and unmet, 0/8.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [jobs](artifacts/jobs.json), [resources](artifacts/resource-result.json), [portable tests](artifacts/portable/core/tests.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).
