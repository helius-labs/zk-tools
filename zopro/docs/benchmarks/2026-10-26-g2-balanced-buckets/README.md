# Coupled G2 bucket width and partition count

Rejected by the fixed local SDK pilot. Accepted production remains unchanged.
Primary latency is 8.809 ms versus 8.628 ms accepted and 8.602 ms with the
candidate switched OFF. Paired median regressions are 2.58% and 2.42%, with
one and zero winning blocks out of seven.

For eligible exclusive G2 queries, the private candidate changes two point
partitions with 12-bit digits to four partitions with 11-bit digits when the
pool has at least 32 workers. Each ordinary window still has 4,096 bucket
slots in total; the number of windows increases from 11 to 12. This exposes
more fill jobs and adds input passes and merging. Scalar decomposition,
arithmetic kernels, top-digit handling and Rayon remain the same.
The forecast covers the maximum of both layouts, including signed top-digit
scratch in every partition. Equal bucket slots per window do not bound total
proof memory. No retained key tables, point/scalar copies or pools are added.

Activation requires the private `zopro_g2_balanced_buckets` cfg and explicit
`ZOPRO_CPU_G2_BALANCED_BUCKETS=1`. Omission and zero select OFF. Smaller pools
and ineligible query sizes retain their layout. This differs from the older
fixed-width four-part trial; the rejected epoch and idle-search trials remain
rejected.

## Complete SDK results

Ryzen 9950X, 16 physical cores and 32 logical CPUs. Seven rotated/reversed
fresh-process blocks, 31 measured requests after three primes per process.
All latency finishes before nine separate 999 Hz user-cycle captures.
The 32-worker cases use CPUs 0–31; the 16-worker case uses CPUs 0–15, one
logical CPU per physical core. Timing includes fresh witness work, randomness,
proving, internal verification and serialization; key loading is excluded.

| Case | Accepted ms | OFF ms | ON ms | Paired ON/accepted | Paired ON/OFF |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2 / 32 | 8.627621 | 8.601741 | 8.809072 | 1.025847 | 1.024219 |
| 2x2 / 32 | 11.725111 | 11.586149 | 11.792859 | 1.011249 | 1.012299 |
| merge-54x1 / 32 | 177.810898 | 177.865187 | 178.709082 | 1.006272 | 1.004786 |
| 1x2 / 16 | 10.755022 | 12.982152 | 11.819470 | 1.010572 | 0.994066 |

Cells are medians of process medians. Paired ratios use corresponding blocks;
ratios of pooled medians need not match. The 16-worker controls vary
substantially: accepted process medians range from 10.330 to 15.913 ms.
Its unchanged arithmetic path supports no speedup claim. The primary
32-worker result fails independently of that variability.

The prospective gate requires a positive primary result against both controls
with at least four of seven wins, a positive four-case geometric aggregate,
no case above 2% paired median or 5% per block, and OFF calibration within a
factor of 1.03. No minimum gain floor is imposed. The paired geometric ratios
are 1.014158 against accepted and 1.009144 against OFF. Every failed condition
is retained in [the result](artifacts/pilot-result.json). There is no adoption,
unchanged timing retry or subsequent width/partition search.

| Primary / 32, unprofiled | Accepted | OFF | ON |
| --- | ---: | ---: | ---: |
| CPU ms per request | 199.109 | 199.011 | 209.267 |
| Voluntary switches per request | 415.16 | 401.16 | 463.94 |
| Maximum process RSS, MiB | 114.00 | 110.25 | 111.00 |

RSS includes loading and priming and does not establish minimum RAM or pressure
feasibility. No additional owned build, profile or CPU benchmark overlaps
latency. The final preflight inventory check overlaps initial latency as
recorded below.

## Profiles and verification

Nine captures contain 243,018 samples, one process per capture and zero lost
samples, with an explicit monotonic clock. Primary sampled G2 arithmetic
share rises from 28.98% OFF to 31.61% ON, while profiled CPU time rises from
199.56 to 209.72 ms per request. These are sampled CPU shares, not removable
wall time or an isolated cause. Inlining limits symbol attribution.

Native validation passes 112 core tests, two raw-image and six cubic tests.
Three new tests cover geometry boundaries, 36 known-generator MSM oracles at
1/4/32 workers, and four concurrent callers sharing a 512 KiB-stack pool.
The oracles cover identities, signs, repeated bases and full-width scalars.
Field arithmetic is unchanged; its earlier qualification is inherited, with
no new field-suite execution claimed.

Preflight contributes 48 SDK requests and 12 independent Gnark exports.
Timing contributes 2,856 requests and 84 exports; profiling adds 660 requests
and nine exports. Total: **3,564 SDK requests, 105 distinct independently
verified exports and 210 proof/public-input files**. Fixture bindings, proof
freshness, source pins and linked artifact hashes are checked.

Two workflow errors are preserved. An analysis build attempted while the
native build held the host lock was rejected before execution, then succeeded
after that build finished. The first inline JQ inventory validator had a
syntax error; its shell lacked fail-fast handling and continued into timing
after all proving and Gnark commands had passed. The repaired validator
checked the original artifacts successfully after initial latency started.
No successful proof or timing process was repeated. The
[chronology](artifacts/preflight-validation-chronology.json) records this
limitation; later launch scripts use fail-fast handling. Archival copying
also needed privileged reads of the root-owned raw captures; source captures
and measured data were preserved.

No portable, memory-pressure, large-circuit, C7a, phone or TEE qualification
of this candidate is claimed. Primary timing is 4.20 times the supplied
37 ms M5 CPU reference and 2.72 times its 24 ms Metal reference; 2x2 is
5.00/2.54 times its 59/30 ms references. These are supplied historical
references, not matched-machine measurements. No merge-54 M5 reference was
supplied. The original all-eight 10x CPU goal remains unmet.

The [accepted manifest](artifacts/accepted-manifest.json) retains the qualified
full-shape configuration. Fresh C7a scaling and Aeglos comparisons remain in
the [preceding C7a report](../2026-10-25-phase-cpu-c7a/README.md).

[Protocol](artifacts/experiment.json), [observations](artifacts/perf/rows.json),
[profiles](artifacts/perf/report.md), [source change](artifacts/source.patch),
[build receipt](artifacts/native/result.json),
[preflight checks](artifacts/preflight-validation.json),
[proof inventory](artifacts/perf/proof-inventory.json).

Full local experiment: `/tmp/zopro-g2-balanced-buckets/`.
Raw captures and expanded symbol streams are omitted from Git and retained
locally under this report's `artifacts/` directory. The published hash index
also covers those files. Compiler caches and executables are represented by
their pinned build/link receipts.
