# Mathematical review and native-selection decision

The exact model passes; no production-backend candidate or performance trial is
selected. This is a selection judgment, not a measured slowdown or impossibility
claim. Original all-eight verifiedCPU10x goal remains active/unmet0of8.

The full-width random Fq cohort has288832->199840 smallmatrixjumps (-30.81%),
37236->51922 integerrows (+39.44%),20460->27302 modularcoefficientrows (+33.44%)
and66121->68144 finishquotients (+3.06%). The new recipe also evaluates580071
additional source quotient-multiply expressions (~283.24/input). Fr has the same
tradeoff:288763->200153 jumps,37270->52064 integerrows,20448->27436 coefficientrows,
65701->68481 finishquotients,580893 new multiplyexpressions. These are semantic
operation counts, not retired instructions, cycles or removable proof fractions.

Fewer small-loop jumps therefore come with substantially more full-width updates,
coefficient work and new quotient products. We decline to spend the next native
iteration on this fixed recipe. No native latency prediction or fixed weighted
instruction-cost threshold is used. No BITS/quotientset/swap/headroom sweep or
rescue follows. The native selection review explicitly required in design.md has
now completed with a negative decision.

The original model-result.json field selects_native=true is RETAINED. It is the
negation of the narrow four-counter domination predicate, not completion of the
required tradeoff review; native_selection_requires_complete_tradeoff_review is
also true. The recipe reduces small-loop jumps, so that predicate is false even
though the tradeoff is unattractive. Do not silently relabel the raw predicate,
rerun the model or treat its label as native/adoption authorization. The separate
authoritative decision is selected_native_backend=false.

Observer scope correction: max_observed_row_L1 is updated after odd jumps. It
does not include an initial-only even strip. Retain those original numbers as
post-jump observed maxima, not complete algorithm maxima. The initial strip has
rows[1]=(0,2^first) with first<=BITS by construction; this satisfies the stated
bound independently. Every post-jump row bound is asserted; final determinant,
exact full-row divisibility, inverse invariants and reference outputs also pass.
No operation-count decision depends on this partial maximum counter. No finished
model execution is repeated to repair a metadata label.

The earlier direct-eight-independent-inverse idea was considered but not selected:
the existing root-sharing arrival/ready-wave studies remain closed, and no new
SIMD inverse algorithm/candidate is implemented here. G1 late-coordinate two-record
storage was also found already tested/rejected. This report does not reopen them.

Production176/SDK6d83, canonical35171 and all inherited proof timings remain exact.
This fixed mathematical recipe closes without candidate library, caller benchmark,
SDK/Gnark export, profile, cloud resource or adoption. Continue with a distinct
covered useful-work reduction; do not claim an inverse-only idea meets the full
all-eight target.
