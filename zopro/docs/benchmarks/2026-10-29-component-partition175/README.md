# Partition G2 windows by scalar component

29 October 2026. Private native source-175 experiment. It changes the ownership of two existing G2 window tasks from point ranges to endomorphism components, with original field arithmetic and two ordinary bucket sets per window. This is a complete-request local pilot, not an adopted implementation or a claim that the all-eight target is met.

Declared pilot gate: **false**. Failed checks: `["1x2-w32/ON/accepted","1x2-w32/ON/OFF","1x2-w16/ON/accepted","1x2-w16/ON/OFF"]`. Production remains unchanged.

Primary32 measures **8.060799 ms ON versus 8.022146 ms OFF and 8.023849 ms accepted** in this fresh comparison. Paired ON/OFF is **0.800% slower**, with bounds [1.000328321, 1.009224738] and 3/15 wins. Its accepted comparison spans parity: ratio 1.003313871, bounds [0.997535867, 1.008865868], 6/15 wins. Both physical16 gain checks also fail; their small positive estimates have wide intervals spanning parity. All fresh-control calibration and relative RSS guards pass. Only the four declared gain checks fail.

Explicit diagnostic instruction counts fall from 1.419136 to 1.385320 billion/request at 32 workers and 1.379453 to 1.345132 billion at 16 versus OFF. Ordinary latency-process CPU service also falls, while observed peak RSS rises slightly: 133.50→135.25 MiB at 32 and 90.25→90.75 MiB at 16. These work/resource observations do not rescue the failed latency result or qualify concurrent throughput. This exact component repartitioning is not adopted.

## Change and memory

The accepted eligible phi2 query splits each scalar into two signed components, then each 12-bit window runs two point-range tasks. A task feeds high, transforms its accumulated buckets, then feeds low. The candidate runs high and low as the two tasks, each borrowing the full original input. It transforms the high component's finished buckets, merges low, and uses the original weighted reduction and Horner recombination. Width, decomposition, grouping and eligible range 1,024 ≤ N < 65,536 are unchanged. The original narrow top reduction is retained per component. Query readiness, A/K/H, FFT, solver, Rayon and all field source remain unchanged.

Both versions hold two ordinary bucket sets per window. Full-range top tasks can retain more routing indices, so the checked workspace forecast adds two complete full-range top-parent envelopes and their leaf maximum, retaining the old forecast as a floor. Existing admission and fallback remain. There is no new key table, point-array copy, shifted-base cache, pool, digit array or persistent plan. Equal bucket-set counts do not prove equal RAM, collision work or load balance; process RSS below includes loading and primes and is not minimum RAM.

This differs from the earlier GLS4 parallel trial, which changed decomposition dimensions, and from transformed-X interleaving, which cached coordinates and fed one set. Those failed results remain closed. [History](artifacts/history-audit.md), [implementation](artifacts/partition.rs), [forecast and complete source](artifacts/source.json.gz).

## Verified SDK latency

Ryzen 9 9950X: 16 physical cores, 32 logical CPUs. Primary32 uses CPUs 0–31; primary16 uses 0–15, one logical CPU per physical core. Fifteen fixed rotated/reversed blocks compare the exact accepted executable, fresh OFF and ON. Each process measures 151 complete fresh randomized requests after three primes. Timers include witness solving, proving, internal verification and serialization; loading is excluded. Each variant passes all-nine 32-worker preflights and primary16 preflight with independent Gnark before latency.

Variant order rotates and reverses within each block. The frozen loop executes primary32 before primary16 in every block; the comparisons pair variants within each topology. No causal cross-topology estimate or shared-host exclusivity follows.

Values are medians of process medians. CPU is median summed user+system service per measured request; RSS is the maximum lifetime process high-water mark.

|Case|Variant|Latency ms|CPU ms/request|Peak RSS MiB|
|---|---|---:|---:|---:|
|1x2-w16|OFF|9.807530|118.881|90.25|
|1x2-w16|ON|9.709446|115.350|90.75|
|1x2-w16|accepted|9.718853|120.388|90.25|
|1x2-w32|OFF|8.022146|181.682|133.50|
|1x2-w32|ON|8.060799|176.783|135.25|
|1x2-w32|accepted|8.023849|182.238|134.25|

|Paired comparison|Median ratio|Bounds|Wins/15|Pass|
|---|---:|---|---:|---|
|1x2-w16/OFF/accepted|1.004451728|[0.985455710, 1.028024356]|6|true|
|1x2-w16/ON/OFF|0.994672747|[0.950221701, 1.034030907]|9|false|
|1x2-w16/ON/accepted|0.999032087|[0.980205938, 1.041793899]|8|false|
|1x2-w32/OFF/accepted|0.999240888|[0.990786896, 1.003753442]|10|true|
|1x2-w32/ON/OFF|1.007996369|[1.000328321, 1.009224738]|3|false|
|1x2-w32/ON/accepted|1.003313871|[0.997535867, 1.008865868]|6|false|

Both cases require candidate upper ratios below 1 and at least 10 wins against both controls. Each OFF/accepted interval must fit factor 1.03; candidate maximum RSS must be at most 1.05 times each control. Sorted ranks 4–12 of 15 provide nominal 96.484375% individual median coverage under exchangeable independent blocks, without familywise coverage. No gain floor, exclusions, threshold changes, successful reruns, extensions or subset adoption. Ratios pair within blocks and need not equal ratios of displayed medians. A passing pilot permits a separate all-nine performance/resource/portable qualification; it does not itself authorize adoption.

