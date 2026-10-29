# Interleaved ordinary-basis complex G2 caller

The separately declared actual-caller gate **fails**. Production remains unchanged. The candidate evaluates both schoolbook complex outputs in one unsigned row-CIOS schedule and streams a complemented second operand one digit at a time. This tests complete G2 batches, including inversion and packing, rather than selecting an isolated multiplication gain.

The all-18 aggregate is **0.303% slower versus OFF**, with bounds [1.002488278, 1.006735725] and 0/15 wins. The threaded aggregate is 0.427% slower. Borrowed 200/352-pair batches regress about 1.13% at 32 workers. All calibration, nonregression, output, allocation and capacity checks pass; only the four aggregate gain checks fail.

## Arithmetic and scope

The ordinary basis i²=−1 is unchanged. With n=2p−b1, ordinary products use X=a0·b0+a1·n and Y=a0·b1+a1·b0. Conjugate products use X=a0·b0+a1·b1 and Y=a1·b0+a0·n. Each row computes one normalized n digit using a signed borrow, then advances both unsigned CIOS accumulators. Both outputs are normalized below 2p. The method retains four raw products and two reductions, 300 IFMA sites per complex product. It removes a separately materialized negative operand at source level; emitted register use and timing are measured separately.

This differs from the earlier dual independent products, three-product streamed Gauss and shared quotient preparation. Their failed SDK/caller gates remain. The [trace-one basis trial](../2026-10-29-golden-basis175/README.md) remains rejected: its multiplication gain did not compensate for square and norm costs. This trial keeps those operations unchanged.

The native private cfg changes Fq2 lane multiplication and scaled-numerator dispatch only. No key table, point-array copy, heap scratch, basis conversion, preparation, worker policy or persistent cache is added. All scratch/reference capacities must match the controls; this is not a process-RSS, minimum-RAM or pressure measurement. The snapshot has 177 files, with 171 of 175 originals exact. Removing the field include, ON-only SIMD forwarding bridge and core adapter suffixes and reversing two dispatch edits recovers all 175 originals; two new children contain the field kernel/tests and caller adapter.

## Method

Ryzen 9 9950X, 16 physical cores and 32 logical CPUs. The fixed 18 contexts cover owned unsigned and borrowed signed G2 prefix batches at 32/200/352 pairs, on CPU 2, 16 physical CPUs 0–15 and 32 logical CPUs 0–31. Fifteen fixed rotated/reversed blocks compare frozen accepted libraries, fresh OFF and ON in the same fat-LTO harness. All frozen source/flags/proc-macro/library inputs are pinned and verified. Each worker runs three checked/reset 1,024-call primes and 8,192 measured complete calls. Whole-wave latency includes all workers completing inversion, packing, arithmetic and output scatter. Allocation, fixtures and independent generator oracles are outside the clock. These synthetic nonexceptional generator batches do not reproduce witness distributions or concurrent proofs.

All 90 preflights include 199/351-pair tails. All 810 latency processes agree with independent generator scalar-sum oracles and cross-variant canonical digests, reference bytes and scratch capacities. Timed allocations are zero. These timings are microseconds per complete wave, not proof latency.

|Caller|Pairs|Workers|Frozen µs|OFF µs|ON µs|Paired change/OFF|Bounds|Wins / 15|
|---|---:|---:|---:|---:|---:|---:|---|---:|
|owned|32|1|1.883814|1.883097|1.886161|+0.230%|[0.996240260, 1.007856075]|7|
|owned|32|16|2.127679|2.128482|2.123315|-0.046%|[0.992153426, 1.008416576]|9|
|owned|32|32|3.179847|3.171627|3.179473|+0.117%|[0.993531381, 1.009225446]|6|
|owned|200|1|7.812943|7.838096|7.836335|+0.048%|[0.989967385, 1.011738442]|7|
|owned|200|16|9.111269|9.069169|9.086059|+0.053%|[0.995138682, 1.010005820]|7|
|owned|200|32|14.684979|14.671269|14.753317|+0.605%|[0.996877467, 1.012530393]|6|
|owned|352|1|13.185453|13.206213|13.225522|+0.185%|[0.993548368, 1.013004100]|4|
|owned|352|16|15.524642|15.577776|15.563322|-0.024%|[0.994872949, 1.002689069]|8|
|owned|352|32|25.344329|25.378359|25.518659|+0.269%|[0.997487487, 1.010723409]|5|
|borrowed signed|32|1|1.951912|1.958375|1.964863|+0.373%|[0.999230352, 1.010691159]|5|
|borrowed signed|32|16|2.170683|2.177344|2.189830|+0.296%|[0.990560025, 1.007419672]|5|
|borrowed signed|32|32|3.311392|3.283327|3.315311|+1.036%|[1.007441119, 1.011743369]|2|
|borrowed signed|200|1|8.472337|8.461170|8.496621|+0.474%|[0.994906131, 1.011927965]|4|
|borrowed signed|200|16|9.840042|9.749447|9.870765|+0.337%|[0.996687656, 1.015931067]|4|
|borrowed signed|200|32|15.933043|15.909237|16.098605|+1.132%|[1.007238439, 1.013780123]|0|
|borrowed signed|352|1|14.244144|14.234730|14.394202|+0.898%|[0.997641073, 1.018864083]|4|
|borrowed signed|352|16|16.746221|16.801223|16.855525|+0.527%|[0.991054771, 1.009926072]|5|
|borrowed signed|352|32|27.449278|27.407729|27.727132|+1.131%|[1.008655882, 1.017395602]|2|

