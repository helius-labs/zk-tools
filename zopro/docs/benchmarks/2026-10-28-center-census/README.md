# Actual G1 signed-centering census

On the accepted primary 1×2 proof, signed centering reduces nonzero G1 digit terms from 1,577,564 to 1,574,296: **3,268 terms, or 0.2072%**. Of that reduction, 3,237 terms occur in final windows. The main streaming windows save 31 terms out of 1,503,791. This preserves a small native-optimization hypothesis; it does not establish a latency gain or a path to a twofold prover improvement.

The isolated observer uses the accepted 172-file source snapshot. Only `lib.rs` and `msm_affine/borrowed_pending.rs` receive reversible diagnostic hooks; 170 original files remain byte-identical. The new module counts the native borrowed G1 ordinary query and fused K/H scalar feeds after production routing/grouping decisions. It does not change proof scalars, bases, grouping, widths, recoding or curve arithmetic. The native Booth function is copied exactly and checked against production. G2 is outside this census.

The [synthetic work screen](../2026-10-28-factor-recoding/README.md) motivated this actual-input check. Two new observer tests pass against native field reconstruction and a separate bitwise Booth reference, and cover boundary/random values, identity points, capture gating, partitions, and combined streams. No successful test or capture was repeated.

Three independent processes each load the same accepted Euler image and request fixture, make three unobserved prime proofs, and observe one proof with 32 workers on CPUs 0..31. All three count maps are identical. All 12 SDK requests verify internally; each of the three exported 164-byte proofs is fresh against the prior 12,580 and independently Gnark-verified with the expected public inputs. The proof ledger is now 12,583. All SDK captures preceded the three independent verifications and offline analysis. No owned build, hashing, archive or other probe overlapped SDK capture.

| Actual stream | Inputs | Original terms | Centered terms | Term reduction | Small original | Small centered |
|---|---:|---:|---:|---:|---:|---:|
| fused-combined/width=13 | 61451 | 1170466 | 1170158 | 0.026314% | 681 | 683 |
| fused-h/width=13 | 30922 | 610427 | 610282 | 0.023754% | 0 | 0 |
| fused-k/width=13 | 30529 | 560039 | 559876 | 0.029105% | 681 | 683 |
| ordinary/part=0/width=12 | 21006 | 407098 | 404138 | 0.727098% | 654 | 655 |

Small counts refer to nonzero magnitudes at most 65,535 with nonidentity bases. Inputs include zeros: ordinary 1,733; K 1,865; H 391. Centered signs are 9,320 ordinary, 13,971 K and 15,293 H. The combined row duplicates the K/H segment rows and must not be added to them.

| Nonduplicated stream | Final-window terms | Other-window terms | Other-window reduction |
|---|---:|---:|---:|
| ordinary/part=0/width=12 | 15554 → 12587 | 391544 → 391551 | -7 |
| fused-combined/width=13 | 58219 → 57949 | 1112247 → 1112209 | 38 |

The ordinary main stream gains seven terms after centering (391,544 → 391,551), while the fused main stream saves 38 (1,112,247 → 1,112,209). The original sum 1,503,791 exactly matches the prior actual-queue census's G1 first assignments plus executed pending additions plus projective updates. This is a separate conservation cross-check against the earlier instrumentation.

Final-window maximum occupied digit indices change 3 → 2 for ordinary and 97 → 48 for each K/H segment. However, production reduces the K and H final windows separately; the combined occupancy count is not their actual reduction cost. Signed centering also introduces negative final digits (6,353 ordinary and 28,994 across K/H), which the existing positive-only indexed top-window reducer does not support. A native implementation must account for those signs rather than treating the count reduction as free.

Per-window term, negative, occupancy, ideal insertion and maximum-index counters are retained, including unfavorable windows. None models affine batching, cancellation, weighted reduction, inversion, allocations, canonical conversion or critical-path scheduling. No latency, PMU, RSS, minimum-RAM, phone, TEE, concurrency, pressure or AWS result is claimed. This study does not explain the C7a/Aeglos latency gap. No production source or key lookup table was added.

Before any native trial, audit exclusive ownership of the existing G1 canonical buffers and sign handling in indexed final-window reduction. Existing unused scalar bits might hold proof-local signs only if every reader is proved compatible; this is unimplemented, not a memory or speed guarantee. Avoid a second point array or retained key plan. Whole-Fr order centering must not be applied independently to existing 127-bit G2 Frobenius components. Retain the positive count result without an arbitrary gain floor, but require controlled full-proof evidence for adoption.

The controller's workflow result-schema assertion was corrected before the SDK build or capture, with the initial source/binary hash preserved. Source hooks and the qualified observer were unchanged. Build and capture workflows then completed on their first attempts. All 366 capture pins still match. The accepted SDK remains `14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a`.

The following are inherited accepted local observations and supplied M5 references, not measurements from this census. The [FFT adoption](../2026-10-27-clean-fft-adoption/README.md) retains its documented engineering exception and false full-performance/qualified-scope gates. The original all-eight CPU 10× objective remains active and unmet for every row.

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

Accepted primary peak RSS remains the historical 134.50 MiB observation. Merge 54→1 remains 161.341835 ms with historical peak RSS 1,910.1875 MiB and no supplied M5 reference. These do not establish minimum RAM.
