# Fused preparation on C7a

The exact locally qualified candidate improves 1x2 at 32 physical cores. It does
not resolve Zopro's high-core scaling gap: 96 cores remain slower than 32, and
96-core CPU cost fails the declared nonregression limit against previous production.
Aeglos Sorted remains faster at its best tested count, using more memory.

| Build | Cores | Median proof ms | Mean CPU ms/request | Peak RSS MiB |
|---|---:|---:|---:|---:|
| Previous Zopro REF | 32 | 11.354326 | 243.86 | 112.06 |
| Candidate OFF | 32 | 11.230296 | 242.84 | 115.82 |
| Candidate ON | 32 | 11.098676 | 239.50 | 111.93 |
| Aeglos Sorted REF | 32 | 11.890257 | 327.88 | 420.32 |
| Previous Zopro REF | 96 | 12.867767 | 686.96 | 104.49 |
| Candidate OFF | 96 | 13.340967 | 774.07 | 102.25 |
| Candidate ON | 96 | 12.725096 | 716.94 | 106.68 |
| Aeglos Sorted REF | 96 | 7.964424 | 518.42 | 475.37 |

Times are medians of nine process medians. RSS is the largest whole-process peak,
including key loading and preparation; it is not minimum RAM or incremental
scratch. Best tested Zopro is 11.098676 ms at 32 cores; best tested Aeglos is
7.964424 ms at 96. This comparison does not establish performance on other inputs.

## Paired evidence and limits

| Comparison | Cores | Paired latency ratio | Marginal bounds | Winning blocks |
|---|---:|---:|---:|---:|
| ON / previous REF | 32 | 0.981382 | 0.970946–0.992778 | 8/9 |
| ON / OFF | 32 | 0.988280 | 0.976049–0.999700 | 8/9 |
| ON / previous REF | 96 | 0.989531 | 0.978806–1.000133 | 7/9 |
| ON / OFF | 96 | 0.950955 | 0.930178–0.958626 | 9/9 |

The 96-core ON/REF latency bounds include parity. Its paired CPU ratio is
1.041294 [1.035956, 1.051914], failing the prospective upper bound of 1.02.
All other latency, CPU and memory guards pass: **11/12** performance/resource
checks and **10/10** environment guards. No threshold, sample or backing changed.
This is a scoped C7a limitation, not a correctness failure or a reversal of the
[completed local qualification](../2026-11-17-fused-preparation182/README.md).
Use the measured 32-core configuration for this C7a input; the 96-core configuration
is not qualified as a resource-neutral replacement for previous production.

Fusion reduces latency by about 4.9% and CPU by about 7.1% relative to its own
OFF mode at 96 cores. However, OFF itself is slower than previous REF: latency
ratio 1.046975 [1.021468, 1.063808], with no winning blocks. Thus the same-binary
optimization benefit and the net deployment benefit are different quantities.
The capture does not identify whether code generation, executable layout or some
other build-dependent effect explains the OFF/REF difference. Do not assign the
entire OFF/ON saving to a specific substage without a representative trace.

Paired 96/32 latency ratios are 1.130701 for previous REF, 1.138608 for enabled
Zopro and 0.672498 for Aeglos Sorted. Fusion has not repaired the overall scaling
curve. The next investigation is the remaining ordinary G2 work and the 96-core
REF/OFF CPU discrepancy, starting with retained binaries and evidence rather than
another threshold sweep. Local cycle profiles support the fusion mechanism but
cannot provide exact attribution on this AWS machine.

## Protocol and verification

One c7a.24xlarge, 96 physical cores without SMT, fixed worker affinity. The same
primary 1x2 key, request and public bindings feed all implementations. Each timed
request includes witness assignment/solving, fresh proof randomness, proving,
SDK verification and serialization. Loading and three primes are outside timing;
Aeglos's warmed key cache is retained and its preparation contributes to peak RSS.
Existing native preparation limits and allocator settings remain unchanged.

Three independently written, byte-identical backings per binary are balanced
across nine rotated/reversed blocks. Seventy-two main processes each run 51 timed
requests after three primes; eight functional preflights cover all arms/counts.
There are **3920 SDK calls and 80 fresh exports**, all independently Gnark-verified
on AWS and again after transfer. Three additional container compatibility exports
verify on container and host. No successful SDK capture was repeated.

Bounds are sorted zero-based ranks 1 and 7 of nine paired process-median ratios:
nominal 96.09375% marginal median coverage under independent continuous blocks,
not a simultaneous guarantee. Raw backing results and all CPU/memory comparisons
remain in the retained evidence. A separate same-agent reader reproduces all 16
contrasts and 12 performance decisions directly from raw SDK outputs.

No cgroup CPU throttling, memory-limit/OOM or swap-event increments were recorded.
The container's 30 GiB infrastructure ceiling is not an algorithm workspace cap.
It does not test a low-memory device. No phase instrumentation was enabled.

Previous Zopro SDK SHA-256:
`b646c7b8c1d8a26a0db45fb3f67c453dd616bd9f02e25c36192721ae02855abe`.
Candidate OFF/ON:
`c285cbc88f783abac203c66db43ea86bda8207338b9fb209df7eb5d846126202`.
Aeglos Sorted:
`eab9fbfd4b94aeef9ced3ce3e3ede21a37ab70ad38b217720f1afd06fceba99b`.
No SDK was rebuilt for AWS. Native opt-in remains
`ZOPRO_BENCH_FUSED_PREPARATION=1`; without it the previous source path is selected.

Current canonical ledger: **56585**, SHA-256
`fd19e2e7d510008759cb6248bac6004db9274c698fc56cfa02a9f8006dc2cf36`.
Private root: `/home/ubuntu/src/zopro-perf-work/fused-preparation-c7a182`.
Instance `i-0624f7e413e56f822` is terminated, its volume absent, and the temporary
ECR tag and transfer object/bucket removed. The first container packaging attempt
failed before any SDK execution due to an invalid local-image reference; the
corrected image passed smoke before publication and launch. That failure is retained.

The supplied M5 CPU/Metal references for 1x2 are 37/24 ms: the best Zopro result
here is 3.3346x/2.1630x faster, respectively, on different hardware. The CPU10x
target is still 3.7 ms. Other seven shapes and merge54 were not rerun on AWS;
retain their fresh local qualification above. The full all-eight CPU10x goal
remains unmet. Publication was authorized after this comparison completed.
