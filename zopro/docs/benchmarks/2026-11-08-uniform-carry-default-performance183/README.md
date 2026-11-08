# Default combined carry arithmetic: performance qualification

The default SDK measures **7.897108 ms** for local 1×2 at 32 workers, versus accepted **8.018438 ms** and fresh accepted-source control **7.997719 ms**. The frozen full-scope performance selection is **failed**, with **42/45 checks passing**. No production source is adopted in this report. The CPU 10× goal remains unmet: **0/8** supplied shapes reach the point target.

The original-eight equal-log aggregate improves by **0.693% versus accepted** and **0.810% versus fresh control**, with both stratified upper bounds below parity. All checks at 32 workers, all memory screens and automatic diagnostic-quality checks pass. The three failures are control calibration and candidate nonregression uncertainty against both controls at 16 workers; a reliable speedup or slowdown at 16 workers is not established.

Sub-8-ms results predate this campaign: earlier uniform-only **7.859756 ms**, combined private **7.869400 ms** and accepted **7.915095 ms** observations remain valid history. This is the first performance qualification of the distinct clean default artifact. It establishes no new absolute record or improvement over a prior candidate across separate runs. The earlier private full gate remains failed; this new result does not rewrite it.

The candidate is the unchanged native SDK `e77c02d7` from [default-source qualification](../2026-11-08-uniform-carry-default182/README.md), with [functional and budget qualification](../2026-11-08-uniform-carry-compat183/README.md). It uses the same uniform scaled prefixes and typed uncarried X-sum arithmetic as its private parent, with normal native feature guards and diagnostic helpers restricted to tests. No new lookup table, key copy, solver plan, runtime buffer or production thread pool is introduced.

## Full SDK latency and process memory

Local AMD Ryzen 9 9950X. Values are medians of matched process means. Every process has 3 untimed primes, followed by 151 timed primary requests or 51 for other shapes. Each timed SDK call includes solving, randomized proving, internal verification and serialization; loading, primes and exported-proof writing are excluded. All observations are retained. CPU affinity is 0–31 at 32 workers and 0–15 at 16 workers.

|Context|Blocks|Accepted ms|Fresh control ms|Default candidate ms|Maximum RSS MiB, accepted / control / candidate|
|---|---:|---:|---:|---:|---:|
|1x2-w32|31|8.018438|7.997719|7.897108|137.00 / 134.25 / 136.50|
|2x2-w32|15|11.199751|11.206807|11.005619|291.75 / 298.50 / 287.75|
|4x2-w32|15|15.957517|16.078278|15.947772|324.75 / 332.75 / 333.75|
|8x2-w32|15|24.250900|24.101776|24.274284|556.00 / 538.50 / 530.75|
|12x2-w32|15|33.735233|33.938479|33.500639|755.00 / 749.00 / 754.75|
|16x2-w32|15|40.165709|40.087967|39.820752|857.75 / 866.50 / 850.00|
|24x2-w32|15|65.802777|66.307363|65.888758|1360.75 / 1354.21 / 1374.61|
|merge-24x1-w32|15|64.926690|65.166397|64.536707|1224.00 / 1214.11 / 1234.82|
|merge-54x1-w32|15|162.442299|162.612539|162.419007|1983.61 / 1873.79 / 1906.92|
|1x2-w16|15|9.510273|9.355410|9.303896|89.75 / 90.50 / 89.75|

RSS is the largest process `ru_maxrss` including loading/primes. It is not minimum physical RAM, a cgroup pressure result or a mobile-device requirement. The declared screen is candidate ≤ larger control × 1.05 + 1024 KiB. The separate few-request budget study observed no active reclaim; those accounting/allocator histories are not merged with these long-loop process peaks.

## Paired estimates and selection

|Context|Candidate / accepted, paired GM [95% bootstrap]|Wins|Candidate / control, paired GM [95% bootstrap]|Wins|
|---|---:|---:|---:|---:|
|1x2-w32|0.985538 [0.982646, 0.988391]|30 / 31|0.988180 [0.985792, 0.990636]|28 / 31|
|2x2-w32|0.984964 [0.980486, 0.989857]|14 / 15|0.984151 [0.980926, 0.987649]|15 / 15|
|4x2-w32|1.000368 [0.993960, 1.007384]|6 / 15|0.991623 [0.984914, 0.999286]|12 / 15|
|8x2-w32|0.995173 [0.983445, 1.006583]|7 / 15|1.005689 [0.998555, 1.013239]|6 / 15|
|12x2-w32|0.993164 [0.988675, 0.997617]|12 / 15|0.985668 [0.978849, 0.991425]|14 / 15|
|16x2-w32|0.994142 [0.987732, 1.001095]|11 / 15|0.997338 [0.990921, 1.004384]|10 / 15|
|24x2-w32|0.999758 [0.994840, 1.005041]|8 / 15|0.990977 [0.986350, 0.995708]|12 / 15|
|merge-24x1-w32|0.991538 [0.985364, 0.997568]|12 / 15|0.991727 [0.988255, 0.995204]|13 / 15|
|merge-54x1-w32|1.000131 [0.996915, 1.003341]|8 / 15|1.000177 [0.995922, 1.004031]|5 / 15|
|1x2-w16|0.999816 [0.945180, 1.059715]|8 / 15|1.018974 [0.964619, 1.077402]|9 / 15|

