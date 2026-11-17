# G2 grouping profiles216

The extra running CPU time in the Boolean-OFF grouping context concentrates
around a stack copy/reload immediately before AHash hashing. Bulk memory
copies account for much less of the sampled difference. This locates a useful
instruction region; it does not establish the hardware cause or a proof-speed
improvement. Production194/196 and the closed209 candidate remain unchanged.

|Block|OFF group cycles|ON group cycles|ON/OFF|OFF/ON instructions|
|---:|---:|---:|---:|:---|
|0|8,152,621|4,535,174|0.556284|3,613,297 / 3,613,673|
|1|8,208,145|4,575,441|0.557427|3,613,087 / 3,614,103|
|2|8,070,604|4,515,649|0.559518|3,613,641 / 3,613,604|

These are process medians from the sampled processes. Internal PMU coverage
is 100% for all3,016 records, including smoke and unsampled controls. The
previous214 CPI gap reproduces: approximately2.23–2.27 OFF versus1.25–1.27 ON.
Instructions remain essentially equal. Group CPU means are1.839–1.849ms OFF
and1.090–1.098ms ON. Whole-proof sampled/plain observer ratios are1.004031 OFF
and1.003838 ON; both pass the declared factor1.05 limit.

## Where samples land

Samples are restricted to the actual G2 grouping worker TID and its grouping
interval. The same function also groups A, so unfiltered function totals would
mix queries. Each arm contains753 timed proofs across three processes.

|Region|OFF samples|ON samples|
|:---|---:|---:|
|Entire G2 grouping interval|1,265|692|
|`scalar_groups` offset0x1080–0x10bf|790|35|
|Exact PC0x234c3a, offset0x108a|757|6|

At a1ms software CPU-clock sampling period, the net interval difference is
about0.761ms/proof. The individual hot region has a larger positive difference;
other regions shift in the opposite direction. These are coarse sampling
estimates, not exact instruction latencies or additive critical-path savings.

The hot PC follows a256-bit store of the scalar to a stack slot and starts a
sequence of128-bit stack loads before AES hashing. The retained disassembly
shows only16-byte stack alignment guaranteed by this function's prologue.
Actual stack addresses were not captured here. Stack placement and
store-to-load behavior are hypotheses to test, not established causes. A
software sample can also land after the instruction responsible for a stall.
The failed215 heap-buffer offset intervention does not test stack placement.

## Capture, validation and analysis repairs

The diagnostic SDK is
`6d9ae8f89e09b4bec8ac7c6a6f795bfbf04594623e4abf39802ada7b4d5d254c`.
Relative to214,181 runtime files remain exact; only the trace parent gains a
clock-synchronization getter. It samples bounded pairs of Rust elapsed time
around Linux CLOCK_MONOTONIC reads outside SDK timers. The intersection of
all16 pairs bounds the clock offset; main-run uncertainty is20–40ns.
No arithmetic, field, grouping, solver or admission code changes.

Seven focused native tests and five decoder tests pass. Four4-worker smoke
processes precede12main32-worker processes: Boolean OFF/ON crossed with plain/
sampled execution, three independent executable backings, three rotated and
reversed blocks, three primes and251 timed calls per main process. Native
tracing and internal PMU counters are enabled in both observer modes. All
captures finish before late decoding and Gnark verification; owned compilers,
verifiers and analysis helpers remain idle during capture.

All16 exports pass Gnark and3,064 SDK calls complete. The receipt/source reader,
CPU reader and PMU reader pass. PID/TID/CPU/executable ownership, event schema,
loss and throttle checks pass. Every profile has at least100 scoped samples;
both pooled arms exceed400. No sample is ambiguous at an interval boundary.

Three original profiles narrowly exceed the fixed5% scoped unknown-symbol
limit. Matching Ubuntu libc debug symbols, identified by ELF build ID, resolve
the missing names without another capture. The symbols are extracted privately
from `libc6-dbg=2.39-0ubuntu8.12`; no installed package or system library changes.
Re-decoding the same perf files passes the unchanged quality gates: at most one
scoped sample per profile remains unknown. A separate Rust reader verifies all
248,097 main samples retain identical PID/TID, CPU, timestamp, period and IP,
and independently reconstructs the exact1,265/692 scoped counts using a linear
interval scan. Original failed-quality results remain in the artifacts.

Analysis-only bookkeeping repairs are retained in `analysis-repairs.md`.
They include a missing comparison alias and an existing-smoke-output check;
no capture, proof, gate or runtime source was changed. The first qualified
profile result arrives19:24:40UTC,20minutes after the19:04:42 protocol freeze,
within the25-minute first-result allowance. Independent review completes
19:25:55UTC. This is diagnostic evidence, not adoption qualification, a new
accepted performance result, or portable/pressure testing.

Canonical distinct exports are **59,032**:
`/home/ubuntu/src/zopro-perf-work/group-profile216/proof-sha256-after-main.json`,
SHA256 `3d8d234da2237304a40b1658a1b5a2d47a06a2c6245a464a3ec87ca9a39756ad`.
Complete evidence remains in that private root. Raw inputs, proof bytes, full
ledgers, binaries and raw perf payloads are excluded here. No AWS resources
were used. The original all-eight CPU10× goal remains unmet0/8.

`artifacts/raw-text.tar.gz` preserves the exact disassembly, clock-source diff,
test output and reused decoder includes, including their original whitespace.
