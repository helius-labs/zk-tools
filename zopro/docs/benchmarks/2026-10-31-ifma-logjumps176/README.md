# Integer IFMA LogJumps in full SDK proofs

The candidate fails the declared full-SDK screen and is **not adopted**. At 32 workers, paired ON/OFF estimates are 1.61% slower for 1x2 and 1.30% slower for 2x2. Primary's interval is entirely above parity with 0/15 winning blocks. Both 32-worker OFF/accepted controls calibrate. The separate 16-worker calibration failures do not account for the absence of the required 32-worker gains.

Positive observations remain recorded. At 16 workers, 2x2 improves 3.13% against the accepted build, with its interval below parity and 12/15 wins. Against fresh OFF it improves an estimated 1.79%, with bounds spanning parity and 10/15 wins. CPU service decreases about 2.60% for primary and 2.01% for 2x2 against OFF at 16 workers. These observations do not establish a gain against both controls or justify adopting the measured candidate.

## Arithmetic and memory

[Domb's LogJumps note](https://hackmd.io/@Ingonyama/Barret-Montgomery) replaces serial digit cancellations with products by fixed inverse-radix constants. [Koh's explanation](https://kohweijie.com/articles/25/logjumps.html) gives an independent presentation. The [2026/1324 abstract](https://eprint.iacr.org/2026/1324) concerns FPGA arithmetic over BLS12-377; its performance is not a BN254 CPU prediction. Both PDF download routes returned HTTP 403; only that paper's abstract was read. The earlier local floating-point RNE51 implementation remains rejected.

This implementation uses the current integer IFMA52 layout and mixed Montgomery radix R=2^256. It forms the full product convolution, performs parallel inverse-radix folds of 104 and 100 bits, and finishes with one 52-bit Montgomery cancellation. The first fold uses rho104/rho52; the second uses rho100/rho48. Here rho(k)=2^-k mod p. Constants are derived by compile-time modular halving, with 128 logical bytes per prime. Code generation may duplicate broadcasts. They are arithmetic constants, with no per-key lookup table or preparation cache.

Only native eight-lane mixed-radix products select the new method under a private compile-time flag. Four-lane and uniform-R260 dispatch remain original. There is no per-product environment or atomic lookup. Point formulas, scalar tails, coordinate layouts, keys, solver, scheduling, pools, allocation policy and image format retain their original source. No new heap buffer, copied key/plan, retained table or fixed RAM cap is introduced. Local vectors and compiler frames can still change stack and process resource use.

Accepted CIOS already elides cancelled low products, so the new method does not reduce its IFMA site count. The changed trade is dependency depth versus coefficient handling, register pressure and range correction. In particular, ordinary products now need the final conditional 2p correction too. Fewer serial cancellations do not establish fewer instructions or faster proofs.

The exact BigUint model proves conservative single/fused raw-output bounds of 2.078p/2.834p for Fq and 2.522p/3.278p for Fr. All are below 4p, so one conditional subtraction of 2p restores the existing lazy range. It checks 16,384 single/fused cases against C*(2^256)^-1 mod p, using every 7^4 corner tuple plus 1,695 deterministic random tuples for each prime. Native tests independently check the actual instructions against Arkworks, inverse constants, lazy boundaries, the inclusive 2p complemented operand, multiplication chains and normalized output limbs.

For the unsigned coefficient accumulators, raw fused convolution coefficients are below 18B, B=2^52. Adding two five-limb folds leaves a conservative bound below 32B. The 100-bit shift splits at 48 bits before shifting by four, rather than shifting an unbounded coefficient. The final cancellation and normalization remain below this bound. Thus these operations fit u64; the scalar integer model alone is not an instruction-level overflow oracle.

## Qualification and capture

Each OFF/ON source snapshot is the same 176-file private tree: 173 originals exact and three modified files (four-/eight-lane field source plus the generator). Removing the prototype recovers all 176 originals. Generator consistency passes. Each compile-time build passes 39 unfiltered field, 125 core and eight prover tests on its first qualification execution. Production remains unchanged. No portable or low-memory-device qualification is claimed.

Ryzen 9950X, 16 physical/32 logical CPUs, full-process masks 0–15/0–31. The original accepted SDK is a separate control; fresh OFF/ON have coherent frozen libraries and final native/O3/fat-LTO/one-codegen-unit links. Fifteen fixed rotated/reversed blocks measure four contexts, 101 SDK requests per process after three primes. Twelve preflights verify proof/public bindings before latency. Six PMU and six cycle captures, 251 requests each, follow all latency.

All 439 capture commands finish once. There are 21,816 SDK requests and 204 freshly exported last proofs, each independently Gnark-verified with public-witness, image and VK bindings. The distinct exported-proof ledger advances 29,051→29,255. Export count is not the count of all generated proofs. No owned build, analyzer, helper or other CPU workload overlaps capture; exclusive host use is not established. No block, arm or context is omitted, extended or rerolled.

The separate Rust checker passes on its first execution, checking 634 pin entries, 453 successful workflow commands, native suite counts, frozen compile-time flags, raw samples, paired gates, all proof bindings, ledger union and profile-sample conservation. It does not import the campaign's analysis. Two preliminary model compiles failed on BigUint shift-type inference before the first model run; their source versions are retained. Postcapture codegen preparation first collided with its executable pathname, then encountered eight original specializations instead of the guessed nine. The repaired reader retains all actual matching functions and reuses the eight already extracted bodies. These helper repairs do not change qualification, capture, gates or proof results.

Times below are medians of process request medians. Paired ratios need not equal ratios of displayed medians. Ratios are candidate/control; lower is faster.

|Context|Accepted / OFF / ON ms|ON/OFF [bounds], wins|ON/accepted [bounds], wins|OFF/accepted calibration|
|---|---:|---|---|---|
|1x2-w16|9.829810 / 9.445784 / 9.735057|0.995701 [0.960510, 1.241069], 8/15|1.015554 [0.914037, 1.258001], 7/15|1.002240 [0.924705, 1.031599], 7/15|
|1x2-w32|7.898365 / 7.923403 / 8.018442|1.016140 [1.006903, 1.017542], 0/15|1.015203 [1.003289, 1.027394], 1/15|1.000709 [0.994425, 1.013590], 7/15|
|2x2-w16|13.170594 / 13.016083 / 12.875429|0.982133 [0.969338, 1.011131], 10/15|0.968708 [0.937640, 0.995201], 12/15|0.982761 [0.966613, 1.022666], 9/15|
|2x2-w32|11.086154 / 11.117988 / 11.233593|1.013027 [0.999951, 1.018392], 4/15|1.012894 [1.007883, 1.018379], 0/15|1.001684 [0.998702, 1.006495], 5/15|

Four-context equal-log ON/OFF: **1.006234 [0.991589, 1.030212], 6/15**; required gain absent.

Four-context equal-log ON/accepted: **1.012993 [0.965725, 1.044266], 7/15**; required gain absent.

All declared failures: `["1x2-w16/OFF/accepted/calibration","1x2-w16/ON/accepted/nonregression","1x2-w16/ON/OFF/nonregression","1x2-w32/ON/accepted/32-worker-upper-bound","1x2-w32/ON/OFF/32-worker-upper-bound","2x2-w16/OFF/accepted/calibration","2x2-w32/ON/accepted/32-worker-upper-bound","2x2-w32/ON/OFF/32-worker-upper-bound","four-contexts/ON/accepted/gain","four-contexts/ON/OFF/gain"]`.

Calibration intervals must fit [1/1.03,1.03]; all ON/control upper bounds must be <=1.03. Each 32-worker shape requires upper <=1; both equal-log aggregates require upper <1 and >=10/15 wins. Ranks 4..12 of 15 have nominal 96.484375% marginal coverage under independent exchangeable blocks, without familywise guarantees. No gain floor, favorable subset, extension, unchanged rerun or layout/attribute rescue. Physical-16 intervals are wide. This rejects this implementation's declared screen, without disproving LogJumps on other schedules, hardware or primes.

## CPU and memory diagnostics

|Context|Accepted / OFF / ON CPU ms/request|Accepted / OFF / ON max-RSS counter MiB|Accepted / OFF / ON minor faults/request|
|---|---:|---:|---:|
|1x2-w16|115.792 / 116.710 / 113.672|85.00 / 85.00 / 84.75|83.94 / 83.64 / 84.19|
|1x2-w32|179.290 / 179.332 / 181.344|128.00 / 126.75 / 126.25|182.03 / 184.42 / 182.60|
|2x2-w16|160.643 / 163.132 / 159.849|192.25 / 191.25 / 191.50|227.85 / 226.13 / 229.45|
|2x2-w32|256.946 / 256.311 / 259.087|303.75 / 304.75 / 303.75|506.78 / 502.11 / 505.59|

At 32 workers, CPU service increases about 1.12% for primary and 1.08% for 2x2 versus OFF. The positive 16-worker CPU observations remain above. RSS counters are diagnostic because earlier work found undercounts; they are not precise lifetime peaks, minimum RAM, stack peaks or pressure/swap bounds. Minor-fault counts do not measure their service time or establish disk paging. Major faults are not exported. No phone, ARM/TEE, huge 250/4999, concurrent-throughput or forced-pressure result is added.

## Counters and code generation

All six PMU groups qualify. Primary ON/OFF retired instructions rise about 4.52% at 16 workers and 4.29% at 32; cycles decrease about 1.38% and 0.21% in those one-shot panels. These are descriptive counters, without paired causal intervals. They do not isolate the range branch, dependency schedule, register allocation or runtime as the cause of the latency outcome.

Twenty-five actual named curve-function contexts are retained. Native specialization counts differ across builds. Borrowed G2 apply grows from 51,862 bytes in accepted/OFF to 55,526 bytes in ON. Its static IFMA count stays 1,450; stack-base operand lines rise from 1,455 to 1,717. Static operand lines are not measured dynamic spills or cache traffic. This is consistent with the intended trade: shorter reduction dependencies can still require more surrounding work and larger emitted code.

Six fixed-period user-cycle profiles conserve 62,636 samples with zero LOST, 8,520–12,328 samples per capture, unknown share 1.19–1.74%. Binary mappings, owning PID/thread forks, CPU masks and 16M periods are checked. All pass the >=1,000-sample/<=5%-unknown attribution screen. Absolute period coverage, kernel cost, local CPI and critical paths remain unvalidated. Full symbol/category totals are retained.

## Accepted CPU and Metal comparisons, unchanged

These are inherited accepted local 32-worker SDK observations and supplied M5 references. Fresh accepted/OFF/ON observations are above; the other six shapes were not remeasured after the initial screen failed. Production remains source176 and accepted SDK SHA-256 `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. All eight CPU 10x targets remain unmet.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10x target ms|
|---|---:|---:|---:|---:|
|1x2|7.928660|37 / 4.667x|24 / 3.027x|3.7|
|2x2|11.151932|59 / 5.291x|30 / 2.690x|5.9|
|4x2|16.086386|109 / 6.776x|48 / 2.984x|10.9|
|8x2|24.179453|189 / 7.817x|74 / 3.060x|18.9|
|12x2|33.884455|284 / 8.381x|101 / 2.981x|28.4|
|16x2|40.077381|335 / 8.359x|124 / 3.094x|33.5|
|24x2|65.850777|528 / 8.018x|176 / 2.673x|52.8|
|merge-24x1|65.060937|507 / 7.793x|175 / 2.690x|50.7|

Accepted merge54 remains 161.883899 ms without a supplied M5 reference. The accepted physical-16 calibration exception remains documented in its adoption report. Latest retained AWS source175 observations remain historical: Zopro 11.915591 ms at 32 workers and Aeglos 7.959184 ms at 96, each campaign's best tested c7a.24xlarge configuration. This experiment adds no AWS or Aeglos result.

The 104+100+52 implementation is closed for this screen. Its memory approach remains bounded in the stated source sense, but no proof-speed gain is established. Future work should reduce the number or cost of dominant field operations while retaining useful parallelism. The verified all-eight CPU 10x objective remains active.

## Artifacts

[Design](artifacts/design.md), [integer model](artifacts/model.json), [source recovery](artifacts/source-review.json), [all results](artifacts/analysis.json), [raw rows](artifacts/rows.json), [independent review](artifacts/review.json), [proof inventory](artifacts/proof-inventory.json), [profile symbols](artifacts/profile-symbols.json), [static code summary](artifacts/codegen.json), [assembly bodies](artifacts/codegen.json.gz), [source snapshot](artifacts/source.tar.gz), [all raw receipts](artifacts/receipts.json.gz), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [identities](artifacts/identities.json), [publisher](artifacts/publish.rs), and [checksums](SHA256SUMS).

The source archive has exactly 176 regular files with an exact member/hash roundtrip; both JSON gzip archives decompress byte for byte. Private keys, programs, witnesses, executables, libraries and raw perf captures remain local. Model/compiler and postcapture codegen helper repairs are preserved; no successful qualification or capture was repeated for bookkeeping.
