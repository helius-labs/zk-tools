# G2 packed exception filter: complete-query results

**The candidate is rejected.** The complete primary G2 query is4.30% slower against fresh OFF at32workers and2.67% slower solo by paired median. The screen passes27/46 checks; both memory failures and all calibration/nonregression/gain failures remain in the results. Accepted full-proof primary latency remains7.77ms locally at32workers. No new AWS or full-proof timing is claimed.

The original borrowed Window checks complete X equality for each nonempty arrival. This candidate moves classification to the existing packed prefix pass, comparing the low radix52 digit of X.c0 in eight lanes. Matching lanes receive exact scalar Fq2 X checks. This is a false-positive filter, never approximate field equality. Equal-X lanes use complete signed addition, a unit norm and skipped dummy scatter. Ordinary prefixes, shared inversion and point formulas stay original; empty first arrivals retain immediate writes. Scalar tails perform exact classification. Exceptional-path work differs from the original separate projective accumulator.

Two append-only field helpers implement low-digit comparison and masked replacement with one; existing field arithmetic and generator are byte-exact. One new reusable Vec<u8> stores one status byte per full group plus a tail byte. Prefix records remain640bytes. Checked workspace charges status payload plus the aligned header, including the extra embedded scratch header for G1. No key expansion, lookup table, retained solver plan, production pool or artificial RAM cap is introduced. The pinned Aeglos PairBatch exception handling suggested moving classification after packing; this independently written Rust implementation is not a measurement of current upstream Aeglos.

|Context|REF ms|OFF ms|ON ms|ON/OFF [bounds], wins|ON/REF [bounds], wins|
|---|---:|---:|---:|---|---|
|1x2-B-w1|37.283680|37.272785|38.281325|1.026708 [1.025410,1.028707], 0/15|1.027203 [1.025357,1.028417], 0/15|
|1x2-B-w16|3.677649|3.684922|3.776031|1.033690 [1.010854,1.049157], 1/15|1.026622 [1.024406,1.046352], 0/15|
|1x2-B-w32|3.213932|3.245680|3.372984|1.042972 [1.023680,1.055426], 1/15|1.041788 [1.031684,1.057731], 0/15|
|2x2-B-w32|3.843394|3.822502|3.934131|1.034162 [1.010891,1.056573], 2/15|1.032686 [1.007905,1.041880], 2/15|
|1x2-A-w32|1.335393|1.337847|1.327101|0.997523 [0.947194,1.017091], 8/15|1.014298 [0.978822,1.031852], 6/15|
|2x2-A-w32|1.698321|1.673251|1.677945|0.998328 [0.989228,1.031487], 8/15|0.985366 [0.972373,1.019956], 9/15|

B contexts call the proof-specific exclusive Phi G2 entry. Their timers include resetting original scalar words, grouping, all MSM stages, allocation/drop and affine normalization. A contexts are general Fr-input G1 controls, including canonical conversion. Fixture decoding and pool setup precede timing; three checked primes precede each timed range. These are complete query measurements, not proofs. REF reuses the retained block-gap OFF core with accepted arithmetic and private query export, plus field1200. OFF/ON use the new API-only field crate. Their common dependencies, native/O3/fat-LTO/one-CGU settings, allocator and dense busy reset remain fixed.

Ryzen9950X shared host,16physical/32logical CPUs. Default worker stacks with no RUST_MIN_STACK; workers pin to CPU2 solo or CPUi in0..workers. Fifteen fixed rotated/reversed blocks retain all270 latency processes,17timed queries each. The block value is arithmetic mean of integer nanoseconds; displayed latency is median of15means. Paired ratio bounds use sorted zero-based ranks3/7/11, nominal96.484375% marginal median coverage conditional on independent exchangeable blocks, without familywise/stationarity guarantees. No changed thresholds, extra blocks, subset adoption or discarded samples.

The primary OFF/REF calibration passes. ON/OFF has bounds entirely above1 at32workers, as does the solo comparison. The geometric two-B32 paired ratio is1.038778[1.022634,1.052285] against OFF and1.034893[1.029787,1.049738] against REF. Three calibrations fail elsewhere: secondaryB32 lower0.969541 narrowly crosses1/1.03, primaryA32 upper1.051233 and secondaryA32 lower0.964927. These limit clean attribution in those contexts; they do not erase the calibrated primary/solo regression.

Ten nonregression guards, four gain guards, three calibrations and two RSS guards fail. All12diagnostic-quality guards pass. No full-SDK or AWS test of this rejected recipe is selected.

|Context|REF / OFF / ON peak process RSS MiB|OFF / ON CPU ms per query|
|---|---:|---:|
|1x2-B-w1|15.25 / 15.25 / 15.25|37.272 / 38.281|
|1x2-B-w16|27.25 / 27.75 / 27.50|44.826 / 45.931|
|1x2-B-w32|39.50 / 36.75 / 42.25|69.581 / 71.629|
|2x2-B-w32|54.50 / 60.75 / 59.75|75.544 / 77.678|
|1x2-A-w32|26.00 / 25.00 / 24.75|29.078 / 29.165|
|2x2-A-w32|33.25 / 32.50 / 33.50|31.099 / 31.258|

