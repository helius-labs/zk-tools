# Borrowed signed GLV in the fused K and H prover

The declared primary speed pilot **fails**. Production remains unchanged. The candidate uses ten balanced signed-component windows and two point partitions inside the existing borrowed fused K/H stream. This changes the work and task graph, with no transformed key-point table.

ON measures **8.481 ms versus 8.140 ms accepted and 8.037 ms OFF**. Its paired regression is **4.27% versus accepted**, with zero wins in 31 blocks against either control. Fresh OFF/accepted calibration passes. CPU time rises from 182.861 to 193.316 ms per request; maximum lifetime RSS is 137.00 versus 133.75 MiB accepted. Fewer weighted-window reductions did not produce a faster proof in this implementation.

## Complete SDK results

Ryzen 9 9950X, 16 physical cores and 32 logical CPUs, 128 GiB RAM. Thirty-one fixed rotated/reversed primary blocks compare the exact accepted executable, fresh OFF and ON at 32 workers on CPUs 0–31. Each process measures 151 complete requests after three priming proofs. Timing includes fresh witness solving, fresh randomized proving, SDK verification and serialization. Loading/preparation is outside latency and inside lifetime peak process RSS.

|Variant|1×2 ms|CPU ms per request|Median peak RSS MiB|Maximum peak RSS MiB|
|---|---:|---:|---:|---:|
|accepted|8.140269|182.861|130.50|133.75|
|OFF|8.037214|182.087|130.50|135.00|
|ON|8.481222|193.316|132.00|137.00|

|Paired comparison|Median ratio|Bounds|Wins out of 31|Paired saving µs|
|---|---:|---|---:|---:|
|ON/OFF|1.055244019|[1.051573654, 1.057443162]|0|-444.800|
|ON/accepted|1.042733048|[1.036482844, 1.045527578]|0|-346.494|
|OFF/accepted|0.987639760|[0.983465542, 0.991395053]|29|100.209|

Displayed cells are medians of process medians; paired estimates use corresponding blocks. The predeclared gate requires upper ON/control ratios below one and at least 21 wins against both accepted and OFF. OFF/accepted bounds must fit [1/1.03, 1.03]. Sorted ranks 10–22 give nominal 97.055% individual median coverage under independent/exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusion, extension, unchanged reroll, width selection or postcapture waiver is used.

Failed checks:

- `1x2-w32/ON/OFF`
- `1x2-w32/ON/accepted`

## Implementation and earlier evidence

The [old G1 bucket-Horner trial](../2026-10-22-universal-g1-bucket-horner/README.md) and its [arena32 followup](../2026-10-22-universal-g1-horner-arena32/README.md) remain negative. They used separate per-query owned streams and positive components. This implementation instead reuses the accepted signed decomposition with the current borrowed, fused K/H caller. It is a materially revised algorithm/caller combination, not a new claim for the unchanged old trials.

For each of ten balanced spans covering 127 bits, high K and H components share one borrowed bucket set. Applying the G1 endomorphism to temporary affine/projective buckets precedes the low components. Two point partitions preserve twenty fill jobs. Original collision handling, affine arithmetic, merging, weighted reduction and Horner assembly remain. Ten component windows use 34,816 magnitude slots per partition, 69,632 across two partitions, versus 71,680 in the original twenty-window stream. The candidate adds partition merges and bucket transforms; a lower window count is not itself a speed gain.

An added proof-local vector stores one 32-byte short tuple per K/H scalar. Before allocating it, resource admission charges both that vector and the entire new held/worker-leaf workspace via the existing checked G1 shape model (ten windows, two parts, maximum 4,096 slots, width-13 batch). The original required proof forecast remains charged. Admission failure uses the original balanced stream. There is no new fixed ceiling, persistent key table, transformed point copy, solver plan, input mutation, pool, unsafe code or field implementation. These costs are conservative requested-capacity accounting, not RSS or minimum RAM guarantees.

Private cfg `zopro_glv_fused_kh` requires native IFMA8, resource scheduling and the existing Horner configuration. Only the already eligible fused K/H width-13 route changes; unsupported sizes/modes preserve their previous path. In the supplied fixture set, primary uses this route. Larger circuits receive correctness preflights, not fresh candidate latency qualification.

