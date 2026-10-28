# Balanced fused K/H signed windows

The primary SDK pilot passes its declared gate. Candidate 1x2 latency is **8.019212 ms**, versus accepted **8.096748 ms** and fresh OFF **8.183011 ms**, at32workers on this Ryzen9950X. It wins all15paired blocks against both controls. The paired improvement is **0.677% against accepted** and **1.917% against OFF**. This permits broader qualification; it is not production adoption. Accepted source174/SDK81033 remains selected.

## Change

The original fused K/H uses20signed13-bit windows:19full streams plus a7-bit positive top processed as two indexed reductions. The candidate covers the same scalar with20balanced windows: widths `[12,13,13,13]` repeated five times, covering255bits. An offset-based Booth reader needs no digit array; variable-width Horner combination reconstructs the original integer.

All20windows use the existing borrowed pending/collision state, current200/352-entry batches, complete exceptional-point handling and weighted affine tree. The final13-bit span starts at242; canonical Fr has bit254zero, so this digit is nonnegative and at most4096. Other fused widths, ordinary A, G2, field kernels, solver, scheduler, key and image formats remain unchanged. The existing native borrowed/fused eligibility guards still apply. No point table, key copy, new pool or execution plan is added.

Full-window bucket capacity becomes71,680, versus19×4,096 plus two128-bucket top reductions previously. These are capacity counts, not measured point-addition or inversion counts. Five smaller batches can increase inversions; replacing the indexed top changes access and arithmetic topology. The latency result evaluates these effects together.

The existing workspace forecast already charges20full4,096-bucket parents, maximum352-entry leaves and both top-index parents. New heap geometry is no larger than this envelope. New query-local stack metadata is20spans/320bytes; compiler frames are additional. This argument is not a hard RSS, minimum-RAM or memory-pressure qualification.

The private175-file snapshot contains173exact originals, one reversibly changed parent and one new child module. Field/G2/solver/key/scheduler/forecast files remain byte-exact. The failed uniform-width, top-first interpretation, combined indexed-top and sequential-band experiments remain closed; this changes digit boundaries and top topology rather than only launch order.

## Complete-proof measurements

Fifteen fixed rotated/reversed blocks compare accepted, OFF and ON in separate processes. Each executes151complete SDK requests after three primes, including solving, fresh blinding, proving, internal verification and serialization. Loading is excluded. Affinity is CPUs0–31 with32Rayon workers on16physical cores. No exclusive-host claim is made.

| Variant | Median block ms | CPU ms/request | Median process peak MiB | Maximum process peak MiB |
|---|---:|---:|---:|---:|
| Accepted | 8.096748 | 186.233 | 129.75 | 132.25 |
| Fresh OFF | 8.183011 | 187.403 | 130.75 | 134.50 |
| ON | 8.019212 | 180.948 | 130.00 | 135.75 |

| Paired comparison | Median ratio | Bounds | Wins/15 | Check |
|---|---:|---|---:|---|
| OFF/accepted | 1.011950 | [1.008829,1.015397] | 0 | Calibration passes |
| ON/OFF | 0.980833 | [0.976571,0.984900] | 15 | Passes |
| ON/accepted | 0.993230 | [0.988028,0.996297] | 15 | Passes |

Bounds use sorted paired-ratio ranks4–12of15, nominal96.484375%coverage per comparison under independent exchangeable blocks, without a familywise guarantee. Candidate upper bounds must be below1 with at least10wins; calibration must fit[1/1.03,1.03]. No minimum gain floor, exclusions, extension, changed threshold or successful-capture repeat is used. Paired median savings are54.904µs against accepted and157.397µs against OFF; these differ from differences of unpaired medians.

**OFF and accepted executables are byte-identical**, SHA256 `81033d7786e14821cb76494de7faa2ebcc9e40c3222708372b21861fdeb56c20`. Their independent process streams nevertheless differ by1.195%paired, with OFF slower in every block. The calibration passes its predeclared3%bound, and the candidate improves against both, but the larger OFF comparison must not be presented as the accepted-code gain. These streams are not independent implementation controls. Broader qualification must retain this control gap as a limitation rather than assigning it to the algorithm, file placement, ASLR, thermal state or scheduling without evidence.

Candidate maximum RSS exceeds both controls; median RSS lies between them. No process-memory improvement is established.

## Verification and profiles

