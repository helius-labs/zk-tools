# Complete weighted-reduction comparison

The eight-group SIMD consumer is rejected. At 32 workers its 20-window wave is 25.0% slower than the fresh disabled control and 25.3% slower than the accepted-source reference, with no wins in 31 paired blocks. It reduces aggregate CPU work slightly but lengthens the critical path. Production is unchanged; this fixed grouping recipe is closed without a rerun or full-SDK follow-up.

This is a local component experiment, not a new proof or AWS/Aeglos comparison. REF uses the accepted 176-source tree with only the common benchmark API appended. OFF and ON reuse the qualified disabled and corrected enabled libraries from the [consumer qualification](../2026-11-09-weighted-double-consumer176/README.md). All use the exact original field, allocator, native O3, one codegen unit and fat-LTO executable linkage. A single existing Rayon pool handles both window and inner reduction parallelism, with fixed worker placement and default stacks.

Each timed wave includes the complete weighted entry, tree reductions, allocations/drop within the consumer, both required group sums, offset corrections, output collection and normalization. The balanced wave has five 2048-bucket and fifteen 4096-bucket reducers. It reflects primary K/H bucket sizes but excludes bucket filling, solver, FFT and proof assembly. Other cases use one reducer. Public generator inputs use 94,720 globally distinct full-width scalar coefficients across 24 vectors, with independent Arkworks weighted-output oracles. Points follow a deterministic double-plus-fixed-point recurrence; they are not independent random draws or captured key buckets. Every timed output and unchanged input hash is checked after timing.

|Case/workers|REF ms/wave|OFF ms/wave|ON ms/wave|ON/OFF paired ratio [95%]|Wins|
|---|---:|---:|---:|---|---:|
|w2048-w1|0.160591|0.160578|0.146400|0.912286 [0.910274, 0.914361]|15/15|
|w2048-w16|0.042345|0.042196|0.146696|3.480437 [3.457734, 3.503092]|0/15|
|w2048-w32|0.059138|0.059229|0.147180|2.513477 [2.488470, 2.542828]|0/15|
|w4096-w1|0.313221|0.313381|0.296047|0.944298 [0.942737, 0.945621]|15/15|
|w4096-w16|0.044542|0.044565|0.157134|3.537048 [3.498909, 3.575218]|0/15|
|w4096-w32|0.070470|0.070877|0.156349|2.222282 [2.198281, 2.247326]|0/15|
|w16384-w1|1.241916|1.243033|1.190836|0.956274 [0.954769, 0.957619]|15/15|
|w16384-w16|0.148333|0.148714|0.209162|1.412625 [1.403229, 1.422140]|0/15|
|w16384-w32|0.158013|0.157859|0.218979|1.380671 [1.368122, 1.392993]|0/15|
|balanced20-w1|5.525201|5.520047|5.283353|0.956210 [0.955177, 0.957231]|15/15|
|balanced20-w16|0.438256|0.437663|0.500589|1.143335 [1.140103, 1.146677]|0/15|
|balanced20-w32|0.350038|0.351382|0.438687|1.249785 [1.241908, 1.258271]|0/31|
|fallback512-w32|0.025602|0.025912|0.025606|0.984727 [0.967458, 1.000681]|11/15|

Displayed latency is the median of process arithmetic means of emitted integer wave nanoseconds. Paired geometric ratios use corresponding blocks and 20,000 log-bootstrap resamples. Medians need not divide to the paired estimate. The intervals assume exchangeable blocks and provide no familywise guarantee. The four-context native-32 aggregate uses independent within-context resampling and equal log weights; it is not the all-eight proof aggregate. No observations were excluded, extended or rerun.

All 13 OFF/REF calibration checks pass. The complete gate fails 29 of 91 checks: 16 nonregression, four primary gain/win, two aggregate gain, two unchanged-fallback stability and five RSS checks. The failed fallback bounds do not by themselves prove a regression. Every failed check is retained in [the results](artifacts/analysis.json). The solo balanced wave improves about 4%, but that subset is not selected for adoption.

