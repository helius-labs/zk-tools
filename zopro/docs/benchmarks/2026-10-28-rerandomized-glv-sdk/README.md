# GLV final assembly in full SDK proofs

The candidate **fails the declared full-proof screen and is not adopted**. Primary32 ON/OFF paired median is **0.988440956**, bounds[0.974532373,0.997095332], 12/15wins. ON/accepted is **1.009361535**, bounds[0.986768971,1.040005627], 6/15wins. All positive estimates, uncertainty and failed checks are retained. Production and the accepted SDK remain unchanged.

The [standalone caller study](../2026-10-28-rerandomized-glv-callers/README.md) improved final assembly by about38.5%. This ports that exact runtime mechanism to the native prover. The existing two-component integer lattice decomposition shortens both G1 and G2 variable-base doubling chains. G1 uses components(a-b,-b) because its eigenvalue is-1-lambda_G2. Width-five digits and small endomorphism images are proof-local. Randomness, proof distribution, field arithmetic, MSM/FFT/solver algorithms, verification and image formats are unchanged. There is no new persistent point/key table or execution plan.

## Verified request latency

Ryzen9950X,15 fixed rotated/reversed process blocks, accepted/freshOFF/ON at32logical and16physical workers. Every latency process has3 primes and151 timed requests. Loading is excluded; witness solving, fresh randomized proving, SDK verification and serialization are included. Times are medians of block medians; CPU includes coordination and RSS includes loading/primes.

| Workers | Variant | Median ms | CPU ms/request | Median peak MiB | Max peak MiB |
|---|---|---:|---:|---:|---:|
| 32 | control | 9.756371 | 196.531728 | 132.50 | 137.50 |
| 32 | off | 10.193917 | 201.217768 | 131.00 | 137.50 |
| 32 | on | 9.951890 | 196.052901 | 133.00 | 136.50 |
| 16 | control | 12.291723 | 132.468437 | 89.75 | 92.75 |
| 16 | off | 12.506498 | 136.136424 | 89.75 | 92.75 |
| 16 | on | 12.262798 | 133.875583 | 90.25 | 94.00 |

| Workers | Ratio | Paired median | Marginal interval | Wins | Check |
|---|---|---:|---|---:|---|
| 32 | off/control | 1.024328693 | [1.011381120, 1.043671776] | 2/15 | fail |
| 32 | on/control | 1.009361535 | [0.986768971, 1.040005627] | 6/15 | fail |
| 32 | on/off | 0.988440956 | [0.974532373, 0.997095332] | 12/15 | pass |
| 16 | off/control | 1.008653166 | [0.996573705, 1.056618751] | 4/15 | fail |
| 16 | on/control | 0.997996582 | [0.942707898, 1.023542976] | 8/15 | fail |
| 16 | on/off | 0.967073983 | [0.949177908, 0.989570587] | 12/15 | pass |

Failed checks: `["w32/off/control","w32/on/control","w16/off/control","w16/on/control"]`.

Bounds use ranks4..12 of15, nominal96.484375% under independent exchangeable blocks, without familywise guarantees. OFF/accepted must fit[1/1.03,1.03] at both worker counts. PrimaryON requires upper<1 and at least11wins against both controls;16-workerON requires upper<=1.02. No minimum gain floor, exclusion, extension, successful-capture repeat or unchanged reroll is applied. Crossing a required bound does not by itself establish a causal slowdown. The component gain is not transferred to proof latency. In this run the 32-worker GLV/fresh estimate improves 1.156%, with bounds showing a 0.290% to 2.547% reduction, and 12/15 wins. The 16-worker GLV/fresh estimate improves 3.293%, bounds 1.043% to 5.082%, also 12/15 wins. Both positive observations are retained. Against accepted code the 32-worker point estimate is 0.936% slower with an interval spanning parity, while the 16-worker estimate is 0.200% faster with a broad interval. Fresh/accepted calibration fails at both worker counts. Thus the study supports the measured comparisons against fresh code but does not demonstrate an improvement over accepted production. It does not prove that all observed changes arise solely from GLV; host variation and compiled context remain unresolved.

## Integration, validation and memory

Only groth16.rs and groth16_rerandomize.rs change reversibly, plus one private module.170 other original source files remain byte-exact; reversing the two edits recovers all172. Original pair/joint bodies remain verbatim fallback. Parent helper-path visibility and inline placement of the exact accepted split helper are the only runtime adaptations. There are no diagnostic observers in the SDK. The private cfg selects the new final assembly; no public runtime option is introduced. Field source is unchanged and its prior qualification is inherited.

