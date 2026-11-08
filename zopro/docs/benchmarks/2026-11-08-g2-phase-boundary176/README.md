# Whole-loop G2 phase boundaries: native qualification

The isolated candidate passes **127 unfiltered core tests in each arm**, including 96,048 new generator comparisons and 10,812 nested comparisons per arm. All 53 actual workers across the tested pools execute the nested test. **No performance improvement is measured or claimed.** Production and the accepted SDK remain unchanged.

The candidate separates the complete forward prefix loop and complete reverse update loop of borrowed G2 addition into two never-inline helpers. The scalar-tail/inversion bridge stays in the dispatcher. This tests a whole-phase code-generation and lifetime boundary, with two transfers per batch rather than a helper call per field operation. A tail jump may replace the last call. The existing arithmetic, prefix recurrence, signed handling, point order, records, allocator and scheduler are retained. No lookup table, key/point copy, retained plan, larger record, new heap buffer or pool is introduced. Call ABI and returned SIMD state may still add stack copies, code or overhead.

## Why this experiment

The saved [accepted instruction census](../2026-11-06-primary-instruction-census176/README.md) identifies about 52 KiB of linked code in borrowed G2. The latest [complete-proof profiles](../2026-11-08-tree-policy-sdk176/README.md) still put the main G1 stream and G2 borrowed processing among the hottest functions. Another identical address census is unnecessary: the earlier census already covers this same accepted executable. Neither code size nor these sample shares establish an instruction-cache bottleneck, removable wall-time fraction or a twofold speed forecast.

Prior G1 scratch-policy specialization, [G2 cold-tail extraction](../2026-10-30-g2-cold-tail175/README.md), [per-fused-product outlining](../2026-10-29-g2-fused-outline175/README.md), [descriptor chunks](../2026-11-06-g2-descriptor-chunks176/README.md) and [frontend counters](../2026-10-26-front-end-counters/README.md) remain negative evidence. This new boundary neither specializes the scratch policy nor outlines individual products or only a cold fallback. No old runtime trial is repeated, and no previous speed estimate is imported. The next timing must determine whether this distinct boundary helps.

The paper search rediscovered mechanisms already audited locally: [truncated SIMD Montgomery multiplication](https://arxiv.org/html/2410.18129v1) and [multi-limb matrix multiplication](https://www.texmacs.org/joris/amx/amx-v2.html), among others. Current reading covers search results and introductory author HTML, not a new full-paper replication. No external code or paper speedup is transferred.

## Source and correctness

The private snapshot contains 177 files: 175 accepted originals are byte-exact, one parent reverses exactly, and one child is added. All field sources and the official generator are unchanged. The original native field library `1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713` and previously qualified generic field library are reused. No unchanged field build or field test is replayed.

The release child retains the original forward-loop statements, scalar-tail/inversion bridge and reverse-loop statements. Explicit source adjustments are an unused outer `mut`, arguments for full-length/inverse/record state, and two `s.prefix_records[j]` expressions becoming `records[j]`. Independent reconstruction checks the complete copied fragments and reverses the parent dispatch. OFF retains its original operation; ON selects the new child. Both test builds directly compare the new helper implementation with the exact original body and generator-derived sums. Existing signed-prefix/vector-sign tests also exercise ON through its parent entry.

The new test covers 20 empty/full/tail sizes through 511 points, both canonical modes, scalar/vector inversion, three sign patterns, three repeated updates, input immutability, unchanged destinations, and equal retained record/tail capacities. Its ordinary batch inputs satisfy nonzero denominators and distinct destinations. The independent generator construction uses unchanged scalar multiplication and field arithmetic; it is not Gnark verification or a formal proof over all inputs.

A second test broadcasts to every actual worker in 1/4/16/32-worker pools, requesting 512 KiB stacks. Each worker invokes nested Rayon work for both canonical and inversion modes with a 17-point tail case. The 53 visits and 10,812 comparisons are repeated correctness observations, not distinct proofs, independent performance samples or minimum-stack guarantees.

All seven fresh native workflow commands pass: five core/test-consumer/generic compiles and two unfiltered test runs, for 254 total test executions. The selector-enabled generic core compiles with no ASM/IFMA; its unchanged native guard excludes the new implementation. This is not ARM, phone or full portable-prover execution. New test builds use native optimization, one codegen unit and fat LTO with the inherited core build profile. Compiler warnings remain in the archived stderr.

An initial preparer assertion expected one mutable inverse binding in the original bridge; it has two. Preparation stops after copying the 176 unchanged originals, before writing transformed code or running any build/test. The corrected preparer changes only that expected count; its `replacen` still removes only the unused outer mutable binding. Initial source/executable and the unchanged snapshot are retained locally; source identities and the repair are checked. There was no runtime arithmetic or test-population fix, and no successful test or capture is replayed. The initial panic appears in the tool transcript rather than a separate workflow stderr receipt. Helper compiler invocations also lack complete workflow receipts.

The qualification reader checks source pins, all seven command argv/exits/stream hashes, parent recovery, all unchanged source hashes, exact loop fragments, test populations, comparison/worker markers and the preparer-only repair. It shares the field and original operation with the producer and does not independently model compiler transformations.

## Required performance follow-up

Actual linked phase boundaries and a coherent complete-caller consumer still need inspection. Then freeze full G1/G2 controls, sizes, worker cohorts, tails, output/resource conditions, calibration/nonregression/gain rules and automatic PMU/profile quality before any preflight. Retain every observation. Only a qualified caller win permits standard-SDK all-eight plus merge54/primary16 and independent Gnark, generator, portable, resource and concurrency qualification before adoption. No favorable subset or unchanged reroll is selected here.

This study adds zero SDK requests or Gnark exports; the canonical inventory remains 37,023. Model/point/test counts are not proofs. No peak-RSS, minimum-RAM, memory-pressure, swap, giant-circuit, mobile/ARM/TEE or concurrent-throughput claim follows from the fixed stack tests or unchanged heap layouts. No AWS resource or Aeglos benchmark is added.

## Accepted full-proof comparisons, inherited

These current local observations come from the [preceding full SDK study](../2026-11-08-tree-policy-sdk176/README.md), not this native qualification. The supplied M5 timings are historical measurements on different hardware. Accepted SDK6d83 remains unchanged.

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× limit ms|
|---|---:|---:|---:|---:|
|1x2|8.040649|37 / 4.602×|24 / 2.985×|3.7|
|2x2|11.149097|59 / 5.292×|30 / 2.691×|5.9|
|4x2|16.069415|109 / 6.783×|48 / 2.987×|10.9|
|8x2|24.131178|189 / 7.832×|74 / 3.067×|18.9|
|12x2|33.940628|284 / 8.368×|101 / 2.976×|28.4|
|16x2|40.164083|335 / 8.341×|124 / 3.087×|33.5|
|24x2|66.187704|528 / 7.977×|176 / 2.659×|52.8|
|merge-24x1|65.361571|507 / 7.757×|175 / 2.677×|50.7|

Merge54 remains 160.890372 ms without a supplied M5 reference. Historical accepted 1×2 at 7.915095 ms and private candidates at 7.859756/7.869400 ms remain valid. The latest accepted 8.040649 ms is a different observation of the same executable, with variation cause unconfirmed and no slower replacement adopted. The full verified CPU AVX 10× goal remains active and unmet on all eight shapes.

[Qualification](artifacts/qualification.json), [source reconstruction](artifacts/source-audit.json), [artifact identities](artifacts/identities.json), [source and complete receipts](artifacts/evidence.json.gz), [checksums](SHA256SUMS).