|Aggregate|Ratio|Bounds|Wins / 15|
|---|---:|---|---:|
|all18/on/frozen|1.005192529|[1.001665512, 1.007275040]|1|
|all18/on/off|1.003027180|[1.002488278, 1.006735725]|0|
|threaded12/on/frozen|1.005012217|[1.001018431, 1.008741964]|3|
|threaded12/on/off|1.004271282|[1.001449142, 1.006445899]|1|

Every OFF/frozen interval must lie inside [1/1.03, 1.03]; every ON/control upper ratio must be ≤1.03. All-18 and threaded-12 equal-log aggregates require upper <1 and ≥10/15 wins against both controls. Ranks 4–12 of 15 provide nominal 96.484375% individual median coverage under independent/exchangeable blocks, without a familywise guarantee. No gain floor, subset exclusion, extension, threshold change, layout fishing or unchanged rerun is applied.

Failed checks:

- `all18/on/frozen/gain`
- `all18/on/off/gain`
- `threaded12/on/frozen/gain`
- `threaded12/on/off/gain`

## Correctness and automatic profiling

The exact BigInt model passes 60,248 complex cases across Fq/Fr, R256/R260, normal/conjugate forms, inclusive 2p, limb boundaries, full-width random operands and dependent outputs. It checks streamed borrow digits, cancellation quotients, mixed packing and bounds. An initial model compilation required an explicit usize loop index; failure logs are retained. No successful model, test or capture is repeated.

The complete candidate field suite passes 35 tests, including 33 existing and two new independent Arkworks tests. Each new test checks 345,088 field outputs across both radices and forms, for 690,176 new comparisons in total; counts follow the retained bounds in field-tests.rs. Complete unfiltered OFF/ON core suites each pass 128 tests and prover suites each pass 8. The initial core ON compilation found a missing FqBatch forwarding method. An ON-only wrapper fixes it without changing field arithmetic or compiled OFF tokens. The 35 successful field tests and all six completed initial OFF/field/build steps are retained, with seven remaining workflow steps separately recorded. Existing field qualification for exact OFF arithmetic is inherited. No SDK proof or Gnark export is produced by this caller screen.

All latency precedes six separate FIFO-gated 199 Hz cycles:u profiles: borrowed 200-pair batches at 16/32 workers and owned 200-pair batches at 32 workers, OFF/ON, with 262,144 calls per worker after primes. CPU/time/MMAP/task metadata are explicit. All captures precede ownership transfer, decoding and event/loss audits; all helpers were compiled before capture and no owned compiler/helper/hash/archive or second CPU workload overlaps it.

|Profile|Samples|Lost records|
|---|---:|---:|
|profile-g2-borrowed-signed-200-logical32-off|26130|0|
|profile-g2-borrowed-signed-200-logical32-on|26306|0|
|profile-g2-borrowed-signed-200-physical16-off|8120|0|
|profile-g2-borrowed-signed-200-physical16-on|8143|0|
|profile-g2-prefix-200-logical32-off|24212|0|
|profile-g2-prefix-200-logical32-on|24276|0|

Every profile has one sampled process and zero lost records. All 930 capture/offline steps complete, covering 196,020,160 kernel calls including primes and profiles. Flat samples and static instruction counts do not establish exclusive wall-time, spill, cache or clock causes. Source and receipt archives round-trip byte-exactly; raw perf/ELF files remain local with hashes.

Read-only static inspection of the selected borrowed G2 body finds 1,450 IFMA sites in every variant. Frozen/OFF contain 8,671 instructions and 1,552 textual rsp operand lines; ON contains 8,685 and 1,563. The function shrinks from 51,942 to 51,935 bytes. These whole-body static counts do not measure dynamically retired instructions, spills or memory stalls. A smaller body and a shorter source-level temporary do not establish a runtime benefit.

## Decision and prover comparisons

This exact caller does not advance under its declared gate. Positive contexts remain in the record; no favorable subset is promoted as passing. There is no SDK integration or production adoption.

Accepted local 1×2 remains 8.022165 ms and 134 MiB observed peak RSS. Latest AWS c7a.24xlarge remains Zopro 11.915591 ms at 32 workers versus pinned Aeglos Sorted 7.959184 ms at 96. These are inherited observations, not this candidate. Supplied M5 CPU/Metal values are historical references:

|Shape|Accepted local ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

Accepted 54→1 is 162.250464 ms without a supplied M5 reference. The all-eight CPU 10× objective remains active and unmet, 0/8. The distinct-proof ledger remains 22,144. The accepted physical-16 calibration engineering exception and separate final-artifact gate remain documented. No new AWS, mobile/ARM/TEE, forester 250/4999, pressure/swap, minimum-RAM or concurrent-throughput result follows. [All nine current local/AWS comparisons](../2026-10-29-current-cpu-summary/README.md).

[Protocol](artifacts/design.md), [all observations/checks](artifacts/analysis.json), [model](artifacts/model.json), [kernel](artifacts/kernel.rs), [source audit](artifacts/source-audit.json), [source](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [archive review](artifacts/publication-review.json).
