# Three-product scalar Poseidon dot

Declared before implementation or performance. Previous goal turn was progress:
implemented/tested/published the negative FIPS experiment f77798c4. Accepted
source175/SDK20fde remains1x2=8.022165ms/max134MiB; all-eight CPU10x goal is
active/unmet0/8, with CPU and Metal comparisons preserved. No budget limit.

Current width-three dense rows compute sum2(a0*s0,a1*s1)+a2*s2, using two
Montgomery reductions. Hypothesis: a pure-Rust fused sum3 can remove one full
reduction. Only these dense rows change; partial rounds retain their existing
pre-head two-product computation and all S-boxes/writes/scheduling are exact.
This is a modest solver work reduction, not a2x prediction. Safe u128 code may
lose against the accepted BMI2/ADX implementation despite fewer products.

New field helper sums three canonical products using four CIOS rows with five
u64 accumulators, then the existing single conditional subtraction. No new
assembly/unsafe/heap, prepared constants, key tables, copied plans, or memory
policy. Method is private-experiment cfg gated; only Fr dense callsites select
it. Both primes are correctness controls. No IFMA representation changes.

Qualification: two new independent tests (small-radix exact identities and
actual four-word results/chains against Arkworks plus accepted arithmetic),
all35 candidate/33 control field tests,124 core and8 prover tests per build.
Source176/172 original exact; reversing three parent edits and deleting the
new module restores all175 accepted files. Actual linked solver/dense code
and source/cfg fingerprints audited before timing.

After native qualification, fixed165-step primary SDK pilot: A accepted,
B newly built OFF, C ON;15 rotated/reversed blocks151 timed requests+3 primes,
all9-shape preflight,3 grouped-event stats+3 profiles.7902 total SDK requests,
78 fresh independently Gnark-verified exports, ledger17395->17473. Every helper
compiled and all pins frozen before captures; no other owned work during them.
Gain upper bound<1 and>=10/15 wins against both controls, OFF/A calibration
within factor1.03. No gain floor, excluded captures, extensions or successful
reruns. A failed pilot closes this exact kernel. A pass requires separate
all8+54/16/portable/resource/clean qualification before any adoption.

Commit/push the completed result, including negatives. No new AWS/Aeglos/phone/
giant-circuit/minimum-RAM/active-pressure claim. Preserve accepted physical16
baseline-calibration engineering exception; never relabel the full gate passed.
