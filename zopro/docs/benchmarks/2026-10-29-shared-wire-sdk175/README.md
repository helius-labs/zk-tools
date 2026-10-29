# Shared witness classification full proof pilot

The prospective speed pilot gate **fails**. Production remains unchanged. The implementation classifies overlapping A/B witness wires once per proof and overlaps that work with product FFT preparation. The earlier operation-count reduction is tested here through the standard SDK, including allocation, remapping and scheduling costs.

Candidate latency is **9.401 ms versus 8.130 ms accepted**, 15.90% slower by the paired estimate, with zero wins in 31 blocks. Maximum lifetime RSS is **99.25 versus 136.75 MiB**, 37.50 MiB or 27.42% lower in this capture. Fresh OFF/accepted calibration passes. The reduced memory observation does not offset this measured latency regression for the speed objective.

## Full proof measurements

Thirty-one fixed rotated/reversed primary blocks use 32 workers on a Ryzen 9 9950X, 16 physical cores and 32 logical CPUs, 128 GiB RAM. Each process measures 151 complete SDK requests after three priming proofs. Timers include fresh witness solving, fresh randomized proving, internal verification and serialization. Loading/preparation is outside latency and inside lifetime process RSS.

|Variant|Primary ms|CPU ms per request|Median peak RSS MiB|Maximum peak RSS MiB|
|---|---:|---:|---:|---:|
|accepted|8.129776|182.753|130.75|136.75|
|OFF|8.077206|181.573|130.00|135.25|
|ON|9.401192|189.313|94.50|99.25|

The accepted executable is the pinned production artifact; OFF and ON are new builds from one private snapshot. Marginal medians and paired estimates need not give identical differences or ratios. Peak RSS is a process lifetime observation, not minimum RAM or incremental proof scratch.

|Paired comparison|Median ratio|Bounds|Wins out of 31|Paired saving µs|
|---|---:|---|---:|---:|
|ON/OFF|1.163638735|[1.159946487, 1.167238766]|0|-1322.042|
|ON/accepted|1.159005632|[1.153296967, 1.159845819]|0|-1287.197|
|OFF/accepted|0.993402985|[0.990222329, 0.997488613]|25|53.692|

The predeclared speed gate requires ON's upper paired ratio below 1 and at least 21 wins against both accepted and OFF. OFF/accepted bounds must fit [1/1.03, 1.03]. Sorted ranks 10–22 give nominal 97.055% individual median coverage under independent/exchangeable blocks; this is not a familywise guarantee. No minimum gain floor, sample extension, exclusion, threshold relaxation or unchanged reroll is used.

Failed checks:

- `1x2-w32/ON/OFF`
- `1x2-w32/ON/accepted`

## Implementation and memory admission

A compact hash table stores representative wire indices, with complete four-word hashing/equality through the existing solved witness. The two validated sorted query index lists are merged without allocating their union. A dense global label vector and one reused local remap restore each query's original scalar order, counts and stable input lists. Existing owned group outputs remain. There is no persistent per-key expansion, point copy, witness reuse, custom pool, new unsafe operation or field arithmetic change.

Scratch admission includes global table growth, both dense vectors and both sample tables, charged in addition to the existing required FFT/proof workspace forecast. Each query also retains its original conservative grouping and G1/G2 consumer reservation. Scratch tokens drop after construction; returned group tokens outlive consumers. Checked-cost or admission failure returns to original per-query grouping. Attempted/declined groups are distinct from this fallback, preventing duplicate hashing. The experiment does not establish pressure behavior or minimum RAM.

The private cfg is `zopro_shared_wire_groups`. Five originals change reversibly; one child is added, for 176 private files with 170 of 175 original files exact. Hashbrown 0.16.1 is a pinned native-only dependency with default features disabled and inline-more enabled. All 17 field source files remain exact; accepted field qualification is inherited without claiming arithmetic binary identity.

The current canonical-wire entry changes 1×2 and 2×2. Larger Montgomery paths remain unchanged and are checked by correctness preflight. Only primary latency is qualified in this pilot; no all-shape candidate gain is claimed. The shared pass is complete before either A/B MSM starts, so a lower total hash count can still delay query readiness.

## Validation and profiles

Both OFF and ON run unfiltered native core/prover suites.

