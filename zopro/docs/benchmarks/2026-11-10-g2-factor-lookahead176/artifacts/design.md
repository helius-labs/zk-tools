# One-group reverse factor lookahead

Goal: verified single-proof CPU AVX >=10x supplied M5 CPU for all eight shapes,
with CPU and Metal comparisons. Current accepted local primary7.829285503ms,
133.50MiB;0/8targets. This is one dependency hypothesis, not a 2x prediction.

Only signed-borrowed G2 prefix finishing changes. Prepare the last factor and
advance its inverse once. Before finishing group j, prepare group j-1's factor
and advance its inverse, retaining that factor until the next iteration.
Every original factor and inverse update occurs exactly once. One prefix chain,
the forward pass, norm/prefix records, scalar-tail service, point formulas,
signs, pair order, output stores, and field1200 stay original. No heap buffer,
extra vector product/inverse, table, key plan, pool, parameter sweep or RAM cap.
Transient live vectors and compiler stack/code size can change.

This is not the rejected two-chain streaming method (which adds3root products),
four-chain phased inversion (extra scratch passes), denominator retention,
descriptor chunking, or direct signed REDC. Those outcomes remain closed.
The new schedule may expose next-factor work earlier to the compiler/CPU;
it may instead add branches or register pressure. Only actual callers can decide.

Private native compile-time guard only; OFF and generic release retain the
original dispatch. Test-only original body is byte-exact. Qualify exact factor
trajectories, independent full-width Arkworks changing G2 outputs, empty/full/
odd/tail groups, FAST modes, both inverse modes, input immutability, scratch
reuse, real512KiB1/4/16/32worker contexts and4concurrent callers sharing8workers.
No successful unrelated field/core/prover suite is replayed for bookkeeping.

Then freeze real complete-query controls/oracles/protocol/reader before any
preflight. Preserve all contexts, positives, and failures. Serial xtask and only
poll exact confirmed live handle during recording; no helpers overlap. No
unchanged rerolls or favorable worker/threshold/curve/layout rescue. Passing a
caller screen only permits full ALL8+54 SDK/Gnark/resource validation. Commit
ANDpush factual complete results; canonical43730ledger unchanged unless new
independently verified exported proofs are actually added. No questions/agents.

Research: MQX(arxiv2509.12494) requires proposed instructions and uses <=124bit
FHE moduli, not the existing BN254 field; its reported speedups are not used.
Hrabar/SimdMSM/memory-bucket mechanisms already have local audits/trials. No
external code is copied. The source schedule above is a local hypothesis.
