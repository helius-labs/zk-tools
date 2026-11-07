# Compact lazy-Y G2 arithmetic feasibility

The new native arithmetic and storage boundary pass correctness checks. **No speedup has been measured and production is unchanged.** The next test must include a complete G2 Window: queues, scalar tails, exceptional points, Phi transformation and final canonicalization.

The candidate retains canonical X and an unreduced Y representative below 2p in each ordinary bucket. A separate word type keeps those values out of canonical `Fq` objects. Bucket size remains **128 bytes**, versus 160 bytes in the closed canonical radix-52 storage experiment. Each vector update can omit two Y export reductions. Extra scalar normalization at tails, exceptional paths and final output may offset that saving.

For canonical incoming Y and lazy bucket Y, signed differences use `2p + by - ay` or `3p - by - ay`. A private helper consumes these values immediately; the general field API retains its original bounds. The conjugate numerator uses a `3p - D` complement and the unchanged two fused Montgomery products. Each unsigned input sum is at most 12p². Since 4p < 2²⁵⁶, REDC is below 4p and the existing conditional subtraction of 2p restores the ordinary strict bound. No extra multiplication or input-sized table is introduced.

The implementation differs from the closed native radix-52 buckets, canonical export reordering and canonical-anchor loose-delta experiments. It reuses their arithmetic and packing knowledge; no algorithmic novelty is claimed. Its distinct change is retaining numerical Y representatives across bucket updates.

Validation on the current AVX-512 host:

| Check | Result |
|---|---:|
| Boundary numerator cases | 16,200 |
| Deterministic random cases | 4,096 |
| Output-fed cases | 1,024 |
| Native numerator outputs checked against integer CIOS, exact BigUint REDC and scalar field arithmetic | 42,640 |
| Additional scaled numerator outputs | 42,640 |
| Inclusive 3p signed differences exercised | 900 |
| Existing fused 2p corrections exercised | 1,516 |
| Valid G2 point updates, with Y retained lazily between updates | 512 |
| Noncanonical Y components observed in those point outputs | 706 |

Raw export/import round trips and canonicalization agree. The independent integer CIOS model also checks coefficient sizes, normalized limbs and exact returned representatives, rather than congruence alone. Maximum observed unsigned coefficient is 68,770,340,523,242,831; finite observations are not a universal overflow proof. Analytically, allowing at most eight radix-52 units of growth per full row bounds the first four rows by 32 units. The final product/reduction accumulators remain below 38 units; radix-48 repacking and normalization remain below 64 units, or 2⁵⁸, safely inside a 64-bit accumulator. Normalized operands still fit four words, including inclusive 3p.

A separate safe terminal `into_iter().map(...).collect()` conversion reuses the original allocation in all **14** tested length/capacity combinations (0–4,096 elements, exact capacity or 37 spare slots). It makes zero allocations, reallocations or deallocations and preserves pointers and capacities, while producing canonical points. This is evidence for the pinned Rust 1.98.1 native fat-LTO build, **not a portable allocation guarantee or a complete Window memory peak**. Integration must conservatively account for potential output overlap or retain the original resource-constrained path. Equal element sizes alone do not prove equal peak memory.

The 177-file private snapshot has 175 exact originals and one append-only parent include; removing it recovers all 176 accepted sources. The generated IFMA arithmetic and generator are unchanged. A new low-level export block uses the original packing/transpose/store operations with a transparent raw-word target. Existing AVX guards and normalized-value bounds apply. The field build uses O3/native/fatLTO/oneCGU with debug and overflow checks. The arithmetic driver uses O3/native without LTO to reuse existing tooling libraries; the allocation driver uses O3/native/fatLTO/oneCGU. These are correctness and allocation observations, not latency measurements.

One driver compile failed because it called `invert` rather than `inverse`. The corrected driver changes only that method spelling; the successful field build was reused. Five workflow commands succeed across the three stages, and the failed compiler command is retained. Review passes; its publisher then expected a `tests.json` that xtask does not create for failed compilation. A separate publisher consumes the saved review and includes that optional file where present. Both failures and original helpers are retained, without replaying the successful review or qualifications. Original successful tests, models, profiles, verifiers and benchmark recordings were not replayed. There are no new SDK requests, proofs, counters, profiles, RSS measurements or AWS/Aeglos results. The independent-proof ledger remains 32,857.

Next, qualify the actual complete Window against the original implementation, including identity/equal-X/projective spill handling, tails, repeated reuse, key preservation, final conversion, allocation and admission accounting, portable fallback and 1/16/32-worker execution. Declare fixed matched latency and automatic profiling protocols before recording. A small isolated saving cannot establish a full-proof improvement; a positive complete-caller result still requires complete SDK measurements and independent verification before adoption.

The accepted SDK is unchanged. Its historical all-shape results below are retained for the original goal; none was remeasured in this stage. The latest matched primary observation is 8.027583 ms using the same binary that previously measured below 8 ms.

| Shape | Accepted CPU ms | M5 CPU ms / speedup | M5 Metal ms / speedup | CPU 10× limit ms |
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge24×1|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Merge54 remains 161.883899 ms without a supplied M5 reference. The original all-eight CPU 10× goal remains active and unmet, **0/8**. This stage qualifies feasibility only.

[Design](artifacts/design.md), [arithmetic checks](artifacts/qualification.json), [allocation checks](artifacts/normalization.json), [source audit](artifacts/source-audit.json), [review](artifacts/review.json), [source and receipt archive](artifacts/evidence.tar.gz) and [checksums](SHA256SUMS).
