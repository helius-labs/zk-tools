# Overlap independent canonical front stages

The accepted canonical-wire path computes public correction, converts witness
scalars in place, gathers A/B query scalars, and then computes the quotient
product. The product reads only the disjoint solved A/B evaluation vectors and
immutable FFT/key plans. It need not wait for the correction/conversion/gather.

Use one existing-pool Rayon join: one closure performs the complete correction
then canonical conversion and gather in the original order; the other performs
the original quotient product. Return the borrowed canonical words and owned
query vectors after both closures join. All MSMs still begin after that join.
No actor, point window, new pool, key/solver plan, lookup table, expanded point
array or new coordinate buffer is added. Existing vectors/capacities/formulas,
product normalization, small fused-KH guard, fallback and assembly remain exact.
The same already-owned inputs can have longer overlapping lifetimes. Measure
RSS; do not infer minimum RAM from the absence of a new field/buffer.

This is distinct from old five/four-query product overlap and solver-A actors:
no MSM starts early and no point-query scratch overlaps the product. It removes
the correction/conversion/gather-to-FFT barrier in the current three-query /
fused-KH dataflow, rather than reopening their schedules or rescoring old data.

Private reversible source; one changed original file. Original accepted field
library and every other production source remain exact. Native core suites and
all9 real SDK shapes plus primary1/16worker preflights must pass with independent
Gnark verification before a fixed complete-SDK measurement. No component-only
timing or microbenchmark can qualify this change.

Prospective complete-SDK screen: REF accepted executable, fresh OFF and ON;
ALL8+merge54 at32workers, primary at1/16workers. Fixed15rotated/reversed blocks,
31requests/process after3primes. CPU-affinity per worker;512KiB requested stacks.
Use process means and paired block ratios; sorted ranks3/7/11 for bounds/median,
no familywise coverage. All11 OFF/REF calibration intervals withinfactor1.03,
all22 ON/control nonregression upper<=1.02, primary32 and ALL8 equal-log gains
upper<1 and>=10/15wins against BOTH controls, all22RSS guards at5%+1MiB,
and every fixed PMU/profile quality check. No minimum gain floor. Merge54 has
no supplied M5 reference and is an explicit nonregression/resource condition.

No sample extension, exclusions, width/compiler/layout sweep, unchanged reroll
or favorable subset rescue. Retain all positives and failures. All recording
must finish before offline decode, analysis, independent Gnark exports or push;
during ANYpreflight/recording ONLYpoll exact confirmed live handle to terminal.
Canonical43730 must seed all new exports. Full objective remains verified CPU
AVX single-proof >=10x supplied M5CPU for ALL8, reporting CPU AND Metal. A
positive screen still needs concurrency/portable/resource qualification before
adoption. Commit AND push completed experiments; no questions or subagents.
