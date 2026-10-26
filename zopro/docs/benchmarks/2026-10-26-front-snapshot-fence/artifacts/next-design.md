# Next audit after the common-fence negative result

Close and PUSH this completed experiment before another. No next experiment is
declared or implemented. Do not rerun the unchanged matrix or sweep tile sizes,
backoff, collection intervals or combinations of prior failed scheduler modes.

This candidate reduces primary unprofiled CPU from200.668 to192.661ms/request
but the primary paired ratios are1.000669 versus accepted and1.007332 versus OFF.
The original8 geometric ratios1.018644/1.014440 are negative in every block.
All OFF calibrations pass. Thus it is not a failed-control ambiguity. Source
and correctness checks pass; no source or timing exclusions alter that result.

Profiles show the mechanism: primary OFFepoch10.008% -> ON3.821%, while scheduler
4.165% ->5.749%, voluntary switches417.65 ->619.32/request and system CPU7.314
->7.942ms/request. For12x2, total CPU957.89 ->951.74ms while system CPU21.55
->35.79ms and switches932.8 ->1821.5. These are separate profiled observations,
not a causal decomposition or removable wall-time estimate. At this point,
reducing scans alone has repeatedly failed to improve complete SDK latency.

One concrete additional finding: the linked scheduler `wait_until_cold` grows
from1,661bytes accepted to5,757bytes candidate, while scalar steal stays356bytes.
The common-fence implementation is inlined there. See codegen-review.json and
codegen.txt; do not infer cache misses or the cause of regression from size alone.

A single bounded follow-up might outline Rayon's `steal_front_snapshot_tiles`
method, keeping its entire body, tile8, first scalar probe, SC fence, normal
guard lifetime and retry behavior identical. Audit source/history for that exact
boundary before declaring anything. If pursued, freeze a source/codegen gate
first: the intended separate routine must actually exist and shrink the caller;
otherwise close without SDK timing. If it passes, use a new predeclared fixed
matrix with accepted, fresh outlined OFF/ON, and a direct frozen old-inlined ON
control so the codegen hypothesis can be tested. Preserve the current negative
result; comparing new sources with a fixed old control is not an unchanged reroll.
No inline-attribute or tile-size parameter search. Do not promise a2x jump.

There is also a stronger arithmetic/critical-path direction: local primary
user-cycle categories remain dominated by G1/G2 arithmetic (roughly67–70% in
these profiles), and larger merge spends roughly42% in FFT. Audit existing
adopted/failed arithmetic and phase-span work before choosing another design.
These CPU shares do not prove which work lies on the elapsed critical path.
The aim must be fewer curve/field operations or a shorter actual dependency
span, not merely fewer samples assigned to one runtime function.

C7a historical epoch shares are much larger than local, so a separately declared
target diagnostic could still test an architecture-specific hypothesis, but
would require fresh credentials, tested znver4 binaries, concrete protocol and
owned-resource cleanup. No target result or cloud resource exists for this run.
Do not turn a local negative into a claimed AWS win.

Production accepted170/source manifest and SDK03e1260... stay unchanged. Goal
all-eight supplied CPU10x with CPU/Metal reporting remains unmet. No large key
tables, C/ASM rewrite, subagents or questions. Commit/PUSH each finished experiment.
