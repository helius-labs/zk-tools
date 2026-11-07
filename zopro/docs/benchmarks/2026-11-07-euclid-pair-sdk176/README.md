# Integer quotient/remainder boundary in the complete prover

The scalar inverse improvement does not establish a proof-time gain. At 32 workers, the candidate measures **8.010 ms**, the unchanged accepted SDK **7.992 ms**, and the fresh control **7.986 ms**. Both paired confidence intervals cross parity. The 16-physical-core calibration also fails because its interval is too wide. This exact candidate is closed without adoption, additional blocks or a parameter sweep.

## Change and qualification

The [component experiment](../2026-11-07-euclid-pair-boundary176/README.md) improved complete Fq inversion by 3.59% and Fr inversion by 1.63%. A private Rust `#[inline(never)]` helper returns both integer quotient and remainder before the inverse's zero-remainder branch. The full SDK now exercises that exact arithmetic change through its ordinary G1/G2 callers. No new caller adapter or timing instrumentation is added. There is no new table, allocation, key copy, retained plan, pool, floating arithmetic, C or assembly in the change.

The fresh control and candidate use coherent eight-library links, with actual O3/native/fat-LTO/one-CGU final flags. The candidate adds only its arithmetic cfg. Of the 176 accepted source files, 175 are byte-identical and the inverse parent is reversibly edited; one helper child is added. All original files are recoverable, and all production hashes remain unchanged. A portable generic/no-ASM/no-IFMA check with the candidate cfg passes. This is compile qualification, not execution on a phone or ARM device.

The component's 16,384 independently checked field inverses and checked integer prototype are inherited rather than rerun. New SDK codegen qualification lists only the new control/candidate and decodes the candidate inverse bodies and helper. The inverse bodies contain no DIV instructions, call the shared helper, and contain no floating instructions; the helper retains two alternative integer-DIV widths. Static sites are not dynamic instruction counts.

Before timing, control and candidate each run all nine proof shapes, with three priming requests and one observed request. All 18 fresh exported proofs pass independent Gnark verification with image, public-input and verification-key bindings. These preflights establish correctness coverage; their timings and RSS are not the matched performance or minimum-RAM result.

## Fixed matched pilot

The host is an AMD Ryzen 9 9950X with 16 physical cores and 32 logical CPUs. The declared pilot uses 31 blocks, accepted/control/candidate, and two CPU sets: 0–31 with 32 workers, and 0–15 with 16 physical-core workers. The six cases rotate and reverse order. Each process primes three requests and observes 151 requests at 32 workers or 51 at 16 workers. All 186 latency captures finish before their 186 independent Gnark verifiers. No owned build, analysis, hashing or publication overlaps capture.

Times summarize medians of 31 process means. Ratios are geometric means of paired block ratios; intervals use the declared deterministic 20,000-resample bootstrap. They need not equal ratios of the displayed medians.

| Workers | Accepted ms | Fresh control ms | Candidate ms | Candidate/accepted [95%] | Candidate/fresh [95%] |
|---|---:|---:|---:|---:|---:|
| 32 logical | 7.991694 | 7.986243 | 8.010248 | 1.001012 [0.998096, 1.004006] | 1.001794 [0.999251, 1.004418] |
| 16 physical | 9.798350 | 9.806593 | 10.289574 | 1.002820 [0.939440, 1.069266] | 1.011143 [0.954533, 1.071414] |

At 32 workers the candidate wins 13/31 paired blocks against accepted and 14/31 against fresh. Fresh/accepted calibration passes: 0.999220 [0.996039, 1.002375], inside the declared factor-1.03 range. Neither candidate interval has an upper bound below one, so neither establishes a gain. These data also do not establish a statistically significant regression.

At 16 workers fresh/accepted is 0.991769 [0.940237, 1.048750], outside the calibration range. Both candidate intervals exceed the declared 1.02 nonregression ceiling. Their uncertainty is wide; the difference between the unpaired medians is not evidence of a precise 5% regression. Its cause is unresolved. All five failed pilot checks are retained. There are no excluded blocks, extensions, favorable subsets or rerolls. No all-nine timing/adoption campaign is selected.

| Workers | Accepted peak MiB | Fresh peak MiB | Candidate peak MiB |
|---|---:|---:|---:|
| 32 logical | 132.75 | 133.50 | 133.25 |
| 16 physical | 85.50 | 84.50 | 85.25 |

