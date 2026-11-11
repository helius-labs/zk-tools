# Cost map and final-fold feasibility probe

Reuse completed source-bound stream arithmetic and the latest three OFF cycle
profiles; do not rerun them. Bind the five stream-model sources to current hashes.
Compute the remaining all-eight latency gap from the latest matched accepted
measurements, retaining supplied CPU and Metal references. Aggregate exact symbol
residence with complete sample conservation. Treat CPU shares as CPU shares,
never as exclusive critical-path time or guaranteed removable latency.

Distinct question: can moving binary-radix final window scaling into the existing
window tasks remove a material serial post-join tail? The alternative would
compute sum_j [2^(bits*j)]S_j in the window tasks, retaining only final projective
results; it adds doubling work and may delay the last task. No implementation or
speedup is presumed. Earlier bucket-Horner algorithms change bucket accumulation;
this question concerns the final fold after weighted window sums.

First measure only the accepted final fold. Pin the existing coherent control
field library from g2-coordinate-leaf177, Rust1.98.1, znver4, O3, fat LTO, one CGU.
No field, core or prover rebuild. One process pinned to CPU2, one thread. Six
prospective G1/G2 binary-radix contexts: widths12/13/14, ceil(255/bits) G1 windows
and ceil(127/bits) G2 windows. Width14 G1 uses19 windows, matching that formula.
For each context prepare eight deterministic full-width generator-weighted banks.
Zero/mixed banks are checked; six ordinary banks are timed. Check complete output
against a separately calculated scalar-power dot product. This shares the field
library and is not independent field qualification or an actual query fixture.

Each context:18 excluded primes,5 consecutive blocks of512 folds, every output
black-boxed, last output checked after each timer. No build/hash/other owned work
overlaps measurement. Preserve all samples; no extensions or fastest selection.
This tests absolute single-thread service scale, not paired candidate performance,
SMT contention, SDK latency, input capture, RSS/minimum RAM or a strict wall-time
upper bound. Contexts are diagnostics, not an adoption gate or parameter sweep.

Compare service scale with the observed primary gap of about4.12ms. If negligible,
deprioritize the final-fold idea without full query or SDK qualification. A small
potential gain is not a failed adoption result; this is research prioritization.
If substantial, next measure actual caller spans and overlap before implementing.
Do not choose another helper/annotation variant from these results.

Canonical verified-export ledger remains53917 at
g2-coordinate-leaf-profile177/proof-sha256-after-profile.json. No new proof or SDK
call, production mutation, AWS resource, push or publication is authorized here.
