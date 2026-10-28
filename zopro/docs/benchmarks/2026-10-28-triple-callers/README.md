# Native three-point addition caller

The input-known-denominator three-point formula improves the tested small native callers. G1 is 10.6–38.0% faster across 8–64 triples; G2 is 22.8% faster at eight and 5.5% faster at sixteen, but regresses at larger sizes. **This qualifies a size-specific caller opportunity, not a production MSM or proof-time improvement.** Accepted local 1x2 remains 8.324853 ms at 134.50 MiB peak RSS.

The [exact algebra screen](../2026-10-28-triple-affine-screen/README.md) derives the formula and shows its greater variable multiplication work. The native implementation uses existing field operations and one shared inversion stage. Its control calls the unchanged production `BatchAdd::apply` twice. Both update the same permuted bucket destinations, adding Q and R each round. There is no field backend change or per-key point table.

## Native results

Ryzen 9950X, CPU 2, one active caller and Rayon setup pool of one. One executable contains both algorithms, with selection outside the round loop. Each of 31 rotated/reversed process blocks covers all eight cases and both variants: 496 latency processes, plus 16 preflights. Sizes 8/16/32/64 use 8192/4096/2048/1024 rounds, after three eight-round primes and reset outside timing. Every output is verified after timing and all samples remain.

Times are median ns per round. Change uses the median within-block direct/binary ratio, rather than the ratio of displayed medians. Intervals use ranks 10..22 of 31, nominal 97.055% under independent/exchangeable blocks, without a familywise guarantee.

| Caller | Binary ns | Direct ns | Change | Direct/binary interval | Wins |
|---|---:|---:|---:|---:|---:|
| G1, 8 triples | 1585.91 | 984.27 | +37.98% | 0.6191–0.6236 | 31/31 |
| G1, 16 triples | 1873.10 | 1301.06 | +30.53% | 0.6924–0.6958 | 31/31 |
| G1, 32 triples | 2437.23 | 1934.74 | +20.72% | 0.7278–0.7947 | 31/31 |
| G1, 64 triples | 3582.60 | 3207.08 | +10.58% | 0.7953–0.8988 | 31/31 |
| G2, 8 triples | 2015.62 | 1553.61 | +22.82% | 0.7622–0.7743 | 31/31 |
| G2, 16 triples | 2567.23 | 2432.65 | +5.48% | 0.9099–0.9514 | 31/31 |
| G2, 32 triples | 3682.95 | 4202.31 | -14.05% | 1.1382–1.1426 | 0/31 |
| G2, 64 triples | 5983.22 | 7733.43 | -29.20% | 1.2882–1.2938 | 0/31 |

The six positive cases pass the declared per-case gain condition: upper ratio below one and at least 21 wins. There is no minimum gain floor. The two G2 regressions are retained and exclude a blanket replacement. No fresh SDK or performance-gate promotion follows from this table alone.

## Correctness and memory

Two new native tests pass once against independent Arkworks generator scalar sums. They cover fixed full-width scalar arrays at lengths 0/1/7/8/9/16/32/64/128, both algorithms, resets and 1/3/17-round chains. Each curve also checks 216 boundary combinations involving zero, one, minus one, two, minus two and three. The exceptional entry is in the second vector group, after a regular group has prepared records, checking that fallback preserves original buckets.

The direct path checks input identities and unpacks denominator norms to detect zero before inversion or output writes. It uses complete scalar curve additions if a guard fires or the call length is not a multiple of eight. The binary wrapper also checks each layer's exceptional inputs and uses complete fallback. This is a correctness-first native integration; scalar tails and rare fallback cost are not optimized. The fixture validates distinct destination indices outside timing, matching the existing streaming batch contract.

Direct records retain coordinates to avoid reverse repacking: eight FqBatch words per G1 group or sixteen per G2 group, 2560/5120 bytes respectively. This is the larger record alternative described before timing, not the earlier minimal reread layout. All measured ordinary loops report zero allocation calls.

