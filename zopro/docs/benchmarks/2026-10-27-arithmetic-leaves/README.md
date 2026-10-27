# MSM arithmetic leaves and input availability

At 32 workers, K/H finished last in seven of nine saved requests, while its windows started within 5.06 µs of their inputs becoming available. This points toward investigating service cost rather than assuming late window launch. These observations describe the instrumented schedule and do not establish the cause of uninstrumented latency.

The corrected recorder produced **18 complete enabled graphs** and **one incomplete preflight graph**. Its 32-worker calibration and observer-overhead checks pass individually, with 4.63% paired overhead. The 16-worker checks fail, so the declared qualification requiring both worker counts **fails**. No proving optimization is adopted and accepted latency remains unchanged.

The diagnostic records elapsed spans at whole queries, kernel input availability, windows and parts. Serial scopes also record thread CPU at fills, streaming batches, inversions and reduction tiles. Explicit IDs cross nested Rayon closures, including top-window joins. Whole-query boundaries keep grouping from inheriting a waiting sibling's context. Serial CPU includes system and recorder work; it is not a field-instruction count. Nested serial CPU is subtracted to avoid double counting. Window callbacks remain wall-only because they can help other jobs while waiting.

A/KH input availability is their kernel entry. G2 window inputs become available after scalar decomposition joins. These markers are not enqueue timestamps: Rayon creates jobs lazily. Uncovered preprocessing, scattering, folding, assembly and verification remain outside the recorded serial service.

## Overhead and calibration

Primary1x2, Ryzen9950X,16workers/CPUs0–15 and32workers/CPUs0–31. Three rotated/reversed fresh-process blocks compare the exact accepted SDK with recorder OFF/ON in one new linked SDK. Each process has31timed requests after3primes. Both recorder modes reset/snapshot outside each request timer. The custom driver and outside-timer work can change context; OFF/accepted checks assess that difference.

|Workers|Accepted ms|OFF ms|ON ms|ON/OFF paired|ON CPU ms/request|ON lifetime RSS MiB|
|---|---:|---:|---:|---:|---:|---:|
|16|11.069737|10.760374|11.380123|1.163105|138.293|111.75|
|32|8.622752|8.612473|9.004663|1.046278|215.288|139.00|

Calibration failures: `["w16b2 OFF/accepted=0.7399864461709272"]`.

Observer-overhead failures: `["w16 ON/OFF=1.1631050045244302"]`.

Calibration uses factor 1.05 per OFF/accepted block and factor 1.03 for its paired median. ON/OFF geometric overhead must be at most 10% at both counts. Failure means the combined low-overhead qualification does not hold; trace results remain descriptions of the instrumented schedule. No samples are excluded and no thresholds are changed. A separate corrected series follows the attribution failure documented below; the initial series remains archived. Small optimization gains have no minimum adoption floor; this diagnostic is not an optimization.

## Covered serial work

Medians below use only complete fixed-position graphs (requests0,15,30 in each ON process). Each request contributes its own query/window boundaries and recorded serial CPU. Last-query counts identify which recorded query finishes last under instrumentation, not a removable critical-path duration.

|Workers|Query|Complete graphs|Last-query count|Query elapsed ms|Covered serial CPU ms|Covered inverse CPU ms|Pre-kernel elapsed ms|Median available-to-window-start ms|Maximum available-to-start ms|
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
|16|a|9|0|8.357201|17.330745|2.570609|0.504541|3.477984|9.112746|
|16|kh|9|5|8.408528|45.872318|5.769150|0.000000|0.002440|7.362726|
|16|g2|9|4|8.434888|48.203592|5.498492|0.918292|3.615048|11.075659|
|32|a|9|0|6.042896|24.026409|3.502944|0.777536|0.140125|2.513150|
|32|kh|9|7|6.022088|63.188863|7.737853|0.000000|0.002705|0.005060|
|32|g2|9|2|6.030132|64.219378|7.155295|1.543110|0.729746|2.509603|

Summed serial CPU can exceed elapsed time because workers run concurrently. It includes bookkeeping, memory access, allocation already in the original arithmetic bodies, and observer cost. Available-to-start includes all work between mathematical availability and callback execution and does not isolate executor dispatch. Neither sum is a predicted proof speedup. Full per-window values and exclusive CPU categories are in [graphs](artifacts/graphs.json); all request timings, resources and event/drop counts are in [rows](artifacts/rows.json). Incomplete graphs remain reported and their timings are retained; they do not support complete-graph attribution.

