# Tree-operation inverse-policy ownership

The private implementation passes focused native correctness and configuration-lifetime checks. Each nonempty grouped, indexed or weighted tree reads the inverse-policy option once and supplies the captured boolean to its child scratch buffers. **Performance is unmeasured; production is unchanged.**

The [accepted configuration census](../2026-11-08-config-lookup-census176/README.md) observed 1,101 TREE_VECTOR_INVERSE reads per primary32 proof and 675 solo. Unset reads allocated nothing. That result does not measure their CPU cost or prove environment-lock contention; this experiment makes no latency-saving estimate. Its purpose is to remove repeated option work without changing arithmetic or parallel task geometry.

Three operation owners change: group_affine_impl, reduce_indexed_buckets and weighted_affine. The original Scratch::default body remains byte-exact. A native-only private constructor initializes the same vectors, arrays and defaults, with its inverse boolean supplied by the owner. Empty operations perform no new lookup. Generic/noIFMA paths retain the original leaf constructor. Changes between operation invocations apply; children of one operation intentionally share its entry snapshot. There is no global/TLS cache, retained key state, point copy/table, extra pool or artificial memory limit.

The five reversible parent edits add three factories and replace their three scratch sites; the common two-site replacement accounts for two occurrences. Removing the appended factory and benchmark API recovers all 176 accepted files. Of 177 private source entries, 174 originals are exact, two parents are reversible and one child is new. Field arithmetic, point formulas, queue/spill rules, split/windows, iterator grains, allocator and resource forecasts retain their original behavior. The source adds a captured boolean, not a heap buffer. Compiler stack use, process RSS and full-proof resource behavior remain unmeasured.

One focused native test passes, with 125 existing core tests filtered and inherited. Its coverage is:

- Six option-parsing cases: unset, zero, empty, one, another UTF8 value and a non-UTF8 value. A factory retains its initial choice across a later option change; the next operation sees the new value. Empty owners make no read.
- Complete G1/G2 weighted, grouped and indexed outputs under both inverse policies, including zero/equal/opposite points, cancellations, empty groups, odd group lengths and scalar tails. Weighted leaf lengths satisfy the original power-of-two precondition. Generator-coefficient references use unchanged scalar/curve arithmetic; these are not fresh independent Arkworks or Gnark checks.
- All 53 actual workers across 1/4/16/32-worker pools requesting 512 KiB stacks execute 636 complete entry calls, counted once. Twelve additional concurrent G1 runs share one eight-worker pool and execute three entries each. These are correctness calls, not proof-throughput measurements.

The generic noASM/noIFMA build passes with the experimental cfg present but its native hook excluded. This is a compile guard, not ARM/mobile execution. OFF and ON optimized cores use the exact accepted field1200 and native O3, one codegen unit and embedded bitcode. Release symbol checks find neither the test-only read counter nor the test function. Actual benchmark executables have not yet been linked or timed.

The initial focused test supplied 65 G2 weighted buckets, outside weighted_sum's existing power-of-two leaf precondition. It failed against the generator reference. Only that cfg(test) fixture length changes to 64; the failed source, test binary and logs are retained. The corrected test is rebuilt and rerun. Both successful release libraries are reused because their non-test source prefix is byte-exact. A separate preparation review also corrected remaining generic-output/dependency paths inherited from a donor plan before any build; no historical artifact was overwritten.

In total, six successful scientific commands comprise the original OFF/ON/test builds and corrected test build/run plus the previously unstarted generic guard. The one failed test remains visible. The separate reader validates 383 final pins, the older pins against the saved test-only source, all source recovery, command/stream receipts, native flags and release symbols. The publisher separately checks both failed streams. Helper/reviewer/publication tool calls are not additional xtask receipts.

Next is a prospectively frozen matched complete-consumer comparison using the existing independent-Arkworks weighted fixtures, accepted REF and these new OFF/ON libraries. A positive component result would still need coherent full-SDK all-eight plus merge54, independent verification, resource/admission, portability and concurrency qualification. No production adoption or twofold improvement follows from the current tests.

Zero new SDK requests or Gnark exports are added; canonical proof count remains 41,104. The accepted full-proof comparisons below are inherited from the latest unchanged SDK campaign. All eight CPU10× targets remain unmet.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.832708|37|24|4.724×|3.064×|3.7|
|2x2|11.065989|59|30|5.332×|2.711×|5.9|
|4x2|15.913588|109|48|6.849×|3.016×|10.9|
|8x2|24.254933|189|74|7.792×|3.051×|18.9|
|12x2|34.007120|284|101|8.351×|2.970×|28.4|
|16x2|40.086324|335|124|8.357×|3.093×|33.5|
|24x2|66.467587|528|176|7.944×|2.648×|52.8|
|merge-24x1|65.569872|507|175|7.732×|2.669×|50.7|

Accepted merge54 remains 162.947719 ms without a supplied M5 reference. No fresh AWS/Aeglos, huge-circuit, phone/ARM, pressure/swap/minimum-RAM or throughput result is added.

[Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [review](artifacts/review-result.json), [release identities](artifacts/identities.json), [test repair](artifacts/test-repair.json), [continuation](artifacts/next.md), [sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Executables/libraries, unchanged accepted source copies, keys/inputs/point/scalar/witness/proof data and the full ledger remain local.
