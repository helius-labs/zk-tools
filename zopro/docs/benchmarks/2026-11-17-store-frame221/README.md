# Store-width-aligned grouping frame221

The smaller frame **passes the declared preliminary screen**. At32 workers,
paired complete-proof latency improves0.181% against retained REF and0.074%
against retained OFF, with two wins out of three against each. The16-worker
estimates also improve and all CPU/RSS/calibration checks pass. These are short
screen estimates without confidence bounds, **not adoption qualification**.
The candidate advances to a clean production-source comparison; small gain
size is not a reason to discard it.

|Context|Comparator|Paired ON/control|Wins|CPU ratio|Peak RSS ON/control MiB|
|:---|:---|---:|---:|---:|:---|
|1x2,16 workers|REF|0.989987|2/3|0.989211|82.00 /85.00|
|1x2,16 workers|OFF|0.993243|2/3|0.992606|82.00 /81.25|
|1x2,32 workers|REF|0.998190|2/3|0.997023|129.75 /130.25|
|1x2,32 workers|OFF|0.999261|2/3|1.000399|129.75 /131.00|

|Context|Block|REF mean ms|OFF mean ms|ON mean ms|
|:---|---:|---:|---:|---:|
|16 workers|0|8.716440|8.748550|8.764689|
|16 workers|1|8.993706|8.904509|8.844338|
|16 workers|2|8.910402|8.952092|8.821186|
|32 workers|0|7.491015|7.528360|7.522793|
|32 workers|1|7.607458|7.568558|7.593689|
|32 workers|2|7.633770|7.628831|7.608452|

Each mean covers251 timed complete SDK requests after three primes. Decisions
use medians of within-block ratios; dividing independently displayed medians
can give a different direction. The32-worker gain gate has no minimum size:
both paired medians must be below1 with at least two wins. The16-worker case
requires nonregression within2%; it also has positive observed estimates.
CPU limits are1.02 and RSS limits are1.10×control+8MiB. OFF/REF calibration
ratios1.003684 at16 and0.999353 at32 both pass the declared factor1.03 band.

## Mechanism and footprint

The preceding220 page-sized marker reduced grouping cycles but regressed the
complete proof. That fixed experiment remains closed. This engineering
follow-up derives alignment32 directly from the observed32-byte key store,
rather than searching intermediate padding or alignment values. Hashing,
equality, map-entry size, point arithmetic and admission remain unchanged.

The one-byte native-only Rust marker now has32-byte alignment and object size,
versus4096 in220. The actual function pushes48 bytes, aligns RSP down32 and
reserves480 bytes. With the ABI's possible alignment adjustment, the maximum
activation is **560 bytes including the return slot**, versus16,432 in220 and
480 in the retained function. It passes the declared2048-byte bound without an
exception. Default thread-stack reservations and heap forecasts do not change.

The emitted32-byte store at0x234c26 targets `[rsp+0x100]`, which is always
32-byte aligned. A32-byte aligned store cannot straddle a4KiB boundary. Its two
16-byte reloads lie in the same span. This is a static binary property, not a
new live-register measurement or proof of a unique hardware cause.

|Context|ON/OFF group-cycle ratios, blocks0/1/2|ON/OFF group CPU ratios|Retired instruction change|
|:---|:---|:---|:---|
|16 workers|0.450126 /0.422509 /0.447616|0.478255 /0.448966 /0.478516|about+2.2%|
|32 workers|0.566387 /0.577620 /0.572096|0.606899 /0.608344 /0.597341|about+2.2%|

Thus the large grouping-cycle benefit survives the smaller frame. It does not
translate proportionally into proof latency: other queries overlap grouping
and the MSM join. The32-worker whole-proof CPU comparison against OFF is
essentially flat. Register allocation and cache-line placement also change;
this is not a clean attribution to page splitting alone. The frozen clean-causal
criterion that failed in220 is not revised or represented as passed here.

## Source repair and qualification scope

The initial native consumer passes all142 core tests, including the stable
BTreeMap group oracle, all53 actual workers across1/4/16/32-worker pools with
512KiB stacks, and four OS callers sharing an eight-worker pool. An attempted
parallel portable workflow is rejected by the host lock before any command.
The sequential portable compile then exposes two pre-existing unguarded calls
to the private native-only tracing module. That failure and source are retained.

The corrected source guards those calls with the trace module's exact cfg.
The marker, algorithm, dependencies and driver stay unchanged. The final native
consumer coherently rebuilds core and dependents and passes both affected G2
entry oracles, with140 tests filtered. Its complete grouping-function
disassembly matches the initially qualified native consumer. The original142
tests are inherited, not claimed as rerun on this final consumer. Final portable
validation uses the **same corrected source** and passes all103 core tests and
both prover tests with noIFMA/noASM. These are x86 scalar-fallback checks, not
ARM/phone execution or memory-pressure qualification.

Relative to216,180 source files remain exact. One parent contains the marker
and its test; a second adds the two diagnostic guards. The independent reader
reverses both changes to their originals. No field, Cargo dependency, pool,
heap buffer, proving-key copy or persistent table changes. The repairs complete
before any221 SDK execution, with unchanged performance gates.

REF is the exact retained216 diagnostic SDK, OFF the exact retained220 disabled
consumer; neither is claimed as freshly rebuilt for221. Final ON is
`c4a57c54babb5729bf66a77dae1c2aa686d6d42829ed2c719bcac2d728ad4774`.
Production194 is not a control in this screen. A clean default build against
that exact accepted consumer, broader latency/resource checks and all eight
supplied shapes plus54-to-1 remain necessary before promotion.

## Capture and decision

Three4-worker smoke processes precede eighteen main processes: three roles,
two worker counts and three rotated/reversed blocks. Three independent
byte-identical backings per arm all participate. Boolean peeling is disabled;
native tracing and internal PMU profiling are enabled in all modes. All main
capture finishes before late Gnark verification and analysis, with no owned
CPU cojobs during capture. The actual SDK/verifier/reader steps pass first
execution after the disclosed preparation repair.

All21 exports pass Gnark;4,584 SDK calls complete. All4,521 recorded requests
pass event, route, placement, CPU/PMU and proof checks with100% PMU coverage.
Two independently written raw readers agree on means, pairing, stage medians,
source reversal and every decision;42 receipts and84 stream hashes pass.
Protocol20:26:30UTC to result20:42:42UTC is16m12s, within30minutes.

Canonical exports are **59,095**:
`/home/ubuntu/src/zopro-perf-work/store-frame221/proof-sha256-after-main.json`,
SHA256 `8bde73fa8b3e9b3507774334d3acf8a1631dd51d8ba60cb5ab71324df3378ba2`.
Complete evidence remains private under that root. Raw witnesses, keys, proofs,
full ledgers and binaries are excluded here. No AWS was used. Production194/196
and its [CPU/Metal comparison](../2026-11-17-certified-all196/README.md) remain
accepted; the original all-eight CPU10× goal remains unmet0/8.
