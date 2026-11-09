# Query-work removal in complete CPU proofs

**G2 and K/H query work materially affect local 32-worker proof latency.** In this diagnostic, primary 1×2 falls from 8.024 ms with all original work to 5.447 ms when the exact G2 query result is reused, 6.497 ms with K/H reused, and 2.452 ms with all three queries reused. These are controlled work-removal results for one exact key and witness, **not fresh-work prover speedups or an adoptable cache optimization**. Production remains unchanged; the all-eight CPU 10× objective stays active and unmet, 0/8.

The data, coverage and independent Gnark checks pass. **The full study fails its combined diagnostic qualification:** both 16-worker control intervals exceed the declared ±3% range, and one of 30 profiles exceeds the 5% unknown-symbol limit. Both 32-worker contexts pass their control checks. All observations and failures are retained; no recording was extended, excluded or rerun.

## Complete-request timings

Every active arm learns original unblinded query results during prime1 for the exact Key generation, witness SHA256, omit-B mode and query path. Prime2/3 and measured requests use the specified mask. All requests still solve, construct the quotient, apply corrections, draw fresh blinding, assemble, serialize and verify. Hash/scope/lookup overhead is inside the complete SDK interval. The prior [native qualification](../2026-11-09-query-neutralization-native176/README.md) checks fallback, ownership, mutation and worker transport.

The table gives the median of 15 process means, each mean containing 31 timed requests after three primes. The 16-worker column failed calibration and remains descriptive. A=1, G2=2, K/H=4; active mask0 performs every original query.

|Arm / removed query work|1×2, 16 workers ms|1×2, 32 workers ms|4×2, 32 workers ms|
|---|---:|---:|---:|
|Accepted SDK6d83|9.454630|8.031249|15.961623|
|Fresh inactive OFF|9.469802|7.959216|16.192813|
|Active0: none|9.659894|8.023722|16.199434|
|A|8.466018|7.630239|15.414283|
|G2|6.514417|5.446772|12.829493|
|A + G2|6.058245|5.098245|11.813558|
|K/H|7.031360|6.496744|12.606017|
|A + K/H|6.132306|5.990267|11.880459|
|G2 + K/H|4.005518|3.875174|8.437663|
|A + G2 + K/H|2.237923|2.452107|5.658279|

Paired ratios below compare each removed-work process mean with active0 in its matched block. Intervals are nominal 95% paired-log bootstrap intervals, 20,000 resamples, fixed SplitMix64 seed and ranks 499/19,499. They assume exchangeable blocks and have no familywise guarantee. Ratios need not equal ratios of the separately pooled medians above.

|Removed work|1×2/32 ratio [interval]|4×2/32 ratio [interval]|
|---|---:|---:|
|A|0.9519 [0.9452, 0.9581]|0.9495 [0.9425, 0.9575]|
|G2|0.6829 [0.6708, 0.6978]|0.7927 [0.7880, 0.7967]|
|A + G2|0.6332 [0.6288, 0.6376]|0.7291 [0.7251, 0.7338]|
|K/H|0.8080 [0.8016, 0.8140]|0.7748 [0.7697, 0.7804]|
|A + K/H|0.7460 [0.7416, 0.7505]|0.7312 [0.7259, 0.7362]|
|G2 + K/H|0.4819 [0.4774, 0.4865]|0.5214 [0.5175, 0.5253]|
|All queries|0.3048 [0.3022, 0.3078]|0.3485 [0.3464, 0.3507]|

Removing A alone has a small wall effect while other queries run; its effect is larger after G2 and K/H disappear. This illustrates overlap and helping: these differences cannot be added as exclusive phase costs. The 2.452 ms residual is neither an immutable lower bound nor a prediction of an implementation that computes the missing queries faster. A real optimization must perform fresh query work for arbitrary witnesses.

## Controls and profile limits