Four original files change reversibly: borrowed-pending wrapper/module, concrete K/H dispatch, balanced Span/digit visibility, and an additional workspace-cost entry. One new child yields 176 private files with 171 of 175 originals exact. All 17 field files and the reused signed-split source remain exact; accepted field qualification is inherited, without claiming arithmetic binary identity.

## Validation and CPU profiles

Both native configurations run unfiltered core/prover suites and freeze their actual linked libraries before using the unchanged SDK driver.

|Suite|OFF passed|ON passed|
|---|---:|---:|
|core|124|127|
|prover|8|8|

Three new tests verify the G1 endomorphism, signed scalar reconstruction and every balanced digit across boundaries/full-width inputs; full fused sums against an independent weighted-generator oracle on one/four-thread pools, identities/opposites/repeated points, zero/one/negative-one/full-width/boundary scalar families, tails and primary-sized synthetic geometry; and checked resource overflow/dimension fallback. Scalar input immutability is checked. Existing grouping, curve, solver and prover oracles also run.

All nine shapes × three variants pass 27 independently Gnark-verified preflights, one request after three primes, before latency. All 93 latency captures precede six FIFO-gated 199 Hz cycles:u profiles: primary 16/32 workers × three variants, 251 requests after three primes. CPU IDs and timestamps are recorded explicitly. All captures precede remaining independent Gnark verification and owner-user decode/loss checks. Physical-16 profiles are diagnostic, not latency qualification. No owned CPU helper/build/hash/archive/other benchmark overlaps capture.

All **15,954 SDK requests** verify internally. All **126 fresh 164-byte exports** pass independent Gnark, exact key/public bindings and freshness checks against 21,133 prior identities. The distinct proof ledger becomes **21,259**. All 276 capture/offline steps complete.

|Profile|Samples|GLV namespace samples|Lost records|
|---|---:|---:|---:|
|cycles-1x2-w16-OFF|5630|0|0|
|cycles-1x2-w16-ON|5618|316|0|
|cycles-1x2-w16-accepted|5631|0|0|
|cycles-1x2-w32-OFF|8895|0|0|
|cycles-1x2-w32-ON|9361|400|0|
|cycles-1x2-w32-accepted|9350|0|0|

Each profile contains one benchmark PID and zero lost records. Both ON profiles sample the new namespace; neither control does. Flat symbol weights are not inclusive function costs, exclusive wall-time fractions or a causal explanation. Actual CPU time and process RSS remain separate observations.

The first candidate compilation failed with E0308 because a concrete G1 routine was called from the generic borrowed stream. Moving only the cfg dispatch to the concrete K/H entry and adding a typed private wrapper repairs compilation without changing arithmetic or window geometry. Successful OFF steps are retained; the remaining five ON steps run in a separate workflow. The initial source/failure and composed completion record are archived; no successful test or capture is repeated.

## Supplied CPU and Metal references

Primary supplied historical references are 37 ms M5 CPU and 24 ms M5 Metal; these are not new Apple measurements.

|Variant|Primary ms|M5 CPU speedup|M5 Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|accepted|8.140269|4.545×|2.948×|3.7|
|OFF|8.037214|4.604×|2.986×|3.7|
|ON|8.481222|4.363×|2.830×|3.7|

The eight complete accepted-production comparisons below remain separate from this primary-only pilot.

|Shape|Accepted local ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

Accepted 54→1 remains 162.250464 ms, without a supplied M5 reference. The accepted full performance gate remains false because of its documented physical-16 calibration exception; the separate accepted final-artifact gate is true. Neither status transfers to this candidate. Accepted production reaches 0/8 CPU 10× targets. The original all-eight verified CPU 10× objective remains active and is not established by this pilot.

There is no new AWS/Aeglos, mobile/ARM/TEE, huge-circuit 250/4999, pressure/swap, minimum-RAM or concurrent-throughput qualification. The [current local/AWS summary](../2026-10-29-current-cpu-summary/README.md) retains those measured comparisons.

## Decision

This exact implementation closes without production adoption. All positive point estimates or memory observations remain valid within their measured scope; the failed speed gate stays unchanged. A materially revised implementation or a separately predeclared memory/nonregression qualification may use the retained evidence.

[Design](artifacts/design.md), [full block/profile data](artifacts/analysis.json), [implementation](artifacts/glv_kh.rs), [source audit](artifacts/source-audit.json), [capture protocol](artifacts/run-plan.json), [source archive](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [proofs and public bindings](artifacts/proofs.json.gz), [closing pins](artifacts/pins.json).
