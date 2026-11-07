# Ternary digit extraction: reciprocal arithmetic screen

The bounded reciprocal codec passes its integer correctness and compiled-code screens. It replaces fixed-divisor 128-bit division with multiplication and one quotient correction. Complete-query timings are pending; this is not a speedup or adoption result. Accepted production source and SDK remain unchanged.

This follows the [rejected fixed ternary query trial](../2026-11-07-ternary-g2-caller176/README.md). That trial reduced aggregate CPU work but increased primary 32-worker query latency from 3.065 to 3.334 ms against its same-binary original-algorithm control. The positive single-worker results did not qualify the candidate.

## Saved profiles and actual compiled work

The new attribution reconciles all 14,779 samples from the three existing primary 32-worker profiles, including every worker and the caller thread. All 32 workers received samples. Effective participation, computed as the inverse concentration of sampled worker service, was 31.707 for the older caller, 31.819 for the fresh original-algorithm control, and 31.457 for the ternary candidate. These aggregate distributions do not establish simultaneous utilization, idle time, per-query tails, or the cause of the threaded slowdown. No profile was recorded or decoded again.

The first focused decode of that ternary consumer's feed resolves five static call sites to `__udivti3` and one to `__umodti3`, using its ELF relocations and symbol table. These are static sites on shared branches, not six dynamic calls per point. This emitted work motivates a distinct codec change; window, queue, bucket, grouping, Phi, and worker geometry are held fixed.

## Reciprocal codec

For a fixed odd divisor `1 < D < 2^127`, use `mu = ceil(2^128 / D)`. For `N < 2^127`, the high half of `N * mu` gives the exact quotient or one greater. Subtract one when its product with `D` exceeds `N`; that product cannot overflow because it is at most `N + D < 2^128`. Divisor one is handled directly. The original balanced-digit numerators satisfy the required bound for every divisor `6561^j`, `j = 0..10`.

The implementation uses universal compile-time constants, no heap allocation, and the existing 32-byte scalar representation. It adds no point table or retained execution plan. This does not qualify the ternary algorithm's larger bucket workspace for production resource admission or establish a process RAM bound.

Three new tests pass in checked and optimized configurations, with the same populations in each configuration:

| Check | Cases per configuration |
|---|---:|
| High-half multiplication against an independent 32-bit-limb product | 16,528 |
| Quotients against built-in division, including product bounds and transitions | 178,264 |
| Signed digits against per-trit conversion and checked reconstruction | 147,753 |

The invalid-window panic is expected and caught. Existing successful point tests and full-MSM oracles are inherited; they were not rerun for this screen.

A fresh complete-query consumer was built with O3, native CPU targeting, fat LTO, and one codegen unit. Of its 178 source files, 177 match the previous qualified ternary snapshot; only the codec changes. Its driver is byte-identical to the qualified driver. The actual feed and carry-component boundaries contain no wide-division/remainder calls or `div`/`idiv` instructions. The feed grows from 2,360 to 3,252 bytes. Removing helpers is a code-generation result, not a dynamic cost or latency measurement.

Consumer SHA-256: `28bce6f68ae92bf79acfc22161f9a59c17c6aaf34e432dd389cf26d2cceeb35a`.

Independent review checks source pins, 11 successful workflow command receipts, all saved sample counts and worker distributions, both new compiled boundaries, the new integer test results, and all 176 unchanged production files. The first review failed on an external PLT symbol lookup; the corrected reader handles annotated external PLT calls and resolves relative production-manifest paths. Both readers and failure logs are retained. No successful test, capture, or decoder was replayed. An earlier attribution launch was rejected by the host lock before any child ran; its serialized launch succeeded.

## Accepted proof status

These are inherited local accepted-SDK observations from the preceding published full-proof trial, not measurements of this codec. M5 CPU and Metal references were supplied separately and use different hardware.

| Shape | Local CPU ms | M5 CPU ms | M5 Metal ms | CPU 10× limit ms |
|---|---:|---:|---:|---:|
| 1×2 | 8.041470 | 37 | 24 | 3.7 |
| 2×2 | 11.237839 | 59 | 30 | 5.9 |
| 4×2 | 16.143230 | 109 | 48 | 10.9 |
| 8×2 | 24.397442 | 189 | 74 | 18.9 |
| 12×2 | 33.736405 | 284 | 101 | 28.4 |
| 16×2 | 40.082192 | 335 | 124 | 33.5 |
| 24×2 | 65.765454 | 528 | 176 | 52.8 |
| Merge 24 | 65.350546 | 507 | 175 | 50.7 |
| Merge 54 | 161.495859 | — | — | — |

The all-eight CPU 10× goal remains unmet, 0/8. The canonical verified-proof ledger remains 32,196: this stage creates no queries, proofs, or recordings. It supplies no fresh AWS/Aeglos comparison, mobile result, large-circuit result, or memory-pressure qualification.

The next stage is a declared matched complete-query trial against the original algorithm and old codec, with fresh original-algorithm calibration, both retained populations, and 1/16/32-worker cases. Charge all conversion, scalar reset, allocation, pool use, and normalization. A credible caller result must still qualify through the verified full SDK and resource policy before adoption.

## Evidence

[Evidence archive](artifacts/evidence.tar.gz), [member hashes](artifacts/evidence-members.json), and [archive checksum](artifacts/SHA256SUMS) include the new helpers, changed codec, plans, receipts, saved attribution, decodes, and review. The archive excludes binaries, libraries, raw perf recordings, and private proving inputs. Existing source and oracle provenance is linked through the preceding report and pinned manifests. Original absolute paths in plans identify the working environment; reconstruct them before executing a workflow elsewhere.
