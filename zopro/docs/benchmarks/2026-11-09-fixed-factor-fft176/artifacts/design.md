# Bounded native fixed-factor FFT qualification

Integrate the already qualified fixed-factor arithmetic into the actual stored
FFT's terminal serial leaves. Preserve the integer recipe and all higher stages,
task splitting, packing, weights and pointwise products. No performance claim
follows from the reduced IFMA count. New preparation expands four compact scalar
factors to duplicated eight-lane factors inside every consuming butterfly.

Retain at most the last five quad layers (31 quads), pair and optional single
quad. Each scalar factor is 80 bytes. Three direction payload is at most 31,680
bytes. Charge allocated Vec capacity and the entire boxed structure, including
inline pair/single, offsets, Vec metadata, padding and reservation token before
allocation through Reservation::analysis. The optional pointer in each Twiddles
and transient conversion/expansion stack are additional; no RSS/minimum-RAM
claim. Empty/denied caches preserve the original arithmetic. No new pool,
whole-domain quotient table or artificial admission ceiling.

Create a separate immutable snapshot from the native qualification. Do not edit
that parent. Four original parents change: field exports, fft.rs, pair.rs
(diagnostic facade only), pair/leaf.rs. Two children are added. The existing
field body and fixed-factor child remain byte-identical. Preserve every old
production source in an audit diff. Production checkout remains unchanged.

Qualification uses a coherent normal-release core and the already-qualified
normal-release field. Route counters and a forced-denial hook are diagnostic
only; no timing will use them. Independent Arkworks sparse-polynomial oracles
check each complete inverse and forward separately, ordinary and twisted roots,
both independent packed halves, root powers 1/3 and cosets 5/7. Product/Euler
consumers, zero/padded inputs, release/rebuild, actual zero configured admission,
and concurrent 512 KiB workers exercise fallback and ownership. Test all outputs;
do not infer production correctness from round trips or plain quotient alone.

Compile generic x86-64 with no_ifma/no_asm and the feature flags requested. This
is a guard compile, not execution on ARM or phones. Prior field/model test
populations are not replayed. No proof, SDK or speed selection in this stage.
After correctness, inspect actual normal-release emitted consumers and freeze
matched reference/OFF/ON timing plus profiles. Only verified full-proof/all-shape
measurements may support adoption or the M5 objective.
