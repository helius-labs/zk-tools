# Controlled scalar-buffer offsets215

Forcing the B scalar buffer to offset32 instead of16 modulo64 did **not**
reproduce214's fast grouping context. Both layouts remain near8million cycles
and CPI2.2. The fixed offset hypothesis and short SDK selection fail; this
prototype is not adopted. This does not rule out every memory-layout effect,
and it does not justify an offset/padding sweep.

|Block|L16 group cycles|L32 group cycles|L32/L16 cycles|L16/L32 CPI medians|
|---:|---:|---:|---:|:---|
|0|8,041,482|8,101,057|1.007408|2.225/2.242|
|1|7,924,785|8,200,147|1.034747|2.192/2.268|
|2|8,033,511|8,089,335|1.006949|2.224/2.240|

Retired instructions remain about3.61million in both layouts. Each table entry
is a process median; ratios and median per-record CPI need not be ratios of
other displayed medians. All PMU records have100% scheduling coverage. This
short screen does not establish a statistically qualified universal slowdown.

|SDK observer mode|L16 ms|L32 ms|Paired L32/L16|Wins|CPU ratio|Peak RSS L16/L32 MiB|
|:---|---:|---:|---:|---:|---:|:---|
|Trace OFF|7.529503|7.585694|1.002869|1/3|1.001508|128.50/134.75|
|Trace ON|7.579927|7.553347|1.000453|1/3|1.002491|128.00/128.50|

SDK times are medians of three process means; paired ratios are medians of
within-block ratios. CPU/RSS guards pass. Observer medians traceON/OFF are
0.999399(L16) and0.996991(L32), both within factor1.05. There is no comparison
against accepted194 or natural allocation alignment, and no adoption claim.
The generic raw reader's OFF/ON labels mean aligned32=false/true in this report;
**Boolean peeling is disabled in every mode**.

## Intervention and safety scope

Both layouts allocate the same capacity of4*n+7uninitialized u64s through the
same allocator path, then choose an interior offset0..7u64s. The existing fused
gather writes directly into that buffer, without an output copy. The owner
retains the original Vec base/capacity for deallocation. A typed mutable word
slice is published only after all recursive fill tasks join. Failed checked
sizes, invalid selectors and reservation failure return before wire conversion;
a fill panic propagates while output lengths remain unpublished. Original
prepare and its tests remain byte-for-byte intact.

The extra requested payload is56bytes per B query; the private typed workspace
forecast adds64bytes conservatively in both modes. No point table, key copy or
new pool is introduced. Required output storage replaces the original B Vec;
it does not coexist with a second full B output array. Peak RSS differences
above are process high-water observations, not a claim that56bytes cause6MiB
or that these peaks are minimum RAM requirements.

All142native core tests pass, including two new tests covering canonical wire/
A/B oracles, empty/tail inputs, concurrent calls in1/4/16/32worker512KiB pools,
invalid-index unwinding, invalid selectors and checked-size overflow. This is
native coverage, not Miri, ARM, pressure or portable qualification. The isolated
source changes four parent files from214;178other runtime files remain exact.
All179production files and all BN254 package files remain unchanged.

## Fixed capture and decision

The diagnostic SDK is
`050841b025e1553d4155134bf953eb3fb4845af788bac22d71e5cf8efd5f3b24`.
Four4-worker smoke processes precede12primary32-worker main processes, with
three independent executable backings and three rotated/reversed blocks.
Main processes use three primes and251timed SDK calls; all main capture ends
before late Gnark verification and no owned CPU cojobs overlap capture.

All16fresh exports pass Gnark,3064SDK calls complete, and all3016recorded
requests show exactly the requested offset,30,089words and56bytes of padding.
The record count includes smoke and timed calls, not unrecorded primes. The
receipt/source/backing reader, CPU reader, PMU reader and separate layout
reader pass.32receipts/64stream hashes and1508active/1508zero PMU records are
checked. All build/test/capture/verifier/reader steps pass first execution.
Protocol18:36:29UTC to result18:45:45UTC is9m16s, within25minutes.

A postcapture check of retained214timestamps also finds grouping starts about
1.5µs after product completion in both Boolean states: the pooled medians differ
by about0.05µs, far below the roughly0.75ms grouping-span difference. This is
additional context, not a changed215selection rule or hardware-cause claim.
The next step should locate the extra cycles within serial grouping, with
profiles scoped to its G2 thread/time intervals, rather than more blind layout
changes. The same function is also used by A, so an unfiltered symbol total
would mix the two queries.

Canonical distinct exports are**59,016**:
`/home/ubuntu/src/zopro-perf-work/scalar-layout215/proof-sha256-after-main.json`,
SHA256`a580ab01695c2785aeaad590b94cff52e5e131e46b5410cfa9f3bf30c21c424b`.
The private root retains complete evidence. Raw inputs, proof bytes, full ledgers
and binaries are excluded here. No AWS was used. Production194/196 remains
accepted and the original all-eight CPU10×goal remains unmet0/8.