FreshOFF and ON each pass all119 native core tests, two raw-image tests and six cubic prover tests. The retained assembly exponent oracle covers prepared/unprepared paths, full-width/negative/zero scalars, identity rejection/fallback, poisoned omitted-B1 bytes and proof encoding. The parent's independent Arkworks/decomposition tests qualify the unchanged helper bodies; they are not counted as new native tests. Some optional external-fixture core tests return when their fixture is absent; those scenarios are not claimed. Complete real SDK proofs are checked separately below.

All transitive rlibs/proc-macro shared objects are frozen immediately in separate target roots. Actual core/field flags and native features are checked, and the standard SDK driver links exact eight-library receipts. Original and selected full SDK hashes differ. No successful build/test/capture/verifier is rerun for bookkeeping.

GLV image arrays add fixed local payloads:512bytes in G1 and3072bytes in G2, with compiler frames/other temporaries additional. Existing table allocations remain within the timed request. These payloads are not measured stack bounds, minimum RAM or memory-pressure qualification. Process peaks above are observations, not a universal resource guarantee. No new persistent key cache or input-sized buffer is added. Broader portable, stack, resource, concurrent and250/4999 qualification remains necessary before adoption.

## Proofs and profiles

All90 latency captures precede six separate FIFO-gated199Hz cycles:u profiles,251 requests after3 primes each. Three4-worker preflights precede latency. All99 SDK captures finish before independent verification, decode or archive. No owned build/helper/hash/archive or second benchmark overlaps capture. The post-capture host snapshot contains unrelated CPU workloads; there is no per-process capture-time trace establishing their contribution. It does not establish exclusive-host use or an exclusive cause of calibration drift.

All15,396 complete SDK requests verify internally. All99 fresh164-byte exports independently pass Gnark, expected public binding, member hashing and uniqueness against13,082 previous identities. The independent proof ledger is now **13,181**. Copies/subinventories are not counted again.

| Workers | Variant | Samples | Lost | Matched groth16 symbol period |
|---|---|---:|---:|---:|
| 32 | control | 8946 | 0 | 0.028% |
| 32 | off | 8826 | 0 | 0.000% |
| 32 | on | 8810 | 0 | 0.000% |
| 16 | control | 6233 | 0 | 0.048% |
| 16 | off | 6271 | 0 | 0.025% |
| 16 | on | 6208 | 0 | 0.000% |

This narrow symbol-name matcher misses inlined and generic curve arithmetic. A near-zero matched share does not mean assembly was eliminated.

The six profiles contain45294samples, one sampled SDK PID each, zero lost records. Assembly-symbol period includes the matched functions and is not removable proof wall time. Complete symbol maps and raw profile hashes are retained; raw perf files stay local. Six loss dumps have verified deterministic-gzip round trips.

This exact native integration is closed without adoption. Its standalone caller gain and every small positive full-proof estimate remain within their measured scopes. No unchanged source/flag/layout/schedule reroll is selected.

## Inherited CPU and Metal comparison

These accepted local observations and supplied historical M5 references are inherited, not all-eight measurements from this primary-only screen. Accepted FFT engineering exception and false full-performance/qualified-scope gates remain unchanged. Original all-eight CPU10x objective is active and unmet0/8.

| Shape | Accepted ms | M5 CPU ms | M5 Metal ms | CPU/accepted | Metal/accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37.0 | 24.0 | 4.445× | 2.883× |
| 2x2 | 11.460675 | 59.0 | 30.0 | 5.148× | 2.618× |
| 4x2 | 15.854365 | 109.0 | 48.0 | 6.875× | 3.028× |
| 8x2 | 24.084626 | 189.0 | 74.0 | 7.847× | 3.072× |
| 12x2 | 33.693397 | 284.0 | 101.0 | 8.429× | 2.998× |
| 16x2 | 39.981636 | 335.0 | 124.0 | 8.379× | 3.101× |
| 24x2 | 66.010670 | 528.0 | 176.0 | 7.999× | 2.666× |
| merge-24x1 | 65.086624 | 507.0 | 175.0 | 7.790× | 2.689× |

Historical primary8.324853ms/RSS134.50MiB; merge54 161.341835ms/RSS1910.1875MiB without a supplied M5 reference. No new AWS/Aeglos, phone/TEE, giant-circuit or minimum-RAM result is claimed.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [new runtime module](artifacts/source/crates/zopro-core/src/groth16/glv_blinding.rs), [native workflow](artifacts/native/result.json), [proof inventory](artifacts/proof-inventory.json), [all99 workflow receipts](artifacts/runs/result.json).

Publication whitespace checking reports a trailing blank line in the measured private runtime snapshot. Those bytes remain unchanged for hash provenance; the README and Rust tooling are checked separately.