Selection for primary 1×2 at 32 workers uses its declared 31-block median-ratio rank interval:

|Primary32 contrast|Median paired ratio|Sorted-rank interval10–22|Wins /31|
|---|---:|---:|---:|
|Control / accepted|0.996257142|[0.995104226, 1.000229516]|22|
|Candidate / accepted|0.986093541|[0.982878427, 0.990527375]|30|
|Candidate / control|0.987420014|[0.984569077, 0.989404118]|28|

Primary 1×2 at 32 workers requires upper < 1 and ≥ 21 wins against both controls, with control/accepted calibration within factor 1.03. Each other context requires calibration within 1.03 and candidate/both-controls bootstrap upper ≤ 1.02. Original-eight equal-log stratified gain upper must be < 1 against both controls, with no minimum gain floor. All RSS and diagnostic-quality checks also apply. These rules, all counts and ordering were frozen before clocks.

|Original-eight aggregate|Equal-log paired GM|Stratified 95% bootstrap|
|---|---:|---:|
|Candidate / accepted|0.993066158|[0.990774021, 0.995372499]|
|Candidate / control|0.991898310|[0.989969573, 0.993842538]|

Each supplied shape has equal log weight, with 31 primary blocks and 15 each otherwise; merge54 and primary 1×2 at 16 workers are outside this aggregate. The exact qualified bootstrap uses 20,000 fixed-seed resamples independently within each shape. Exchangeable-block/independent-stratum assumptions are not validated against host drift or cross-shape correlation; these are marginal intervals, not a familywise guarantee.

The earlier study systematically put the first run at 16 workers after a run at 32 workers. Here all 15 primary triplets at 16 workers run first, followed by a separate cohort at 32 workers: 15 rotating all-shape blocks, then 16 additional primary blocks. Three primes and all first-block observations remain. This prospective design removes that systematic repeated transition; it does not establish the cause of the old tails or exclusive host use. There are no exclusions, extra blocks, unchanged retries or post-result threshold changes.

Separating the cohorts did not make the 16-worker comparison calibrate in this campaign. Control/accepted is **0.981199 [0.938677, 1.029203]**. Candidate/accepted is **0.999816 [0.945180, 1.059715]** and candidate/control is **1.018974 [0.964619, 1.077402]**. The lower candidate marginal median does not override this paired uncertainty. These observations leave the cause unresolved.

Failed checks remain failed:

- `1x2-w16:OFF/accepted-calibration`
- `1x2-w16:candidate/accepted-nonregression`
- `1x2-w16:candidate/OFF-nonregression`

|Context|Control / accepted, paired GM [95% bootstrap]|
|---|---:|
|1x2-w32|0.997326 [0.995061, 0.999632]|
|2x2-w32|1.000827 [0.996723, 1.004857]|
|4x2-w32|1.008819 [1.002056, 1.015955]|
|8x2-w32|0.989543 [0.981962, 0.997148]|
|12x2-w32|1.007604 [1.003431, 1.012129]|
|16x2-w32|0.996795 [0.991943, 1.000832]|
|24x2-w32|1.008862 [1.001852, 1.015792]|
|merge-24x1-w32|0.999809 [0.995115, 1.004257]|
|merge-54x1-w32|0.999954 [0.995006, 1.004771]|
|1x2-w16|0.981199 [0.938677, 1.029203]|

## Automatic CPU diagnostics

All 498 latency processes precede 18 diagnostic processes. Primary 1×2 at 16/32 workers uses 31 counter and 251 profile requests per arm; merge54 uses 7/17, each after 3 primes. Four grouped user PMUs and FIFO-gated user cycles at period 16 million / 512 mmap pages use the unchanged SDK gating protocol. All 516 SDK processes finish before any independent proof verifier or offline decoder. No owned build/helper/hash/analyzer/decoder/verifier/report/push runs during a recording. Exclusive host use is not established.