PrimaryB32 ON/OFF and secondaryB32 ON/REF fail the prospective ONmax<=1.05controlmax+1MiB memory guard. Max RSS includes setup/primes/allocator retention. The one-byte logical mask size does not imply a corresponding whole-process RSS change, and these observations are not minimum RAM or pressure/swap qualification.

|G2 diagnostic context|Instructions OFF / ON per query|Cycles OFF / ON per query|
|---|---:|---:|
|1x2-B-w16|545046302 / 551091439|222687054 / 228564656|
|1x2-B-w32|563862454 / 568992317|317957174 / 331602142|
|2x2-B-w32|591005636 / 597341064|347307894 / 360560614|

At primary32 the single PMU panel records about0.91% more instructions and4.29% more cycles. Six grouped PMU panels run101queries each,100%scheduled. Six FIFO-gated fixed16Mcycles:u profiles run1021queries each:114906samples,zeroLOST,at least14213samples/profile,unknown below1.03%,sampled PID/TID/CPU/period/executable mapping checks pass. Primary32 flat samples move8133in the original borrowed kernel to8852in the candidate; feed samples do not make that a causal split. Absolute period coverage is unvalidated. These panels do not isolate executed spills, cache effects or memory capacity. The scalar equality removed was never established to dominate query cost; new filtering/bookkeeping need not be cheaper.

New field tests pass32768groups/262144lanes/all256masks,including131072deliberate low-digit false positives and32768output-fed lazy-product lanes. Exact checks follow filter matches. All130unfiltered core tests pass:127existing plus two batch/thread tests and complete-window comparison. Direct batches check327618independent Arkworks slots over2072cases with117068exceptional lanes,all256signs/fixed full-tail workloads/grow-shrink reuse/input preservation; ordinary cases also match the old kernel. Another122shared/threaded cases exercise53actual workers across1/4/16/32-worker pools and four OS callers sharing an eight-worker pool. Complete-window tests include identities/doubling/opposites/repeatedbases:624cases/1277952original bucket comparisons.11workspace/overflow cases pass. Requested512KiB stacks apply to native test pools only.

The native field helper APIs, OFF/ON cores and no_ifma exclusion compile pass. The exclusion uses the new native field and is a compile guard, not a full generic-field/mobile execution claim. All176production source hashes stay exact. The private snapshot contains171unchanged originals,five reversible parents (including private query export),and five children. Native preparer stdout retained an obsolete172/four-parent label; source-edits and this audited count include the fifth export parent. Caller preparer stdout similarly retains a general-MSM label, while B actually calls the exclusive entry shown in probe.rs.

All346serialized xtask commands through analyses succeed on first execution:8native,6caller/helper builds,18preflights,282capture,30offline,2post. All300caller processes produce12240oracle-checked complete queries including primes. Independent whole-MSM Arkworks oracles are retained inputs; producers are not rerun. Both separately compiled readers are frozen before preflight and reconstruct270means/46decisions with different integer folds/rank implementations. Receipt/profile/PMU parsers are shared; this is a same-agent cross-check. Direct preparers/compiler invocations and publication/checkpoint are separate. During preflight/recording only the confirmed live handle was polled; no owned build/hash/helper/report overlapped.

The comparison below is inherited accepted full-proof local evidence, unrelated to this rejected candidate. Supplied M5 CPU/Metal timings are historical observations on different hardware.

|Shape|Accepted local CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10x target ms|
|---|---:|---:|---:|---:|
|1x2|7.772288|37 / 4.761x|24 / 3.088x|3.7|
|2x2|10.929386|59 / 5.398x|30 / 2.745x|5.9|
|4x2|15.731643|109 / 6.929x|48 / 3.051x|10.9|
|8x2|23.965774|189 / 7.886x|74 / 3.088x|18.9|
|12x2|33.742953|284 / 8.417x|101 / 2.993x|28.4|
|16x2|39.704242|335 / 8.437x|124 / 3.123x|33.5|
|24x2|66.034077|528 / 7.996x|176 / 2.665x|52.8|
|merge-24x1|65.387417|507 / 7.754x|175 / 2.676x|50.7|

Accepted primary remains7.772288ms at32workers,9.049269ms at16,94.218289ms solo;merge54 remains162.096312ms without a supplied M5 reference. No new proof/export occurs;canonical45335 is unchanged. The complete CPU AVX10x goal remains active and unmet,0/8. No new AWS/Aeglos/mobile/TEE/huge/pressure/minRAM/concurrent-throughput result is added. User requested a separate fresh AWS comparison of accepted source; this rejected candidate will not be used.

[Results](artifacts/caller-analysis.json),[reader](artifacts/caller-review.json),[decision](artifacts/decision.json),[design](artifacts/design.md),[protocol](artifacts/caller-protocol.md),[source edits](artifacts/source-edits.json),[reference edits](artifacts/reference-core-source-edits.json),[receipts](artifacts/receipts.json),[accepted CPU/Metal comparison](artifacts/inherited-comparison.json),[continuation](artifacts/next.md),[sources and receipts](sources-receipts.tar.gz),[archive index](artifacts/archive-index.json),[checksums](SHA256SUMS).
