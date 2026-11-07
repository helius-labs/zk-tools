# G1 merge-copy coverage across nine proofs

The common bucket-merge helper copies **zero ordinary G1 right-operand records in the observed 1×2 and 2×2 requests**. On 4×2 it copies 85,993 records, 6.19 MB at the source level, while the G1 streaming kernels write 587.66 MB of group records. A borrowed G1 merge port does not address the primary path and is not selected as the route to a twofold proof improvement.

These are new work counts, **not performance measurements or physical memory traffic**. No optimization is implemented or adopted. Accepted production176/SDK6d83 and accepted timings remain unchanged. The original all-eight CPU 10× goal stays active and unmet.

## Measurement boundary

The [preceding larger-query audit](../2026-11-07-g1-large-coverage176/README.md) distinguishes G1 streaming dispatch from the tree reducer and bounds historical A-group populations. This diagnostic resolves a different missing quantity: actual ordinary records copied in `merge_buckets`, versus signed/unsigned G1 streaming work, in complete SDK requests. It does not rerun that prior census or any old decoder/model.

Three reversible, cfg-only parent edits add a module declaration, one G1 streaming-entry hook, and one hook after the original merge-pair classification. All original arithmetic, point selection, loops, bucket geometry and allocations remain. Of 176 originals, 173 remain byte-identical and all originals recover; one fixed-size TLS counter child is added. There is no new product table, point/key/plan copy, production pool, field primitive or unsafe prover operation. The driver uses the existing trusted mmap loading boundary.

The recorder has 24 u64 cells and an enable flag per participating thread. It resets the caller and global Rayon workers outside the observed request, counts one request, then disables/snapshots before printing or proof export. Metadata fields hold actual type sizes rather than summing them. Ordinary merge records, bucket entries, stream points, full lane groups and tails are anonymous; scalar values and coordinates are never exported. Reset/snapshot allocation and diagnostic TLS are not proposed product caches or RAM requirements.

Each shape loads its existing image and request, primes three fresh SDK proofs with counters disabled, then observes one fresh SDK request. All nine SDK captures precede all nine pinned independent Gnark verifications. The request includes the standard SDK solve/prove/verify/serialization path. This deliberately does not measure elapsed time, CPU service, RSS, cache traffic or observer overhead. Results cover these retained requests at 32 workers on CPUs 0–31, not arbitrary witnesses or concurrent proof load.

## Actual counts

MB below means decimal source bytes divided by 1,000,000. A G1 affine point occupies 64 bytes and an owned `(u32, point)` record 72 bytes on this target. A full native streaming group holds eight point additions and writes a 1,600-byte group record. This repeated write volume is not simultaneously live RAM or allocated bytes.

| Shape | G1 merge jobs | Ordinary merge pairs | Owned merge-record MB | Unsigned stream points | Signed stream points | All G1 group-record MB |
|---|---:|---:|---:|---:|---:|---:|
| 1×2 | 0 | 0 | 0 | 0 | 1,431,940 | 286.3552 |
| 2×2 | 0 | 0 | 0 | 0 | 1,997,321 | 399.4208 |
| 4×2 | 168 | 85,993 | 6.191496 | 1,954,488 | 984,282 | 587.6640 |
| 8×2 | 168 | 85,995 | 6.191640 | 3,832,192 | 1,239,906 | 1,014.3328 |
| 12×2 | 0 | 0 | 0 | 5,705,890 | 1,526,530 | 1,446.4320 |
| 16×2 | 0 | 0 | 0 | 7,575,616 | 1,720,202 | 1,859.1280 |
| 24×2 | 168 | 85,938 | 6.187536 | 12,651,800 | 879,260 | 2,706.0720 |
| Merge24 | 0 | 0 | 0 | 10,783,809 | 2,086,847 | 2,574.0832 |
| Merge54 | 336 | 171,973 | 12.382056 | 28,100,857 | 0 | 5,620.0208 |

