# Production streaming-update arithmetic census

**The mathematical census passes; no optimization or speedup is selected.** It binds the current accepted G1/G2 streaming bodies to a standalone Arkworks model, checks full batches and tails, and distinguishes multiply, square and fused-product work. The [preceding root-product experiment](../2026-11-06-inverse-root-callers176/README.md) remains rejected.

The source-bound arithmetic model passed: 168 batch executions, 26,676 point-output comparisons against independent Arkworks group addition, eight explicit guard classifications, seven source-statement bindings and 72 comparisons with previously captured record capacities. Both workflow commands and the independent reviewer succeeded on first execution. No production kernel, native test suite, proof, timing or hardware profile was rerun. A compiler warning reports one unused model import; it does not affect the successful model and is retained without rebuilding for cleanup.

For g=floor(n/8) full SIMD groups and t=n%8 scalar tail points, G1 uses 6g+5 vector multiplications, 5t+1 scalar multiplications, t scalar squares and one scalar inverse. G2 uses 7g+5 vector multiplications, 5g vector fused-two-product operations, 8t+1 scalar multiplications, 4t scalar fused-two-product operations, 2t scalar squares and one scalar inverse. The shared eight-lane inverse bridge accounts for the fixed five vector multiplies, one scalar multiply and one scalar inverse. Empty raw-kernel calls retain that unconditional bridge in this model; this does not assert production dispatch invokes empty kernels.

G1's point work is six vector multiplies per eight points, including generic multiplication for slope squaring. G2's point work is seven vector multiplies and five fused-two-product operations. Source IFMA proxies are 100 per vector multiply and 150 per fused2: 600 versus 1,450 per group. At 200 points, including the bridge, the proxies are 15,500 and 36,750; at 352, 26,900 and 64,300. These are source operation proxies, not emitted dynamic instructions, cycles or lower bounds. Additions, carries, range correction, packing, loop control and memory access remain relevant. The scalar x86 Fq2 product uses two fused2 reductions; the ARM three-product formula must not be substituted in these tail counts.

Moving the prefix scale between numerator and inverse does not eliminate a multiplication. Two ordinary fused2 complex outputs use the same 300-IFMA proxy as three separately reduced Karatsuba products; fewer bilinear terms do not alone imply less native work. The more specialized raw-Gauss/combined-reduction studies already exist and remain closed. Known-unit endpoint peeling removes three vector multiplies per eligible batch, only about 1.94%/0.82% of these G1/G2 200-point proxies; its separate full-caller experiment already failed and is not reopened. G1 two/four prefix chains, G2 two/four chains, denominator trees, ternary/triple paths, packing/layout and RNS/PMNS alternatives also remain recorded negatives, with their original numerical estimates and limitations preserved.

The literature's fused 2P+Q trick relies on a repeated input point. It is not a generic substitute for arbitrary bucket P+Q. The handoff already records that distinction; this is a source check, not a new discovery. Ordinary affine addition, batch inversion and pairwise sequence reduction are already present. This model does not prove a universal lower bound or that another representation cannot improve the prover.

G1 full-group coordinate packing/export calls are 4g/2g; G2 is 12g/4g because its smaller prefix records reread x coordinates in the reverse pass. These source-level calls are not literal loads/stores or byte traffic: the previous root-get study demonstrated dead-lane elimination. Record bytes exactly match the inherited observations: G1 1,600g and G2 640g, with scalar-tail retained capacities 640/1,280 bytes in the measured one-tail cases. Neither these capacities nor model allocations establish a production memory peak or minimum RAM.

A targeted history check found that operation/fetch IBS and L1I/frontend studies already exist. The October 29 stream IBS report records IFMA operand/resource waits, very few L2 misses and nonzero G2 fetch misses, while explicitly warning that these are not intrinsic execution latencies or removable wall fractions. No duplicate hardware capture is selected merely to rediscover that result. Existing RNS/PMNS, frontend/copy sensitivity, prefix/chain and unit-boundary results are not silently reopened.

No native arithmetic candidate is selected by this census. A distinct next feasibility question is whether simultaneous worker roots can share scalar inversion without new threads or input-sized state, rather than removing a few midpoint instructions. Montgomery sharing would replace b inversions with one inversion and 3(b−1) scalar multiplies; ownership, coordination, latency and arrival patterns can erase the saving. This is an unvalidated application hypothesis, not a forecast of a twofold proof gain. It needs a targeted history/critical-path/work/coordination screen before any production broker or native performance trial.

Accepted production176/SDK6d83, local 1×2 7.928660 ms, inherited 132.75 MiB RSS and the 30,846-proof ledger remain unchanged. All eight CPU10× requirements remain active/unmet 0/8, with supplied M5 CPU and Metal comparisons retained. No AWS/Aeglos/Gnark, new proof, huge/mobile/pressure/swap/minimum-RAM or throughput result is added.


## Model and independent checks

One normal Rust/xtask build and one execution complete successfully once. The model uses Arkworks field arithmetic to follow the source operation schedule, with independent projective addition as the point oracle. It is not an execution of the production SIMD kernel. Five accepted source files match the production176 manifest; the separate reviewer checks seven groups of exact source statements, 77 immutable pins, actual build/run argv/exits/stream hashes, all 168 case identities, count formulas and 72 inherited capacity observations. No original native test, proof or profile is rerun.

