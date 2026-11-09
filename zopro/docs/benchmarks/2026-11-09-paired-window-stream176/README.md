# Paired-window streaming work screen

The fixed paired-window recipe is closed before native implementation. It reduces modeled stream-plus-merge inversion stages by 52.09%, but projective arrivals rise 2.08×, normalization buckets rise 3.27× and bucket-coordinate storage doubles. Two of four prospectively declared necessary checks fail. No timing or production change is selected.

The proposed organization pairs adjacent original balanced K/H windows and splits each original input slice in two, retaining twenty coarse fill actors. Each actor interleaves the two digits of an input point into disjoint bucket ranges with one shared ready batch and the original newest-first collision queue. Ready capacity is the sum of the original batch sizes: 552 or 704. Two part outputs merge per window through the original 512-bucket organization; weighted reduction and Horner assembly retain the original twenty outputs. No point table, scalar matrix, endomorphism, index sorting, key copy or new pool is proposed.

This differs from the rejected [serialized weighted consumer](../2026-11-09-weighted-double-bench176/README.md), which changed the weighted finish. It also differs from the [eight-window color screen](../2026-11-08-g2-eight-window-colors176/README.md): SIMD lanes accept dynamically ready additions rather than fixed window identities; there is no per-bucket color cap or masked inactive-lane schedule. The present experiment is a generic state/count model, not either native design.

|Modeled primary work|Original saved observation|Paired windows + two parts|
|---|---:|---:|
|Nonzero/nonidentity signed arrivals|1,170,733|1,170,733|
|First affine assignments|70,681|141,243|
|Stream affine updates|1,088,726|1,005,879|
|Additional generic part-merge pairs|0|70,562|
|Stream inverse stages|3,713|1,640|
|Additional merge inverse stages|0|139|
|Stream + merge inverse stages|3,713|1,779|
|Projective arrivals|11,326|23,611|
|Projective initializations / normalization buckets|4,415|14,437|
|Adds to existing projective accumulators|6,911|9,174|
|Queue arrivals|68,404|72,375|
|Queue-overflow flushes|52|52|
|Stream + merge scalar-tail pairs|78|537|
|Generic mathematical joins including projective arrivals|1,100,052|1,100,052|
|Logical G1 bucket-coordinate bytes|4,587,520|9,175,040|

Inverse stages count generic streamed and merge batches. They exclude projective normalization service, unchanged weighted-tree inversions and arithmetic inside the actual inverse kernel. The candidate reduces ordinary affine updates including merges by 1.13%, while moving more work through projective fallback and normalization. A mathematical join does not have the same execution cost in these paths. Scalar tails, packing, sign handling, coordinate conversion, allocations, scheduling and cache traffic remain unpriced. There is no measured slowdown or speedup.

The extra 4.375 MiB covers bucket coordinates alone. Busy maps, projective index/point arrays, ready/queue records, five-field packed scratch, normalization, merge arrays, suspended Rayon parents, allocator capacity and stacks remain additional costs. This is proof-local state, not a retained key lookup table, and is neither a complete native resource forecast nor RSS/minimum RAM. A native implementation would require admission and original fallback before construction; none is selected here.

The fixed necessary checks require preserved generic join totals, fewer stream-plus-merge inverse stages, no increase in projective arrivals and no increase in normalization buckets. The first two pass; the last two fail. All unfavorable results remain visible. No changed pair count, input partition count, batch/queue size, favorable subset or unchanged replay is selected. This closes this fixed organization, not every cross-window algorithm.

The model assumes generic noncancelling, nonexceptional occupancy. Signed integer-token outputs conserve across every actor, part merge and weighted window; they do not certify curve identities, equal-X/doubling/opposite-point behavior or proof correctness. The retained cache has 30,529 K rows and the exact 30,922-row used H prefix; its domain geometry is 32,768. It is not silently padded or re-extracted. The twenty spans use floor(i×255/20), with five 12-bit spans interspersed among fifteen 13-bit spans.

Six serialized Rust/xtask commands pass on first execution: three builds, two focused tests in one test process, one fixed census and one separate reader. New tests cover 60 signed collision/queue/reuse cases and nine part/window-merge cases. The reader independently checks 1,229,020 digits with a bit-based formula, weighted token sums, all twenty actor and merge histograms, conservation, four decisions and prior command receipts. Terminal publication checks all six receipts, including the reader's own exit and output hashes. Nineteen source/input/tool pins and all 176 accepted production-source hashes are checked. Prior solver/FFT extraction, successful old models and field/core/prover suites are inherited without replay.

No new SDK requests, Gnark exports, native curve calls, latency observations or CPU profiles are added. The canonical proof inventory remains 41,104. The next change must reduce useful streamed work or dependencies without trading the saving into expensive fallback and larger live state. The all-eight CPU 10× objective remains active and unmet.

The following full-proof comparisons are inherited from the last unchanged accepted SDK campaign, not measured by this model.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.832708|37|24|4.724×|3.064×|3.7|
|2x2|11.065989|59|30|5.332×|2.711×|5.9|
|4x2|15.913588|109|48|6.849×|3.016×|10.9|
|8x2|24.254933|189|74|7.792×|3.051×|18.9|
|12x2|34.007120|284|101|8.351×|2.970×|28.4|
|16x2|40.086324|335|124|8.357×|3.093×|33.5|
|24x2|66.467587|528|176|7.944×|2.648×|52.8|
|merge-24x1|65.569872|507|175|7.732×|2.669×|50.7|

Accepted merge54 is 162.947719 ms without a supplied M5 reference. No fresh AWS/Aeglos, mobile/ARM, huge-circuit, pressure/swap, minimum-RAM or concurrent-proof comparison follows.

[Design](artifacts/design.md), [counts](artifacts/analysis.json), [review](artifacts/review-result.json), [decision](artifacts/decision.json), [six command receipts](artifacts/receipts.json), [continuation](artifacts/next.md), [source archive](sources.tar.gz), [source index](artifacts/source-index.json), [checksums](SHA256SUMS). Private arbitrary-token sums are represented by hashes; all work counts and decisions remain. Raw cache, scalar/point/key/witness/proof data, full ledger and executables remain local. Reproduction requires the pinned private cache and inherited saved original counts.
