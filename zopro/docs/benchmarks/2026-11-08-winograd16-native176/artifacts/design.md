# Fixed global DFT16 factorization, before model execution

Original objective remains verified CPU AVX single-proof >=10x supplied M5 CPU
on all eight shapes, with CPU/Metal comparisons. This one arithmetic step does
not replace that objective or claim a primary-proof breakthrough. Production176,
acceptedSDK6d83 and canonical35171 unchanged. No budget/questions/subagents.

History: 2026-10-29-winograd8-caller175 already tested the four-product DFT8
factorization; its actual caller was about12% slower and remains CLOSED. Do not
rerun it unchanged or treat extending its batch width as a new result. The new
DFT16 globally factors the eight odd outputs into two independent three-product
rotations, instead of attaching seven twiddle products to a separate DFT8. Its
13 scalar constant products are two fewer than composing two factored DFT8s plus
seven roots, and four fewer than unit-pruned radix2's17. All13 products have one
bilinear layer, preceded/followed only by linear operations. No old DFT8 timing
or successful qualification is replayed. Standard split-radix176 just closed
with no Fr arithmetic saving; that gate remains false.

Primary reading: Raciborski/Cariow2022 Electronics11(9)1342 abstract and partial
indexed equations were accessible; publisher direct open429 and PDF inaccessible.
Do NOT claim the full paper or its factor matrices were reproduced. No upstream
GPL Winograd implementation is read/ported. The identity below is independently
derived from the DFT definition and two three-product symmetric rotations, using
the previously recorded local DFT8 mathematical identity for the even half.
No novelty or imported hardware-speed claim. See derivation.md for exact signs.

For primitive16th root w, i=w^4, a=(w+w^-1)/2, b=(w^3+w^-3)/2,
d=(w^2+w^-2)/2. After x_j±x_(j+8), even outputs use four products of the local
DFT8 equation. Odd outputs use d*A2, i*d*B2, i*T4 and two three-product rotations
on (A1,A3) and (B1,B3). Total13 constant products and74 scalar add/sub operations
per DFT16, counting i as an ordinary dense Fr product. Root setup is separate.

First stage is mathematical qualification in Fr, with independent Arkworks
direct DFT and both direction/order interpretations: DIF natural->bitreverse;
DIT bitreverse->natural. All eight primitive roots, all16 basis inputs, zero,
one, negative one, fixed fullwidth randoms, modulus-neighbor/word boundaries,
and repeated changing states. Check every coefficient of each resulting16x16
matrix independently; linear matrix equality qualifies this identity conditional
on field arithmetic. It does not prove new native lane routing or field bounds.

Two independent16-point sequences could pack26 product inputs into four existing
eight-lane Ifma8<FrConfig> products (32 lanes,6 dummy). The accepted four-layer
leaf uses six such products for the same two transforms. This is a prospective
source count, not linked instructions or speed. A fresh native implementation
must include all inputs, shuffles, add/sub/correction, constants, dummy lanes,
outputs, both orders, and actual final caller. Do not substitute scalar count or
isolated field multiplication. No new field arithmetic, unsafe pointer access,
C/ASM, key table/copy, input-sized buffer, retained execution plan or pool.
Four factor vectors have1,280 raw vector payload bytes per root plus original
controls/routing state; not process RSS, a stack bound or minimum RAM.

Necessary mathematical gate: independent outputs, all matrix entries and count
identities pass; candidate constant products must be strictly below17 for every
root. Extra linear work is explicitly allowed (74 versus64) and must be charged
by future actual callers. No percent gain floor or rescoring after observation.
If math fails, preserve it and repair only genuine implementation mistakes before
any timing; do not pick new coefficients/order/roots to rescue an algebraic claim.
If math passes, select native packing qualification, NOT performance or SDK.

Only if native qualification passes: freeze a separate complete caller protocol
with actual linked codegen, frozen accepted/fresh original/ON, both directions,
all primitive roots, 1/16/32 workers, changing checked output state, zero timed
allocations and automated PMU/profiles. No owned CPU build/helper/hash/analysis/
report/push during any performance recording including preflights. Successful
model/build/test/capture/decoder/verifier commands are not rerun for bookkeeping.
Fixed native gate must precede clocks; no worker/root/order/layout/host/compiler
sweep, favorable subset promotion or retiming after failure. Any passing caller
still requires a separately qualified packed whole-FFT/Euler operator and then
actual SDK/all8+54/primary16/Gnark/resource/portable evidence before adoption.

Commit AND push each finished experiment, with factual all8 CPU/Metal comparisons
and explicit inherited scope. Goal remains active until verified all8 targets.
