# Compact ternary recoding: complete-query trial

**Packing digits once does not qualify the ternary path at 32 workers. The candidate fails 13/28 declared checks and is not adopted.** Primary G2 query time is 3.238363 ms versus 3.055219 ms for its original-algorithm control; the paired slowdown is 5.64% [4.77%,6.86%], with 0/31 wins. The accepted local 1×2 full-proof result remains **7.783594 ms at 32 workers** and **9.016864 ms at 16 workers**.

This follows the [reciprocal ternary trial](../2026-11-07-ternary-reciprocal-caller176/README.md), which removed repeated wide division without qualifying threaded latency. The new implementation instead recodes each Phi component once, then extracts digits with loads, shifts and sign extension during bucket feeds. Ternary window geometry, point operations, grouping, queues, buckets, reduction and worker layout remain fixed. This tests a distinct explanation of the prior slowdown; it does not reopen the unchanged prior recipe.

## Encoding and qualification

Each signed-magnitude component is strictly below 2^126. Its ten ordinary balanced radix-6561 digits occupy 13 bits each; its leading digit is −1, 0 or 1 and occupies two bits. Store the lower 128 of those 132 bits in the existing component words and the upper four bits in one nibble. Two components therefore reuse the original 32 bytes and add **one byte per row**, a 3.125% scalar-payload increase. There is no digit matrix, point table, retained solver plan or expanded key.

The diagnostic reserves the whole sidecar before Phi or scalar mutation and returns to the original raw path on reservation failure. Recoding then runs alongside Phi in the existing pool. Ordinary digits, including the split ninth slot, and signed leading carries decode exactly. The initial fixed-divisor version emitted wide-division helpers in preparation; before any timing, a distinct version reused the already-qualified corrected reciprocal formula. Both source versions and their evidence remain.

Each version passes checked and optimized qualification: **134,316 row pairs and 2,954,952 component-digit comparisons per build**, against independent balanced per-trit conversion, checked signed Horner reconstruction and the prior reciprocal codec. Populations include all 6,561 ordinary digit values in every slot, 1,585 boundary values, 65,536 random pairs, signed carries, four negative-zero cases and 5,632 cross-component isolation checks. Six storage cases per build preserve the word pointer/capacity and make zero allocation/reallocation/deallocation calls during recoding after sidecar reservation. The reciprocal probes contain no wide-division helpers.

Two native tests pass: twenty complete generator-based queries across 1/4/16/32 workers and grouping/exception cases, plus four signed carry-boundary Windows. The new snapshot has 179 files: 177 match the prior reciprocal snapshot, one existing child changes and one packed-codec child is added. The complete-query driver is byte-identical. **This is the older diagnostic snapshot, not a new current-production SDK build.** Existing diagnostic size/feature guards remain; production resource admission and portable execution are not newly qualified.

## Complete-query results

Ryzen 9950X, 16 physical/32 logical CPUs, shared host. OLD_BASE/RECIPROCAL use the qualified prior reciprocal executable with B/T selectors. FRESH_BASE/PACKED use the new executable with B/T selectors. Solo pins to CPU2; other cases pin workers to CPUs0–15 or0–31, with 512 KiB stacks. Both real fixtures and their independent full Arkworks MSM oracles are reused.

Scalar reset, grouping, allocation, Phi, sidecar reservation, recoding, all MSM work, pool install and normalization are timed. Input loading/hashing, pool creation and output-oracle checks are excluded. Every prime, observed output and postcheck is checked; point inputs remain unchanged. This measures a complete G2 MSM query, not a proof or concurrent server throughput.

The frozen protocol retains 31 rotated/reversed blocks, all six contexts and four arms. Each process has three primes, seventeen observed calls and one postcheck. Times below are medians of process means; paired ratios need not equal ratios of those displayed times. Bounds use sorted ranks10/16/22 of31, nominal97.0551% marginal coverage conditional on exchangeability, without familywise or stationarity guarantees. Preflight clocks select nothing. There are no exclusions, extensions, rerolls, subset promotions or changed thresholds.

|Shape / workers|OLD_BASE ms|FRESH_BASE ms|RECIPROCAL ms|PACKED ms|PACKED / FRESH_BASE [bounds], wins|PACKED / RECIPROCAL [bounds], wins|
|---|---:|---:|---:|---:|---|---|
|1×2 /1|37.115231|37.083778|33.891911|33.329210|0.899027 [0.898445,0.899521],31/31|0.982278 [0.981384,0.984546],31/31|
|1×2 /16|3.492958|3.476615|3.783189|3.724387|1.081787 [1.065795,1.090330],0/31|0.984994 [0.976106,0.998022],22/31|
|1×2 /32|3.086754|3.055219|3.269849|3.238363|1.056401 [1.047669,1.068623],0/31|0.994520 [0.979828,1.003444],18/31|
|2×2 /1|39.403865|39.392495|36.325989|35.992531|0.913613 [0.912644,0.914097],31/31|0.991244 [0.990521,0.992031],31/31|
|2×2 /16|3.959026|3.948167|4.247318|4.270205|1.082624 [1.068198,1.094818],0/31|1.002874 [1.000929,1.008710],9/31|
|2×2 /32|3.547966|3.512247|3.760248|3.747793|1.065231 [1.050787,1.080971],0/31|0.997362 [0.988376,1.009554],18/31|