For 4×2, ordinary merge pairs are 4.40% of unsigned G1 streaming points. Owned merge-record bytes are 1.054% of all G1 streaming-group write bytes. The corresponding byte ratios are 0.610% for 8×2, 0.229% for 24×2 and 0.220% for Merge54. These compare semantic counts, not costs; they are not an Amdahl latency bound or proof that the copies have zero cost. The zero rows concern the helper under test, not every possible merge, normalization or copy in the prover.

The code still contains these copies on covered paths. A memory-oriented implementation could have value, but a performance claim needs a separate complete-caller test. The [G2 borrowed-merge experiment](../2026-11-06-g2-borrowed-merge176/README.md) already reduced a larger operand record and failed its wide-worker gain checks. Its failure does not prove all G1 ports lose; it is additional reason to require actual G1 coverage rather than assume a general bottleneck.

## Qualification and retained failures

One new recorder test passes with four workers, covering caller/worker accounting, reset, metadata consistency and disabled behavior; 125 old core tests are filtered out rather than rerun. The portable generic/no-ASM/no-IFMA check passes. The diagnostic driver links a coherent eight-library set with actual O3/native/fat-LTO/one-CGU flags. Field/math/kernel qualification is inherited from unchanged sources, not claimed as freshly executed tests or phone/ARM execution.

Preparation first stopped when an unscoped stream-hook needle matched G1 and G2; the successful source copy/module edit was retained and the missing hook scoped to G1. The first portable check then rejected a missing delimiter in the recorder. The corrected child reuses cached dependency compilation in a new receipt directory; original failed source and receipts remain. A capture-helper borrow error was repaired before execution. Its first source audit then rejected an unchanged binary member as UTF-8; the corrected reader hashes unchanged bytes and decodes only edited parents. No proof had run before these reader repairs. No successful test, check, build, proof or decoder was repeated to repair them.

After those corrections, all nine census commands and all nine Gnark commands succeed once. Counter arithmetic checks cover point = lanes×groups + tail, 1,600 bytes per full streaming group, 72 bytes per owned G1 pair, and ordinary merge pairs being a subset of unsigned streaming entries. Output proof sizes, fixture public inputs, image/VK hashes and freshness all pass. Totals are **36 SDK requests and nine fresh independently Gnark-verified exports**, advancing the canonical ledger **32,628 → 32,637**. Internal requests and copied inventories are not extra independent exports.

Offline review checks 22 successful workflow receipts, the retained failed portable receipt, 397 current capture pins, coherent libraries, raw census arithmetic and all nine proof/export bindings. It does not rerun a counter test or cryptographic verifier. No new latency, PMU/profile, AWS/Aeglos, mobile/TEE, huge-circuit, pressure/swap, minimum-RAM or concurrent-throughput result is supplied.

## Accepted reference comparison

Inherited [adoption](../2026-10-30-g2-vector-sign-adoption176/README.md) timings and user-supplied historical M5 comparisons follow. None is measured by this census.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU 10× target ms |
|---|---:|---:|---:|---:|
| 1×2 | 7.928660 | 37 | 24 | 3.7 |
| 2×2 | 11.151932 | 59 | 30 | 5.9 |
| 4×2 | 16.086386 | 109 | 48 | 10.9 |
| 8×2 | 24.179453 | 189 | 74 | 18.9 |
| 12×2 | 33.884455 | 284 | 101 | 28.4 |
| 16×2 | 40.077381 | 335 | 124 | 33.5 |
| 24×2 | 65.850777 | 528 | 176 | 52.8 |
| Merge24 | 65.060937 | 507 | 175 | 50.7 |

Latest separate primary observation is 7.991694 ms for the same accepted binary. Merge54 adoption is 161.883899 ms with no supplied M5 reference. All-eight CPU 10× targets met: **0/8**. The original objective remains active.

[Evidence](artifacts/evidence.tar.gz), [member hashes](artifacts/evidence-members.json), [checksum](artifacts/SHA256SUMS). Includes diagnostic source edits/counter/driver, declarations and corrections, actual counts, command receipts and proof-binding hashes. Targets, executable/library blobs, private continuation notes and input/key/witness/public/proof payloads are excluded; local absolute paths require reconstruction elsewhere.
