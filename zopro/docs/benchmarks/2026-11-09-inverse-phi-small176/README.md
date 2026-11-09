# Inverse cubic map with a short fixed multiplier

The complete inverse-map transform is **19.24% faster than ordinary SIMD at2,048 points/32 workers**, paired ratio0.807589 [0.800655,0.821923],15/15 wins. At1/16 workers it improves about29%/28%. This is a transform result, **not a full-query or proof speedup**. The declared overall screen **fails two16-worker scalar-control calibration checks**. Production is unchanged.

The positive same-binary comparisons remain useful. All six inverse-fixed/SIMD comparisons win15/15, and all required inverse-fixed/scalar gains pass. Both32-worker scalar controls calibrate. A separately declared native full-query experiment is selected as an **explicit engineering exception to the original closure rule**; the screen is not relabelled passed. No unchanged transform rerun, threshold change, favorable subset or production adoption is selected. The next experiment must charge the final map and changed point order. [Decision and limits](artifacts/decision.json).

## Change and exact identity

Accepted G2 uses phi(x,y)=(beta*x,y), acting as scalar lambda. Its inverse has multiplier beta²=p−beta−1, the positive191-bit integer2203960485148121921418603742825762020974279258880205651966. The earlier [fixed-beta transform](../2026-10-31-beta-fixed176/README.md) used the full254-bit factor and remained slower than ordinary SIMD at32 workers. It stays rejected. The new factor and its197-bit Shoup quotient each occupy four radix52 limbs, permitting twelve more product halves to disappear: **72 IFMAs versus84 in that fixed-beta kernel and100 in ordinary mixed multiplication**. This is a different map orientation, not an unchanged retest of that kernel.

For the existing signed split s=k1+lambda*k2, the identity `s=lambda*(k2+lambda²*k1)` permits swapping the two encoded components and applying phi once to the final query result. Component magnitudes, signs,32-byte rows, window widths and bucket capacities need no widening. The final projective-X correction costs two scalar Fq products. The full query is not implemented here: switching high/low stream order can change collisions, exceptions, spills and scheduling, all of which remain unmeasured.

The scalar/kernel prototype adds one native-only field child and a module declaration in an isolated snapshot. All176 originals recover;175 are unchanged, including field templates, generated kernels and generator. Qualification of unchanged arithmetic/generator is inherited. There is no global multiplication dispatch change. Eight fixed factor/quotient words occupy **64 logical constant bytes**, independent of keys/circuit size; emitted broadcasts and frames are additional. No per-key table, retained point/scalar/witness copy, solver plan, production pool or artificial RAM cap is added.

The Shoup-style reduction uses R=2^260 for its quotient estimate while preserving numerical R256 storage. Truncated high products underestimate `floor(a*q/R)` by at most one; omitted contribution is below8/2^52. For a≤2p, the remainder satisfies `0≤a*beta²−Q*p<(1+a/R+8/2^52)*p<2p`. Two signed low products modulo R recover that remainder. Signed coefficients consist of at most five low and four high product halves per side, plus bounded carries, comfortably within signed64; the mathematical interpreter uses i128. A discarded top carry of this modulo-R calculation need not be zero. No inverse-domain conversion or negative-output correction is needed. This adapts the [previous qualified fixed-factor construction](../2026-10-24-fixed-multiplier-component/README.md), with no new algorithmic novelty claim.

## Qualification

The exact BigUint/Arkworks model passes6,920 raw output evaluations,4,864 scalar-split identities and128 curve-map/correction checks. Inputs cover inclusive0..2p, modulus and bit/radix neighbors,4,096 deterministic full-width values and256 eight-step chains. The original split helper is copied exactly; independent Fr arithmetic checks both eigenvalue reconstructions and the unchanged below2^126 magnitudes. Finite checks supplement the range argument, not a native machine-code proof.

Three new native tests pass, with35 unchanged tests filtered and inherited:

- 36,928 field-lane comparisons against Arkworks, including direct raw-digit construction, inclusive2p, normalized digits, strict output range and changing chains.
- 14,610 complete point outputs across tails/identities through2,049 points, threefold identity, inverse scalar-map agreement and final forward correction. Pointers/capacities remain unchanged.
- 53 actual test workers across1/4/16/32-worker groups with512KiB requested stacks.