|Suite|OFF passed|ON passed|
|---|---:|---:|
|core|124|126|
|prover|8|8|

Two new tests exercise full metadata equivalence, forced collisions, overlap/disjoint/empty query geometry, zeros and full-word values across 75 synthetic cases; they also cover a full admitted pair, fallback dimensions and checked-cost overflow. Existing curve/grouping oracles run in the unfiltered suite.

All nine shapes × three variants pass 27 preflights, one request after three primes, each with independent Gnark verification before latency. All 93 latency captures precede six FIFO-gated 199 Hz cycles:u profiles: primary 16/32 workers × three variants, 251 requests after three primes. CPU IDs and timestamps are recorded explicitly. All captures finish before remaining Gnark checks, owner-user decoding and lost-record checks. Physical-16 profiles are diagnostic, not latency qualification. No owned CPU benchmark/build/hash/archive/helper work overlaps capture.

All **15,954 SDK requests** verify internally. All **126 fresh 164-byte exports** pass independent Gnark and exact key/public binding checks, with freshness against 21,007 prior proofs. The distinct proof ledger becomes **21,133**. All 276 capture/offline steps complete.

|Profile|Samples|Lost records|
|---|---:|---:|
|cycles-1x2-w16-OFF|5566|0|
|cycles-1x2-w16-ON|5543|0|
|cycles-1x2-w16-accepted|5436|0|
|cycles-1x2-w32-OFF|9005|0|
|cycles-1x2-w32-ON|9207|0|
|cycles-1x2-w32-accepted|9267|0|

Each profile has one benchmark PID and zero lost records. Full flat symbol weights remain in the analysis; they are not exclusive wall-time fractions or a causal explanation.

The initial workflow plan used an unsupported working-directory field and was rejected before any build/test. It was corrected to explicit Cargo manifest paths. The first ON compilation then failed on cfg-only Rust re-export visibility; widening that export fixed compilation without changing the runtime algorithm. The completed OFF steps were retained and the remaining five ON steps ran in a separate workflow. Both initial inputs and failures are archived; successful tests and captures were not repeated.

## Supplied M5 references and current state

Supplied historical primary references are 37 ms M5 CPU and 24 ms M5 Metal. No Apple measurement is repeated.

|Variant|Primary ms|M5 CPU speedup|M5 Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|accepted|8.129776|4.551×|2.952×|3.7|
|OFF|8.077206|4.581×|2.971×|3.7|
|ON|9.401192|3.936×|2.553×|3.7|

All eight accepted-production comparisons follow. They remain separate from this primary-only candidate pilot.

|Shape|Accepted local ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37.0|4.612×|24.0|2.992×|3.7|
|2x2|11.335969|59.0|5.205×|30.0|2.646×|5.9|
|4x2|15.945544|109.0|6.836×|48.0|3.010×|10.9|
|8x2|24.090485|189.0|7.845×|74.0|3.072×|18.9|
|12x2|33.844855|284.0|8.391×|101.0|2.984×|28.4|
|16x2|39.931557|335.0|8.389×|124.0|3.105×|33.5|
|24x2|66.233309|528.0|7.972×|176.0|2.657×|52.8|
|merge-24x1|65.273224|507.0|7.767×|175.0|2.681×|50.7|

Accepted 54→1 remains 162.250464 ms without a supplied M5 reference. The accepted full performance gate remains false because of its documented physical-16 calibration exception; the separate accepted final-artifact gate is true. Neither status is transferred to this candidate. The original all-eight verified CPU 10× goal remains active and unmet, 0/8.

No new AWS/Aeglos comparison or mobile/ARM/TEE, forester 250/4999, active-pressure, swap or concurrent-throughput qualification is produced. The [current summary](../2026-10-29-current-cpu-summary/README.md) retains all local/AWS comparisons.

## Decision

This exact pilot closes without production adoption. Preserve positive point estimates or memory observations within their measured scope; the failed speed gate is unchanged. A materially different implementation or a separately predeclared memory/nonregression qualification can use these findings.

[Design](artifacts/design.md), [full block and profile data](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [capture plan](artifacts/run-plan.json), [source archive](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [proofs and public bindings](artifacts/proofs.json.gz), [closing source pins](artifacts/pins.json).
