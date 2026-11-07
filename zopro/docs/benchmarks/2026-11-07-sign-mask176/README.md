# Borrowed descriptor sign-mask extraction

**Native correctness and code-generation screen passes; no speedup measured or adopted.** The candidate reduces the valid-length mask wrapper from 19 to 11 instruction sites and 110 to 74 bytes. Both original and candidate already use two masked vector loads and a permutation. The change replaces nine shift/AND/horizontal-reduction/extract instructions with one `vmovmskps`. Static sites are not dynamic cycles or a full-proof improvement.

The original function assembles eight sign bits from encoded indices. For 16-byte descriptors, the new Rust-intrinsic helper loads only the initialized 32-bit index fields, permutes them and extracts their sign mask. It computes the first field address directly; it does not assume a tuple field offset. Other descriptor strides use the exact original function; unsigned calls return zero. Existing field arithmetic, point types, signs, bucket representation and query selection are unchanged. There is no new table, point copy, heap buffer, plan, pool, assembly source or production edit.

|Check|Result|
|---|---:|
|Borrowed G1/G2 configurations:256 masks × 8 starting offsets × 2 curves|4,096|
|Owned descriptor fallback configurations|512|
|Protected-page buffer-boundary configurations|512|
|Additional opaque-wrapper comparisons|1,024|
|Original / candidate fast-path sites, including return|19 /11|
|Original / candidate wrapper bytes, including cold bounds handling|110 /74|

All checks pass against independently chosen mask values and the original function. Inputs and point-reference identities remain unchanged. Borrowed descriptors have stride 16 and index offset 0; owned G1/G2 strides are 72/136. The protected page follows the 128-byte borrowed array, but with index offset 0 neither nominal vector load crosses the boundary. This is a boundary correctness check, not a demonstration of masked-off out-of-bounds fault suppression or a proof about uninitialized padding. The safety argument is the selected 32-bit fields: mask 0x1111 reads byte offsets 0/16/32/48 and 64/80/96/112. Both emitted loads preserve masking; no padding field is read in the wrapper fast paths. [Rust intrinsic documentation](https://doc.rust-lang.org/core/arch/x86_64/fn._mm512_maskz_loadu_epi32.html) describes the unaligned zero-masked operation.

The native consumer uses O3/native/fatLTO/oneCGU and the unchanged accepted field library 1200e635… for actual G1/G2 types. G1 and G2 wrappers alias the same body within each arm: four symbols, two distinct implementations. Raw `nm` and complete new-consumer disassembly are retained. This wrapper does not establish the instruction selection or register pressure when inlined into complete field-heavy callers.

One initial harness compile failed because the extracted source expected its native `LANES=8` constant and an integer range needed a `usize` annotation. The corrected harness changes only those two context/type details; original function, candidate and populations remain exact. Four subsequent workflow commands build, qualify, inventory and decode once. No original field/core suite, old model, native recording or verifier is rerun. No field/core library is rebuilt, and no timing or profiling occurs.

A targeted history audit distinguishes this mask-collection change from the adopted G2 signed-coordinate arithmetic and its correctness suites. The research scan also rediscovered [SRE](https://arxiv.org/abs/2610.27452), already covered by previous local scalar-reuse experiments. [gECC](https://arxiv.org/abs/2501.03245) targets GPU batch arithmetic and supplies no matched CPU speedup here. No external implementation was copied or executed.

Next, qualify the helper inside complete G1/G2 callers, including native full/tail/reuse/all-mask/threaded behavior, unchanged scratch/allocation/input ownership and portable fallback. Freeze matched latency and automatic PMU/profile protocols before recording. A passing helper wrapper alone does not justify adoption, a 2× projection or a full-proof claim. Prior lazy-Y, packed-record, late-coordinate, queue and arithmetic negatives remain closed.

Accepted production 176/SDK 6d83 remains unchanged. The latest matched primary observation is 8.027583 ms for the same binary previously measured below 8 ms. The following all-shape adoption results and supplied M5 references are inherited, not remeasured here.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10x limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 /4.667×|24 /3.027×|3.7|
|2×2|11.151932|59 /5.291×|30 /2.690×|5.9|
|4×2|16.086386|109 /6.776×|48 /2.984×|10.9|
|8×2|24.179453|189 /7.817×|74 /3.060×|18.9|
|12×2|33.884455|284 /8.381×|101 /2.981×|28.4|
|16×2|40.077381|335 /8.359×|124 /3.094×|33.5|
|24×2|65.850777|528 /8.018×|176 /2.673×|52.8|
|merge24×1|65.060937|507 /7.793×|175 /2.690×|50.7|

Merge54 remains 161.883899 ms without a supplied M5 reference. The original all-eight verified CPU 10× goal remains active and unmet, 0/8. No new SDK/Gnark/AWS/Aeglos, huge-circuit, mobile, pressure, peak-RAM or throughput result is established. The independent-proof ledger remains 32,857.

[Design](artifacts/design.md), [qualification](artifacts/qualification.json), [code generation](artifacts/codegen.json), [review](artifacts/review.json), [source and receipts](artifacts/evidence.tar.gz), [checksums](SHA256SUMS).
