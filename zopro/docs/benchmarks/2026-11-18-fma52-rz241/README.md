# Exact 52-bit floating-point product with embedded RZ

**The fixed product recipe fails its progression screen.** It is exact in the
tested scope, but about2× slower with integer-storage inputs and41–42% slower
with prepared floating inputs. Every context has0/3wins. No field, point-kernel
or SDK integration is selected, and production remains unchanged.

This is a raw-product prerequisite, not a modular multiplication or full-proof
benchmark. Declining this recipe does not establish a bound on all floating-point
field algorithms or predict C7a performance.

## Distinct question and exactness

The closed [October23 FMA trial](../2026-10-23-exact-fma-rne-component/README.md)
uses51-bit round-to-nearest residue folding. This trial instead uses52-bit limbs
and embedded round-toward-zero/SAE. It retains the five-limb storage shape used
by the current field implementation, with either48 or52 bits in the top limb.

The biased high/low product comes from the indexed algorithm excerpt of the
[ARITH2018 GPU paper](https://www.ecs.umass.edu/arith-2018/pdf/arith25_17.pdf);
full PDF retrieval timed out. The Rust implementation, integer derivation and
native qualification are independent; no unseen section, imported source body
or published speedup is claimed.

For B=2^52, a,b<B, the biased high FMA encodes floor(ab/B). An exact subtraction
constructs the second FMA addend, whose result encodes ab modB. Fixed exponent
biases are removed once per convolution coefficient after integer accumulation.
A5×5 schoolbook product then normalizes all10 output limbs. There is no
prime-specific reduction, changed general field API, key table or heap allocation
inside these kernels.

Native qualification checks8192 input packets across both top-limb layouts
and all16 rounding/DAZ/FTZ combinations, restoring the original floating control
state afterward. Native IFMA, raw FMA and prepared FMA all match an independent
scalar-u128 schoolbook oracle: **1,966,080 output-limb comparisons** pass.
Boundary and deterministic full-width random inputs are included. These finite
checks are not a formal machine-code proof or complete field qualification.

The actual object has50 FMA sites, all with{rz-sae},50 IFMA sites in the control,
and10 integer-to-double conversion sites in the raw wrapper. Biased accumulation,
subtractions, carries, calls and output storage remain additional work. Static
sites do not establish dynamic instruction costs or removable proof time.

## Fixed component measurement

Ryzen9 9950X, Rust1.98.1, znver4, O3/fatLTO/oneCGU. One coherent executable
contains both arms. Three independently written byte-identical backings all
participate in three matched rotated/reversed blocks. Each process pins to CPU2
and uses one OS thread. No owned CPU cojob overlaps any capture.

Four functional preflights cover both arms and input layouts in the raw envelope.
All native terminal product oracles and workflow exits pass, followed by the
separate smoke reader, before the guarded24-process main run. Each process has
three excluded10,000-product primes, then100 or1,000,000 timed iterations for
smoke/main respectively. There are24,000,400 timed packets and840,000 priming
packets across28 capture processes. Each packet contains eight independent
integer products; these are not proofs.

Both envelopes materialize and consume all10 output coefficients, with the
low output limb feeding the next input. Black-box barriers prevent dead output
elimination and conversion hoisting. Prepared FMA excludes the initial input
conversions but includes conversion of the updated low limb each iteration;
raw FMA starts from integer arrays and converts all10 operand vectors in its
kernel. Normalization, calls and loop/output handling are timed. Initial setup,
final full-product oracles and checksum formation are outside the timer.
All112 terminal packet checks across primes and timed loops pass; an oracle is
not run on every intermediate timed iteration.

Values below are nanoseconds per eight-lane packet, not per field operation.
The256/260 labels describe input bit geometry; no Montgomery reduction occurs.

|Input bits|Envelope|IFMA ns/packet|FMA ns/packet|Paired FMA/IFMA|Wins/3|
|---|---|---:|---:|---:|---:|
|256|Prepared|8.640169|12.186414|1.417320|0|
|256|Integer storage|8.630631|17.153709|1.997758|0|
|260|Prepared|8.616744|12.209258|1.411720|0|
|260|Integer storage|8.617867|17.209985|1.996244|0|

Times are medians; paired ratios compare corresponding blocks and need not equal
ratios of the displayed medians. All observations and order remain retained.
Three blocks provide no confidence interval or precise production penalty.
The fixed primary gain and other-context guards fail, without a minimum gain
floor, extension, exclusion, width/rounding sweep or favorable subset promotion.
This closes the exact backend prerequisite without another field implementation.

## Execution, current goal and artifacts

The first native link fails because an unrelated retained libc rlib lacks LTO
bitcode. Direct Rust FFI for the existing system CPU query removes that dependency;
the original source, command and failure are preserved. No C source is introduced,
and no qualification or timing ran in that failed link. Qualification and all
actual captures then succeed on their first scientific executions.

The separate reader binds plans, raw records, checksums, command arguments,
stream hashes, codegen and three independent backing identities. A final audit
checks32 prior preparation/capture command receipts and64 stream hashes,
including that one expected failed compile. Main waits for the successful
native smoke semantics and separate reader. No successful capture is repeated.
This is a separate same-agent review, not an external audit.

Protocol to first complete result is271.72seconds; active capture sums0.320seconds.
Both fit the1200/120-second allowances. Final audit preparation reaches385.52seconds.
Earlier history/paper review is outside these clocks; no authentication wait is
subtracted.

This turn's AWS STS check again fails expired SSO/refresh, the third consecutive
goal turn with that observation. Read-only checks confirm the188 production
sources, prepared239 tools/package/payload and four existing proof/public hashes
are intact. No cloud resource is created. The [C7a comparison239](../2026-11-18-adopted-c7a239/README.md)
remains prepared and unrun. The full goal stays active because this turn resolves
an independent arithmetic question.

No SDK call, proof export, solver run or new performance profile is added.
Canonical inventory stays62191, SHA256
`c2407f7fd7d43e32a685db92664fc94facac7c2608ba7843419be07629700cf3`.
Accepted full-proof CPU/Metal comparisons remain [234](../2026-11-18-paired-weighted-all234/README.md),
with236 provenance; the all-eight verified CPU10× objective remains unmet0/8.
No mobile, memory-pressure, minimum-RAM, huge-circuit or throughput qualification
is implied. Publication awaits user approval.

[Assessment](artifacts/assessment.json), [protocol](artifacts/protocol.md),
[qualification](artifacts/qualification.json), [separate review](artifacts/review-main.json),
[timing](artifacts/timing.json), [sources/plans/raw commands](artifacts/sources-plans-raw.tar.gz).
The archive contains no proving keys, witness/scalar fixtures, proofs, full
inventories, executable binaries or raw PID/TID streams.