Sizes are 0, 1, 7, 8, 9, 15, 16, 199, 200, 201, 351, 352, 353 and 511 for both curves, all-positive/all-negative/mixed signs and two successive updates. Hashed full-width public scalar offsets and generator walks construct the points. All 26,676 ordinary point outputs equal Arkworks group addition. Eight explicit identity/equal/opposite classifications cover the precondition distinction; they do not qualify the native exception dispatcher. Scalar-tail field operations are counted separately from vector operations.

For g=floor(n/8), t=n%8, V=one eight-lane base-field multiply, F=one eight-lane fused sum/difference of two products, M/S/Fs=scalar multiply/square/fused2 and I=scalar inverse:

|Curve|Vector work|Scalar work|Coordinate pack/export calls|
|---|---|---|---|
|G1|(6g+5)V|(5t+1)M+tS+I|4g / 2g|
|G2|(7g+5)V+5gF|(8t+1)M+2tS+4tFs+I|12g / 4g|

The fixed five V plus M and I form the common lane-inverse bridge. Coordinate pack/export counts exclude that bridge and describe source calls, not machine memory operations. An IFMA-source proxy weights V=100 and F=150. It excludes scalar arithmetic, adds/carries/corrections, packing and control; it is not a runtime or stall model.

|Curve / points|V|F|M|S|Fs|I|Vector IFMA proxy|Coordinate packs / exports|
|---|---:|---:|---:|---:|---:|---:|---:|---|
|G1 / 200|155|0|1|0|0|1|15500|100 / 50|
|G1 / 201|155|0|6|1|0|1|15500|100 / 50|
|G1 / 352|269|0|1|0|0|1|26900|176 / 88|
|G1 / 353|269|0|6|1|0|1|26900|176 / 88|
|G2 / 200|180|125|1|0|0|1|36750|300 / 100|
|G2 / 201|180|125|9|2|4|1|36750|300 / 100|
|G2 / 352|313|220|1|0|0|1|64300|528 / 176|
|G2 / 353|313|220|9|2|4|1|64300|528 / 176|

## What this changes about the next experiment

The source model makes a proposed arithmetic saving checkable before another native trial. Moving the slope scale changes the dependency graph without reducing these products. Three separately reduced Karatsuba products and two fused2 outputs have equal 300-IFMA proxies; a raw-product/combined-reduction variant is a different circuit and already has recorded native negatives. Neither fewer mathematical products nor shorter source code establishes a machine gain. Known-unit endpoints and independent prefix chains have already been measured and stay closed.

The [Eisentraeger–Lauter–Montgomery paper](https://arxiv.org/abs/math/0208038v2) saves a multiplication for the repeated-input operation 2P+Q. [Barretenberg documents](https://barretenberg.aztec.network/api/namespacebb_1_1group__elements) its chord-only batch realization with two inversion stages and explicit nonexceptional preconditions. Those conditions do not hold for arbitrary bucket P+Q. Its [sequence reducer](https://barretenberg.aztec.network/api/classbb_1_1_batched_affine_addition) groups additions and shares denominators; corresponding mechanisms already exist here. No external performance figure or full paper/source is copied.

The history also already contains [streaming IBS operation/fetch evidence](../2026-10-29-stream-ibs175/README.md), [unit-prefix peeling](../2026-10-31-unit-prefix176/README.md) and [G1 prefix chains](../2026-10-22-universal-g1-prefix-chains-sdk/README.md). Their captures, mathematical qualifications, positive estimates and failed gates are inherited. No duplicate frontend/memory profile or unchanged small-field candidate follows from this census.

The next distinct feasibility question is opportunistic scalar-root inversion sharing across existing workers, subject to targeted history checks, exact arithmetic/canonical-output validation, fixed proof-owned state, ownership/coordination/cancellation costs and actual critical-path coverage. No new thread, key table or production broker is implemented or approved by this report. One batch replacing b inversions requires one inverse plus 3(b−1) multiplies; that identity alone does not establish a practical gain. The [next stage](artifacts/next.md) requires this screen before any native implementation or timing.

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references follow. They are not new measurements.

|Shape|Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge24|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Merge54 remains 161.883899 ms without an M5 reference. Primary RSS132.75 MiB is inherited. Source176/SDK6d83 and the 30,846-proof ledger are unchanged. The all-eight CPU10× objective remains active/unmet, **0/8**. No new AWS/Aeglos/Gnark, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput result is added.





## Artifacts

The standalone model and complete raw build/run receipts are public. Accepted production bodies are bound by their existing source manifest and five hashes; no production source delta is introduced. All helper members are UTF8 and roundtrip/hash checked. No compiled libraries/binaries, key/image/witness/proof bytes or third-party full text is embedded. This census adds no performance, native-kernel, AWS/Aeglos/Gnark, device, low-memory or throughput qualification.

[Result](artifacts/result.json), [independent review](artifacts/review.json), [model](artifacts/model.rs), [design](artifacts/design.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [helpers/receipts](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS).
