# Deferred equality checks in ordinary G1 bucket filling

This is a new private diagnostic candidate. It does not change production or
claim a proof-latency improvement. The preceding digit-plane experiment remains
rejected; this candidate allocates no digit matrix.

The original finite bucket admission path compares each current X with the
incoming X, handling doubling/cancellation immediately. The candidate retains
the current-identity shortcut, borrowed points, unique busy indices, queue,
batch sizes, scratch, original field operations and finish. It admits finite
equal-X pairs into the normal batch instead. The product tree already used by
the central eight-chain inverse detects zero through the existing scalar
inverse's Option. A zero aborts BEFORE any bucket write and applies complete
curve addition to the distinct batch destinations. Nonzero uses the original
forward and reverse arithmetic. Tail factors join that same test before any
tail output. No probabilistic exceptional-case assumption is made.

The new checked inverse copies the existing inverse_nonzero operation sequence,
replacing its scalar inverse unwrap with ?. This adds one fallible return per
batch while removing per-arrival equality tests from this diagnostic feeder.
It is a different mechanism from native coordinate columns, norm trees, prefix
peeling, shared inversions across windows, and changed queue draining. Original
functions remain byte-exact. No new unsafe code, field-crate arithmetic,
input-sized buffer, key table, solver plan, pool or memory cap is introduced.
The old and new feeders may produce different internal exception states, so
complete outputs, adversarial repeated/opposite points and queue interactions
must be checked. A fallback pays the speculative forward pass plus complete
scalar curve operations; pathological valid inputs can be slower.

Qualification precedes any performance capture: all original core tests plus
checked-inverse zero masks, independent complete signed-batch generator oracles,
and complete-window exception/queue cases in 1/4/16/32-worker 512 KiB pools.
Field qualification is inherited only after exact source checks. No portable,
whole-prover, pressure, phone, giant-circuit or SDK qualification is implied.

The intended next screen reuses the existing independently checked raw-A
fixtures and their 21 ordinary width-12 bucket oracles for both 1x2 and 2x2.
It must include admission, allocation, final normalization and destruction.
These raw pre-group queries are not necessarily the populations of the final
production proof route. A full-proof trial is required after a successful
caller screen. No speedup percentage is predicted from removed comparisons.

History inspected before selection: native bucket52, full digit planes,
unit-prefix, packed trees, G1 prefix-only records, G2 cached X, grouping and
queue-drain, GLV final assembly, B1 omission and basis images. A bounded report
and handoff search found no prior aggregate-zero deferred equal-X feeder trial;
this is not an exhaustive novelty claim. Paper searches rediscovered already
closed SRE/Hrabar/PSSM work and supply no transferred performance claims.

The original objective remains verified single-proof CPU latency at least 10x
the supplied M5 CPU timings for all eight shapes, with CPU and Metal comparisons.
Current accepted source has zero of eight at that target. This prototype alone
is not evidence of a twofold full-proof opportunity.
