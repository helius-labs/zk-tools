# H-only native unweighted G1 correctness screen

The cancelled-slope three-point formula now has a native IFMA implementation. Its focused correctness tests, independent Arkworks checks and small-stack worker tests pass. **No latency measurement or production change is included.** Accepted local 1×2 remains **7.928660 ms**; the CPU 10× target remains unmet for all eight shapes.

## Change

The [previous algebra screen](../2026-11-06-triple-h-cancel176/README.md) eliminates the first slope and uses H rather than dH as the shared denominator. This implementation retains just preceding-prefix/H for each eight-lane group and rereads original coordinates in the reverse pass. Pair remainders share the same inverse chain and also use two-vector records. The previous triple record used eight vectors. This record-size change does not establish a total-memory or speed improvement.

All regions are checked for matching lengths, bounds and disjoint order before use. Identity inputs and a zero product reject the layer before any point/count writes. Ordinary and exceptional input continue through the unchanged complete binary reducer as needed. Reverse outputs stay in their original disjoint first-input slots; forward compaction happens only after all original coordinates have been consumed. The copied checked-inverse method is existing prior art, not a new optimization. Weighted binary reductions are unchanged.

The candidate snapshot has 178 files; 173 accepted files are byte-identical and the remaining three recover exactly by removing cfg-only appendices. Both new modules are isolated behind the experiment cfg. All 176 production files remain exact. The accepted field library and coherent support libraries were reused without rebuild. The original complete-caller fixture, order, Work/reduction loop and external driver are byte-exact. No lookup table, key copy, extra production pool or field rewrite is added.

## Validation

The six-command Rust/xtask workflow completed on its first attempt. Five new focused native tests pass; 125 original core tests were filtered and are inherited, not reported as freshly run. The independent Arkworks complete-reduction test was executed against the changed native kernel. It covers full-width scalar arrays, empty/multiple groups, tails and 216 boundary triples, both selection paths and fresh/reused buffer lifetimes.

The focused tests check 48 partial-lane/remainder layouts, all 255 nonempty zero-lane masks plus a nonzero inverse case, 72 identity/equality/cancellation/pair rejections with unchanged inputs/counts, eight admitted opposite-first-pair lanes, and seven malformed geometries rejected before mutation. Separate 1/4/16/32 test workers each use 512 KiB stacks, repeatedly grow/shrink private scratch, and compare complete reductions and forced fallback outputs with complete projective group arithmetic. No unsafe blocks were added.

These test workers validate independent reduction scratch and stack behavior. They are not concurrent-proof throughput measurements or a general resource bound.

## Memory and emitted code

The representative grow/shrink sequence retained the same capacity vector in every tested worker:

| Capacity category | Bytes per worker |
|---|---:|
|Common points/counts/output|139816|
|Binary fallback scratch|211392|
|Ternary indices and both record buffers|61168|
|Fixed Work struct|760|
|Total reported capacity|413136|

This is about 403.45 KiB per worker, including binary fallback scratch that coexists with about 59.73 KiB of ternary scratch. It is **not peak RSS, minimum RAM, total process memory or a universal bound**. No equivalent fresh measurement of the old eight-vector implementation was added; this screen makes no total-memory comparison. Future runtime routing must use existing resource admission.

Core and final-consumer commands explicitly use O3/native/oneCGU; final test/caller consumers use fatLTO. The inspected final caller is `24d98263b24cc861f4c4dbe3c80b4070289ab2797f525b1aec207d1f78f2dc86`. Its H-only layer contains native IFMA operations and subtracts 4,936 bytes from RSP, with 48 bytes of pushed registers. This prologue is not whole-thread stack usage or a dynamic spill cost. The saved disassembly was inspected without redecoding.

## Next and limits

The next stage is a separately declared complete-caller timing comparison against matching original FROZEN and fresh binary OFF controls. Preserve Tile384/Tile511/Tile2048/Group35973 as main complete cases and boundary/mixed-tail diagnostics. Freeze identities, worker contexts, exact round/block/order plan and all calibration/nonregression/gain/resource gates before capture. No favorable subset, layout/attribute/threshold sweep or extra blocks are selected from these correctness results.

Fewer shared inversions may help; coordinate rereads, extra linear operations, scratch, stack and fallback coexistence can offset that saving. Only credible complete-caller gain selects a separate verified SDK trial covering all eight shapes, merge54, physical16, portable execution, concurrency and resources. Passing these tests does not permit production adoption.

The [old dH complete-caller result](../2026-10-28-unweighted-triple-caller174/README.md) and [old mixed SDK result](../2026-10-28-mixed-sdk/README.md) remain negative. The new denominator/formula changes arithmetic; it does not rewrite those failed gates. No timing/PMU/profile, SDK/Gnark proof, current AWS/Aeglos comparison, huge-circuit, phone/ARM/TEE, pressure/swap, process peak/minimum RAM or new 2× path is established. The canonical proof ledger remains 30,846.

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references follow. They are not new measurements.

|Shape|Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge24|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Merge54 remains 161.883899 ms without an M5 reference. Primary RSS132.75 MiB is inherited. Source176/SDK6d83 and the 30,846-proof ledger are unchanged. The all-eight CPU10× objective remains active/unmet, **0/8**. No new AWS/Aeglos/Gnark, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput result is added.




[Review](artifacts/review.json), [design](artifacts/design.md), [source identity manifest](artifacts/source-manifest.json), [native prefix](artifacts/prefix.rs), [focused tests](artifacts/tests.rs), [unchanged external driver](artifacts/probe.rs), [codegen review](artifacts/codegen.json), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [continuation](artifacts/next-timing.md), [five exact source deltas](artifacts/source-deltas.json.gz), [exact helpers/receipts](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS). Unchanged source files are bound by accepted hashes; full snapshot and binaries stay local. No private input, witness, key/image/proof bytes or executable/library blobs are included.
