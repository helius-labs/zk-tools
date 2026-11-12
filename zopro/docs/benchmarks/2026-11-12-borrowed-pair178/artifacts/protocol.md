# Borrow both G2 pair operands

Hypothesis: parallel bucket trees pay avoidable intermediate-point staging costs.
The retained borrowed-tree append copies each generic pair's left point into its
destination; the existing in-place adder then reads and overwrites it. A new
24-byte descriptor borrows both inputs, encodes both signs and a destination, and
writes each result once. The former descriptor was16bytes. Keep the tree geometry,
route construction, pair order, exception behavior, padding, merges and weights.
Retain initialized output buffers across levels/bands to avoid replacing the left
copy with repeated zero initialization. Charge growth and retained memory.

Start from the177-file adopted baseline. Three process-fixed modes in one caller:
0 original streaming;1 retained staged tree;2 new borrowed-pair tree. Old/new trees
use checked reservations before scalar mutation; unchanged fallback on denial.
Both preserve two point parts, width12,256-bucket groups, native phi grain and top
batching. No lookup table, key copy, new pool or production edit. This is a private
prototype, not an adopted general-purpose API.

First qualify focused arithmetic and complete tree cases: both sign masks,
partial/full SIMD batches, poisoned destinations, exceptional identities/equal-X,
skew/reuse and1/4/16/32-worker512KiB stacks. Reuse retained independent complete-MSM
fixtures/oracles. Native tests share accepted curve/field arithmetic; real query
outputs have separate retained Arkworks oracles. No proof exports or SDK claims.

Freeze source, flags, binaries, inputs, environment, order and helpers before any
caller preflight. Reuse Rust1.98.1/znver4/O3/fatLTO/oneCGU and the existing driver.
Use3distinct byte-identical executable backings. Six contexts:1x2 and grouped2x2,
at1/16/32 fixed-affinity workers. Three excluded primes,17timed calls,one postcheck;
all outputs checked.18ordinary one-round preflights plus primary32 mode2 with
ZOPRO_CPU_PREPARE_MAX_BYTES=0 must pass, with exact admitted/fallback counters.
Then54 main processes in3rotated/reversed blocks. Retain every sample and copy;
no timing extension, exclusion or changed implementation after capture starts.
During live preflight/recording only poll its confirmed workflow handle.

Report all arms, paired ratios, CPU service and peak RSS. This is a directional
screen, not precise small-gain qualification. New-versus-old tests the staging
change; new-versus-streaming tests whether it helps the accepted algorithm.
Progress to full-proof qualification only if new beats streaming on primary32 in
each block and neither small shape's16/32 context exceeds1.03 in any block.
An inconclusive or failed screen is not an arithmetic bug. Reduced bytes alone
cannot establish speed. No AWS or phone conclusion follows from local results.
Keep canonical proof ledger54927 and all-eight CPU10x goal unchanged.
