# Combined CPU candidate memory and compatibility qualification

The combined borrowed-G1/adaptive-G2/Poseidon candidate passes the declared portable, fallback, small-stack, concurrent-call and constrained-memory cases. Both 4,999-batch proofs independently verify under a **95 GiB cgroup limit with swap disabled**: **166.524 s cold / 164.686 s warm**, with **94.684 GiB peak process RSS**. This establishes feasibility for the tested cases and capacities, not minimum RAM or a new speed comparison.

The [complete latency study](../2026-10-27-composed-cpu-sdk/README.md) retains its primary32 **3.13%** and original-eight aggregate **1.98%** paired gains. Its original full performance gate remains false solely for physical16 control calibration. This resource study does not change that verdict or adopt source. Supplied CPU10× targets remain **0/8**.

## Sources and validation

Native qualification drivers link the exact measured accepted and combined libraries; native arithmetic is not rebuilt. The standard combined SDK retains SHA-256 `8241648f298893953bcacac515d3d52a7f2d363cb264e08d232af3037decb265`. A separate fresh portable target builds the identical combined source with x86-64/noIFMA/noASM flags and passes **11 field, 99 core and eight prover tests** (raw-image two/cubic six), including adaptive retained-capacity/tree boundaries and all-width Poseidon comparisons. Native116/raw2/cubic6/generator qualification is inherited from the exact tested source/library receipts. No new native test execution is claimed here.

Rust xtask runs all cases serially from a protocol frozen before requests. New Rust cgroup/cache controllers replace the earlier qualification's Python runners. Two controller checks cover memory-event parsing and nonfaulting mincore; a separate non-SDK systemd true smoke checks the memory controller. One build-plan-helper digest-format compilation failure preceded all native links and portable builds; the original helper/type note and repair are retained. No prover build/test/request failure or successful-process repeat occurred. All 170 production source files remain unchanged.

## Functional and concurrent cases

All **37 functional processes** pass: nine portable standard shapes; portable raw/preparation-denied primary and54; native1/4 workers × requested512KiB/1MiB worker stacks × primary/54 × basis/raw/preparation-denied. Each performs three primes plus one fresh verified SDK request and independently verifies its final export. Worker stacks are requested through RUST_MIN_STACK; no stack watermark is measured. Raw images and preparation limit zero exercise the declared fallback configurations.

Portable standard-image diagnostics below are one request after three primes at four workers. They are correctness/resource observations without latency confidence intervals or acceptance claims. RSS includes loading and primes.

|Shape|Observed request ms|Process peak RSS MiB|Independent proof|
|---|---:|---:|---|
|1x2|158.754|33.29|pass|
|2x2|247.414|54.88|pass|
|4x2|434.999|100.39|pass|
|8x2|796.986|196.96|pass|
|12x2|1135.891|290.79|pass|
|16x2|1384.221|356.46|pass|
|24x2|2085.016|564.05|pass|
|merge-24x1|1959.927|544.12|pass|
|merge-54x1|4255.641|1167.51|pass|

Four separate concurrent processes compare exact accepted/combined libraries for primary and54. Four callers share one immutable mapped image and the existing32-worker pool, two requests each; all32 exports independently verify. Assertions observe32 workers+four callers+main while active and32+main afterward, without a hidden pool. These are concurrency correctness checks, not throughput or scheduling-fairness qualification.

|Variant|Shape|Callers / proofs|Process peak RSS MiB|
|---|---|---:|---:|
|accepted|1x2|4 / 8|122.00|
|accepted|54→1|4 / 8|3204.79|
|combined|1x2|4 / 8|119.75|
|combined|54→1|4 / 8|3169.46|

## Primary memory limits

Cold cases start entirely under192MiB with swap disabled. The controller copies the owned primary image using a bounded buffer inside the cgroup so its new file pages are charged there. Both allocator policies perform three primes plus one request. Warm cases start at512MiB, prime three times, retain/touch32MiB ballast in the same cgroup, then lower MemoryHigh to128MiB and MemoryMax to192MiB before two further independently verified requests. Every case exits normally with exact declared limits and no max/oom/oom_kill events.

