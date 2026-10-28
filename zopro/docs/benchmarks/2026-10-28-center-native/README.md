# Native signed-centered G1 A query

The candidate was not adopted. With 32 workers, median times were 8.498849 ms for accepted control, 8.545417 ms for candidate OFF, and 8.565815 ms for candidate ON. The paired ON/control ratio was 1.008541670: 0.854% slower, with interval [1.002988799, 1.015216649] and 2/15 wins. Positive estimates and failed checks are retained below.

The [actual scalar census](../2026-10-28-center-census/README.md) found a 0.7271% reduction of A-query digit terms, mostly in its last window. This trial tests whether that saving survives scalar conversion, signed final-window reduction and the full proof's parallel execution.

The exclusive A-query path reuses the existing owned canonical scalar vector. After the original grouping decision, it encodes `min(k,r-k)` and its point sign in bit255 of the same four words; the signed reader clears that bit before Booth extraction. A successful grouping uses its existing owned values instead. The specialized borrowed stream folds the sign into its existing signed bucket index. The last window uses the same tiled affine reduction with signed input indices and the narrower magnitude bound. Existing point scratch is reused; there is no new scalar vector, sign bitmap, point-key copy or retained table. Other K/H and G2 paths keep their original readers. Unsupported paths and OFF fall back before encoding.

Four original files have reversible edits; 168 others match the accepted 172-file snapshot. Three new native tests cover tagged coefficient reconstruction at widths 4–15, signed indexed reductions with complete curve cases, and centered borrowed MSMs at widths 10–14 with one or two partitions against an independent weighted-generator reference. All 122 native core tests pass OFF and ON. The initial raw/cubic filters matched zero tests; closeout caught this and its assertion log is preserved. The correctly named unfiltered suites then passed two raw tests and six cubic tests, with explicit required-test checks, after all captures and independent proof verification. No measurement was repeated. An initial compile visibility error and profile FIFO setup were corrected before capture; their original sources and logs are preserved.

The fixed study ran 15 rotated/reversed blocks at 32 logical and 16 physical workers, comparing accepted control, candidate OFF and candidate ON. Each latency process made three primes and 151 verified SDK requests. Three four-worker preflights preceded latency; six separate 251-request warm profiles followed all 90 latency processes. All 99 SDK captures preceded all 99 fresh Gnark verifications, profile decoding and archive work. All 99 distinct 164-byte exports verify independently with bound public inputs and fresh hashes against the prior 12,583. The study totals 15,396 SDK requests; the proof ledger is now 12,682. The ON preflight records four `CENTER_G1 n=21006 width=12 parts=1` entries, including primes. No owned build, helper, hashing or archive workload overlapped capture.

| Workers | Variant | Median block ms | CPU ms/request | Median process peak MiB | Maximum process peak MiB |
|---|---|---:|---:|---:|---:|
| 32 | control | 8.498849 | 191.752735 | 129.75 | 135.75 |
| 32 | off | 8.545417 | 191.543742 | 131.50 | 137.25 |
| 32 | on | 8.565815 | 192.631815 | 129.75 | 133.50 |
| 16 | control | 11.002251 | 124.046987 | 89.25 | 91.25 |
| 16 | off | 10.920346 | 124.915815 | 88.75 | 91.50 |
| 16 | on | 10.932550 | 125.254623 | 86.25 | 90.50 |

RSS includes image load, prime proofs and the measured loop. These are observed process high-water values, not minimum RAM requirements or phone/pressure qualification. Code reuse of scalar storage does not establish whole-process memory equivalence.

| Workers | Paired ratio | Median | Conditional interval | Wins | Declared check |
|---|---|---:|---:|---:|---|
| 32 | off/control | 1.005479330 | [1.001782919, 1.008609179] | 2/15 | pass |
| 32 | on/control | 1.008541670 | [1.002988799, 1.015216649] | 2/15 | fail |
| 32 | on/off | 1.003578673 | [0.997952673, 1.010223556] | 6/15 | fail |
| 16 | off/control | 0.989234804 | [0.945084080, 1.109464777] | 8/15 | fail |
| 16 | on/control | 0.987496801 | [0.966209551, 1.021948515] | 8/15 | fail |
| 16 | on/off | 0.993021400 | [0.907016940, 1.032444922] | 8/15 | fail |

Ratios use 15 paired block medians. The fourth through twelfth ordered ratios give nominal 96.484375% per-comparison coverage under independent exchangeable blocks, without a familywise guarantee. OFF/control calibration requires its interval within [1/1.03, 1.03]. Primary ON requires an upper ratio below 1 and at least 11 wins against both controls; secondary upper ratios must be at most 1.02. There is no minimum improvement floor. No sample exclusion, adaptive extension or selective rerun was used.

| Workers | Variant | Warm cycles samples | Lost | Centered namespace share | Centered borrowed-loop share |
|---|---|---:|---:|---:|---:|
| 32 | control | 8913 | 0 | 0.000% | 0.000% |
| 32 | off | 8961 | 0 | 0.000% | 0.000% |
| 32 | on | 9091 | 0 | 0.071% | 2.180% |
| 16 | control | 5554 | 0 | 0.000% | 0.000% |
| 16 | off | 5712 | 0 | 0.000% | 0.000% |
| 16 | on | 5529 | 0 | 0.000% | 2.484% |

The initial namespace matcher saw mangled Rust names and returned zero. Offline demangling corrects that labeling; no sample or capture changed. The centered namespace includes the encoder, while the centered borrowed-loop symbol includes existing A-query queue, arithmetic and final-window work. These periods are not isolated overhead. Primary ON period shares include G1 stream arithmetic (~19.04%), G2 prefix arithmetic (~14.67%), epoch pinning (~8.55%), and scalar inversion (~7.83%). These shares do not establish the cause of the timing difference.

Each profile contains one SDK PID. FIFO enable occurs after primes and disable before proof export. Symbols and periods are retained; a symbol share is not a removable wall-time fraction or a causal attribution of the full timing difference. Raw perf data remains local with recorded hashes. No new assembly or hardware-counter experiment is implied by these user-cycle profiles.

Production remains unchanged. This is a primary 1×2 screen; no new all-eight/54 performance, portable/mobile/TEE, concurrency, pressure or AWS qualification was run. The all-eight 10× CPU goal remains active and unmet. The accepted [FFT adoption](../2026-10-27-clean-fft-adoption/README.md) retains its engineering exception and false full-performance/qualified-scope gates. The 16-worker paired medians are positive (ON/control 0.987497, ON/OFF 0.993021), but calibration fails and both upper nonregression bounds exceed 1.02. Those estimates do not establish a win.

The following accepted local timings and supplied M5 references are inherited, not new measurements from this trial.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU ratio | Metal ratio |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 24 | 4.445× | 2.883× |
| 2x2 | 11.460675 | 59 | 30 | 5.148× | 2.618× |
| 4x2 | 15.854365 | 109 | 48 | 6.875× | 3.028× |
| 8x2 | 24.084626 | 189 | 74 | 7.847× | 3.072× |
| 12x2 | 33.693397 | 284 | 101 | 8.429× | 2.998× |
| 16x2 | 39.981636 | 335 | 124 | 8.379× | 3.101× |
| 24x2 | 66.010670 | 528 | 176 | 7.999× | 2.666× |
| merge-24x1 | 65.086624 | 507 | 175 | 7.790× | 2.689× |

Accepted primary historical peak RSS remains 134.50 MiB. Merge 54→1 remains 161.341835 ms with historical peak RSS 1,910.1875 MiB and no supplied M5 reference. The C7a/Aeglos comparison has not been repeated for this candidate.