These are maximum whole-process RSS observations across the corresponding pilot processes, including load, priming and allocator history. They are not minimum RAM, pressure/swap, concurrent-proof or large-circuit measurements. Median recorded process CPU time per observed request is 179.456/178.928/179.129 ms at 32 workers and 116.544/116.506/115.558 ms at 16 workers, accepted/fresh/candidate respectively; request-loop summaries include surrounding loop and export work.

## Automatic diagnostics

After latency capture, six grouped counter panels and six `cycles:u` profiles cover every topology/arm. Panels observe 31 requests and profiles 251, each after three primes. Profiles use period 16,000,000, 512 mmap pages and FIFO gating. All diagnostic captures finish before verification and offline decoding. Every panel reports 100% counter running time. All six profiles pass the declared sample, ownership/MMAP, CPU/PID/TID, gate and loss checks: 63,367 samples, zero LOST records and at most 1.878% unknown samples. Real G1 and G2 MSM symbols occur in every profile.

The changed helper is sampled 60 times in the 32-worker candidate and 72 times in the 16-worker candidate; controls have zero helper samples. Thus the actual proof executes the changed path. Sample attribution is not an exclusive wall-time fraction.

The single 32-worker candidate/fresh panel ratios are 1.002168 cycles, 0.998118 instructions, 0.995941 cache misses and 0.984126 branch misses. The instruction reduction is about 0.188% in this one panel. This does not supply repeated-panel uncertainty, explain the timing variation or establish a causal speedup. The isolated inverse gain remains a valid component result, but supplies no demonstrated end-to-end gain in this SDK pilot.

Across preflight, latency and diagnostics, 21,144 SDK requests produce 216 distinct exported proofs accepted independently by Gnark. The canonical export ledger advances 32,196 → 32,214 → 32,400 → **32,412**. Internal requests and copied inventories are not counted as additional independent exports.

Offline review checks 467 successful workflow receipts, six new listing/decode receipts, 428 pilot pins, source recovery, all proof bindings, raw means and paired arithmetic, and diagnostic summaries. The first reviewer demanded bit-identical floating-point means after JSON round-tripping and failed on a roughly 2e-15 ms discrepancy. Its source and captured failure remain. A separate reviewer permits 1e-11 ms arithmetic reconstruction error; it passes without changing any measurement, confidence interval or selection gate. Native builds, SDK correctness, captures, cryptographic verifiers, bootstrap analysis and decoders are not rerun for that repair.

## Accepted results remain separate

Accepted SDK SHA256 remains `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. Its primary adoption result was **7.928660 ms**, a later baseline was **7.915095 ms**, a previous matched observation was 8.041470 ms, and this pilot observes **7.991694 ms**. These are different runs of the same binary. No slower version was adopted, and the run differences alone do not identify a cause.

The complete [adoption](../2026-10-30-g2-vector-sign-adoption176/README.md) table below is inherited, not newly measured or combined with this pilot's primary observation. M5 references are supplied historical comparisons across different machines.

| Shape | Local adoption ms | M5 CPU ms | M5 Metal ms | CPU 10× target ms |
|---|---:|---:|---:|---:|
| 1×2 | 7.928660 | 37 | 24 | 3.7 |
| 2×2 | 11.151932 | 59 | 30 | 5.9 |
| 4×2 | 16.086386 | 109 | 48 | 10.9 |
| 8×2 | 24.179453 | 189 | 74 | 18.9 |
| 12×2 | 33.884455 | 284 | 101 | 28.4 |
| 16×2 | 40.077381 | 335 | 124 | 33.5 |
| 24×2 | 65.850777 | 528 | 176 | 52.8 |
| Merge24 | 65.060937 | 507 | 175 | 50.7 |

Merge54 adoption is 161.883899 ms with no supplied M5 reference. The original all-eight CPU 10× goal remains active and unmet, 0/8. This experiment supplies no new AWS/Aeglos, mobile, minimum-RAM or large-circuit comparison.

[Evidence archive](artifacts/evidence.tar.gz), [member hashes](artifacts/evidence-members.json), [checksum](artifacts/SHA256SUMS). Source edits, driver/support, declarations, receipts, all latency observations, codegen and analysis/review sources are included. Raw perf data and large decoded profile streams remain local with receipt/hash references. Build targets, binaries/libraries, private continuation notes and proving/input/proof payloads are excluded. Absolute paths require reconstruction elsewhere.
