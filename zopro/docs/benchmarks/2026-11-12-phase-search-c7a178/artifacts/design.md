# C7a solver-phase participation screen

Use exact retained accepted SDKb646 and candidate1f1f (znver4). OFF/ON share the
candidate and differ only in process-fixed diagnostic divisor0/3. Do not rebuild
the prover or repeat successful native/local qualification. The local screen
remains failed; this is a separately declared96physical-core hypothesis.

Target: one c7a.24xlarge,96physical cores/noSMT, fixed workerCPUindices, no other
owned workload. Four contexts:1x2 and2x2 at32/96workers. Twelve target one-round
preflights, then9blocks x4contexts x3arms,151timed proofs after3primes. Context/arm
order rotates/reverses. Block%3 chooses one of3independent byte-identical backings
for each consumer; OFF/ON use the same backing.120target exports/16680SDKcalls.
Include scope entry/exit, wake transitions, proving, verification and serialization.
Exclude input loading and primes. Retain every copy, sample and failed outcome.

All main captures precede their independent Gnark verification. Functional
preflights verify before the main capture. Check input/executable pins,
fixed worker placement, ONscopecount==rounds/OFFzero, restored pool, expected tile
dispatch, public-input binding and proof freshness. Report CPU service, lifetime
RSS, faults, switches and before/after cgroup events. The30GiB/no-swap container
limit is a harness setting, not a prover architecture cap or minimumRAM claim.

Directional progression: primary96 per-backing median ON/OFF and ON/REF <1;
other3contexts both ratios<=1.03 perbacking; all4 ONmaxRSS<=REF*1.05+1MiB.
Report ON96 against REF32 as a separate best-count question, without making a
high-core improvement equivalent to improving the best latency. NoCIs, tuning,
sample extension, favorable copy selection or adoption from this screen. All8
M5CPU10x goal and full release qualification remain required. No freshAeglos claim.

Packaging preflight:2shapesON plusprimaryREF/OFF at4workers,1timed+3primes,
4freshGnark exports/16SDKcalls, no latency conclusion. Seed55024; local55028;
target would reach55148. Existinglocal experiment/pins/proofs are reused.

Target profile follow-up must be prepared before launch; capture primary96 OFF/ON
with gated warm counters/cycles after latency recording, retaining original screen
decision. Target profiler permission/gate/readers remain a preparation requirement.
Do not launch just because a latency-only container is ready.

Use the established private temporary AWS lifecycle: fresh own-instance/no-ingress
checks before launch, encrypted delete-on-termination volume, automatic90minuteTTL,
at least110minutes usable credentials, recorded instanceID, and verified cleanup.
No cloud publication or resource creation occurs during local packaging.