Both control intervals must lie wholly in [0.970874, 1.030000]. These limits were frozen before any preflight.

|Context|OFF / accepted interval|Active0 / OFF interval|Control checks|
|---|---:|---:|---|
|1x2-w16|[0.988116, 1.105243]|[0.948921, 1.103843]|Fail; retained|
|1x2-w32|[0.985313, 1.001358]|[0.998895, 1.010775]|Pass|
|4x2-w32|[1.002095, 1.016660]|[0.995858, 1.011714]|Pass|

Thirty gated four-event PMU panels have 100% scheduled time for all 120 event values. Thirty separate warm cycle profiles use period16,000,000 and 512 mmap pages, totaling **720,598 samples**. All have zero LOST and at least 95% assigned-thread samples. Twenty-nine meet every profile rule. **4×2/32 mask7 fails:** 5.338738% unknown exceeds 5%, despite 17,270 samples, 100% assignment and zero LOST. It is not reclassified or extended. The minimum profile population is 2,030, above the declared 1,000.

PMU panels are single descriptive observations, not repeated-counter uncertainty estimates. For primary32, the active0 panel records about 1.387 billion instructions/request; noG2 about 826 million; noK/H about 894 million; noqueries about 119 million. G2/KH removal also changes cache-miss counts and allocator activity. This does **not** establish RAM bandwidth, page faults or cache stalls as the only cause. Pin/steal symbols become a larger sampled share when query work is removed; that share is not removable wall time.

## Memory and faults

The diagnostic retains four G1 results and one G2 result: 576 bytes of coordinates, inside a 944-byte native controller before Arc/allocator overhead. It retains no key, scalar, witness, execution-plan or point-array copies and creates no new production pool. The state is diagnostic only.

Selected primary32 process peaks and timed resource medians:

|Arm|Peak RSS MiB|User CPU ms/request|System CPU ms/request|Minor faults/request|
|---|---:|---:|---:|---:|
|Accepted|112.75|174.180|5.501|438.774|
|Active0|112.25|174.358|5.804|441.516|
|No G2|72.25|107.625|4.223|181.032|
|No K/H|115.25|119.831|5.858|442.968|
|No queries|69.00|28.127|3.835|158.194|

All 549 processes with the added major-fault counter report zero timed major faults. The immutable accepted SDK does not expose this counter, so its value is unavailable. RSS includes setup and is a maximum over processes; fault/CPU medians cover timed requests after primes. These do not prove minimum RAM, absence of startup paging, mobile suitability, pressure/swap behavior or concurrent throughput. Full resource results for every arm/context remain in the analysis.

The two benchmark consumers were relinked solely to add `ru_majflt` to the support module's JSON. The qualified driver is byte-identical and all native prover libraries/field1200 are reused exactly. Source/link recovery is checked. The new actual consumers have 21 inspected point-query Rayon callback bodies; the earlier consumers' 21 decodes are retained separately. No native suite or successful decoder is replayed on its unchanged artifact.

## Verification and workflow

The prospective reader checked the complete schedule, masks, CPU affinities, inputs and analysis source before any request. The serialized workflow performs 100 ALL8+merge54/primary16 preflights, 450 latency processes and 60 separate PMU/profile processes. **31,840 SDK requests** include three primes per process. Every process exports only its last proof: **610 distinct exports**, all independently Gnark-verified with image/public/verifying-key bindings. The seeded 37,023-proof inventory becomes **37,633**. Exact hit/compute/miss, ownership, path and entry-hop counts pass for every active process; primary uses fused path1 and 4×2 uses ordinary path4. The SDK also verifies each proof internally.

All latency precedes PMU/profiles; all recording precedes the 610 independent verifiers and 150 ownership/decode/events/metadata/loss commands. The parent only polls the confirmed live handle throughout recording; no owned build/helper/hash/analyzer/decoder/verifier/report/push overlaps it. All 1,370 capture/offline commands finish successfully.

