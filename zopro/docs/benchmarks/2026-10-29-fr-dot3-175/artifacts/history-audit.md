# Scope and distinction

Current sparse_poseidon.rs dense width-three branch uses the adopted two-product
field primitive plus an ordinary third product. The existing partial-round rest
already uses sum2, which remains untouched. Current x86_64 sum2 uses BMI2/ADX and
one reduction; generic Fr square remains the accepted backend.

Scoped search of local source and archived report descriptions found no scalar
sum3 primitive trial. This is not an exhaustive novelty claim. Earlier direct
Poseidon dot fusion used sum2+mul, and its candidate-only code was subsequently
included in accepted composed adoption. Earlier scalar symmetric squaring,
within-hash IFMA, across-hash batching/grouping, G2 raw Gauss products, RNS and
FIPS trials retain their recorded outcomes and are not repeated here.

New research searches inspected primary SimdMSM repository overview, ZKPU and
ePrint chiplet/MSM abstracts. No implementation or algorithm is ported. The new
candidate follows the current source's remaining second reduction directly;
no external paper or hardware performance result predicts its CPU gain.
