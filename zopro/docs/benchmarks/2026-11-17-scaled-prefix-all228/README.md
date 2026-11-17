# Scaled-prefix composition: all-shape qualification

The exact227 SDK passes **70/70 performance and resource checks**, confirmed by
the separate raw reader. Primary16 meets the gain requirement against both
controls and every executable backing. Primary32 pooled intervals also fall
below parity, but its per-backing consistency requirement does not pass.
**Adoption remains pending current CPU diagnostics and final source review.**
The CPU10× goal remains unmet,0/8. No new AWS result is claimed.

This retains the [227 composition](../2026-11-17-scaled-prefix-current227/README.md)
and its exact SDK `b70a9472fa64ac6b6b41b227ca64ddfcf61c6443879c9187331eae6d5e22c153`.
No source, driver, options or binary is changed or relinked for this campaign.
REF is exact accepted194; OFF is the retained fresh accepted-source222 control.
All nine227 executable backings are reused; each participates in ten blocks.
Old183/186 failed decisions remain unchanged.

## Complete-proof time and memory

Local Ryzen9 9950X. Thirty matched blocks per context,3primes per process,
151timed requests for either primary count,17otherwise. Values below are
medians of process means,ms; RSS is maximum process-lifetime peak,MiB, including
loading and priming. The SDK interval includes solving, randomized proving,
internal verification and serialization. Loading and exported-proof checks are
outside. Fixed global Rayon pool/default stacks, worker i on CPU i, main mask
0–15 or0–31. This remains a benchmark condition, not a production affinity rule.

|Context|REF ms|OFF ms|ON ms|Peak RSS REF / OFF / ON MiB|
|---|---:|---:|---:|---:|
|1x2-w16|8.940239|8.960938|8.826703|84.75 /82.50 /82.50|
|1x2-w32|7.662845|7.668766|7.634234|128.00 /128.50 /130.00|
|2x2-w32|10.754766|10.764185|10.695071|239.75 /239.50 /238.00|
|4x2-w32|15.886016|15.949752|15.853243|283.25 /281.25 /291.50|
|8x2-w32|24.067227|24.100413|24.089668|474.25 /471.00 /467.75|
|12x2-w32|33.914354|34.059969|33.894323|683.75 /663.75 /665.25|
|16x2-w32|39.999822|40.142108|39.850686|774.79 /776.25 /773.25|
|24x2-w32|66.461971|66.488969|66.290215|1242.25 /1209.26 /1225.50|
|merge-24x1-w32|66.383183|65.958104|66.016778|1095.21 /1099.50 /1104.96|
|merge-54x1-w32|163.786056|163.593128|164.152637|1846.80 /1860.30 /1865.60|

## Paired primary comparisons

|Workers / comparator|Latency median [rank interval]|CPU-time median [rank interval]|
|---|---:|---:|
|1x2-w16 /REF|0.988189 [0.984749,0.989777]|0.973870 [0.972527,0.977101]|
|1x2-w16 /OFF|0.988257 [0.978513,0.989546]|0.972792 [0.971249,0.978998]|
|1x2-w32 /REF|0.994443 [0.988510,0.999304]|0.980827 [0.978589,0.984872]|
|1x2-w32 /OFF|0.996234 [0.987623,0.999188]|0.979217 [0.977587,0.984330]|

Primary16 paired latency improves **1.18% versus REF**; CPU time falls2.61%.
At32 workers, the pooled latency median improves0.56% and CPU time1.92%; one
backing prevents declaring the stronger consistency-qualified primary32 gain.
There is no minimum gain floor. Marginal medians and paired-ratio medians are
different estimators; do not divide the displayed marginal medians to reproduce
a paired estimate. All raw paired ratios remain in the assessment artifact.

The frozen method uses sorted30 values at indices9/20, with the median of14/15.
Marginal coverage is95.7226% under exchangeable independent blocks, not a
simultaneous confidence guarantee. All contexts pass latency and CPU upper
ON/each-control≤1.02; OFF/REF median calibration passes factor1.03. At least one
primary count must also have latency upper<1 against both controls and positive
median within each backing. This is met at16 workers.

Every context passes the prospective RSS guard of each-control×1.10+8MiB.
This explicitly differs from5% rules in older campaigns, whose decisions stay
unchanged. Reported RSS increases are retained; no equal-RSS, minimum-RAM or
pressure/swap claim follows. Scratch geometry/admission remains unchanged and
no lookup table, key copy, new heap buffer or pool is introduced. The227 merge54
RSS increase is not transferred as a constant to this independent campaign.

## Correctness and provenance

Native field39/core138 tests are inherited from227 by exact source/configuration
identity. Fresh native prover2+cubic6 and generic noASM/noIFMA field12/core102/
prover2+cubic6 suites all pass unfiltered. New test targets preserve the exact
measured SDK and its original link inputs. Generator/source identity is inherited
from the previously qualified field implementation; no Python helper or run is
introduced. No physical ARM/mobile or concurrent-throughput qualification follows.

All930 fresh exports (30preflight+900main) pass independent Gnark verification.
42240 SDK calls include primes;39450 timed records are retained. Both smoke
readers passed before the guarded main launcher. All capture precedes late
verification and analysis, without owned CPU cojobs. No SDK command, verifier,
test suite or successful analysis is replayed. The separate raw reader recomputes
all930 exports,1860 receipts,3720 stream hashes, rank comparisons and decisions.
Full inventories/proof bytes/raw thread/address streams remain private.

Inventory60138→61068, SHA `5cbfca294ebc3797d50f68c6e548b38577410d0e082cbaa630d4658f2b14125d`.

## Supplied M5 references

These compare this local CPU implementation with supplied results on different
hardware; they are not new matched-machine measurements. All eight target checks
remain false. Merge54 has no supplied M5 reference.

|Shape|ON CPU ms|M5 CPU speedup|M5 Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.634234|4.847×|3.144×|3.7|
|2x2|10.695071|5.517×|2.805×|5.9|
|4x2|15.853243|6.876×|3.028×|10.9|
|8x2|24.089668|7.846×|3.072×|18.9|
|12x2|33.894323|8.379×|2.980×|28.4|
|16x2|39.850686|8.406×|3.112×|33.5|
|24x2|66.290215|7.965×|2.655×|52.8|
|merge-24x1|66.016778|7.680×|2.651×|50.7|

[Protocol](artifacts/protocol.md), [assessment](artifacts/assessment.json),
[raw review](artifacts/independent-main-review.json),
[qualification](artifacts/qualification-review.json),
[sources/plans](artifacts/source-and-plans.tar.gz), [checksums](SHA256SUMS).
Current CPU diagnostics and final source/artifact adoption review remain next.
This report is committed locally; publication awaits approval.

Preparation took337s; smoke/main capture spans were10.41s/1156.26s. Active SDK
commands total1165.76s and Gnark commands113.21s. Final review finished94.91s
after the last verifier; protocol creation to final review was1730.00s. The
30min preparation and25min recording allowances were met. Descriptive spans use
command/exit/review file timestamps; active command durations use xtask receipts,
not instrumented monotonic phase timers. [Timing](artifacts/timing.json).
