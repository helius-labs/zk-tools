# Fixed causal-profiler qualification

Frozen before the first controlled-work or causal-profile measurement.
Use the already-built distinct-worker Rust executable and pinned Coz library.
No prover or production source is changed. This is a diagnostic tool check.

Run15 rotated/reversed blocks across base, critical and slack: one second per
process,45 observations. The changed variants remove exactly25% of the named
worker's arithmetic steps. Workers are pinned to physical CPUs2/3 and the
coordinator to4. Require every oracle, work count, prime count and reported
CPU/TID set to agree with source. Use sorted paired-ratio ranks3/7/11 of15.
The critical/base upper ratio must be below0.90; the slack/base interval must
lie within[0.97,1.03]. These are fixed marginal descriptive controls, not a
prover speed gate or simultaneous confidence claim.

After all45 observations, run exactly one180second base process under upstream
Coz with randomized lines and speedups. Scope is the exact toy source file;
binary scope MAIN. Use direct environment injection, no Python runner, no
fixed-line/fixed-speedup override, no upstream source patch. root access is only
for this process's perf sampling and subsequent exact-file ownership repair;
perf_event_paranoid and system packages stay unchanged. All helper binaries,
analysis, line mappings, executable/library/source identities and workflow are
frozen before capture. No owned build/helper/hash/analysis/report/push overlaps
any observation or recording. No pilot, favorable rerun, extra block, duration
extension or amended threshold. A runtime failure closes this recipe.

Preserve the complete JSON stream and stderr, including upstream warnings or
internally skipped experiments. Do not extend an insufficient profile. Require
one startup and runtime, at least100 emitted complete experiments, positive
effective durations and selected_samples, and speedup values from0 to1 in
steps of0.05. Each experiment must have exactly one throughput(batch_done)
and latency(batch) record. Arrivals/departures and throughput deltas agree
within1; the single-inflight difference is0 or1. Durations are nanoseconds
after upstream subtracts inserted delays, not measured wall latency. Require
at least5 zero-speedup baseline experiments,20 nonzero experiments and8
nonzero experiments in the declared fitting region for EACH worker line.
Report all selected locations and all speedup bins, including outside the fit.

For each worker line, zero-speedup baseline period is sum(duration)/sum(delta).
Define y=1-(duration/delta)/baseline. Fit y=slope*x through the origin, weighted
by delta, for0<x<=0.40. This region is declared before timing: a25% critical
work reduction stays above the half-sized slack workload; effects beyond the
critical-path switch need not be linear. Report all higher-speedup data too.
The predicted25% reduction is0.25*slope. Require critical prediction>=0.10,
absolute slack prediction<=0.10, critical-minus-slack>=0.10 and agreement with
the actual measured critical reduction within0.10 absolute (percentage-point
scale). These broad direction/scale checks do not establish precise predictive
uncertainty. Fits have no confidence interval; no precision claim follows.

Missing quality evidence produces a failed qualification, not a mathematical
assertion that causal profiling cannot work. A separate reader must reconstruct
control ratios, sample bins, baselines, fits and all decisions directly from raw
receipts. Tool success only selects a later accepted-source SDK diagnostic with
line tables/progress markers and no-profiler calibration/Gnark checks. It does
not satisfy any of the original all-eight verified CPU10x requirements.
