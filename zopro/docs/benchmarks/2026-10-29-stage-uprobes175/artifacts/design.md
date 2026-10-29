# Frozen SDK stage entry/return diagnostic

Declared before capture, 29 October 2026. Prior turn is progress: joint-output
CIOS was modelled, tested, measured in actual callers and rejected/pushed.
Production remains source175/SDK20fde. All eight M5 CPU10x targets remain unmet;
CPU and Metal comparisons must remain. No new adoption gate or speed claim.

Research/history reviewed this turn: SRE/Hrabar scalar reuse, Noël–Plantard
memory-constrained bucket bands, binary/ternary joint sparse forms, mixed-width
windows and older stage/task diagnostics. Scalar reuse/bands/mixed widths and
rank folding already have local studies; do not repeat them as new. Prior
25Oct task counts are inclusive joins rather than a critical-path trace;
26Oct stage costs concern source170 and miss the larger route. This diagnostic
observes the exact CURRENT accepted executable without source or binary edits.

Use Linux namespaced uprobes/uretprobes at exact ELF file offsets, resolved from
ELF PT_LOAD headers and unique demangled native symbols. Nine boundaries: SDK
prove, solve_inner, solve_canonical_wires, solver, product_values, G1 canonical
MSM, G2 exclusive query, scalar grouping, balanced K/H feed. Entry/return data
include PID/TID, CPU and nanosecond timestamps. Probe registrations are owned
under one unique group and removed on success or failure; never remove another
group. Probe registration does not authorize modifying the accepted executable.

The K/H feed is a partial stage, not the whole query. Nested spans and spans on
different workers overlap. Inclusive stage durations are not additive CPU time.
Unobserved code remains unattributed, not zero. A late query return can identify
an observed tail; it does not prove scheduler-ready delay or an exclusive cause.

Same frozen SDK, primary1x2 image/request/environment. Three topologies: CPU2
with1worker, physicalCPUs0-15 with16workers, logicalCPUs0-31 with32workers.
Three untraced preflight processes(1request+3primes), each independently verified
BEFORE capture. Seven fixed rotated/reversed blocks compare plain and traced
processes at each topology,31requests+3primes each. Trace events FIFO-enabled
only for the request loop.42captures/45fresh164-byte exports/1440SDKrequests.
All captures finish before trace cleanup, ownership transfer, remaining Gnark,
decode/event/loss/metadata audits and analysis. No capture repeats or exclusions.

All helpers/analyzers/publishers compiled BEFORE captures. No owned compiler,
analysis/hash/archive helper or second CPU workload overlaps capture. Use
Rust/xtask, no new Python or agents. No AWS resources, per-key tables, native
field/core/prover source change, memory-pressure/mobile or minimum-RAM claim.

Trace qualification requires31complete SDK intervals per recorded process,
balanced entry/return stacks, all events inside SDK intervals,1sampled PID,
no lost records, explicit CPU/TIME and MMAP/task metadata, and stage coverage
reported without assuming missing symbols ran. Every export must independently
verify and be distinct from the prior22144 ledger before the ledger advances.

For overhead, retain all7paired process-median ratios; extrema provide nominal
98.4375%individual median bounds under independent/exchangeable blocks, not a
familywise statement. A topology with any endpoint outside[1/1.03,1.03] cannot
support a claim that trace timings represent unperturbed scheduling. Retain
its trace as instrumented observations. No timing qualification/adoption follows
even if overhead is small. No extension, threshold change, reroll or subset
promotion. Commit and push the completed diagnostic and failures alike.

Primary research sources (scope only, no copied speedup):
- https://arxiv.org/abs/2610.27452
- https://uwspace.uwaterloo.ca/items/43cc12b9-cb03-42bd-a01c-4b0ed5e12403
- https://link.springer.com/article/10.1007/s13389-026-00393-z
- https://eprint.iacr.org/2008/285