## Validation, memory and provenance

All 111 native core tests, two raw tests, six cubic tests, generator checks and portable/no-IFMA consumers pass. Two recorder tests cover explicit parent boundaries, serial CPU clocks, disabled/unwind behavior, and bounded capacity/drop reporting. Existing field qualification is inherited unchanged. A first native build completed before capture; source review then added whole-query boundaries before a fresh validated build. That superseded, unmeasured version is retained with [correction notes](artifacts/pre-capture-correction.md).

The initial measured series completed 652 SDK requests and 28 independently verified exports, but its decoder failed because the A-query hook covered `msm` while the canonical-wire path calls `msm_canonical`. Its query attribution is invalid. All initial timings, event records, proof exports and failure logs remain in [the initial capture](artifacts/initial-capture/initial-analysis.json). A separate source overlay moved that hook to the actual entry point, rebuilt the diagnostic and checked query/kernel parent relationships in a semantic preflight before the corrected matrix. The tables above use that corrected series only; the correction does not erase or qualify the initial failure.

Six original files contain only recorded diagnostic insertions/wrappers; reversing them recovers every original byte. The other164originals and all170production files remain unchanged. One new cfg-only module records metadata. No arithmetic, scheduling, field primitive, table, second plan or pool is added.

The recorder allocates8192events of80bytes per worker/caller before primes and retains capacity between requests. At32workers this is at most20.625MiB plus Vec metadata. Snapshots outside request timing copy recorded events; only fixed early/middle/late graphs are retained. There is no universal product RAM cap or default tracing allocation. Peak RSS includes key load, primes, recorder and snapshots and is not minimum RAM. Overflow preserves proof execution and reports dropped counts; no buffer is increased to obtain a complete graph. Raw event files remain local and hashed; gzip copies are archived after all captures. The decoder checks record sizes, IDs, parents, containment, worker identities and serial CPU nesting.

In each series, all-nine OFF plus primary ON four-worker preflight contributes 40 SDK requests and 10 exports. The primary matrix contributes 612 SDK requests and 18 exports. Across both series, the total is **1,304 SDK requests**, **56 fresh independently verified Gnark exports** and **112 proof/public members**. The proof ledger advances from **7,177 to 7,233**. Every export passes the 164-byte length check, pinned public binding, freshness against previous and newly exported identities, and independent verification. The [combined inventory](artifacts/combined-proof-inventory.json) includes both series; the individual inventories must not be added again. No helper, build, hash, archive or verifier overlaps timed captures.

The four-worker corrected preflight overflowed its fixed event capacity; all 18 saved main-matrix graphs at 16/32 workers are complete with zero dropped events. Its proof and timings remain valid, but that preflight graph cannot support complete attribution.

The [last complete SDK matrix](../2026-10-26-pairing-x-chain-sdk/README.md) retains all supplied M5CPU/Metal comparisons. No new all-shape performance qualification, AWS/Aeglos, phone, TEE, giant-circuit, swap, minimum-RAM, pressure, concurrency or throughput qualification is claimed. The all-eightCPU10× objective remains active and unmet. The earlier37µs fusion signal and failed qualification remain unchanged.

The next arithmetic hypothesis needs to address actual service work without repeating the previously measured [window-local serial reduction](../2026-10-22-universal-window-local-weighted-sdk/README.md) or [weighted task-grain](../2026-10-22-universal-weighted-task-grain/README.md) variants. The trace alone does not justify another scheduling sweep, and recorded inverse CPU is not a removable wall-time fraction.

Executable: `/tmp/zopro-arithmetic-leaves-corrected/native/trace-sdk`; SHA-256 `a8a7823e396081ce6036f7756deebea249f0afe3c3ee73e1a597354644087ef8`.

[Design](artifacts/design.md), [serial-scope audit](artifacts/serial-scopes.md), [source audit](artifacts/source-audit.json), [native tests](artifacts/native/core/tests.json), [proof inventory](artifacts/proof-inventory.json), [raw local hashes](artifacts/raw-local-sha256.json), [artifact hashes](artifact-index.json).
