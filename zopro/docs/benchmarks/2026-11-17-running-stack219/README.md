# Live stack registers during G2 grouping219

The slow grouping context's32-byte stack store crosses a4KiB boundary in every
observed loop sample; the fast context's store does not. Both contexts have the
same64-byte alignment. This identifies a concrete page-crossing hypothesis
missed by the earlier debugger audit. It is **not yet a causal result or a new
full-proof improvement**.

|Boolean context|Loop samples|Frame RSP modulo4096|Store address modulo4096|32-byte store crosses4KiB?|
|:---|---:|---:|---:|:---|
|OFF, slow|1,194|3,840 (0xf00)|4,080 (0xff0)|Yes, all1,194|
|ON, fast|626|192 (0xc0)|432 (0x1b0)|No, all626|

The store uses `[rsp+0xf0]`. In both contexts the frame is0 modulo64 and the
store is48 modulo64, so both stores cross a cache line. Distinguishing cache
lines from4KiB boundaries matters here. The host's base page size is4,096bytes.
Crossing an address boundary does not itself imply a page fault, and no
page-fault explanation or historical mapping/huge-page qualification follows.

|Block|OFF cycles|ON cycles|ON/OFF cycles|ON/OFF instructions|OFF/ON loop samples|
|---:|---:|---:|---:|---:|:---|
|0|8,024,617|4,629,840|0.576955|1.000059|387 /204|
|1|8,212,807|4,570,150|0.556466|1.000003|412 /210|
|2|8,124,453|4,595,741|0.565668|1.000141|395 /212|

The old CPI difference reproduces in all three blocks under the frozen
criterion: cycle ratios at most0.8, instruction ratios within1%. The exact hot
PC at `scalar_groups+0x108a`, immediately after the store, has744 OFF samples
versus one ON sample. All interpreted loop SP samples have the table's
respective placement, across three independent executable backings.

## Measurement and limits

The unchanged216 executable is reused:
`6d9ae8f89e09b4bec8ac7c6a6f795bfbf04594623e4abf39802ada7b4d5d254c`.
No prover, field, driver or dependency is rebuilt. All182 pinned216 source
files remain exact; the private root links to that immutable source snapshot.
The receipt reader also retains its older209-lineage counts, which are not
claims of new219 source changes.

Perf adds only the user stack-pointer register to the existing software
CPU-clock event: period1ms, monotonic timestamps, register mask0x80 and64-bit
ABI. FIFO control enables capture after three primes and disables it before
output. The actual G2 worker TID and bounded clock conversion restrict samples
to its grouping interval. Only `scalar_groups` offsets0x1030–0x1375 are
interpreted as this frame; that loop has no stack-pointer adjustment. Callee,
prologue and epilogue samples are excluded from the frame analysis. All original
samples remain in the private capture.

Unlike217, these observations use normal unpaused SDK execution and ordinary
address randomization. The earlier GDB result remains valid for its own scope:
both inspected contexts had the same32/64-byte alignment. It did not record the
page offsets in the original running profiles. This study supplies that missing
quantity; it does not retroactively change old measurements.

Different page offsets correlate with the running-CPU gap. They do not prove
that the store causes the gap, establish a particular store-forwarding penalty,
or exclude other call-context effects. A bounded source intervention that
prevents the crossing, with fresh controls and unchanged arithmetic, is the
next discriminant. No arbitrary offset sweep or large scratch buffer is
selected here. The previously failed218 Hashbrown recipe remains closed.

## Validation

All seven decoder tests pass, including malformed/missing/duplicate register
fields, ABI checks, metadata ownership and integer timestamp boundaries.
Four4-worker smoke processes precede twelve32-worker main processes: Boolean
OFF/ON crossed with plain/sampled execution, three backings, three rotated and
reversed blocks,251 timed calls and three primes per main process. Native
tracing and internal PMU counters remain enabled in all modes. All capture
finishes before late decoding/Gnark verification; owned CPU helpers stay idle
during capture.

All16 exports pass Gnark;3,064 SDK calls complete. Receipt/source, event, CPU,
PMU, profile and independent register/scope readers pass first execution.
All3,016 native PMU records have100% scheduling coverage. All six profiles pass
the unchanged sample-count, ownership, unknown-symbol, loss, throttle, clock
and ambiguity requirements. The added loop thresholds also pass. SDK observer
ratios are0.997366 OFF and0.999935 ON, both within factor1.05. Reused matching
libc debug symbols resolve names upfront without another capture or threshold
change. Original unused-helper and test-name compiler warnings are retained.

The independent Rust reader reconstructs the clock intersection and scopes
using a linear interval scan, checks all247,449 main samples' register schema,
and reproduces1,950 G2-scoped samples and all1,820 loop register records. It
independently computes the page/cache-line histograms and CPI reproduction
decision. Thirty-two SDK/verifier receipts and64 stream hashes pass in the
separate proof reader. All build-helper/capture/verifier/reader steps pass first
execution. No native suite is replayed for the unchanged executable.

Protocol19:57:14UTC to qualified result20:02:17UTC is5m03s; independent review
finishes20:02:23UTC, within the25-minute allowance. Canonical exports are
**59,062**:
`/home/ubuntu/src/zopro-perf-work/running-stack219/proof-sha256-after-main.json`,
SHA256 `ed597d8585b303890bfaceb4b4084948d87d88a7a375c35c1dcde1aeee58b58c`.
Raw SP/IP addresses, perf data, keys, witnesses, proof bytes, binaries and the
full ledger remain private. Artifacts contain bounded address classes only.
No AWS was used. Production194/196 remains accepted and the all-eight CPU10×
goal remains unmet0/8; no new CPU/Metal speedup is claimed.
