# Concurrent G2 leading components: native qualification

The new paired-component composition passes native correctness qualification. **Latency is unmeasured; production remains unchanged.** It combines the exact earlier bounded indexed-tile reducer with concurrent high/low leading-window components. The earlier reducer's failed threaded latency/resource gate remains closed.

## Change and motivation

The [earlier bounded reducer](../2026-11-08-indexed-tile-bundles176/README.md) reduced primary solo complete-query time by 10.75% and primary 32-worker sampled instructions by 8.52%, but established no 32-worker latency gain. Its primary component/partition phases contained 14/14/15/15 bundled jobs. Reduced available parallelism is a hypothesis, not an established cause.

The new composition uses one Rayon join per leading-window partition to publish the original high and low component computations together. The original component helper and ordinary-window function are called directly. The split, width 12, two partitions, scalar grouping, signed bins, existing tile pair trees, field arithmetic, endomorphism, component/partition merge order and final fold remain unchanged. It adds no point table, key/solver plan, scalar matrix or runtime pool. Each leaf still copies at most 1,024 points and does not fork while scratch is live.

This differs from the earlier sequential bundled implementation and from the [last-finisher scheduler](../2026-11-07-g2-last-finisher176/README.md), which kept high/low components sequential. It is a composition experiment, not a new arithmetic formula. It does not predict a twofold proof gain or completion of the all-eight objective.

## Admission and native evidence

Concurrent components can retain a second set of index/task/partial-output parent arrays. Before mutating scalar words, the candidate reserves the existing typed top-parent forecast per partition plus conservative merge buffers. The reservation lives through the final fold. Unsupported geometry, overflow or denied admission returns to the existing sequential schedule before scalar mutation. The exact inherited bundled reducer independently charges its job/offset metadata. There is no synthetic RAM ceiling.

The extra parent forecast is 143,208 bytes at 1,024 input rows, 263,168 bytes at 30,089 and 409,440 bytes at 65,535. The test checks monotonicity across every eligible length. These are conservative requested-capacity forecasts, **not measured allocations, process peak or minimum RAM**. Complete resource qualification remains outstanding.

Four serialized Rust/xtask commands complete on their first execution: focused native test compilation, the four new tests, a coherent release core and generic noASM/noIFMA guard compilation. The new tests check 573 complete queries against independent Arkworks generator-weight references:

- 108 raw cases across four paired/bundled combinations, including length fallbacks, identities, repeated/opposite points, zero and full-width/boundary scalars.
- 40 grouped cases across those four combinations, plus one forced-denial fallback query.
- 424 nested queries on 53 actual Rayon workers across 1/4/16/32-worker pools with 512 KiB requested stacks.

The four tests pass; 127 inherited tests are filtered. Paired-route admission and two completed leading parts are asserted in the focused cases. The length-mismatch assertion, 1025 versus 2, is deliberately caught by a passing input-preservation test. The unchanged field/dependencies and successful earlier model/field suites are reused without rebuilding or retesting. Generic compilation does not establish ARM/mobile execution, and these nested correctness cases are not a concurrent-proof throughput benchmark. The unchanged bundled child inherits its arithmetic qualification; release route counters remain necessary before timing.

Of 176 original source members, 172 remain byte-exact. Reversing four parent edits recovers all 176, including the binary fixture as bytes. Three children give 179 members. The bounded reducer child is byte-exact to its earlier qualified source. The original reducer and ordinary-window bodies remain unchanged behind reversible hooks. [Source audit](artifacts/source-audit.json), [new scheduler](artifacts/paired_top.rs), [focused tests](artifacts/tests.rs), [extra forecast](artifacts/extra-workspace.rs), [qualification review](artifacts/qualification-review.json).

The initial preparation audit tried to decode the binary fixture as UTF8; the corrected helper validated existing staged bytes and finished the missing plan. An initial plan contained unsupported guard_only metadata and was rejected before any child. The first offline reader expected build-only Cargo variables in the filtered command receipt; the corrected reader checks the runner's managed-variable projection and the complete protocol environment separately. Failed helpers/plan and repairs are retained. No successful scientific test/build/capture is replayed. [Tooling details](artifacts/tool-failures.md).

## Decision and next measurement

Native qualification permits preparing the complete-query experiment; it does not select production adoption. No timing, profile, SDK request or Gnark export is added. The next stage must observe actual release dispatch/admission and compare ORIGINAL, BUNDLE, PAIR and BOTH using complete real B queries, both controls and automatic profiles under a frozen protocol. The earlier negative result and separate controls must remain visible. A successful query screen would still require verified ALL8+Merge54 SDK, independent Gnark, resource/admission, portable and concurrency qualification. [Design](artifacts/design.md), [continuation loop](artifacts/next.md).

## Accepted CPU and Metal comparisons, inherited

These full-proof observations use fixed worker placement, not an adopted production default. They are not measurements of this composition. Historical standard 1×2 **7.915095 ms** remains valid; later standard **8.075143 ms** and fixed **7.869302 ms** retain their own conditions. No accepted code regression or new proof record is established.

|Shape|Accepted fixed ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.869302|37 /4.702×|24 /3.050×|3.7|
|2×2|10.984868|59 /5.371×|30 /2.731×|5.9|
|4×2|15.734200|109 /6.928×|48 /3.051×|10.9|
|8×2|23.929023|189 /7.898×|74 /3.092×|18.9|
|12×2|33.722646|284 /8.422×|101 /2.995×|28.4|
|16×2|39.776845|335 /8.422×|124 /3.117×|33.5|
|24×2|66.183856|528 /7.978×|176 /2.659×|52.8|
|Merge24×1|65.448733|507 /7.747×|175 /2.674×|50.7|

Source: [preceding accepted comparison](../2026-11-09-forward-phi-query176/README.md). Supplied M5 references use different hardware. Merge54 remains 162.489760 ms without an M5 reference. Production176/SDK6d83 and the 38,668-proof ledger remain unchanged. No AWS/Aeglos, device, huge-circuit, pressure/swap/minimum-RAM or throughput measurement/resource is added. The verified all-eight CPU10× goal remains active and unmet, **0/8**.

Private root:`/home/ubuntu/src/zopro-perf-work/paired-top-bundles176`. [Build plan](artifacts/build-plan.json), [helpers and exact native receipts](artifacts/evidence.json.gz), [publication checks](artifacts/publication.json), [checksums](SHA256SUMS). The lossless UTF8 archive includes changed children, edits, pins and receipts; executable/library blobs, unchanged full source, fixture/key/witness/proof bytes and private proof inventory are excluded. No performance capture is live.