All six original-algorithm calibrations pass. The candidate fails eight threaded nonregressions, four primary/aggregate gain checks and one RSS guard. Equal-log two-population32 PACKED/FRESH_BASE is **1.068593 [1.054656,1.079069],0/31**; against OLD_BASE it is1.055087 [1.045472,1.065813],0/31. Solo improvements remain visible but do not qualify a threaded candidate. Neither 32-worker population establishes an incremental gain over RECIPROCAL.

Peak process RSS at primary32 is35.00 MiB PACKED,35.00 MiB RECIPROCAL and34.75 MiB FRESH_BASE. At2×2/32 it reaches **65.25 MiB**, versus56.75 MiB and53.50 MiB respectively, failing the predeclared comparative guard against RECIPROCAL. The extra scalar sidecar alone does not explain that8.50 MiB process difference; the recording does not establish its cause. These process high-water observations are neither requested-live workspace accounting nor minimum RAM, pressure/swap or phone results.

## Automatic profiling and interpretation

After latency, all four fixed grouped-counter panels and all four cycles profiles pass coverage, FIFO gate, event, PID/TID/CPU/executable-MMAP, sample and loss checks. Counters run100%; profiles contain **19,118 samples**, less than1% unknown per arm and zeroLOST.

|Primary32 diagnostic ratio|PACKED / FRESH_BASE|PACKED / RECIPROCAL|
|---|---:|---:|
|User cycles|0.917738|0.991129|
|Instructions|0.918499|0.994850|
|Branch misses|0.827858|1.004054|
|Generic cache misses|0.944504|1.065513|

The new preparation routine receives209 samples; ternary feed samples fall from559 to377. These one-shot populations show work moving into preparation, not an exclusive cost or critical-path attribution. The smaller aggregate CPU work again fails to produce lower threaded wall time. This specific compact recoding does not make the fixed ternary geometry competitive, and it does not justify another unchanged codec rerun or a large lookup table. The profiles retain scheduler/epoch and reduction work as inspection leads; they do not establish avoidable overhead or its cause.

## Evidence and accepted proof status

All776 new caller processes complete **17,584 oracle-checked queries**. Across qualification, tool preparation, capture and offline analysis, **823/825 workflow commands succeed**. A pre-capture identity-filename collision and an offline reader's environment-receipt assumption each cause one retained failure. Four successful reader builds and both successful producers are reused; only the failed preparation/reader work is corrected. No successful correctness test, oracle, timing process, profile or decoder is replayed. The separate raw reader reconstructs all28 decisions and checks4,154 pin occurrences; the production176 files and accepted SDK remain exact.

No new proof is exported; canonical inventory remains44,265. No current SDK, Gnark, AWS/Aeglos, ARM/mobile, huge250/4999, pressure/swap or minimum-RAM measurement is added. The following full-proof CPU observations are inherited from the [latest SDK campaign](../2026-11-10-frontend-overlap176/README.md); M5 references were supplied separately on different hardware.

|Shape|Accepted local CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.783594|37 /4.754×|24 /3.083×|3.7|
|2×2|10.932731|59 /5.397×|30 /2.744×|5.9|
|4×2|15.757088|109 /6.918×|48 /3.046×|10.9|
|8×2|23.973295|189 /7.884×|74 /3.087×|18.9|
|12×2|33.827269|284 /8.396×|101 /2.986×|28.4|
|16×2|39.900917|335 /8.396×|124 /3.108×|33.5|
|24×2|66.041083|528 /7.995×|176 /2.665×|52.8|
|merge24×1|65.717994|507 /7.715×|175 /2.663×|50.7|

Merge54 remains162.170158ms without a supplied M5 reference. The verified CPU10× goal remains **active and unmet,0/8**. Next work should inspect actual complete-query task spans and reduction/scheduler boundaries against prior experiments, then select a distinct main-work or dependency reduction. This closed recipe is not promoted to SDK.

[Latency](artifacts/latency-analysis.json), [diagnostics](artifacts/diagnostic-analysis.json), [raw review](artifacts/raw-review.json), [decision](artifacts/decision.json), [encoding](artifacts/packed-reciprocal.rs), [native qualification](artifacts/qualification-native.json), [integer qualification](artifacts/qualification-reciprocal-optimized.json), [source audit](artifacts/source-audit.json), [protocol](artifacts/capture-design.md), [identities](artifacts/identities.json), [commands](artifacts/receipts.json), [failures](artifacts/failures.md), [continuation](artifacts/next.md), [public bindings](artifacts/public-views.json), [evidence archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). The indexed UTF-8 archive retains raw rows, full commands, failed stderr, reader versions and changed source. Private fixtures, binaries, libraries, raw perf payloads, unchanged snapshots and proof ledgers are excluded; reproduction needs the pinned local artifacts.
