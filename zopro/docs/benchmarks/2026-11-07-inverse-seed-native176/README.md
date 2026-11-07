# Initial-scale inversion: native performance

The integrated scalar inverse passes its fixed performance screen. Against the unchanged OFF control, Fq improves **3.75%** and Fr **1.89%** by paired geometric mean. Both fields also improve against the accepted-field control, and control calibration passes. **No full-proof improvement or production adoption is established.**

The [qualified arithmetic](../2026-11-07-inverse-seed-scale176/README.md) moves h=2⁶⁴ into the initial modular coefficient. The ordinary Euclid finish retains its signed radix64 cancellation and returns its canonical result directly, removing the final field multiplication. Both early returns instead multiply by h⁻¹. The native snapshot changes only four expressions in `inverse.rs`; removing those edits recovers all176 accepted source files. Observation hooks and the prototype adapter are absent. GCD matrices, integer updates, divisions, field representation, feature/architecture guards and portable fallback are retained. No heap buffer, table, key copy, plan, pool or unsafe arithmetic is added.

## Complete inverse latency

Ryzen9950X, CPU2, one active thread. Thirty-one rotated/reversed blocks compare three immutable consumers: accepted-field frozen, separately built original OFF, and the new candidate. Controls and the benchmark driver are reused byte-for-byte. Each process uses4096 canonical inputs, three256-call primes and65536 observed complete inverse calls. Setup is outside the timer; inverse calls, output checksum and loop overhead are included. No other owned CPU workload overlaps recording. All186 processes and observations are retained.

|Field|Frozen median ns/call|OFF median ns/call|Candidate median ns/call|Candidate/OFF paired ratio,95% bootstrap|Wins/31|
|---|---:|---:|---:|---|---:|
|Fq|530.050|530.610|510.747|0.962537 [0.961915,0.963218]|31|
|Fr|519.287|519.220|509.106|0.981069 [0.979730,0.983062]|30|

Candidate/frozen ratios are0.963803 [0.963215,0.964441] for Fq and0.979861 [0.977457,0.982166] for Fr. OFF/frozen calibration intervals are[1.000598,1.001996] and[0.996621,1.000308], inside the declared factor1.03. Both fields require candidate upper bounds below1 against both controls; all four gain checks pass. There is no gain floor, field subset, exclusion, extension or reroll. Ratios pair process means by block and need not equal ratios of displayed medians. Bootstrap assumptions and the fixed seed are retained in the [protocol](artifacts/protocol.json) and source.

## Automatic profiles and counters

After all latency, four FIFO-gated counter panels and four separate profiles run both fields with OFF/candidate libraries. Profile-only consumers use the same original input construction and inverse wrappers, but are distinct executables from the latency consumers. Three primes precede the gate. Panels cover524288 calls; profiles cover4194304. Profiles use `cycles:u`, period4million,512 mmap pages and CPU2.

|Field|Cycles/call OFF→candidate|Instructions/call OFF→candidate|Profile samples OFF/candidate|
|---|---|---|---|
|Fq|2870.334→2791.069|8164.347→7988.810|3010 /2930|
|Fr|2889.746→2836.272|8143.320→7993.964|3026 /2961|

All counter events run100%. All four profiles pass the declared≥1000sample/≤5%unknown/zeroLOST/singlePID-and-TID/CPU2/4Mperiod/header/metadata checks. Maximum unknown share is0.0661%. Samples mostly fall in the complete inverse wrapper, where LLVM inlines the implementation. These are single diagnostic panels and descriptive samples, not replicated timing estimates, exclusive critical-path fractions or full-proof savings. Raw perf data stay local with hashes; exact decoded streams and receipts are archived.

All eight diagnostic loops report zero minor faults. Observed whole-process maximum RSS across them is3492KiB. This includes synthetic fixtures/runtime and does not establish prover RAM, incremental scratch, minimum RAM or memory-pressure behavior. Diagnostic loops add18,874,368 observed inversions and6144 primes to latency's12,189,696 observed calls and142,848 primes.

## Validation and limitations

The actual new library passes16,384 independent BigUint/output/product-identity checks over Fq/Fr, including zero and canonical outputs. Generic/no-assembly/no-IFMA compilation passes. Parent arithmetic qualification is inherited without rerunning its successful checks; old field/core suites are not replayed. The parent's outer GCD-exit branch remains unobserved; its correction is identical to the tested `b == 1` expression. This is an explicit runtime-coverage limitation.

Actual library and consumer links use O3/native/fatLTO/oneCGU and distinct audited hashes. The candidate's Fq/Fr inverse functions are8709/8752bytes versus7723/7786 in the retained controls. Each retains six static width-alternative division sites. Larger cold early-return code and compiler layout are tradeoffs; static size is not executed work. Source removes the normal-path scale product, while the separate panels show fewer retired instructions. No exclusive causal attribution is claimed.

Seven native commands, two qualification/preparation commands and215 capture/offline/analysis commands succeed on first execution. The benchmark-only path of the reused driver runs; its obsolete floating-quotient correctness command is not invoked. Independent review binds all source/consumer/tool pins and receipts, reconstructs186 raw process means, paired ratios/geomeans/wins and gates, and verifies the statistical utility bytes against their qualified parent. It does not rerun successful models, tests, captures, bootstrap or decoders. Full source recovery and exact publication/archive checks accompany the report.

Next qualify the real threaded G1/G2 inversion callers, including complete SDK measurements, automatic profiles, independent Gnark proofs and resource/fallback checks. These scalar gains do not predict a2× proof improvement. No unchanged retry or favorable subset is needed: both fields passed their original screen.

Accepted SDK/source176 and its [all-eight CPU/Metal comparisons](../2026-11-07-g1-merge-work176/README.md) remain unchanged. Local accepted1×2 observations remain7.928660ms at adoption, later7.915095 and7.991694ms. No proof, solver or fresh M5/AWS/Aeglos measurement is added; the independently verified proof ledger stays32637. The original CPU10× objective remains active/unmet,0/8. Mobile, huge-circuit, concurrent-proof, pressure/swap and minimum-RAM qualification are not supplied by this component.

[Latency](artifacts/analysis.json), [diagnostics](artifacts/diagnostic-analysis.json), [native qualification](artifacts/native-qualification.json), [source audit](artifacts/source-audit.json), [review](artifacts/review.json), [exact source/helpers/receipts](artifacts/source-and-receipts.tar.gz), [publication checks](artifacts/publication.json). Unchanged sources are bound to the accepted manifest; absolute dependency paths need adaptation elsewhere. Private root: `/home/ubuntu/src/zopro-perf-work/inverse-seed-native176`.
