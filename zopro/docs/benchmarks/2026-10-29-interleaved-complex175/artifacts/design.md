# Interleaved ordinary-basis complex products

Declared before model/native build or timing, 29 October 2026.

Previous goal turn is progress: a trace-one basis was modelled, tested, measured
and rejected. Its multiplication-only gain did not survive complete expression
costs. Production remains the accepted 175-file source and SDK 20fde19f; the
all-eight M5 CPU 10x objective remains unmet and CPU/Metal comparisons retained.

This candidate keeps the ordinary basis i²=-1, all field/scalar representations,
squares, norms and point formulas. Both outputs of a schoolbook complex product
share an interleaved row-CIOS schedule. Instead of materializing a five-vector
negative operand before either output, compute n_i=(2p-b1)_i in each row, using
one signed borrow and normalized radix-52 digits. For ordinary multiplication,
X=a0*b0+a1*n and Y=a0*b1+a1*b0. For conjugate multiplication,
X=a0*b0+a1*b1 and Y=a1*b0+a0*n. Four raw products and two REDCs retain
300 IFMA sites. This is not a three-product Gauss method, quotient precomputation,
a new field basis, or a repeated dual-independent-product experiment.

History reviewed: 25 October dual-CIOS pairs/triples share one multiplicand
between ordinary products and fail the later SDK primary gain; 26 October
streamed-complex-B uses three products and signed reductions and fails SDK;
29 October complex-rowq precomputes quotient weights (315 IFMAs) and fails the
threaded caller. Their results remain. No README/source match for this specific
joint schoolbook output schedule plus streamed complemented second operand was
found in that audit. This is a limited novelty statement about local history.

For normalized inclusive inputs [0,2p], n is exactly in [0,2p], each wide output
is at most 8p². For p<2^254 and R=2^256, REDC<3p; subtracting 2p when needed
returns a normalized output below2p. R260 outputs are already below2p. Model
both primes/radices/forms, digit borrow, quotient cancellation, mixed packing,
lazy edges, full-width random inputs and dependent chains before native code.

Native tests must independently compare normal and conjugate products against
Arkworks and check normalized output bounds. Run the complete native field suite
once for the candidate, then complete unfiltered OFF/ON core and prover suites.
Only a private native cfg changes Fq2 lane multiplication/numerator dispatch.
All 175 original files must recover exactly through inverse edits. No heap
buffer, per-key table, point-array copy, scratch expansion or worker change.

Measure actual complete owned unsigned and borrowed signed G2 prefix batches,
not an isolated arithmetic loop. Reuse the accepted frozen libraries only after
checking all source/flags/input pins. Compare frozen, fresh OFF and ON with the
same common fat-LTO harness. Retain all18 contexts:32/200/352 pairs x1CPU2,
16physicalCPUs0-15,32logicalCPUs0-31 xowned/borrowed. Additional199/351 tails
are correctness-only preflights.15 fixed rotated/reversed blocks;3 checked/reset
1024-call primes and8192 timed calls/worker. Whole-wave timing includes inversion,
packing, coordinate arithmetic, output scatter and completion of all workers.
Generator scalar-sum oracles, outputs, capacities and zero timed allocations
are mandatory. Scratch/reference capacities must match all variants exactly.

Every OFF/frozen interval inside[1/1.03,1.03]; every ON/control upper<=1.03.
All18 and threaded12 equal-log aggregates need upper<1 and>=10/15wins against
BOTH controls. Rank4..12/15 intervals give nominal96.484375%individual coverage
conditional on exchangeable independent blocks, nofamilywise guarantee.
No gain floor, subsets, threshold changes, extensions, layout fishing or unchanged
reruns. A pass only selects a separately declared complete SDK qualification.

All latency precedes six separate FIFO199Hz cycles profiles:borrowed200w16/w32
andowned200w32 OFF/ON,262144 calls/worker. All captures precede owner transfer,
decoding, event/loss audits and analysis. CPU/time/MMAP/task metadata explicit.
All measurement/analysis/publication helpers compiled before captures; no owned
compiler/helper/hash/archive or second CPU workload overlaps them. Rust/xtask,
no new Python. Commit and push the completed result even when negative.