|Context and arm|Profile samples|Unknown %|Assigned SDK execution %|FFT-leaf samples|Quality|
|---|---:|---:|---:|---:|---|
|1x2-w16-accepted|8629|1.298|100.000|123|true|
|1x2-w16-candidate|8563|1.331|100.000|114|true|
|1x2-w16-control|8766|1.460|100.000|116|true|
|1x2-w32-accepted|12274|1.817|100.000|231|true|
|1x2-w32-candidate|11987|1.852|100.000|197|true|
|1x2-w32-control|12321|1.680|100.000|228|true|
|merge-54x1-w32-accepted|22018|0.622|100.000|4943|true|
|merge-54x1-w32-candidate|21824|0.729|100.000|4805|true|
|merge-54x1-w32-control|21463|0.718|100.000|4975|true|

The quality requirement is ≥ 1000 samples, ≥ 95% SDK main + assigned Rayon workers, ≤ 5% unknown, zero LOST records, valid PID/TID/MMAP/period/CPU bindings and FFT-leaf presence; each of four PMUs must report ≥ 99.9% running. Full event values, running percentages and ranked symbols are in the diagnostic artifact.

|Context, candidate / control|User cycles|Instructions|Generic cache misses|Branch misses|
|---|---:|---:|---:|---:|
|1x2-w16|0.980771|0.979916|0.998106|0.999795|
|1x2-w32|0.990543|0.981115|0.928453|0.995710|
|merge-54x1-w32|1.005970|0.980216|0.999331|0.986442|

One counter panel/profile per context and arm does not establish repeated-counter uncertainty, exclusive wall-time fractions or a causal cache/DRAM/spill/scheduling explanation.

## Proof verification and provenance

All 1077 workflow commands complete: 498 latency + 18 diagnostic SDK processes, then 516 independent Gnark verifiers and 45 offline ownership/decoder/events/metadata/loss commands. **42,510 fresh SDK requests** include primes; **516 fresh exported proofs** are independently verified, 164 bytes each with 44 public-input bytes, bound to image/public/expected VK and distinct from prior inventory and one another. Canonical inventory advances **35,961 → 36,477**. Unexported internally verified requests are not counted as independent Gnark exports.

The accepted SDK `6d83` and default `e77` are reused byte-exact. One fresh control `0a16` uses all 176 exact accepted source files, the exact shared driver/perf source, pinned Rust 1.98.1 / native O3 / fat LTO / one codegen unit, with all 8 direct libraries found in its actual Cargo artifact stream and all link input hashes checked. Default qualification of 183 source files, 297 tests, generator/codegen and 284 SDK requests / 98 Gnark exports with functional/budget checks is inherited by identity; no successful preflight, test, old decoder or old proof verifier is replayed for bookkeeping.

The prospective reader checks the actual command matrix and qualified statistics before clocks. The final reader checks pins/command and stream receipts, reconstructs every fresh binding/inventory union, raw means, paired ratios, rank intervals, wins, RSS, aggregate points, selection and CPU/Metal comparison points. It shares established crypto-binding and detailed diagnostic parsers; bootstrap source is checked against the qualified implementation rather than independently reimplemented or replayed. Source preparation corrected the verifier CLI arity before compilation/freezing; no SDK trial was selected or repeated for that correction.

Public evidence retains sources/helpers, plans/pins, command receipts, raw SDK sample logs and decoded diagnostics with per-member hashes. Actual keys/images/requests/witness/proof/public payloads, executables/libraries, rawperf and full canonical inventories remain private.

## All supplied M5 comparisons

These are local default-CPU observations against supplied M5 references on different hardware.

|Shape|Local default CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.897108|37 / 4.685×|24 / 3.039×|3.7|
|2x2|11.005619|59 / 5.361×|30 / 2.726×|5.9|
|4x2|15.947772|109 / 6.835×|48 / 3.010×|10.9|
|8x2|24.274284|189 / 7.786×|74 / 3.048×|18.9|
|12x2|33.500639|284 / 8.477×|101 / 3.015×|28.4|
|16x2|39.820752|335 / 8.413×|124 / 3.114×|33.5|
|24x2|65.888758|528 / 8.014×|176 / 2.671×|52.8|
|merge-24x1|64.536707|507 / 7.856×|175 / 2.712×|50.7|

Merge54 takes **162.419007 ms** and has no supplied M5 reference. No new AWS/Aeglos, huge-circuit, phone/ARM/TEE, pressure/swap, minimum-RAM or concurrent-throughput result is added. The original all-eight verified 10× objective stays active. The next optimization must reduce dominant useful CPU work substantially; this small arithmetic change does not by itself establish the requested 2× jump.

[Design](artifacts/design.md), [protocol](artifacts/protocol.json), [artifact audit](artifacts/artifact-audit.json), [prospective review](artifacts/plan-review.json), [latency](artifacts/latency-analysis.json), [diagnostics](artifacts/diagnostic-analysis.json), [final review](artifacts/review.json), [publication](artifacts/publication.json), [evidence archive](artifacts/evidence.json.gz), [checksums](SHA256SUMS).
