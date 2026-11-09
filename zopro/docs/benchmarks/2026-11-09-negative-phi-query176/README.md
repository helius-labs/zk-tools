# Original-order short-forward G2 query

The primary 32-worker query improves 2.33% against the same-binary control, but the declared overall gate fails six of 37 checks. Production is unchanged. No full-proof gain or adoption is established.

The private query replaces only three affine forward endomorphism loops with the already-qualified short-factor kernel. Scalar split and component order, projective maps, windows, grouping, queues, exceptions, weighted reductions, normalization and resource fallback retain their original behavior. It adds no input-sized array, key table, retained workspace, pool or memory cap. The fixed factor occupies 64 logical constant bytes independent of the circuit. The earlier changed-order inverse query is not repeated.

Three targeted native query tests pass: 40 raw/owned cases, 21 grouping/admission cases and 106 nested queries executed by all 53 workers across 1/4/16/32-worker pools with requested 512 KiB stacks. Generic guard compilation passes. The existing field library and its mathematical qualification are reused without rebuilding or rerunning field tests. Release consumer symbols contain no test-route atomics. All 176 accepted originals recover; 174 remain byte-exact in the private 179-file snapshot.

The 399-command capture completes on its first execution: 18 preflights, 318 latency processes, nine PMU panels, nine cycle profiles and 45 offline commands. All 354 callers check every timed result against the retained independent Arkworks complete-MSM receipt, preserving input points and worker placement. The 28,140 complete query calls include primes and postchecks; they are not proofs. The five preceding native/link workflow commands also pass once.

REF reuses the original-field original-query executable in its original mode only. OFF and ON share a newly linked executable with the qualified field; OFF calls the unchanged query, ON calls the private short-forward query. Timers include scalar reset, grouping, allocation/drop, all query arithmetic and final affine normalization. Default Rayon stacks and fixed worker placement match across arms. Primary32 has 31 paired blocks with 151 queries per process; the other five cases have 15 blocks, with 11 solo or 51 threaded queries per process. All latency precedes profiling and all recording precedes offline analysis.

|Case|REF ms|OFF ms|ON ms|Paired ON/OFF [95% interval]|Wins|
|---|---:|---:|---:|---|---:|
|1x2-w1|37.240534|37.397714|36.740371|0.981321 [0.979187, 0.983543]|15/15|
|1x2-w16|3.539118|3.559994|3.502449|0.980409 [0.962359, 0.998623]|12/15|
|1x2-w32|3.165439|3.193850|3.139492|0.976660 [0.960402, 0.991455]|27/31|
|2x2-w1|39.928734|40.113110|39.300715|0.980824 [0.977343, 0.984266]|15/15|
|2x2-w16|4.116545|4.093600|4.061216|0.997179 [0.967888, 1.026414]|9/15|
|2x2-w32|3.661771|3.949166|3.689760|0.982165 [0.950131, 1.016663]|10/15|

Intervals use 20,000 paired log-bootstrap resamples with the fixed declared seed. They assume exchangeable blocks and have no familywise coverage guarantee. Displayed medians need not divide to the paired estimate. No samples are excluded, extended or rerun.

Primary ON/REF is 0.988764 [0.976083, 1.002075], with 25/31 wins: its gain against the original-field reference remains uncertain. Both solo cases improve against both controls in all 15 blocks. All RSS checks pass. The failed checks are:

- `1x2-w32/ON/REF/primary-gain`
- `2x2-w16/ON/OFF/nonregression`
- `2x2-w16/ON/REF/nonregression`
- `2x2-w32/OFF/REF/calibration`
- `2x2-w32/ON/REF/nonregression`
- `two-shape32/ON/REF/gain`

The 2×2 guards fail to establish the required bounds; their intervals do not establish a clear regression. Its 32-worker OFF/REF calibration fails at 1.031124 [1.004839, 1.058040]. The two-shape32 ON/OFF aggregate improves at 0.979408 [0.961404, 0.997737], while ON/REF is uncertain at 1.000677 [0.985258, 1.016356]. All failures remain part of the result.

All nine profiles qualify: 82,739 samples, zero lost records, assigned process/thread/CPU identities, fixed 16M periods and under 0.91% unknown symbols. All 36 PMU events have full running coverage. At 32 workers, instructions fall about 2.29% for primary and 2.28% for 2×2 against OFF; cycles fall about 2.51% and 2.54%. These single diagnostic panels support lower aggregate CPU work but do not establish a wall-time critical-path saving or a proof gain.

Maximum query-process RSS for primary32 is 27.25/27.75/27.25 MiB for REF/OFF/ON; 2×2 is 35.25/35.00/35.25 MiB. These values include fixture loading and primes, exclude a full proving key/SDK request, and establish neither minimum RAM nor pressure or mobile behavior.

The declared protocol said that failure closes this query without a preauthorized exception. The query gate remains false and this measurement is closed without a rerun. A separate **engineering exception**, made after seeing the results, selects a distinct full-SDK integration experiment. The primary same-binary gain, gains against both solo controls, lower instruction counts and unchanged workspace justify testing the actual full-proof objective. This decision is not a passed gate or permission to adopt: the new consumer requires coherent controls, all-eight plus merge54 correctness, Gnark verification, latency and resource qualification. [Decision and limitations](artifacts/decision.json).

The source/receipt reviewer reconstructs all raw means, paired statistics, 37 decisions, RSS and quality checks. It shares the statistical and diagnostic parsers with the analyzer; it is not an independent mathematical implementation of those parsers. All 416 pins are rechecked. Helper preparation repairs add a missing demangler dependency and preserve the copied prefix’s final newline; scientific source, successful builds/tests and recording are unchanged. [Preparation notes](artifacts/preparation-notes.json).

The canonical proof ledger remains 40,469: zero new SDK requests or Gnark exports. The following full-proof CPU/Metal comparison is inherited from the latest unchanged accepted SDK campaign, not measured by this query experiment. The all-eight CPU10× objective remains active and unmet, 0/8.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.699867|37|24|4.805×|3.117×|3.7|
|2x2|11.038591|59|30|5.345×|2.718×|5.9|
|4x2|15.929492|109|48|6.843×|3.013×|10.9|
|8x2|24.033392|189|74|7.864×|3.079×|18.9|
|12x2|33.930932|284|101|8.370×|2.977×|28.4|
|16x2|40.013619|335|124|8.372×|3.099×|33.5|
|24x2|66.023007|528|176|7.997×|2.666×|52.8|
|merge-24x1|65.216469|507|175|7.774×|2.683×|50.7|

[Results](artifacts/analysis.json), [review](artifacts/review-result.json), [prospective review](artifacts/prospective-review.json), [protocol](artifacts/capture-protocol.md), [source audit](artifacts/source-audit.json), [continuation](artifacts/next.md), [source and receipt archive](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Binary libraries, raw perf recordings, private point/scalar fixtures and the full proof ledger are excluded.
