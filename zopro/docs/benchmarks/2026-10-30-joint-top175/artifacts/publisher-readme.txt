# Joint narrow-top G2 reduction: complete-query qualification

Declared query gate: **false**. Diagnostic qualification: **true**. Full-SDK eligibility: **false**. Production remains unchanged; this is a query measurement, not proof latency or adoption.

Failed checks: `["2x2-w32/ON/OFF","2x2-w32/ON/FROZEN","two_shapes/ON/OFF/gain","two_shapes/ON/FROZEN/gain"]`. Every result and both controls remain in [analysis](artifacts/analysis.json). No exclusions, extensions, changed thresholds or successful capture repeats.

## Change

The candidate shares the two existing signed six-bit top digits through compact joint (a,b) classes within each point-range half. It reduces each class once, then reduces both signed component marginals through existing 256-class affine groups. The lower ten windows, Phi2/Booth decomposition, width 12, two parts, scalar grouping, signs, phi map, final weighting, field arithmetic, key and solver remain unchanged. This follows the [passed revised work screen](../2026-10-30-joint-top-count175/README.md); its original one-stage-coalesced failed gate stays false. No paper speedup or operation-count saving is assumed to transfer to timing.

A private native-only child and a cfg hook enter the existing narrow-top branch. Unsupported geometry and denied optional preparation execute the original top. New routing and derived point sums are proof-local; original key points are borrowed. Coalescing creates larger marginal leaves and changes task/inversion-batch geometry as well as removing additions.

## Correctness and memory

Fresh OFF and ON each pass all 127 core and eight prover tests, unfiltered. Three new tests check known-generator complete top sums and input preservation across empty/odd/tail/group boundaries under 1/4/16/32-thread pools, signed ±64 sparse/dense classes and repeated/opposite/identity points, plus checked cost overflow and injected admission denial. The unchanged 17 field files inherit 33 field tests; they are not rebuilt for a new field formula.

The private source contains 176 files: 173 originals exact, two reversible parents and one new child. Reversing the API-only export and top hook/module recovers all 175 accepted bytes. The original API-only frozen source also recovers 175; the rejected queue-scan candidate is not included. Native dependencies/lockfile/features/flags and all nine frozen library paths are checked. SDK consumers link but do not run proofs in this experiment.

Before routing allocates, a checked optional Reservation::analysis charge covers the maximum of histogram/compact routing, retained first-stage arrays plus all disjoint group scratch, and retained shared sums/high/sign outputs plus the larger coalesced marginal leaf. Existing typed vector and workspace_grouping_linear bounds include capacity/allocator margins; field/key representation is unchanged. A denied reservation returns before construction and retains original inputs. The charge lasts through every new auxiliary consumer; original-shaped returned buckets remain covered by the old query workspace. This is a conservative new optional charge in addition to the retained original schedule forecast. There is no arbitrary global RAM cap or per-key table.

Observed RSS below includes fixture load/primes and allocator behavior. It is not minimum RAM, whole-SDK proof memory, compiler-stack bounds or pressure/concurrency qualification. Native allocation-failure recovery, portable/ARM/phone/TEE, huge-key and swap behavior are not established by these tests.

## Complete query measurements

Ryzen 9950X, 16 physical / 32 logical CPUs, shared host. Solo pins CPU 2, physical 16 pins 0–15, logical32 pins 0–31. Exact prior frozen API-only driver, fresh OFF and ON use byte-identical complete-query/perf-helper source. Thirty-one fixed rotated/reversed blocks measure 17 queries after three checked primes per process. Reset copy, grouping, decomposition, fills, merges, reductions, allocation/drop and normalization are inside; parsing/solving are outside. All queries match the inherited independent complete-Arkworks fixture hashes, all original points remain unchanged and worker affinities are checked. These are two real saved query populations, not concurrent proof throughput.

|Case|FROZEN ms|OFF ms|ON ms|ON/OFF ratio [bounds]|Wins/31|ON/FROZEN ratio [bounds]|Wins/31|FROZEN/OFF/ON max RSS MiB|
|---|---:|---:|---:|---|---:|---|---:|---|
|1x2-w1|39.507454|40.000173|34.883139|0.871435924 [0.870421359, 0.872438686]|31|0.882744128 [0.882315089, 0.883703744]|31|15.25/15.25/15.25|
|1x2-w16|3.706567|3.746934|3.726696|0.996367847 [0.991518328, 1.003389110]|17|1.008172309 [1.000178816, 1.015345185]|9|27.00/29.00/58.50|
|1x2-w32|3.183734|3.190687|3.155190|0.988675913 [0.978765418, 0.998652296]|24|0.992113672 [0.988428497, 0.999033607]|22|33.00/40.00/85.00|
|2x2-w32|3.667293|3.659139|3.712890|1.014901876 [1.007851410, 1.022645559]|5|1.015976389 [1.009296438, 1.024738542]|8|60.75/74.75/119.25|

Primary32 and the equal-log two-shape32 aggregate require upper ratios below 1 and at least 21/31 wins versus both controls. All other context upper ratios must be ≤1.02; all fresh-OFF/frozen calibration intervals must fit factor 1.03. Sorted ranks 10–22/31 give nominal 97.055% individual median coverage conditional on exchangeable blocks, without a familywise guarantee. There is no gain floor or subset promotion. A pass permits a separate complete SDK/resource/portable/final-artifact experiment, not adoption.

