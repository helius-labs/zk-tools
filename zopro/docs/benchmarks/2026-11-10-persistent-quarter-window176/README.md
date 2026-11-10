# Persistent quarter-scaled complete G1/G2 Windows

**The integrated Windows pass correctness and allocation checks; latency has not been measured.** The accepted prover is unchanged at **7.783594 ms** for local 32-worker 1×2 full proofs. This stage makes the candidate ready for a complete-caller timing comparison, without establishing a speedup.

The [qualified field representation](../2026-11-10-persistent-quarter176/README.md) now persists across actual borrowed G1 and two-feed Phi G2 Windows. Keys retain their original format. Queues, busy bitmaps, dense reset, projective spills, batch sizes and final normalization retain their original behavior. Private buckets remain 64/128 bytes for G1/G2, with canonical X and opaque quarter-scaled Y below `2p`. There is no converted key, lookup table, second solver plan or new production pool.

The new kernels retain long-prefix inverse amortization. G1 records occupy 1,600 bytes per eight points; G2 prefix/norm records occupy 640 bytes, matching the existing active layouts. Scalar tails convert coordinates into the same scaled domain before joining the vector prefix inverse; they convert their outputs back into canonical X and opaque Y. Incoming-Y conversion, equal-X handling, projective merges and final Y decoding are real caller work. The previous source estimate of unchanged IFMA product counts still applies; simpler representation boundaries do not guarantee faster execution.

|New qualification|Coverage|
|---|---:|
|Long-prefix G1/G2 updates|21,504 cases / 1,695,744 point comparisons|
|Complete original/candidate Windows|4,752 calls / 5,018,112 bucket comparisons|
|Every-worker and shared-pool Windows|220 calls, 15,045-point inputs|
|Terminal conversion allocation cases|24|
|Generic fallback|Same-source no-ASM/no-IFMA field and core compile|

Independent Arkworks arithmetic checks prefix results and complete bucket sums. The Window oracle uses its own single-bit Booth decoder and explicit Phi map. Prefix lengths include 0–33 boundaries and 199/200/201/352, all 256 repeating sign masks, three changing updates and deliberately lifted Y representatives. Window populations include ordinary, identity/duplicate/opposite and hot-bucket inputs, widths 7/12, columns 0/1/9 and batch sizes 8/9/31/32/33/200. Original points and scalar descriptors remain unchanged. The threaded test broadcasts twice on every worker at 1/16/32 Rayon threads with requested 512 KiB stacks, then runs twelve simultaneous callers through one eight-worker pool.

The independent native tests use an assertions-enabled field/core build. Separate native release field/core libraries are retained for timing. The allocation driver links those release libraries with fat LTO, checks original/candidate outputs, and delegates allocation to the production allocator. All 24 terminal `map/collect` cases preserve pointer and capacity with zero allocations, reallocations or deallocations. This is evidence for the pinned Rust 1.98.1 native build, not a portable `Vec` allocation guarantee.

Complete Window allocation observations below include construction, feeds, final normalization, output comparison and drop, after configuration priming. Both groups use 15,045 points, width 12, column 0, batch 200.

|Group / arm|Allocations|Reallocations|Peak requested live payload bytes|Requested bytes including reallocations|
|---|---:|---:|---:|---:|
|G1 OFF|19|5|191,360|193,255|
|G1 ON|15|5|191,360|193,251|
|G2 OFF|21|5|303,360|307,049|
|G2 ON|17|5|303,360|307,045|

Tracked live payload returns to zero after each call. Peak requested payload is identical between arms; the small allocation-count difference establishes no peak-memory saving. These observations exclude allocator overhead, retained input/key storage, stack and process mappings. They are not RSS, a universal memory forecast, minimum RAM, pressure or swap results. Eventual production admission must charge all live storage, including any final-conversion overlap on other platforms.

The 181-file private snapshot has 171 byte-exact originals, five append-only declarations/diagnostic helpers that recover all 176 accepted sources, and five new children. The field child retains the exact prior qualified prefix with only host-domain accessors appended. Original Window derivations are replayed from recorded edits. Production query selection and resource admission are unchanged; the candidate is an explicit private probe path. A separate same-agent reader validates sources, receipts, test totals and allocation rows without replaying numerical tests.

Across preparation and review, **18 workflow commands comprise 15 successes and three retained failures**. The first core test compile needed module-comment placement, a concrete `BatchAdd` type and componentwise Arkworks Phi multiplication. The allocation driver compile then needed a shadowed counter function renamed. The first offline reader incorrectly read a binary source asset as UTF-8; its repair hashes source bytes. These repairs changed no numerical formula or test population. Successful field/core builds and scientific qualifications were reused. The final reader binds 410 pins. Full failure sources, commands, streams and repair helpers remain in the archive.

No timing, PMU/profile, SDK request, fresh proof, AWS/Aeglos, phone/ARM, giant-circuit or concurrent-throughput result is added. Canonical independently verified proof inventory remains 44,265. Next is a frozen matched complete-Window comparison with automatic profiling, followed by all-shape SDK/Gnark qualification if the actual caller improves.

Accepted local 32-worker full-proof observations and supplied M5 references below are inherited from the [latest full SDK campaign](../2026-11-10-frontend-overlap176/README.md).

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.783594|37 / 4.754×|24 / 3.083×|3.7|
|2×2|10.932731|59 / 5.397×|30 / 2.744×|5.9|
|4×2|15.757088|109 / 6.918×|48 / 3.046×|10.9|
|8×2|23.973295|189 / 7.884×|74 / 3.087×|18.9|
|12×2|33.827269|284 / 8.396×|101 / 2.986×|28.4|
|16×2|39.900917|335 / 8.396×|124 / 3.108×|33.5|
|24×2|66.041083|528 / 7.995×|176 / 2.665×|52.8|
|merge-24×1|65.717994|507 / 7.715×|175 / 2.663×|50.7|

The full CPU 10× goal remains active and unmet, **0/8**. Accepted primary16 remains 9.016864 ms; merge54 remains 162.170158 ms without a supplied M5 reference. Historical primary full-proof RSS observations of 111.25 MiB and 133.50 MiB belong to different campaigns and are not remeasured here.

[Protocol](artifacts/design.md), [qualification](artifacts/qualification.json), [allocation rows](artifacts/allocations.json), [source audit](artifacts/source-audit.json), [review](artifacts/review.json), [command index](artifacts/receipts.json), [preserved failures](artifacts/failures.md), [continuation and autoresearch loop](artifacts/next.md), [source/receipt archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Exact new children and full commands are in the archive. Binaries/libraries, unchanged source snapshots, private key/witness/proof payloads and full ledgers are excluded.
