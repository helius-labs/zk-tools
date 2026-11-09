# Matched complete Euler-consumer trial

The previous turn made progress: actual FFT integration, release qualification,
and a fixed preparation-order defect were pushed as 8aa7277445/4c2a7ff75d. No
performance result exists. Goal remains verified CPU AVX single-proof >=10x M5
CPU for all supplied shapes, reporting CPU/Metal. This consumer trial can only
select whether full-proof qualification is justified; it cannot achieve that goal.

Link the same new driver into accepted REF, matched OFF, and corrected ON-v2.
Reuse qualified libraries without rebuilding or altering production arithmetic.
All diagnostics are omitted. Pin workers individually: one worker on CPU2,
16 on CPUs0..15,32 on0..31;512KiB stacks. Check complete outputs against NEW
independent Arkworks sparse-polynomial inputs before each process's measurement
and check a complete final digest afterwards. Keep actual input cloning and
factor expansion inside the complete Euler call. Drop independent-oracle arrays
before priming; three untimed warm calls. Assert ordinary column admission.

Contexts: domains65536/262144 x workers1/16/32. Prepare withTWISTED=1, then
convert toEuler; this exercises the corrected lifecycle. The large context has
forward column roots and no inverse column cache in every arm. This is an
explicit study configuration, not a new full-prover latency result. Plan setup
reported separately; full-process RSS includes oracle setup and is not proof RAM.

Prospective recipe after actual binary qualification/code inspection: 18
preflights of3 calls, seven counterbalanced blocks of31 calls percontext/arm
(126latencyprocesses),18region-gated PMU panels of128 calls,18region-gated cycle
profiles of1024calls. PMU and sampling cover the warmed whole consumer loop,
including clones and result deallocation. Per-call wall excludes result drop.
Perf counters start disabled and use acknowledged enable/disable only after
preparation, oracle and priming. No helper/analysis/build/hash/decode between
first preflight and terminal completion of all recordings. Offline decode later.

Declare before clock: 7paired-ratio ranks[1,3,5] (descriptive order-statistic
range, not a confidence interval); OFF/REF calibration both outer ranks inside
[1/1.05,1.05] for all6. ON/OFF and ON/REF upper<=1.02 eachcontext. Geometric
aggregate of both32worker contexts must have upper<1 and >=5/7wins againstboth
controls. Whole-harness peakRSS <=1.05*controlpeak+256KiB eachcontext. Profile
>=1000samples, <=5%unknown, >=95% assignedtasks, noLOST, CPU/TIDmapping andactual
ELFmapping verified. PMU running>=99.9%, all4events available. Any failed gate
closes this fixed recipe without extension, reroll, subset rescue, compiler,
layout, threshold or arithmetic changes. Retain all failures and observations.
Record an obvious matched regression when lower>1.02. Profiles are flat user
cycles; they cannot establish exclusive walltime, spills, DRAM or causality.

If selected: full verified SDKALL8+54, independent Gnark outputs, memory,
portable/concurrent fallback before adoption. Accepted local1x2 remains7.857170ms.