|Aggregate versus|Ratio [bounds]|Wins/31|Pass|
|---|---|---:|---|
|OFF|1.002742545 [0.997336395, 1.010632717]|12|false|
|FROZEN|1.004227627 [1.000174799, 1.011181869]|9|false|

Calibration intervals and every process sample/CPU/RSS observation remain in analysis/rows. Displayed marginal medians need not give the paired ratios. A confidence-bound failure does not establish that its entire upper bound is a slowdown.

## Automatic counters and profiles

All latency captures precede six FIFO-gated four-event counter panels and six fixed-period cycles:u profiles, OFF/ON at primary16/32 and 2x2/32. Counters measure 31 queries; profiles 251; three primes are excluded by acknowledged FIFO. Every capture finishes before ownership changes, decoding, loss/event/mapping checks and analysis. No independent owned compiler/helper/hash/archive/analyzer workload overlaps capture. All helpers and publication code are compiled/pinned beforehand.

|Counter panel|Instructions/query M|Cycles/query M|Qualified|
|---|---:|---:|---|
|stat-1x2-w16-OFF|581.968|237.573|true|
|stat-1x2-w16-ON|521.529|208.511|true|
|stat-1x2-w32-OFF|601.838|339.294|true|
|stat-1x2-w32-ON|539.323|294.068|true|
|stat-2x2-w32-OFF|629.824|365.499|true|
|stat-2x2-w32-ON|570.354|328.338|true|

All four events in each panel require ≥99.9% running coverage. Each is one diagnostic panel, not a latency confidence interval or causal decomposition. Separate profiles require ≥1000 samples,≤5% unknown period, one actual query PID, recorded worker-TID/CPU/mapping/time/period checks and zero lost records. The inherited absolute-period coverage deficit is not resolved; symbol shares are qualitative, not local CPI, exclusive costs or removable proof time.

|Profile|Samples|Unknown period fraction|Attribution qualified|
|---|---:|---:|---|
|cycles-1x2-w16-OFF|3724|0.007787|true|
|cycles-1x2-w16-ON|3254|0.007376|true|
|cycles-1x2-w32-OFF|5243|0.008964|true|
|cycles-1x2-w32-ON|4622|0.005625|true|
|cycles-2x2-w32-OFF|5680|0.009859|true|
|cycles-2x2-w32-ON|5170|0.007930|true|

## Accepted CPU and Metal comparisons

All original eight CPU 10× targets remain unmet (0/8). These inherited local 32-worker full-proof results are compared with supplied M5 CPU/Metal references, not new Apple measurements or candidate proof timings. The earlier accepted physical 16 calibration exception remains documented and does not transfer to this gate.

|Shape|Accepted ms|Peak RSS MiB|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|
|1x2|8.022165|134.00|37 / 4.61×|24 / 2.99×|3.7|
|2x2|11.335969|275.00|59 / 5.20×|30 / 2.65×|5.9|
|4x2|15.945544|315.00|109 / 6.84×|48 / 3.01×|10.9|
|8x2|24.090485|512.50|189 / 7.85×|74 / 3.07×|18.9|
|12x2|33.844855|707.25|284 / 8.39×|101 / 2.98×|28.4|
|16x2|39.931557|826.25|335 / 8.39×|124 / 3.11×|33.5|
|24x2|66.233309|1328.75|528 / 7.97×|176 / 2.66×|52.8|
|merge-24x1|65.273224|1175.50|507 / 7.77×|175 / 2.68×|50.7|
|merge-54x1|162.250464|1831.87|—|—|—|

Latest retained C7a primary: Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96, best tested configurations in that campaign. All nine local/AWS rows, worker counts and RSS remain in [accepted rows](artifacts/accepted-rows.json) and the [current snapshot](../2026-10-30-current-bench-snapshot175/README.md). No new AWS/Aeglos, full proof, independent proof export, fixture solve, minimum-RAM, forester 250/4999, phone or concurrency result follows. Distinct verified-export ledger remains 25,231.

All 426 capture/workflow steps complete; 396 query processes execute 9216 complete queries including primes, with zero new SDK requests/Gnark exports. Source, native qualification, identities, fixture/oracle, helpers and library pins close; every source/receipt/helper archive round-trips byte-exactly. Original raw profiles and binary/library payloads stay local with hashes; raw fixture/query values are not published. No successful native test, old oracle, capture or verification is repeated.

[Declared design](artifacts/helpers.json.gz), [source recovery](artifacts/source-audit.json), [all analysis](artifacts/analysis.json), [every observation](artifacts/rows.json), [profile symbol periods](artifacts/profile-symbols.json), [complete private source](artifacts/source.json.gz), [exact receipts](artifacts/receipts.json.gz), [helper/protocol archive](artifacts/helpers.json.gz), [archive review](artifacts/archive-review.json), [checksums](SHA256SUMS).

Candidate-only linked symbols and runtime joint-top activation are separately checked: `{"joint_module_only_in_candidate_linked_symbols":true,"joint_top_observed_in_all_three_ON_profiles":true}`. Positive samples are required in each of the three ON profiles for SDK eligibility; symbol shares do not isolate a removable wall-time fraction. Both query links completed in a separate two-step, 667-pin workflow before capture.


The initial offline controller compilation omitted its explicit demangler dependency and retained an unused legacy summary command. Both errors were repaired before the controller ran or capture began. Original controller source and failure description remain in the helper archive. Native source, protocol and successful test/capture receipts were unchanged.
