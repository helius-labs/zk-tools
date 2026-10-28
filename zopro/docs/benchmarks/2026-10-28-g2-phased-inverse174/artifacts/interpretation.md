# Interpretation

The fixed four-chain G2 inverse-phase candidate is not adopted. Both required
primary gain checks fail; all five other required conditions pass. No observation
is excluded or extended, no capture is repeated and no gate is waived.

Primary32 ON/OFF paired ratio is1.005845557421, bounds
[0.978894773612,1.023659324765], two wins out of seven, paired saving
-0.049783ms. ON/accepted is1.012307345055 [0.975418804332,1.024760416336],
three wins, saving-0.104145ms. Both intervals include parity: these are slower
point estimates, not proof of a universal slowdown. Displayed block-median
latencies are accepted8.456249, OFF8.471277 and ON8.566165ms. A ratio of these
displayed medians is not the reported median of paired block ratios.

Fresh OFF and accepted executable bytes are identical, SHA256
81033d7786e14821cb76494de7faa2ebcc9e40c3222708372b21861fdeb56c20. Their paths
and processes differ. Primary OFF/accepted calibration passes, ratio1.006424234403
[0.973314800066,1.020901307399]. Identical code rules out a compiler-code
difference between these controls; it does not prove equal process behavior or
identify the cause of the remaining variation. An unrelated photon process used
about one CPU in the pre-capture snapshot. It was not modified or stopped. No
exclusive-host or causal attribution to that process is claimed.

The16x2 point estimates improve against both controls: ON/OFF0.994182039334,
five wins, positive0.242718ms paired saving; ON/accepted0.994253354539, five
wins, positive0.239713ms. Bounds[0.986070680,1.022874540] and
[0.968125402,1.009891668] include parity. Both nonregression checks pass.
Its OFF/accepted descriptive calibration fails at lower bound0.951915920.

Merge54 ON/OFF0.994848951144, five wins and positive0.860993ms paired saving,
has bounds[0.984363676,1.007323036]. ON/accepted1.003436390275, three wins,
saving-0.574594ms, has bounds[0.981569693,1.007184689]. Both upper1.03
nonregression checks and merge54 calibration pass. The positive fresh-control
estimate is retained; it is not a demonstrated gain against both controls.

Physical16 ON/OFF0.976845061721, five wins, positive0.250673ms saving, with
bounds[0.924842315,1.060875548]. ON/accepted0.989681763978, four wins,
positive0.109085ms, bounds[0.960081038,1.040826319]. This is descriptive.
Its OFF/accepted calibration fails,1.016906220[0.965054489,1.069696411].
No physical16 speedup is established by these point estimates alone.

Primary32 CPU rises from188.298ms/request OFF and188.096 accepted to189.603
ON. Peak RSS median/max is OFF129.75/131.25MiB, accepted130.25/132.75 and
ON131.25/132.00. Merge54 ON max1839.86MiB is lower than OFF1903.18 but
higher than accepted1783.26. These whole-process high-water values include
loading/primes and do not establish minimum RAM, a uniform memory saving or
pressure behavior. The unchanged record capacities do not bound extra stack
temporaries; no stack-peak qualification or production adoption follows.

Single separate primary32 PMU processes record cycles/request accepted855.222,
OFF852.085 and ON858.600million; instructions1456.055,1459.465 and1462.451
million. ON cache misses3.219million/request compare with OFF3.013 and accepted
3.009million. All24 event observations run100% of enabled time. These are
descriptive counters, not seven-block estimates or a cache-level, port, spill,
frequency or critical-path decomposition. They supply no CPU-work-reduction
evidence for the candidate. More independent chains do not automatically make
the extra root products and scratch passes worthwhile in this caller.

Six199Hz cycles recordings have42428samples, one sampled PID each and zero
lost events. The physical16 processes have17 sampled TIDs, logical32 have33;
the main process thread is included along with workers. All sampled periods
remain in analysis.json. Their shares are not removable wall-time fractions.
No per-phase wall markers or AWS/Aeglos timing are introduced here.

Correctness and source: both native modes pass124 core and8 prover tests.
Two new tests verify each stored inverse norm with scalar field arithmetic,
complete G2 results, canonical/inversion modes, full/tail lengths and reusable
record capacities. Existing G2/MSM generator and exceptional-point oracles pass.
Field source is unchanged and its qualification is inherited, not rerun or
recounted. This is a private175-file source copy:172 exact originals, two
reversible declaration/dispatch edits and one private module. It adds no heap
buffer, point-key copy, plan, table or pool. A selected batch adds nine FqBatch
products in the mathematical graph. Original grouping, field formulas,
incoming points, scalar preparation and resource forecasts stay unchanged.

The source history audit distinguishes this norm-inversion phase from the old
G1 multi-chain trial and owned-G2 output-Y finishing. Both prior failures remain
closed. Aeglos's pinned source supplies the independent-chain/materialized-inverse
example, but keeps different point records and scheduling. This negative does
not establish why Aeglos is faster on C7a or that its complete design is worse.
No third-party code was copied into the implementation. This experiment closes
only this fixed four-chain/two-record schedule; no2/4/layout/prefetch sweep is
justified by its failed gate.

The earlier symmetric-norm report was corrected and pushed asedb4c0093 before
this capture: the same arithmetic mechanism already existed in the2026-10-24
G2 norm study. That was a history-audit failure. The captured original design
and helpers are preserved with pins; the correction supersedes their novelty
implication. The earlier measurements and failed verdict remain valid, but
are not evidence for a new arithmetic idea. No symmetric-norm runtime enters
this source copy.

All234 workflow steps complete.9672 internally verified SDK requests and108
fresh164-byte Gnark exports pass public binding/freshness; ledger15433→15541,
216 proof/public members. Native33585 and capture39800 are terminal/reaped0.
Original source175/library/SDK/helper/input pins match after capture. No owned
build/helper/hash/archive/verifier/second benchmark overlaps proof captures.
Only six owned root0600 perf files are chowned after completion for hashing.
No global kernel setting or installed probe is used. No fresh portable,
concurrent-proof, mobile/ARM/TEE, giant, minimum-RAM, swap or pressure result.

Accepted174/SDK81033, primary8.366931ms and its complete inherited CPU/Metal
table remain unchanged. The original all-eight10x M5 CPU goal stays active and
unmet,0/8. The next work must use a distinct mechanism and preserve all positive
estimates and failed conditions reported here.