|Case|Allocator|Cgroup lifetime peak MiB|High / max events|Independent exports|
|---|---|---:|---:|---:|
|cold / accepted|default|60.69|0 / 0|1|
|cold / accepted|retained|63.45|0 / 0|1|
|cold / composed|default|61.83|0 / 0|1|
|cold / composed|retained|61.66|0 / 0|1|
|warm / accepted|default|98.55|0 / 0|2|
|warm / accepted|retained|104.46|0 / 0|2|
|warm / composed|default|100.89|0 / 0|2|
|warm / composed|retained|100.64|0 / 0|2|

These small cases remain below the128MiB soft limit: **they do not trigger high-limit reclamation**. They establish the requested limit transitions and successful bounded launches, not active reclaim stress at that size. The large4999 case below supplies actual hard-limit reclaim evidence. Peaks include controller, copied image pages, SDK and warm ballast; they are not comparable to standalone unconstrained process RSS. Resource scheduling also sees the cgroup limit. Do not infer a universal memory saving or minimum RAM from these values.

## Forester250 and4999

Original pinned solver images are6,451,333,588 and124,707,841,608 bytes. Before each launch, a bounded streaming hash checks the image, then fsync/POSIX_FADV_DONTNEED acts only on that owned image. A PROT_NONE mincore mapping confirms zero resident pages without faulting them in. No global cache flush or key/image conversion occurs. One cold and one warm core-prover call use different original witnesses; both public bindings and proofs independently verify.

|Batch|Load s|Cold proof s|Warm proof s|Process peak RSS GiB|Cgroup peak GiB|High / max events|OOM / kill|
|---|---:|---:|---:|---:|---:|---:|---:|
|250|4.431|3.177117|3.335854|11.233|11.277|0 / 0|0 / 0|
|4999|29.924|166.523904|164.686399|94.684|95.000|128 / 128010|0 / 0|

Both use95GiB hard limits and memory.swap.max=0; recorded swap peak is zero. For4999 the cgroup reaches its cap and records128,010 max events and128 high events, while both proofs complete without OOM or kill. Those event counters are not page counts or exclusive wall-time costs. Process RSS and cgroup accounting cover different memory. These are single cold/warm observations, not matched accepted/Aeglos/Gnark comparisons; they do not establish a large-circuit speed improvement. A lower feasible capacity was not searched.

## Counts, integrity and remaining work

All **216 SDK calls plus four forester core calls** complete. All **85 distinct fresh exports** pass independent Gnark verification,164-byte proof length, exact public binding and freshness against the checked9,069 prior identities: ledger **9,069→9,154**,170 proof/public members. No intermediate inventory is counted twice. All640 input pins are checked by the workflow and frozen finish helper, including bounded streaming hashes of the large images before/after execution. Closing checks rehash source/driver/binary inputs and all proof/public members; no extra giant-image hash pass is needed. All owned cgroup services are stopped and their runtime property overrides removed. No cloud machine is created.

The retained-capacity/forecast argument is in the [combined implementation report](../2026-10-27-composed-cpu-sdk/README.md#implementation-and-memory). These runtime cases complement that argument; they do not prove transient allocation bounds for every input, physical phone/ARM, TEE,193GiB key behavior, giant concurrent proofs or minimum RAM. There is no new lookup table, key copy or persistent solver plan in the implementation. Owned test image copies belong only to the cgroup harness.

The candidate now has positive complete-proof evidence and the declared resource/compatibility checks. Review adoption with the physical16 control-calibration limitation explicitly recorded; do not discard its gains, silently rewrite the failed frozen gate, or repeat unchanged acceptance timing. The all-shape historical M5 CPU/Metal comparisons remain in the [complete latency report](../2026-10-27-composed-cpu-sdk/README.md#supplied-m5-references); no new Apple/AWS/Aeglos measurements are claimed. The all-eight CPU10× goal remains active and unmet.

[Design](artifacts/design.md), [qualification analysis](artifacts/analysis.json), [per-case jobs](artifacts/jobs.json), [resource results](artifacts/resource-result.json), [portable tests](artifacts/portable/core/tests.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).