At 32 workers, balanced-wave aggregate CPU time is 10.994/11.015/10.416 ms for REF/OFF/ON, while wall latency worsens from 0.350/0.351 to 0.439 ms. PMU panels show approximately 4.2% fewer user cycles and 3.8% fewer retired instructions versus OFF. One 4096-bucket reducer cuts cycles much more, yet takes over twice as long: fewer independent tasks prevent the work from using the available cores. Its enabled profile samples only seven registered workers plus the coordinator, compared with all 32 workers in the control. This is evidence of lost parallelism, not a general memory-bandwidth limit. Flat sample attribution also shows more inversion work relative to useful tree reduction; it does not identify a complete wall-time critical path.

All nine grouped PMU panels meet 99.9% running coverage. All nine profiles qualify: 83,731 samples, zero lost records, at least 1000 samples each, fixed 16M cycle periods, verified executable/PID/TID/CPU ownership and at most 1.50% unknown symbols. Raw symbols and counter summaries are in the source/receipt archive. Recording finished before analysis and offline publication work.

Maximum whole-process RSS for the balanced wave is REF/OFF/ON 11.25/11.50/11.50 MiB at 32 workers, and 11.25 MiB for all arms at 1 and 16 workers. Small single-reducer cases expose page-granularity differences: e.g. 2048 buckets/one worker is 2.75/2.75/3.00 MiB, failing the 5% RSS guard. The qualified logical addition of 6176 bytes per leaf is a workspace forecast, not measured RSS, full-proof memory pressure, minimum RAM or a mobile result. Fixtures, primes and retained benchmark outputs contribute to process peak RSS.

The independent raw reader confirms all 735 command receipts, 690 caller outputs, 633 latency rows, 91 decisions and all profile identities. It shares the qualified paired-bootstrap and diagnostic parsers, and independently reconstructs raw means, checks and the stratified aggregate. A missing fixture-generator trait import was repaired before successful generation. The prospective reader was corrected for xtask’s intentional CARGO-environment redaction before any clocks. A separate post-capture reader replaces exact floating-point JSON equality with the existing 1e-12 tolerance after a one-ulp serialization mismatch. Original sources/readers and both failures are retained; scientific data, thresholds and schedule are unchanged. [Preparation notes](artifacts/preparation-notes.json).

There are 205,833 timed latency waves, 291,576 total waves including preflights, diagnostics, primes and postchecks, and 1,357,590 complete weighted calls. These are repeated component calls, not proofs or unique cryptographic inputs. The canonical proof ledger remains 41,104 with no new SDK requests or Gnark exports.

The next experiment should target dominant full-proof work and its dependency structure. The accepted proof profiles put substantial samples in G1 streaming and G2 bucket application. A separate accepted weighted wave of 0.350 ms gives little support for a twofold full-proof gain from weighted-fold tuning alone; it is not a validated subtraction from SDK wall latency.

The following accepted full-proof figures are inherited from the last unchanged SDK campaign. This experiment adds no new proof timing. The CPU 10× objective remains unmet on all eight shapes.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.832708|37|24|4.724×|3.064×|3.7|
|2x2|11.065989|59|30|5.332×|2.711×|5.9|
|4x2|15.913588|109|48|6.849×|3.016×|10.9|
|8x2|24.254933|189|74|7.792×|3.051×|18.9|
|12x2|34.007120|284|101|8.351×|2.970×|28.4|
|16x2|40.086324|335|124|8.357×|3.093×|33.5|
|24x2|66.467587|528|176|7.944×|2.648×|52.8|
|merge-24x1|65.569872|507|175|7.732×|2.669×|50.7|

[Protocol](artifacts/protocol.md), [review](artifacts/review-result.json), [prospective review](artifacts/prospective-review.json), [decision](artifacts/decision.json), [continuation](artifacts/next.md), [sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Executables, libraries, raw perf recordings, binary point fixtures and the full proof ledger remain local. Unchanged accepted sources and qualified candidate sources are identified by hashes and their earlier reports, rather than duplicated in this archive.
