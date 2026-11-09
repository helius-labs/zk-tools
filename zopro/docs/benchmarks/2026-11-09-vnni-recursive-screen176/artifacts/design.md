# Recursive convolution and VNNI: complete-field work screen

Goal remains verified single-proof CPU AVX >=10x supplied M5 CPU on ALL8,
with CPU+Metal comparisons. Active/unmet0/8. This is a new arithmetic screen,
not performance evidence or a substituted goal. No production change,
lookup table, copied key/point/scalar/plan, pool, C/ASM rewrite or RAM cap.
Rust1.98.1/xtask, no questions/subagents, commit AND push completed work.

The October30 radix13 VNNI screen covers direct schoolbook convolution.
It explicitly leaves other multiplication algorithms open. New question:
can recursive Karatsuba or Toom4 reduce the complete Montgomery work below
accepted IFMA budgets once word ranges and actual G1/G2 consumers are priced?
Do NOT rerun the old VNNI/IFMA calibration or its old semantic fixtures.
Prior component-lane layouts, FMA, PMNS, guard batching, phase outlining,
rank contraction, ring/GLS4/window and PGO results remain closed.

Four fixed algebraic recipes, no parameter search:
K16:16digits×16bits, split16->8->4,9schoolbook leaves.
K20:20digits×13bits, split20->10->5,9leaves.
K24:24digits×11bits, split24->12->6->3,27leaves.
T24:24digits×11bits, Toom4 four6digit blocks,7schoolbook leaves.
All exact integer polynomial identities apply; direct native signed-word
eligibility must be checked separately. K16 is intentionally included as
an optimistic count/range stresscase, not an assertion it fits i16.

Toom4 evaluation points0,1,-1,2,-2,1/2,infinity. Half evaluation is scaled
by8 for input polynomials and64 for degree6product. Precompute fixed7x7
rational inverse once; every reconstructed coefficient must divide exactly.
Raw products are computed with signed i128 coefficients. Independently
compare polynomial evaluation with BigInt integer products.

Montgomery recipe is explicit: full recursive product T; full recursive
product of (T modR) and -p^-1 modR, taking lowR as q; full recursive q*p;
U=(T+q*p)/R. Fusedsum/difference form two separate raw products before the
shared q and q*p steps. No unimplemented truncated recursive conversion or
free modular quotient is charged as an established algorithm. Each complete
field uses3ordinary or4fused convolutions. Normalize into[0,2p), including
K16sum's possible extra correction; prove sufficient bounds separately.

Test bothFq/Fr,256boundarytuples+512fullwidth randomtuples per recipe/field,
all3operations:18432newfieldoutputs. Independently use direct BigInt product
and Fermat radix inverse for residues; q uses extendedEuclid modulus inverse.
Also validate every constituent convolution. Fixed16boundary operands
include lazy2p ranges and canonical2^253-1; preserve full signed coefficients.
Record counts and exact q/divisibility/outputclosure assertions.

Count scalar wordproducts from actual recursive leaves; translate to ideal
16field VNNI instructioncapacity (32wordproducts/instruction) and literal
per-coefficient two-product pairing (16fields/instruction, one coefficient
per outputlane). Grant allcarry/interpolation/packing/control free in those
optimistic ledgers. Compare complete16field budgets with accepted200/300
IFMAs, and actual16point G1(6ordinary) andG2(7ordinary+5fused)=1200/2900.
No weighted native latency forecast follows from counts. Reject these fixed
recipes before fieldimplementation if even ideal packet counts are >=the
acceptedpacket counts. A primitive-only favorable row cannot select a
slower wholepoint mixture. Non-fitting wordranges additionally disqualify
the literal nativepacking; no postdata digit/radix/interpolation rescue.

Range checks: highest Karatsuba evaluation coefficients; leaf central-column
bounds and i32accumulation. Native DPWSSD checks only NEW evaluated-word
populations:3fixed(maxword,terms) panels=(16376,3),(32764,5),(30705,6),
1024vectors×16lanes each=49152laneoutputs, plus fixed valid-field overflow
counterexamples. Check wrapping i32 semantics against exact i64 sums; do
not call wrapped overflow an exact wide coefficient. Preserve counterexample
2^253-1 for Toom4 leaf accumulation and sparse canonical K16packing failure.
This is a focused intrinsic/range test, not a native field implementation.
Decode the new actual dot helper once; no clocks or performance selection.

Pin original176production,acceptedSDK6d83,canonical37633,oldinstructionrows/
AMDguide identities and allnewsource/model/codegen/receipts. New producer,
separately written reader, source recovery, all output/hash/bound/count checks
before report/push. SharedBigInt and ISA semantics limit independence.
No new proof/SDK/Gnark/AWS, RSS/minRAM/pressure/phone or fresh latency claim.
Carry forward latest accepted1x2=8.031248742 and4x2=15.961623323;other6
inherited. Preserve historical7.915095; no adoptedregression fromnew8ms.
