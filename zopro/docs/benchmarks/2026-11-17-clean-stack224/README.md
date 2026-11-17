# Stack samples in the exact production executable224

The measured production G2 grouping loop already keeps its key spill within
one page: **891 of891 loop samples** place the frame at page offset1,104 and
the32-byte spill at offset1,344. That spill is also32/64-byte aligned. The
page-crossing placement in the diagnostic216/219 executable is absent here.
This directly supports closing the aligned-frame recipe after its
[clean full-proof comparison](../2026-11-17-store-frame-clean222/README.md)
failed to establish a gain. It is not a universal absence-of-stall proof.

The accepted194 executable is reused without rebuilding or relinking:
`bcc8e0775bb5bc6c09ad86b1987ca45eebac62617340911266c84aa601fa05b1`.
All three existing REF backings from222 participate. Only two native symbols
receive entry/return uprobes: `g2_query_exclusive` and `scalar_groups`, four
events per backing. The175 observer's frequent feed and other boundaries are
omitted. No diagnostic module, arithmetic, prover driver or worker stack is
changed. ELF program headers bind native virtual addresses to file offsets.

|Workers|Block|G2 grouping samples|Loop SP samples|Samples at +0x108a|Frame offset mod4096|Spill offset mod4096|Crossing samples|
|---:|---:|---:|---:|---:|---:|---:|---:|
|16|0|130|109|1|1,104|1,344|0|
|16|1|130|114|2|1,104|1,344|0|
|16|2|142|116|0|1,104|1,344|0|
|32|0|213|188|0|1,104|1,344|0|
|32|1|218|187|1|1,104|1,344|0|
|32|2|206|177|1|1,104|1,344|0|

Each software CPU-clock sample includes the original user stack pointer,
ABI2 and one nonzero SP register. Only `scalar_groups` offsets0x1030–0x1375
are interpreted as this frame; its loop has no stack adjustment. The spill
is `[rsp+0xf0]`. Other functions and prologue/epilogue samples remain in the
private recording. Six main profiles contain205,950 software samples,
1,039 scoped G2 grouping samples and891 loop SP samples. Unknown-symbol
fractions are1.43–1.72%, with zero LOST/throttle records.

Scope uses actual G2 worker TID and nested grouping entry/return times from
the same perf monotonic clock, without a recompiled timestamp hook or clock
offset fit. Each traced main process has251 complete nonoverlapping G2 spans,
exactly one grouping span nested on its thread per query, and251 other
grouping spans retained separately. PID/TID/CPU and executable MMAP ownership
are checked. A separate reader reconstructs intervals, selected raw samples,
frame histograms, observer ratios and receipt hashes and agrees throughout.

|Workers|Panel|SDK means ms, blocks0/1/2|Grouping median ms, blocks0/1/2|
|---:|:---|:---|:---|
|16|Plain|8.689955 /8.795575 /8.813896|Unobserved|
|16|Sparse probes|8.744457 /8.783139 /8.784004|0.681647 /0.674954 /0.688220|
|16|Probes+SP sampling|8.761589 /8.812357 /8.820065|0.695473 /0.686326 /0.698759|
|32|Plain|7.570293 /7.571609 /7.552918|Unobserved|
|32|Sparse probes|7.617753 /7.547120 /7.563832|1.130315 /1.158448 /1.103264|
|32|Probes+SP sampling|7.609699 /7.580277 /7.566851|1.134522 /1.137087 /1.120887|

All four declared observer medians pass factor1.05: probes/plain0.998586 at16
and1.001445 at32; combined/plain1.001908 and1.001845. This qualifies the
observer under its short descriptive screen, not a prover speedup or a
confidence bound on individual stage perturbation. Grouping wall time still
rises with SMT despite the same page-safe frame. These are inclusive wall
spans, not measured running CPU, PMU CPI or proof of an SMT resource cause.
They must not be added to overlapping query spans or treated as removable
full-proof time.

Three4-worker smoke captures precede18 main captures: three fixed
rotated/reversed blocks ×16/32workers ×plain/probes/sampled. Each main
process makes251 timed fresh verified SDK requests after three primes.
FIFO starts recording after primes and stops before output. All captures
finish before offline decoding and Gnark checks, with no owned CPU cojobs
during recording. The12 owned probe definitions are removed after each
capture phase; final tracefs inspection finds none. No other probes are touched.

All21 fresh exported proofs pass pinned Gnark verification and public/image
bindings. The SDK internally verifies4,584 requests, including primes;
4,521 timed records are checked. Native, field and portable qualification is
inherited for the unchanged production consumer, without replay. There are
no new phone, huge-circuit, pressure/swap, minimum-RAM, throughput or AWS
results. Lifetime process RSS remains in the raw result and is not an
architectural RAM requirement.

Preparation repairs remain visible. Initial offline decoding used unsupported
`--show-user-regs` and exited129 before emitting records. Version2 uses the
documented `uregs` field for sampled panels and skips the already successful
plain verifier and ownership transfer. A pre-invocation version3 correction
points its completion check at the repaired offline directory. Its smoke
reader then rejects the event's explicit-period spelling; version4 accepts
the actual `cpu-clock/period=1000000/u` identity. Original sources, plans and
failed receipts remain. No successful SDK capture, verifier or test is
repeated. Both repaired smoke readers pass before the main capture starts;
main capture, decoding, Gnark and readers then pass first execution.

The preceding whole-process hardware-cycle review223 and this software
CPU-clock scope have different sample populations. Their counts are not
direct cycle/CPI comparisons. The new direct SP observation is specific to
the exact production artifact, current inputs and observed running contexts.
It does not establish why every diagnostic caller had a different frame,
nor transfer any local conclusion to C7a.

The next distinct question is the cost of building a full G2 grouping map
that ultimately declines. The retained212 primary input contains26,142
distinct nonzero scalars among30,089; the unchanged source's80% cutoff
rejects that population. An isolated G2-only bypass can bound this possible
saving before designing any adaptive decision. It must retain A/K/H grouping
and admission forecasts: the earlier global dedup-off174 result slowed other
queries and does not answer that isolated question. No bypass, learned cache
or new grouping policy is adopted by this report.

Canonical ledger is now **60,046**:
`/home/ubuntu/src/zopro-perf-work/clean-stack224/proof-sha256-after-main.json`,
SHA256 `4d2bb688410d7b16c36cd9a20420548e01bc27278c61f6336c818f72e9b67b87`.
The full evidence stays private at that root. Raw SP/IP/TID streams, perf
data, keys, witnesses, proof bytes, full ledgers and binaries are excluded
from this repository package. Production194/196 remains accepted and the
all-eight CPU10× goal remains active/unmet0/8. Current CPU/Metal comparisons
remain [196 local](../2026-11-17-certified-all196/README.md) and
[201 C7a](../2026-11-17-accepted-c7a201/README.md). No push is authorized.
