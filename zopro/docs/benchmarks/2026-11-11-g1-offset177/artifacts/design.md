# G1-only deferred affine offsets

This is a new candidate selected using the earlier combined G1/G2 screen.
That screen stays failed. Its positive G1 observations are hypothesis-generating,
not qualification of this candidate. The old instruction against rescuing a
subset is not treated as a prohibition on a new prospective experiment: no old
gate, sample, or result is changed, and no old subset becomes an adoption result.

Use accepted production e387296070ea96090de8c50a521d494c1aed0158 as the base.
Apply the previously qualified branchless offset multiplication only to G1
streaming endpoints. Preserve G2, FFT, reduction helpers, inversions, scheduling,
portable fallback and five-field scratch geometry. The private cfg is off by
default. No key tables, retained plans, new pools or memory caps are introduced.

Hypothesis: retaining canonical bx instead of xsum and folding X/Y coordinate
offsets into product normalization reduces G1 endpoint work without paying the
G2 cost seen in the combined implementation. Earlier G1 caller gains were only
0.6–1.3% at 1/16 workers and uncertain at 32. This predicts at most a small
full-proof gain, not the remaining twofold improvement or the full goal.

First prepare exact current-source integration and run native core/prover tests,
including the prior changing-state, tails, storage-mode and threaded integration
tests against this new integration. Reuse exact field arithmetic/bounds evidence
only after verifying its module and surrounding field source identities. A new
consumer build is not covered by old caller tests. No successful old capture or
field model is rerun.

Before any SDK preflight, separately freeze a full SDK screen comparing accepted,
fresh OFF and G1-only ON across independent byte-identical executable backings.
Retain every backing and result; no fastest-copy choice. Include primary16/32,
all eight supplied shapes and merge54, fresh independent Gnark verification,
CPU/RSS and automatic profiles. A positive screen must still complete portable,
resource and applicable concurrency qualification before adoption. Freeze exact
samples, ordering, paired analysis and decision criteria before preflight.

Current ledger: 52000, gather-backing177/proof-sha256-after-capture.json,
SHA256 7069c3c56ce4619e024753ef25eccd24b8d36ac1211929b923fdd2767e1e76ca.
No new SDK calls, proofs, AWS resources or performance evidence in preparation.
No push or publication without the user's explicit approval.