Tests use native O3/fatLTO/one-CGU with debug and overflow checks. The release field library is reused unchanged by the actual callers. No core/prover/portable suite, SDK proof or Gnark export is added.

## Complete caller results

Ryzen9950X,16 physical/32 logical CPUs. Frozen accepted field and fresh scalar OFF compute forward phi; fresh ordinary SIMD also computes forward phi; FIXED computes inverse phi. Thus the maps differ after one invocation, while both cubed are identity. The benchmark measures complete map costs rather than a same-work MSM. All packing, stores, branches and scalar tails are included; Y is retained.

Fifteen rotated/reversed separate-process blocks cover64/2,048 points and1/16/32 workers. Each worker pins to CPU2 solo or its index in CPUs0..15/0..31, with512KiB requested stacks. Three checked16-cycle primes precede192 measured cycles of three transforms. The statistic is maximum worker time divided by576 transforms. Fixtures/startup and final checks are outside clocks. Every prime and final coordinate matches its original after cubing; intermediate timed outputs are not individually checked. Capacities and final CPU placement agree. Timed allocation calls are not instrumented.

|Points/workers|Frozen scalar µs|Fresh scalar µs|Ordinary SIMD µs|Inverse FIXED µs|Paired FIXED/SIMD [rank bounds]|
|---|---:|---:|---:|---:|---:|
|64/1|1.351891|1.333141|0.368090|0.260873|0.707708 [0.700685,0.711465]|
|64/16|1.455193|1.433102|0.393625|0.279677|0.709856 [0.704490,0.714270]|
|64/32|2.385073|2.363104|0.548814|0.444762|0.819651 [0.813854,0.828193]|
|2048/1|42.926365|41.989807|11.644498|8.240231|0.709092 [0.704913,0.710836]|
|2048/16|47.388026|46.201675|12.859429|9.174422|0.717901 [0.707544,0.727549]|
|2048/32|80.447394|80.094003|19.379957|15.615302|0.807589 [0.800655,0.821923]|

Every FIXED/SIMD comparison wins15/15. Ratios pair blocks and need not divide the marginal medians. Ranks4–12of15 have nominal96.484375% marginal coverage under exchangeable independent blocks, without familywise coverage. All required FIXED/scalar and both32-worker FIXED/SIMD gain checks pass. Failed checks are exclusively `n64-w16/off/frozen/calibration` and `n2048-w16/off/frozen/calibration`: intervals[0.960534,0.990914] and[0.953387,0.987020] extend below1/1.03. Their point estimates are0.978893/0.983426. The failure is retained; it does not imply that the candidate itself regressed or that the20–29% same-binary result is a full-proof gain.

All426 capture/offline commands finish successfully:24 preflights,360 latency callers,12 separate PMU/profile callers, then30 offline steps. There are396 caller processes and32,644,248 complete transform invocations including primes. No owned build/helper/hash/analyzer/decoder/report/push overlaps recording; only actual live handle29662 is polled until terminal0. All timing precedes diagnostics, and all recording precedes offline decoding. No successful capture or old verifier is replayed.

Observed maximum process RSS, identical across arms in each context, is2.75/6.25/10.5MiB at2,048 points with1/16/32 workers. This includes benchmark fixtures/workers and is neither prover peak nor minimum RAM. Fixed scratch/constant payload and unchanged capacities do not establish pressure/swap/mobile behavior or allocation-free execution.

## Automatic CPU profiles and code

All six profiles qualify:139,409 samples,zero LOST,correct PID/TID/CPU/MMAP/FORK/16M period, and at most0.040% unknown. All24 PMU event values run100%. Single counter panels are descriptive, not repeated causal measurements.

|Workers / caller|Instructions/transform|Cycles/transform|
|---|---:|---:|
|16 ordinary SIMD|127,015.698|63,475.231|
|16 inverse FIXED|120,361.691|44,830.941|
|32 ordinary SIMD|127,015.711|85,993.424|
|32 inverse FIXED|120,361.704|68,589.543|

The32-worker panel has about5.24% fewer instructions and20.24% fewer cycles than ordinary SIMD. Cache misses do not move uniformly with latency; no cache/DRAM/spill cause is established. Sample attribution is not removable proof-wall time.

