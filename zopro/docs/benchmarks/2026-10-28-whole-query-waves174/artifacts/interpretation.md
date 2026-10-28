# Interpretation

The fixed whole-query G1-then-G2 candidate fails both primary gain conditions.
Primary32 ON/OFF paired ratio is 1.060781652, bounds [1.054621614,
1.083825746], zero wins out of seven; paired saving is -0.498760 ms.
ON/accepted is 1.069986588, bounds [1.067483880, 1.079545258], zero wins.
Primary OFF/accepted calibration passes: 1.007296339, bounds
[0.994140894, 1.022263357]. No candidate adoption or capture reroll.

Primary32 block-median latencies are accepted 8.138147 ms, OFF 8.155049 ms,
ON 8.714394 ms. CPU rises from 187.818 to 190.968 ms/request. Whole-process
peak RSS decreases: median/max OFF 130.50/133.25 MiB, ON 80.00/82.25 MiB.
The unchanged forecast does not imply unchanged allocator history. Lower RSS
does not establish minimum RAM, pressure behavior, or an overall improvement.

Physical16 ON/OFF has a tiny positive point estimate, 0.999840345, four wins,
with wide bounds [0.889893020, 1.085455509]. It is descriptive, not a confirmed
benefit. All four larger ON upper-bound conditions pass. OFF/accepted
calibration fails descriptively for physical16 and 16x2; preserve those
failures. The primary calibration and all merge54 calibration conditions pass.
The candidate branch is limited to the pre-existing fused-K/H eligibility;
larger cells provide fallback and code-layout context, not proof of a
whole-query scheduling change in their own dispatch.

The two PMU panels contain one capture each per cell, so changes below are
descriptive and do not carry the seven-block latency confidence statement.
Primary32 fetch-panel ON/OFF cycles/request rises from 854.972 to 868.807
million, instructions from 1453.904 to 1460.757 million. L1 instruction-cache
misses decrease from 627384 to 613714 per request. In the separate stall panel,
cycles rise from 854.274 to 872.948 million, frontend-stall cycles from 59.250
to 60.219 million and L1 data-cache misses from 33.945 to 34.157 million.
All 48 event measurements report 100% running. The candidate does not show a
CPU-work reduction; the instruction-cache count decrease alone does not
establish a cache advantage or explain the latency change.

The six 199 Hz cycle recordings have 43908 samples, one sampled PID each,
17 sampled TIDs in physical16 processes and 33 in logical32 processes, and
zero lost events. The main thread can participate in Rayon alongside its
configured workers, so sampled TID counts are not a different worker setting.
Whole-proof samples include active workers and runtime overhead; percentages
are neither critical-path fractions nor removable wall time. No per-stage
timing markers were installed in this experiment.

The raw labels accept/offset/bypass mean retained accepted SDK/private OFF/
private G1-then-G2 ON. Names are reused from the comparison harness; ON does
not bypass grouping. Both global DEDUP and all accepted workspace forecasts
remain unchanged. Core arithmetic and G1/B2 query closures are exact.

The earlier mixed-pass, worker-partition and product-overlap negatives remain
closed; this result additionally closes this fixed homogeneous whole-query
ordering. There is no evidence here to retune the same barrier or add
synthetic allocations/delays. Further progress needs measured CPU work
reduction or less coordination in an actual kernel, with a distinct source
change and complete-proof validation. Runtime samples alone do not justify
reviving earlier rejected stealing/guard changes.

All 246 workflow steps complete, 9876 SDK requests internally verify, and all
114 fresh 164-byte exports independently verify with Gnark/public binding;
ledger 15211 -> 15325. Original source/library/helper/input pins pass on closing.
No builds, hashing, archive, verifier or second owned benchmark overlaps
capture; verifiers run only in the declared phases. PMU/profile subprocesses
run as root while unprofiled latency runs as the normal user; all diagnostic
variants use the same privilege. Diagnostic timings do not replace latency.

Archival initially encountered root-owned perf files (mode 0600) after all
captures/verification/analysis had completed. Only these six owned files were
chowned to the workspace user, then archival resumed into a fresh report
directory. The partial archive and failure log are retained locally; no SDK,
test, measurement, profile, decode or proof verification was rerun.

Accepted174/SDK81033, its complete all-eight CPU/Metal table and 8.366931 ms
primary result remain unchanged. The original goal remains active and unmet,
0/8 at 10x the supplied M5 CPU figures. No AWS/Aeglos, giant, phone/ARM/TEE,
swap, minimum-RAM or forced-pressure conclusion follows from this local pilot.
