# Subtracting coordinates before radix splitting

The fixed component gate **fails; no actual-caller or SDK integration follows**. Production remains unchanged. This compares packing and arithmetic components, not complete G1/G2 additions, queries or proofs. [Review](artifacts/review.json).

The private constructor transposes canonical R256 operands into four saturated 64-bit word vectors, computes the exact lazy value p+a-b with carry/borrow masks, then splits only the result to radix 52. The original computes two radix 52 conversions followed by subtraction. Both produce identical raw lazy limbs. Norm performs two differences and their fused sum of squares; anchors retains both original converted operands for delta*a+b. The latter checks the cost when later curve work still needs the anchors. All field multiplication/reduction bodies remain original. No table, persistent allocation, key-point copy, new plan, scheduler or representation change is added. The constructor is not used by production.

The [DoT paper](https://arxiv.org/html/2604.21566v3), sections 3.1–3.2, separates word computation and carry handling. It addresses limbs within a SIMD integer; this prototype keeps eight independent fields and changes where input subtraction occurs. No external speedup is transferred. Earlier parallel normalization, mask-carry and direct-reference packing trials retain their original failed gates. This is a new input dataflow rather than a rerun of those kernels.

## Interpretation

This recipe is closed. The Fq norm component improves 2.47% at one worker and 2.48% at 16 workers, but its 32-worker estimate is 0.60% slower, with an interval spanning parity. Retaining both anchors is 3.34% slower at 32 workers, with its entire interval above parity. The three-pattern Fq32 aggregate is 0.612% slower versus fresh OFF, with interval [1.001053, 1.014833]. All 18 calibration checks pass. Positive serial results remain in the record; they do not qualify the constructor for integration.

The input conversion saving does not establish fewer instructions in its surrounding expression. Read-only inspection finds Fq delta kernel sizes 527→529 bytes, norm 2,529→2,566, and retained anchors 1,868→1,991, with unchanged respective IFMA counts. These are static observations, not a causal explanation for the measured result. [Post-capture review](artifacts/postcapture-review.json).

## Fixed measurements

Ryzen 9950X, 16 physical / 32 logical CPUs. Solo pins CPU 2, physical16 CPUs 0–15, logical32 CPUs 0–31. Thirty-one fixed rotated/reversed blocks compare frozen original, fresh ordinary and new constructor. Every worker uses independent sets of 32 rows of changing full-width inputs, three 1,024-update checked primes, then 32,768 updates. Each row cycles deterministically; results accumulate and are checked outside timing against scalar-field sums. The same fresh executable contains both operations with dispatch before the loop. Original and new noinline pattern kernels remain in separate emitted bodies. There is zero allocation in every measured wave; wave latency includes all workers completing. These component inputs are synthetic raw canonical fields, not captured curve coordinates or whole proof workloads.

|Field / pattern / workers|Frozen ns|OFF ns|New ns|Paired New/OFF [bounds]|Wins/31|
|---|---:|---:|---:|---|---:|
|Fq-anchors-1|30.401|30.470|31.767|1.041455531 [1.037431461, 1.048021388]|0|
|Fq-anchors-16|33.900|33.953|35.298|1.041219486 [1.038004483, 1.044853243]|0|
|Fq-anchors-32|48.603|48.062|49.704|1.033412137 [1.019407677, 1.045913567]|3|
|Fq-delta-1|9.060|9.125|9.709|1.067274059 [1.055767905, 1.076011587]|2|
|Fq-delta-16|10.599|10.690|11.296|1.068715943 [1.047507715, 1.074165070]|4|
|Fq-delta-32|24.089|24.013|23.462|0.984590709 [0.963850226, 1.002971113]|20|
|Fq-norm-1|37.996|37.953|37.008|0.975324133 [0.968820003, 0.982156116]|31|
|Fq-norm-16|42.426|42.445|41.478|0.975150851 [0.966675683, 0.980780719]|29|
|Fq-norm-32|61.971|61.497|61.637|1.006011338 [0.997272443, 1.012813806]|12|
|Fr-anchors-1|30.472|30.311|31.700|1.042491045 [1.040148476, 1.047443471]|0|
|Fr-anchors-16|33.808|33.966|35.367|1.040341234 [1.036157479, 1.045040015]|0|
|Fr-anchors-32|48.463|48.575|50.063|1.033704492 [1.028072929, 1.040209481]|0|
|Fr-delta-1|9.000|9.035|9.689|1.073633153 [1.063595849, 1.078756049]|0|
|Fr-delta-16|10.551|10.556|11.348|1.071408950 [1.062389242, 1.084550422]|0|
|Fr-delta-32|24.018|24.008|23.693|0.986882293 [0.975548014, 1.002920693]|20|
|Fr-norm-1|37.904|38.048|37.057|0.977210755 [0.966670750, 0.982184828]|30|
|Fr-norm-16|42.543|42.529|41.552|0.975740290 [0.972797307, 0.981579063]|31|
|Fr-norm-32|61.764|61.519|61.885|1.002060728 [0.994550786, 1.008974687]|12|

Fq32 three-pattern equal-log aggregate, and norm32/anchors32 individually, require upper paired ratios<1 and at least 21/31 wins versus both controls. Every cell's new/control upper ratio must be<=1.03; all fresh/frozen calibration intervals must fit factor 1.03. Ranks 10–22/31 have nominal 97.055% individual coverage conditional on independent exchangeable blocks, without a familywise guarantee. No gain floor, exclusions, extension, changed thresholds, successful reruns or favorable-subset promotion. [Every paired comparison and failed check](artifacts/analysis.json).

Aggregate versus OFF: **1.006119889 [1.001052693, 1.014833289], 9/31 wins**.

Aggregate versus frozen: **1.002421326 [0.991734111, 1.012276036], 14/31 wins**.

## Correctness, source and diagnostics

Two new native field tests pass 139,184 exact lazy-lane inputs over both primes, covering raw powers/modulus boundaries and deterministic random operands. An independent u128/overflowing-sub word oracle checks exact p+a-b, then canonical results and following products match original arithmetic. The 33 unchanged field tests are inherited, not rerun. No actual curve/prover integration or portable qualification is claimed. All 175 production hashes remain exact; removing one include recovers all 17 private field originals. The accepted raw word transpose is copied byte-exactly. The new private API assumes canonical inputs below p and only supports R256. [Bounds](artifacts/bounds.md), [source](artifacts/field-source.json.gz).

The initial offline dependency resolution requested an unavailable cached libc version; the original manifest is retained and only its pin changed to cached 0.2.191. The first new-test compilation stopped before executing any test because its extracted transpose needed an explicit unsafe block and the test array needed an explicit length. Original sources/plans/pins/failed receipts and the two syntax/type repairs are retained. Frozen successful build is reused. No arithmetic or performance protocol changed after a successful test; no successful test/build/capture repeats.

All 1,752 successful workflow commands complete, including 54 preflights, 1,674 latency processes, four separate profiles and 16 postcapture commands. All 1,732 processes match scalar output oracles and cross-variant digests; total 1,519,939,136 component updates including primes. No SDK request or Gnark proof is generated.

Profiles follow all latency: fixed 16M cycles:u, 4,194,304 updates per worker, FIFO enabled after primes and disabled before output checking. All recordings finish before ownership changes, decoding, hashes and archive work. All capture helpers and the original analyzers are compiled and pinned beforehand, with no overlapping owned build, helper, hash, archive, analyzer or other benchmark. Per-worker TID/CPU, time, mappings, fixed-period and loss metadata remain in exact receipts.

|Profile|Samples|Unknown period fraction|All workers sampled|Qualified|
|---|---:|---:|---|---|
|profile-anchors-new|1920|0.000000|true|true|
|profile-anchors-off|1856|0.000000|true|true|
|profile-norm-new|2208|0.000000|true|true|
|profile-norm-off|2238|0.000447|true|true|

Sampled symbol weights are descriptive; no exclusive stall/cost, absolute period coverage, CPI or removable proof-wall fraction is inferred. Stack size, RSS, minimum RAM and pressure behavior is unqualified. Process high-water RSS and faults remain per-observation diagnostics, not prover-memory results. Full original command streams, decoded profiles, source/protocols and observations round-trip byte-exactly in compressed archives; raw perf and native executables/libraries stay local with pins.

After capture, the original publisher stopped on a one-ULP JSON floating-point round-trip difference before writing a report. Its source, executable and failure log remain retained. A separate offline publisher tolerates only 1e-12 relative error in four numeric consistency checks. No observation, statistical threshold or gate changed; independent review recomputes all 56 paired distributions and all 60 decisions from integer wall times. No capture or test was repeated. The first offline reviewer also stopped because the same failed-check set was serialized in a different order; it now compares exact sets while preserving the original source and failure. The fixed publisher was compiled after capture; the original capture helpers and pins remain unchanged.

## Accepted CPU and Metal comparisons

Inherited accepted local proofs versus supplied historical M5 references, unchanged by this component. Original all-eight 10× CPU goal remains active and unmet, 0/8. Primary accepted peak RSS remains 134 MiB. The historical physical-16 calibration engineering exception and accepted final-artifact qualification remain documented.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|8.022165|37 / 4.612x|24 / 2.992x|3.7|
|2x2|11.335969|59 / 5.205x|30 / 2.646x|5.9|
|4x2|15.945544|109 / 6.836x|48 / 3.010x|10.9|
|8x2|24.090485|189 / 7.845x|74 / 3.072x|18.9|
|12x2|33.844855|284 / 8.391x|101 / 2.984x|28.4|
|16x2|39.931557|335 / 8.389x|124 / 3.105x|33.5|
|24x2|66.233309|528 / 7.972x|176 / 2.657x|52.8|
|merge-24x1|65.273224|507 / 7.767x|175 / 2.681x|50.7|

Accepted 54→1 remains 162.250464 ms with no supplied M5 reference. Latest retained C7a 1×2 remains Zopro 11.915591 ms / 32 workers versus Aeglos 7.959184 ms / 96 workers, best tested configurations in that campaign; no AWS result is refreshed. [Exact nine accepted local/AWS rows](artifacts/accepted-rows.json). Proof ledger remains 25,231. No new phone/ARM/TEE, forester 250/4999, swap/pressure/minimum RAM or concurrent-throughput conclusion follows.

[Declared protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [every observation](artifacts/observations.json.gz), [helpers](artifacts/helpers.json.gz), [command receipts](artifacts/receipts.json.gz), [review](artifacts/review.json), [checksums](SHA256SUMS).