The actual linked FIXED function has72 IFMA sites,267 static instructions,1,713 bytes and6 explicit RSP-operand sites. Ordinary SIMD has100/278/1,784/12. Both scalar controls have403 static instructions and2,077 bytes. Static sites do not count dynamic spills or bound stacks. A source-name repair occurred only inside the frozen-disabled cfg branch, so its successful frozen link is reused; its earlier source remains bound. No byte-identical whole-binary/control claim is made.

## Failures, review and next step

Preserved setup failures: two model compiler failures from Arkworks0.5 trait/extension APIs; one model assertion incorrectly requiring zero discarded modulo-R carry; a source-pin guard that included the already-built output model binary; and one fresh-caller compile after a broad method-name substitution. Repairs retain original sources/pins/streams. The successful corrected model build is reused after removing the output from source pins, rather than rebuilt. The field/tests/library and successful frozen caller are also reused. There are435 successful workflow command receipts and4 nonzero command-exit receipts; the separate pin-guard rejection is also retained. Manual helper compilation/publication has tool-transcript evidence; one reviewer compile initially omitted the demangler dependency and then compiled unchanged source with that dependency.

The result reader passes independently reconstructed raw worker maxima, all360 latency rows,30 paired comparisons, both gate failures, calls,399 pins,176 source recoveries and all command stream hashes. It shares detailed PMU/profile parsers and original field references with the producer; it is same-agent review, not an external audit. JSON float reconstruction tolerance is1e−12 relative; integers/hashes/counts are exact. The successful mathematical model, native tests, recordings and decoders are not rerun.

The full-query follow-up is an explicit selection exception, not a performance-gate pass. Reuse the qualified field and implement the actual inverse-oriented G2 query with existing component rows, all top/group/tail/exception paths, one final phi correction and current admission rules. Freeze fresh complete-query controls and automatic profiles before any clock. Complete proofs and all-eight/resource/portable/concurrent checks remain required before adoption. No proof-speed improvement is currently established.

## Accepted CPU and Metal comparison, inherited

These are the preceding full campaign's fixed-placement accepted-library observations, not this candidate's timings or an adopted pinning default. M5 references were supplied on different hardware. Historical standard1×2 **7.915095ms** remains valid; latest standard was8.075143ms, and fixed was7.869302ms. No production regression, new proof record or changed accepted speed is established.

|Shape|Accepted fixed ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.869302|37 /4.702×|24 /3.050×|3.7|
|2×2|10.984868|59 /5.371×|30 /2.731×|5.9|
|4×2|15.734200|109 /6.928×|48 /3.051×|10.9|
|8×2|23.929023|189 /7.898×|74 /3.092×|18.9|
|12×2|33.722646|284 /8.422×|101 /2.995×|28.4|
|16×2|39.776845|335 /8.422×|124 /3.117×|33.5|
|24×2|66.183856|528 /7.978×|176 /2.659×|52.8|
|Merge24×1|65.448733|507 /7.747×|175 /2.674×|50.7|

Source: [preceding full campaign](../2026-11-09-weighted-prefix-fixed-sdk176/README.md). Merge54 remains162.489760ms without an M5 reference. Accepted176/SDK6d83 and the canonical38,668 verified exports remain unchanged. Zero new SDK requests/Gnark exports, AWS resources, Aeglos/device/huge-circuit/pressure/swap/minimum-RAM or concurrent-proof results are added. The verified all-eight CPU10× goal stays active and unmet, **0/8**.

[Design](artifacts/design.md), [capture protocol](artifacts/capture-design.md), [model](artifacts/model-result.json), [native primitive](artifacts/inverse_phi_small.rs), [codegen](artifacts/codegen.json), [all observations](artifacts/analysis.json.gz), [prospective review](artifacts/prospective-review.json), [result review](artifacts/review-result.json), [selection exception](artifacts/decision.json), [continuation loop](artifacts/next.md), [source/receipts/decoded profiles](artifacts/evidence.json.gz), [publication checks](artifacts/publication.json), [checksums](SHA256SUMS). Archives retain member hashes and byte-exact gzip roundtrip. Binaries/libraries, raw perf.data, unchanged full source trees, keys/witnesses/proofs and private proof inventory are excluded.