The first results reader rejects exact float-array equality after JSON roundtrip: 12.907180516129031 versus 12.907180516129033. A separate preserved reader extends its existing 1e-12 scalar comparison to float arrays/contrast objects and reviews all eight CPU/Metal references. Integer/hash/count checks remain exact. No sample, bootstrap rule, control gate or profile threshold changes; the three qualification failures remain failures. The analyzer, recordings and independent verifiers are not rerun. The same-agent reader independently reconstructs raw means, ratios, resources and decisions, while sharing the bootstrap, coverage, crypto-binding and PMU parser; it is not an external audit.

The stage has 1,376 workflow command receipts: 1,375 successes and the initial results-reader failure. Before execution, three helper compilations failed on `json!([0;5])`; an explicit five-zero array fixes the macro syntax. Initial source is retained. Manual helper preparation/compilation/callback decoding/publication has tool-transcript evidence, not extra workflow receipts.

## Next real optimization and accepted references

Prioritize a substantial reduction in online G2/KH point work, with complete fresh proofs and bounded memory. The data does not select an implementation. Existing phi-window merging is already implemented, and the prior independent A/B matrix-rank census, GLS4, wNAF, shifted-key layouts, ring recoding, inverse-chain and PMNS studies remain closed. A new mechanism must be distinct and checked against those reports before a caller/full-proof trial. The [continuation loop](artifacts/next.md) preserves the full objective and constraints. No new paper, AWS, Aeglos, huge-circuit, phone or TEE measurement is added here.

Only the **accepted full-work** executable appears below. Primary1×2 and 4×2 are freshly measured; the other six timings are inherited from the [previous full-shape campaign](../2026-11-08-tree-policy-sdk176/README.md). Supplied M5 CPU/Metal values use different hardware. Cached-query times are excluded.

|Shape|Accepted local ms|Evidence|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---|---:|---:|---:|
|1x2|8.031249|Fresh|37 / 4.607×|24 / 2.988×|3.7|
|2x2|11.149097|Inherited|59 / 5.292×|30 / 2.691×|5.9|
|4x2|15.961623|Fresh|109 / 6.829×|48 / 3.007×|10.9|
|8x2|24.131178|Inherited|189 / 7.832×|74 / 3.067×|18.9|
|12x2|33.940628|Inherited|284 / 8.368×|101 / 2.976×|28.4|
|16x2|40.164083|Inherited|335 / 8.341×|124 / 3.087×|33.5|
|24x2|66.187704|Inherited|528 / 7.977×|176 / 2.659×|52.8|
|merge-24x1|65.361571|Inherited|507 / 7.757×|175 / 2.677×|50.7|

Merge54 remains inherited160.890372 ms with no supplied M5 reference. The same accepted SDK previously measured primary7.915095 ms; private7.859756/7.869400 ms candidates were unadopted. Current8.031249 ms establishes neither an adopted regression nor its cause. Accepted local primary remains roughly8 ms. No shape meets CPU10×; the goal remains active.

[Full timings, resources and proof bindings](artifacts/analysis.json), [PMU/profiles](artifacts/diagnostics.json), [findings](artifacts/findings.json), [frozen protocol](artifacts/protocol.json), [design](artifacts/design.md), [prospective review](artifacts/prospective-review.json), [results review](artifacts/results-review.json), [reference review](artifacts/reference-review.json), [consumer audit](artifacts/consumer-audit.json), [actual callbacks](artifacts/callback-inspection.json), [source/raw receipts/decoded profiles](artifacts/evidence.json.gz), [checksums](SHA256SUMS). The archive stores hash-bound UTF8 members, with verified gzip roundtrip. Binaries/libraries, raw perf recordings, keys/witnesses/proofs, unchanged production source and the repeated full historical proof ledger are excluded. Raw perf and ledger identities remain recorded; reproduction needs the pinned inputs/tools and adapted absolute paths.