Both native builds pass their core/prover suites:122OFF/124ON core tests and eight prover/raw-cubic tests each. Field source is unchanged, so its inherited33-test qualification is reused, not rerun. New tests perform4,861exact320-bit scalar reconstructions,97,220independent bit-loop digit checks,36complete balanced weighted-generator calls and72unchanged-width control calls across one/four workers. Inputs include zero/one/minus-one, full-width and boundary scalars, ragged lengths, identity, repeated and signed points, through63,287terms. Existing full core tests remain included.

Three preflight exports independently verify before latency. ONpreflight alone enables metadata tracing and emits four route records including primes. Its existing verification step runs Gnark and checks those records before the first latency process. Tracing is absent from all latency and diagnostic environments; closing rechecks the records.

After all latency, three FIFO-gated four-event counter runs and three separate199Hzcycles:u profiles cover all variants. Counter runs use31requests and profiles251, after three primes. Loading/primes are outside the FIFO gate. All six captures finish before their verification and decode. Diagnostic timings do not enter latency decisions.

| Per-request event | OFF | ON |
|---|---:|---:|
| cycles:u | 854,010,564 | 823,780,717 |
| instructions:u | 1,453,316,374 | 1,418,950,269 |
| cache-misses:u | 3,084,333 | 2,805,102 |
| branch-misses:u | 1,244,758 | 1,180,866 |

All counters run100%. Candidate instructions fall2.36%and cycles3.54%in these separate counter captures. Profiles contain9,368accepted,9,200OFF and8,888ON samples, **27,456total**, one PID each and zero lost records. The candidate balanced feed symbol appears in213samples/4.91%of sampled period, with another helper sample. Route records and source dispatch establish execution; symbol shares are not exclusive costs or removable wall-time fractions. Generic cache events do not identify a cache level, DRAM limit or causal explanation.

All117capture steps complete. All **7,806 SDK requests** verify internally. All **54fresh164-byte exports** independently pass Gnark, have correct public-input bindings and are distinct from the complete prior set. All108proof/public members are archived; ledger15,865→15,919. All closing source/library/helper/input/compiler pins match.

Initial candidate compilation found a missing closing brace in a new test loop, before any candidate test executed. The full initial175-file source and failure receipts are preserved. Two files change only that test repair; their entire runtime prefix remains byte-exact. Five successful control steps are reused, and a separate workflow completes the five remaining candidate steps. The merged completion index references both authoritative receipts. Capture preparation checks the exact two-file repair and all other initial pins. No successful test or performance capture is repeated. [Repair record](artifacts/repair-note.json), [completion index](artifacts/native-completion.json).

Full current/pre-repair source including non-Rust files, manifests, licenses, tests, commands, observations, proof exports and decoded profiles are retained. Three raw perf recordings remain local with SHA256receipts. No owned build/helper/hash/archive/second workload overlaps captures. There is no new AWS/Aeglos, phone/ARM/TEE, giant-circuit, portable, concurrent-proof, swap or pressure qualification. Broader shape, physical16 and resource behavior remain outstanding before adoption.

## CPU and Metal references

The supplied historical M5CPU/Metal1x2 references are37/24ms. Candidate is4.614×/2.993×faster, versus accepted4.570×/2.964×. These are cross-machine reference ratios, not fresh M5captures. The primary10×CPUtarget is3.7ms and remains unmet.

The complete accepted table below is inherited from the [source174 adoption](../2026-10-28-g2-borrowed-adoption/README.md), including its failed performance gate and explicit engineering exception. This primary-only pilot does not replace that all-eight qualification.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.366931 | 37 | 4.422× | 24 | 2.868× |
| 2x2 | 11.539335 | 59 | 5.113× | 30 | 2.600× |
| 4x2 | 16.540233 | 109 | 6.590× | 48 | 2.902× |
| 8x2 | 25.136722 | 189 | 7.519× | 74 | 2.944× |
| 12x2 | 34.858408 | 284 | 8.147× | 101 | 2.897× |
| 16x2 | 41.377043 | 335 | 8.096× | 124 | 2.997× |
| 24x2 | 67.764001 | 528 | 7.792× | 176 | 2.597× |
| merge-24x1 | 67.096516 | 507 | 7.556× | 175 | 2.608× |

The original all-eight CPU10×goal remains **active and unmet,0/8**. The candidate is unadopted; qualified accepted primary remains8.366931ms.

[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [identities](artifacts/identities.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [closing pins](artifacts/pins.json), [capture completion](artifacts/runs/result.json), [research/history](artifacts/research.md).
