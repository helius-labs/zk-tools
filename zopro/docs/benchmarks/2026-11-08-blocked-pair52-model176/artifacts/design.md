# Two local Karatsuba tiles in paired CIOS rows

Frozen before the new model runs, 8 November 2026. Goal: verified complete
CPU AVX proof latency at least 10x the supplied M5 CPU timings for all eight
shapes, with both CPU and Metal comparisons. This screen cannot satisfy that
goal. Production/source176, accepted SDK6d83 and proof inventory35171 stay fixed.

Prior two-row CIOS (25 October) accumulated two product rows before two
cancellations with seven coefficients and unchanged 100/150 IFMAs. It failed
component and fused-only SDK gates. Prior full signed-pair convolution uses
ten coefficients and 80/110 IFMAs; complete callers failed. The latest deferred
sign change improved the old full-pair recipe but lost against original CIOS.
Do not rerun or reinterpret any of those observations.

This distinct fixed schedule replaces only the 2x2 tile at (a0,a1;b0,b1)
and the tile at (a2,a3;b2,b3) by three products. Remaining terms use original
schoolbook products. It retains seven coefficients, two product rows followed
by two cancellations, and the original final row. No tile/layout/flag sweep.

For a tile with diagonal products D0=ai*bi, D1=aj*bj and
P=(ai-aj)*(bj-bi), its cross term is D0+D1+P. Use the already-qualified
absolute-first/oriented-second signed pair word identity, not a new native
signed multiplication. Accumulate split radix52 words at the correct two
positions; P's high word can be negative. The same construction applies to
both terms of a fused product. Modulus/inverse constants and R256/R260 are
unchanged. Difference2 uses the original positive 2p-c complement.

Expected ordinary: 46 raw + 50 reduction =96 IFMAs; fused:92+50=142.
Original CIOS:100/150. Full pair:80/110. Local tiles add signed-pair and
recombination ALU work. These counts are not runtime predictions. Maximum
coefficient-array payload is seven vector words versus six CIOS/ten full-pair;
temporaries, inputs, compiler allocation and stack traffic are additional.
There is no table, key/point copy, retained plan, new pool or heap buffer in
the proposed arithmetic. The mathematical model is a host analysis tool.

Run the new model once through the existing Rust xtask workflow, retaining
failures. It must check both primes/radices, multiply/sum2/complement-difference,
inclusive [0,2p] inputs, radix/carry boundaries, full-width deterministic
tuples and changing chains. Check every quotient and complete integer after
each cancellation against an independent whole-integer CIOS recurrence;
check final result against full-R REDC and modular inversion. Check both
local tile identities coefficient-wise and by an independent 2x2 schoolbook
oracle, including exhaustive small radices. Propagate conservative signed
interval bounds through the proposed coefficient schedule and verify all
modeled intermediates fit signed64. Preserve exact observations, no clocks.

A separate offline reader must verify pins/receipts/source identities and
reconstruct counts, interval bounds and tile polynomial identity without
reexecuting the mathematical population. A passing screen selects only a
native implementation/correctness/codegen check. It establishes no measured
speedup, memory peak, portable-device behavior or adoption. A native screen
must compare exact original, fresh OFF, row-pair schoolbook and local-tile
arms in changing-input threaded callers with automatic profiles, before any
SDK/all-shape/Gnark/resource qualification. No unchanged historical reroll.
