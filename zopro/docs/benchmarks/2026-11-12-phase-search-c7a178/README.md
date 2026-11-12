# C7a solver-phase search experiment

The completed c7a.24xlarge experiment did not establish a latency benefit from
restricting private-deque search during solving. The candidate failed its fixed
screen (23/28 checks passed) and is not adopted. The accepted implementation is
unchanged. Correctness, profile quality and all four RSS guards passed.

## Latency and memory

Nine rotated/reversed blocks used three independent executable backings. Each
latency process primed three proofs and timed 151. Values below are medians of
nine process means; RSS is the maximum process lifetime peak across those nine.
REF is accepted SDK `b646c7b8`; OFF and ON share candidate SDK `1f1f432a`.
ON limits private-deque search to ceil(N/3) workers during the canonical solver
and restores full participation before scalar preparation. Local pops, injected
work, broadcasts, completion checks and arithmetic retain their original behavior.
The machine exposed 96 physical CPUs without SMT.

| Circuit | Workers | REF ms | OFF ms | ON ms | REF peak MiB | OFF peak MiB | ON peak MiB |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 32 | 11.178852 | 11.278771 | 11.366803 | 124.12 | 129.19 | 124.89 |
| 1x2 | 96 | 12.714640 | 12.682769 | 12.729304 | 122.58 | 121.72 | 120.96 |
| 2x2 | 32 | 15.010650 | 15.014997 | 15.192398 | 303.92 | 312.93 | 316.76 |
| 2x2 | 96 | 16.835800 | 16.923149 | 17.050971 | 323.59 | 313.61 | 312.23 |

Paired median ON/OFF ratios are 1.002941 and 1.001355 for 1x2 at 32 and 96
workers, and 1.011204 and 1.007555 for 2x2. ON wins 3/9, 4/9, 0/9 and 2/9
blocks respectively. These paired ratios are not ratios of the displayed medians.
Accepted 96/32 paired ratios are 1.134085 for 1x2 and 1.120209 for 2x2;
32 workers remain faster in both tested contexts.

The predeclared directional screen required primary 96-worker improvement over
both controls on each backing, plus <=3% regressions in the other contexts and
RSS <=REF*1.05+1 MiB. Four primary gain checks failed, and the 2x2/96 ON/REF
ratio on backing 0 was 1.030499. This was a mechanism screen, not an adoption
qualification. There is no post hoc subgroup promotion or selected divisor sweep.
[Analysis](artifacts/target-analysis.json) and [paired ratios](artifacts/supplement.json)
preserve every check and block.

RSS includes loading, preparation and priming; it is not a minimum RAM estimate.
The latency workflow's cgroup peak was 407,343,104 bytes (388.47 MiB), including
preflights and verifiers. Its snapshots showed zero CPU throttling and zero
memory, swap or OOM events. The 30 GiB/no-swap harness setting is not an
architectural requirement. Median major faults per timed request were zero in
all contexts. For 1x2/96, minor faults were approximately 140/request for each
arm; voluntary switches rose from 1,621 OFF to 1,956 ON. These observations do
not establish paging as the scaling bottleneck. The profile workflow lacks its
end cgroup snapshot, so the latency resource-event result does not cover it.

## Profiles and interpretation

Four panels followed the latency run: 201 timed requests per counter arm and
251 per sampled-cycle arm, each with three primes. Both cycle profiles passed
PID/TID, placement, event, mapping, gate and loss checks. PMU running coverage
was 100%, with zero LOST records. Absolute sample-period coverage was not
validated. One counter panel per arm is diagnostic, not replicated causal evidence.

| 1x2, 96 workers | OFF | ON |
|---|---:|---:|
| Instructions/proof | 1,691,919,567 | 1,706,885,371 |
| User cycles/proof | 2,281,288,402 | 2,259,642,607 |
| Cycle samples | 35,156 | 33,987 |
| Epoch-symbol sample share | 48.33% | 46.76% |
| Other scheduler-named share | 14.16% | 14.32% |

The symbol classification is `crossbeam_epoch`, then remaining `rayon_core` or
`crossbeam_deque` names. It may include inlined payloads and is not an idle-time
measure or a removable wall-time fraction. Limiting search only during solving
slightly reduced sampled coordination share and counted cycles, while instructions
increased and full-proof latency did not improve. This rejects the tested
intervention; it neither clears all scheduler behavior nor identifies another
phase as the cause. No next implementation is selected from these aggregate
profiles. A subsequent intervention needs a distinct mechanism and a bounded
end-to-end test rather than another divisor sweep.

Aeglos was not measured in this run. The November 10 comparison's 7.902802 ms
Aeglos result at 96 workers is historical, not a contemporaneous control.
The full all-eight CPU10x goal remains unmet (0/8); accepted local CPU/Metal
results remain in the [adoption report](../2026-11-12-bundle-adoption178/README.md).

## Verification, recovery and cleanup

The run completed 17,596 SDK calls and produced 124 fresh exports: 120 from
preflights/latency and four from profiles. All 124 passed independent Gnark
verification, including local verification after transfer. The latency review
records 245.06 seconds in capture commands and 4.44 seconds in remote verifier
commands; profile commands total 14.66 seconds. These are command durations,
not total preparation or end-to-end turnaround time.

All AWS captures completed once. The subsequent offline loss decoder failed
because its `perf` PATH wrapper could not locate tools for the running kernel.
A continuation decoded the retained data in the same pinned image, with the
versioned perf directory on PATH, and finished the four profile verifications.
It contained no capture commands. The original failed workflow is preserved.
Reader repairs account for the split workflow and PATH omission in command
receipts; the declared continuation protocol retains PATH. No successful capture
or numeric analysis was repeated. Transfer also needed a trailing-newline trim
in its run ID before bucket creation; original source and pins remain intact.

The downloaded private archive was 2,143,648 bytes, SHA-256
`016f74ff508869cdad6f9f719ee24be6c01edab905ba42ead3c7602986842704`.
The host finalizer successfully merged the frozen target branch with the later
local branch, retaining 55,209 unique verified exports. Canonical ledger SHA-256:
`64f44d498212b79be20498597f39fd6b5eb2e063004816f9f2d5c73f524cd350`.
Its [review](artifacts/canonical-review.json) separates correctness, performance
and profile quality; it does not authorize adoption.

The owned instance was terminated, its volume is absent, and the temporary
registry image tag and transfer bucket were deleted. No owned benchmark resources
or live measurement handles remain. Production code was not changed.

Private evidence root: `/home/ubuntu/src/zopro-perf-work/phase-search-c7a178`.
Run ID: `phase-search-c7a-20261112-012656`.
Pinned image: `sha256:39216f29e4cdaa9cafb3d309a077b70a8f59e8ba6ec92bb88e3ce8a0d333a7e7`.
The preparation artifacts retain their historical state. Completed results,
reader sources and selected receipts are included alongside them; proof/public
bytes, raw perf data, keys and cloud credentials remain private. This report
is saved locally; GitHub publication requires user approval.