## Automatic counters and profiles

After all latency, six FIFO-gated grouped counter captures measure 31 requests after three primes, and six fixed-period user-cycle profiles measure 251 after three primes. Explicit counts, not sample-period sums, measure total work. Counter qualification requires at least 99.9% running coverage. Values below are single diagnostic observations per case/variant, not repeated-trial confidence bounds.

|Panel|Instructions/request, million|Cycles/request, million|All counters qualified|
|---|---:|---:|---|
|stat-1x2-w16-OFF|1379.453|576.420|true|
|stat-1x2-w16-ON|1345.132|566.044|true|
|stat-1x2-w16-accepted|1379.062|577.945|true|
|stat-1x2-w32-OFF|1419.136|826.104|true|
|stat-1x2-w32-ON|1385.320|807.924|true|
|stat-1x2-w32-accepted|1417.797|836.016|true|

Profiles use period 16 million with explicit CPU/time/period and SDK mapping checks, one PID and zero lost records. Attribution requires at least 1,000 samples and at most 5% unknown period. They retain the previously measured absolute-period undercoverage limitation: flat symbol shares are qualitative guides, not exclusive costs, per-symbol CPI or removable wall time. Generic symbol contexts can contain inlined work. No underlying RAM/clock/cache/scheduler cause is isolated.

|Profile|Samples|New partition context period|Attribution qualified|Lost|
|---|---:|---:|---|---:|
|cycles-1x2-w16-OFF|8869|0|true|0|
|cycles-1x2-w16-ON|8777|752000000|true|0|
|cycles-1x2-w16-accepted|8982|0|true|0|
|cycles-1x2-w32-OFF|12556|0|true|0|
|cycles-1x2-w32-ON|12173|1040000000|true|0|
|cycles-1x2-w32-accepted|12653|0|true|0|

## Validation and source identity

Fresh OFF and ON each pass all 128 unfiltered native core and eight prover tests. Four new tests cover complete G2 queries against independent known-generator sums at eight lengths through 65,535, full-width/adversarial scalars, one/four-worker pools; selected ordinary/top component buckets with identities, repeated and opposite points; compiled dispatch; and conservative full-range top-envelope charging. Unchanged 17 field files inherit the existing 33-field-test qualification, without a rerun. These correctness tests and all-nine preflights do not establish all-nine latency or memory-pressure behavior.

Private source has 176 files: 172 of 175 originals are exact, with three parent edits and one child. Checked reverse edits recover all 175 originals. Coherent libraries and proc macros freeze immediately after each build, with actual flags/features verified; the original SDK consumer links only frozen libraries. The initial plan was rejected before native compilation for two unsupported link-sdk fields; those were removed to match the inherited schema. Prebuild review also qualified a shadowed top helper. Initial files and correction records are preserved; no successful native build/test/capture was repeated.

All 294 capture/verification/decode steps complete. The trial contributes **15,708 completed internally verified SDK requests and 132 fresh 164-byte independently Gnark-verified exports**. Every proof is image/public-bound and distinct against 22,234 prior identities; ledger **22,366**, 264 proof/public members. All latency precedes diagnostics; all diagnostics precede remaining independent checks and decoding. All capture, analysis and publication helpers were compiled before capture; no owned compiler/hash/archive/helper or independent CPU workload overlaps it. The host is shared.

[Source audit](artifacts/source-audit.json), [final source review](artifacts/final-source-review.json), [build plan](artifacts/build-plan.json), [capture plan](artifacts/plan.json), [all observations](artifacts/rows.json), [analysis](artifacts/analysis.json), [verification inventory](artifacts/proof-inventory.json), [receipts](artifacts/receipts.json.gz), [proofs](artifacts/proofs.json.gz), [archive review](artifacts/publication-review.json). Raw perf and binary/library payloads remain local with hashes.

## Inherited CPU and Metal comparison

Accepted local source-175 results below are unchanged; M5 figures are supplied historical references, not new matched-machine measurements. The accepted physical16 calibration engineering exception and separate final-artifact qualification remain documented.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37.0|4.612×|24.0|2.992×|3.7|
|2x2|11.335969|59.0|5.205×|30.0|2.646×|5.9|
|4x2|15.945544|109.0|6.836×|48.0|3.010×|10.9|
|8x2|24.090485|189.0|7.845×|74.0|3.072×|18.9|
|12x2|33.844855|284.0|8.391×|101.0|2.984×|28.4|
|16x2|39.931557|335.0|8.389×|124.0|3.105×|33.5|
|24x2|66.233309|528.0|7.972×|176.0|2.657×|52.8|
|merge-24x1|65.273224|507.0|7.767×|175.0|2.681×|50.7|

Accepted primary remains 8.022165 ms / 134 MiB maximum RSS; 54→1 remains 162.250464 ms without a supplied M5 reference. Latest AWS c7a.24xlarge remains Zopro 11.915591 ms at 32 workers versus Aeglos Sorted 7.959184 ms at 96. No new AWS/Aeglos, phone/ARM/TEE, giant-circuit, pressure/swap/minimum-RAM, concurrent-proof or throughput qualification occurs. The original all-eight CPU10× goal remains active and unmet, **0/8**.
