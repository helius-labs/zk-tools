# Instruction and cycle attribution in the frozen SDK

Declared before capture, 29 October 2026. Previous goal turn is progress:
the worker-work PMU diagnostic measured only4.97%instruction growth from1to32
workers despite89.89%more summedCPU time. Its observations are retained; this
does not establish a new optimization. Source175/SDK20fde remains accepted,
all eight supplied CPU10x targets remain unmet, and CPU/Metal comparisons stay.

This new diagnostic records architectural instructions:u samples, alongside
separate cycles:u profiles, using the EXACT frozen SDK and primary1x2 fixture.
No source/library rebuild, new point arithmetic, scheduling change or uprobes.
Do not rerun the preceding PMU-stat or stage-probe captures.

Three fixed rotated/reversed blocks x16physicalCPUs0-15 and32logicalCPUs0-31
xcycles/instructions =12processes. Each runs151fresh verified requests after
three primes. Both events use199Hz frequency sampling, explicit CPU/time fields,
MMAP/task records and FIFO gating only around the request loop. All captures
precede ownership transfer, all12Gnark verifications, decoding, event/loss/MMAP
audits and analysis.1848SDKrequests,12fresh164-byte exports,ledger22198->22210.
Reuse the exact preceding independent topology preflights after pin checks.

Every panel must have one sampled SDK PID, zero lost records, positive finite
periods, correct event/frequency, explicitCPU/TIME metadata, valid affinity and
accepted ELF mapping. At least1000samples and unknown-symbol period share<=5%
are required for symbol-attribution qualification; retain failing panels as
unqualified without resampling. Samples/skid and inlining limit attribution.

Use fixed literal-name categories before looking at results: grouping,
FFT, inverse, MSM_curve, solver, verification_field_extensions, allocator,
memory_primitives, crossbeam_epoch, runtime_or_inlined_payload,other,unknown,
in that precedence. Generic symbols can contain inlined work; categories are
flat sampled residence, NOT exclusive CPU costs or removable wall fractions.
Store all symbols, periods and sample counts, not only displayed top symbols.

Normalize period totals by151requests and report medians/ranges over3panels per
topology/event. Separate instruction/cycle captures are not simultaneous per-IP
counters. Relative shares or period ratios are descriptive guides, not measured
local CPI, a causal stall attribution, a timing gate or a proof speedup.
No gain floor, exclusions, threshold changes, sample extension or capture reruns.

All helpers/analyzers/publishers must be compiled BEFORE captures. No owned
compiler, hash/archive/helper/analysis or second CPU workload overlaps capture.
Rust/xtask, no new Python or agents. Every proof must verify independently and
be image/public-bound and distinct before advancing the ledger. No AWS, RAM
pressure/mobile/minimum-memory or concurrent-throughput qualification. Publish,
commit and push even negative or unqualified results; keep the full goal active.
