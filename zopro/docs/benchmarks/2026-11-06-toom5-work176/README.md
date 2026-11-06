# Base-field Toom-5 work screen

The fixed nine-point Toom-5 recipe passes the arithmetic/range screen but is **closed before native implementation**. Once widened products, exact interpolation divisions and coefficient assembly are counted, it has more ordinary IFMAs than the retained recipe and many additional operations. This is a work-based decision about this explicit schedule, not a measured slowdown or a lower bound for all Toom algorithms. Production is unchanged.

## Arithmetic and scope

The scheme treats five normalized radix52 limbs as degree-four polynomial coefficients and evaluates at0,±1,±2,±3,4 and infinity. A sparse even/odd interpolation reconstructs the degree-eight product. Seven point operands can exceed52bits; at4 an operand is below341B, B=2^52. Their products therefore require high-part corrections rather than one52×52 product each. Exact divisions by small odd constants also require work across the three-word representation. The common interpolation tape was checked symbolically over all nine coefficient basis vectors, with64 operation nodes and exact coefficient divisibility. General range arguments and the actual three-word instruction recipe are in [bounds.md](artifacts/bounds.md).

The new normal Rust model checked **240,048 field convolutions and mathematical Montgomery references**, **6,144 full-width limb convolutions**, and **10,305 signed exact divisions**. Field cases cover Fq/Fr, R256/R260, ordinary/sum2/positive-complement difference2, inclusive[0,2p] boundaries, full-width random tuples and changing chains. Every reconstructed coefficient and full product matched independent BigInt schoolbook multiplication. Every modeled stored arithmetic lane satisfied signed64 bounds; intended modulo64 inverse products were checked against exact signed quotients. No invalid unreduced Fp objects were constructed. Public fixtures were inherited and reapplied to a different algorithm; the old model was not rerun and these are not new unique proofs.

Montgomery quotient references use independent extended Euclid and modular-power inverses. The inherited50-IFMA reduction skeleton was counted, not executed or requalified in this stage. Assembled coefficients are below3B, narrower than its earlier32B input envelope. No native field/kernel, ISA check, CPU profile, clock, solver, SDK or proof ran.

## Literal recipe work

| Stage | Ordinary IFMAs | Ordinary64-bit multiplies | Ordinary other ops | Fused IFMAs | Fused64-bit multiplies | Fused other ops |
|---|---:|---:|---:|---:|---:|---:|
| Evaluation and point products |18|21|175|36|42|431|
| Exact interpolation |56|14|510|56|14|510|
| Coefficient assembly |0|0|27|0|0|27|
| New convolution total |74|35|712|92|56|968|
| Including common reduction IFMAs |**124**|**35**|**712 + common reducer work**|**142**|**56**|**968 + common reducer work**|
| Retained IFMA reference |100|—|—|150|—|—|

Counts describe this literal normalized three-word recipe, not optimized instructions or latency. Common reducer ALU operations, constants, moves, code size, register allocation and spill costs remain excluded. Some source operations may disappear or be combined by a compiler; other schedules might reduce the cost. The optimistic ordinary total is already124 versus100 IFMAs, plus35wide multiplies and712other operations. The fused case saves only eight IFMAs while adding56wide multiplies and968other operations before common reducer costs. This fixed implementation does not justify a native benchmark.

Nine materialized three-word point products represent27 logical vector words, or1,728payload bytes for eight lanes, before other operands/scratch. This is a source payload model, not a concurrent-lifetime bound, actual compiler frame, RSS peak or minimum RAM. No lookup table, proving-key copy, execution plan or thread pool is introduced.

## Validation and evidence

The first build stopped on cast/comparison syntax in two assert macros, before any executable or arithmetic run. Four casts were parenthesized, with original source/pins/compiler receipts preserved. The repaired build and the model execution each succeeded once; no successful stage was repeated. A separate review verified22pins, the two successful command receipts, the retained compiler failure, unchanged176production-source identities, all64 symbolic bounds and the work ledger reconstructed from the operation tape. The public archive includes selected UTF-8 source, protocols and receipts, with extraction hashes checked.

The motivation uses the evaluation/interpolation framework described by the primary [GMP Toom-3 documentation](https://gmplib.org/manual/Toom-3_002dWay-Multiplication) and its [higher-degree generalization](https://gmplib.org/manual/Higher-degree-Toom_0027n_0027half). GMP's arbitrary-precision crossover results do not predict BN254/IFMA performance; no GMP implementation was copied. This base-field limb scheme is distinct from the closed [imaginary Fq6 Toom-3 trial](../2026-10-29-fq6-toom175/README.md). The proposed final48-bit coefficient-shift follow-up was found to be the already closed [CIOS packing qualification](../2026-10-25-cios-pack-qualification/README.md), so it was not reimplemented.

Production source176/SDK6d83 remain unchanged: local32 1×2 **7.928660ms**, merge54 **161.883899ms**. Inherited132.75MiB RSS is not a new peak or minimum-RAM measurement. Canonical proof ledger remains30,846. The inherited [all-eight M5 CPU and Metal comparisons](../2026-11-06-signed-pair52-model176/README.md#accepted-full-proof-baseline-inherited) remain current; the all-eight10× CPU goal remains unmet. No new AWS/Aeglos, large-circuit, phone, pressure/swap or concurrent-throughput result is claimed.

[Model result](artifacts/result.json), [symbolic tape](artifacts/symbolic.json), [protocol](artifacts/protocol.json), [review](artifacts/review.json), [pins](artifacts/pins.json), and [source/receipts](artifacts/source-and-receipts.tar.gz) are preserved. The inherited public fixture remains in the parent model archive; binary tools/libraries, private keys, witnesses and proof blobs are excluded. The observed wide maximum records checked normalized outputs, not a general bound or every standalone-division input; the general bounds are separate.
