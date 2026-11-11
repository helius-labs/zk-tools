# G2 coordinate leaf

Use accepted production e387296070ea96090de8c50a521d494c1aed0158 as the base.
Recent matched byte-identical OFF profiles show higher G2 op-cache miss fraction
in the slower backing, without higher operation waits or a material demand-RAM
signal. This motivates an instruction-delivery hypothesis, not a proven cause.

The retained linked borrowed G2 function occupies 51,862 symbol bytes; a
conservative static block inventory finds a 1,351-instruction/550-IFMA straight
region spanning 8,671 bytes. Those counts are not dynamic latency or cache cost.

Introduce exactly one native borrowed G2 coordinate leaf per full eight-point
reverse group. It receives ax, ay, bx and the computed slope by reference and
returns X/Y arrays. Keep the original square/add/multiply/subtract formulas and
field implementation. FAST=false retains the original expression; private cfg
zopro_g2_coordinate_leaf defaults OFF. G2 owned-prefix, G1, inversion, forward
prefix, signs, order, scalar tails, scratch records, heap allocation and scheduling
are unchanged. No table, plan, point-key copy, production pool or memory cap.

The chosen never-inline boundary is the experiment, not an attribute sweep.
It differs from five per-product fused-Fq calls (rejected), whole forward/reverse
loop boundaries (rejected), and multi-group operator tiles with extra stack
records (rejected). No favorable subset of those old trials is reused as evidence.
Unlike a rolled CIOS recipe, this does not change field loop construction.

Hypothesis: a coordinate-level call breaks a long inlined reverse expression
at a useful instruction-delivery/lifetime boundary. Pointer arguments and array
returns can increase stack traffic and code, so a gain is uncertain. Inspect
actual linked caller/helper sizes, calls, IFMA sites and stack adjustments;
require that the intended helper is present and reached from the SDK G2 path.
No size reduction alone counts as a performance win.

Reuse the qualified changing-caller integration test adapter verbatim, test-only,
to exercise both curves, signs/tails/repeated updates, retained capacities and
all actual workers in 1/4/16/32 pools with 512KiB requested stacks. Run new native
core/prover suites and the selector-enabled generic core suite. Field code is
byte-exact; no new cryptographic primitive is asserted. Source recovery must
recover all 176 accepted files. Link coherent SDK controls/candidate from exact
Cargo libraries using Rust1.98.1, znver4, O3, fat LTO, one codegen unit.

Only after build/correctness/linked-code checks freeze a new whole-SDK screen
against accepted and fresh OFF, with predetermined byte-identical backings for
each arm, all retained. Include ALL8 plus merge54 functional checks, primary16/32
and larger-shape latency/resource checks, automatic profiles and fresh independent
Gnark verification. Missing ALL8 timing/resource/compatibility qualification
must finish before adoption. No hardware workaround or fastest-copy selection.

Canonical ledger starts52972 at ibs-backing177/proof-sha256-after-profile.json,
SHA4f86193860e35d6f16a765ab2aac7cbff561b747928a73f56a153d86a3cc202d.
Goal remains verified CPU10x for all eight shapes, with CPU/Metal comparisons.
No AWS/push/publication without applicable authorization; current hold forbids
push/publication. Existing native SDK and production sources remain unchanged.