| Triples | G1 direct / binary scratch bytes | G2 direct / binary scratch bytes |
|---|---:|---:|
| 8 | 2560 / 1600 | 5120 / 2560 |
| 16 | 5120 / 3200 | 10240 / 2560 |
| 32 | 10240 / 6400 | 20480 / 2560 |
| 64 | 20480 / 12800 | 40960 / 5120 |

These are algorithm-specific retained vector capacities after priming, not total process RSS or minimum RAM. The common fixture also reserves direct records when running the binary control, and holds inputs, source buckets and scalar references; its full footprint is larger. Future production integration must charge scratch through existing resource admission and avoid allocating both algorithm workspaces unnecessarily. No universal memory cap, pressure, phone, TEE or large-circuit qualification is inferred.

## Profiles and limitations

All latency captures precede four whole-process cycles:u profiles at 199 Hz, each with 1,048,576 eight-triple rounds. G1 binary/direct have 338/213 samples; G2 binary/direct have 418/329, 1298 total, one PID per recording and zero lost records. Setup, primes and final checks are included, so these are not warm-gated profiles. Raw recordings remain local with hashes; decoded symbols and loss dumps are archived.

Scalar inversion receives about 68%/61% of G1 binary/direct sampled period and 55%/30% of G2. The formula structurally removes one inversion stage; sampled percentages do not establish a causal wall-time fraction. G2's binary profile also includes production dispatch/environment lookups absent from the direct adapter. The table measures each complete declared caller, including its screening, packing, dispatch and data movement; it does not isolate the algebra alone. Static code confirms 1550/4450 IFMA sites in the G1/G2 direct bodies, excluding the separate inversion bundle. Counts and stack operand lines are not executed instructions, spill counts or whole-process peak RAM.

All 436 pinned paths match before and after capture. The private snapshot changes only two original files by diagnostic module/reexport suffixes, whose removal recovers accepted bytes; 170 original private files and all 172 production files remain unchanged. No build, helper/hash/archive or second owned workload overlaps capture. Both builds, the two tests, all latency and all profiles complete without a failed attempt or repeat.

The next requirement is actual MSM geometry: determine where two dependent additions share a destination, how many occur together, and whether combining them reduces a real flush or tree stage. Existing streams often have large batches or only one pending addition per bucket; the fixture does not prove frequent small triple batches. Require production caller integration and full SDK/proof verification before any adoption. No new Gnark checks, proof exports, AWS/Aeglos comparison or proving-time claim is added; the independent proof ledger remains 12,524.

## Inherited CPU and Metal comparison

The [accepted all-eight report](../2026-10-27-clean-fft-adoption/README.md), including its full-performance-gate failure and engineering exception, remains authoritative. M5 figures are supplied historical references. The original all-eight CPU 10x goal remains active and unmet, 0/8.

| Shape | Accepted ms | M5 CPU ms | CPU/accepted | M5 Metal ms | Metal/accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 4.445x | 24 | 2.883x |
| 2x2 | 11.460675 | 59 | 5.148x | 30 | 2.618x |
| 4x2 | 15.854365 | 109 | 6.875x | 48 | 3.028x |
| 8x2 | 24.084626 | 189 | 7.847x | 74 | 3.072x |
| 12x2 | 33.693397 | 284 | 8.429x | 101 | 2.998x |
| 16x2 | 39.981636 | 335 | 8.379x | 124 | 3.101x |
| 24x2 | 66.010670 | 528 | 7.999x | 176 | 2.666x |
| Merge24x1 | 65.086624 | 507 | 7.790x | 175 | 2.689x |

Accepted merge54 remains 161.341835 ms without a supplied M5 reference.

[Protocol](artifacts/design.md), [results and profile symbols](artifacts/analysis.json), [native tests](artifacts/tests.log), [implementation](artifacts/triple_probe.rs), [source audit](artifacts/source-audit.json), [pins](artifacts/pins.json), [workflow completion](artifacts/runs/result.json), [artifact hashes](artifact-index.json).
