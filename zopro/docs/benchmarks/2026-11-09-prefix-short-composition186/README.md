# Scaled G2 prefixes with the short forward map

The combined implementation passes native correctness qualification: **167 complete G2 queries match Arkworks**, including nested calls on 53 actual Rayon workers. Native field/core and generic guard builds pass. **Latency and peak memory are unmeasured; this is not adopted.** The all-eight CPU 10× objective remains unmet.

The implementation combines the [clean default scaled-prefix arithmetic and typed uncarried X sum](../2026-11-08-uniform-carry-default-performance183/README.md) with the [qualified short forward endomorphism multiplier](../2026-11-09-negative-phi-short176/README.md). It replaces three affine transforms: leading-window, owned-window and borrowed-window. Prefix arithmetic exports canonical point coordinates before the map; projective transforms, scalar splitting, window geometry, ordering, queues, tails, grouping and admission remain unchanged. It adds no heap buffer, retained point/key table or worker pool. The short map uses fixed-size SIMD packing temporaries; its 64 logical bytes of constants are inherited. Source inspection does not establish process peak memory.

Both earlier performance failures remain failures. The prefix parent's full SDK gate failed three 16-worker checks, despite its qualified 32-worker gains. The short-map complete-query gate failed seven checks. Their separate results do not establish an additive gain for this composition. The inversion-seed and paired-top scheduler changes are excluded. This experiment makes no twofold speedup prediction.

Three focused integration tests pass, with 129 inherited tests filtered:

- 40 raw and owned cases cover query cutoffs, identities, repeated/opposite points, full-width/boundary scalars and length preservation on rejection.
- 21 grouped/admission cases include a forced preparation-denial fallback.
- 106 nested queries run on every worker of 1/4/16/32-worker pools, using 512 KiB requested stacks. Inputs remain unchanged.

The oracle uses independent Arkworks field/group arithmetic and known generator weights. Test-only counters assert that the top, owned and borrowed transforms, scaled-prefix multiplication and uncarried-Xsum paths execute. The deliberately caught length-mismatch assertion is part of a passing test. Nested correctness checks do not measure service throughput. Generic no-ASM/no-IFMA field and core compilation does not establish execution on ARM or mobile devices. Existing successful field/model suites are inherited without replay.

The 186-member snapshot recovers all 183 parent members through recorded edits. The two short-map field children are exact copies of their qualified versions. Production's 176 members remain unchanged. The review checks 209 pins, all six successful command receipts, source recovery and test results. An initial source-anchor typo and missing test imports were repaired; the first integration compile failed before test execution. A repair-helper pin check and the first reviewer's null/empty environment comparison were also corrected. Failed helpers and compiler records are retained. No successful scientific test was replayed. [Review](artifacts/qualification-review.json), [source audit](artifacts/source-audit.json), [focused tests](artifacts/prefix_phi_tests.rs).

The next step is a complete-proof comparison with accepted and fresh controls plus both component controls, under one prospective protocol with automatic profiles and independent verification. Full all-eight/merge54, resource, admission, portable and concurrency qualification must precede adoption. [Design](artifacts/design.md), [continuation loop](artifacts/next.md), [exact evidence](artifacts/evidence.json.gz).

The following accepted full-proof results are inherited, use fixed worker placement, and are **not timings of this composition**. Historical standard 1×2 **7.915095 ms** remains valid; a later standard run measured **8.075143 ms** on the same accepted code. Fixed placement is a separate configuration, not an adopted default.

|Shape|Accepted fixed ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.869302|37 / 4.702×|24 / 3.050×|3.7|
|2×2|10.984868|59 / 5.371×|30 / 2.731×|5.9|
|4×2|15.734200|109 / 6.928×|48 / 3.051×|10.9|
|8×2|23.929023|189 / 7.898×|74 / 3.092×|18.9|
|12×2|33.722646|284 / 8.422×|101 / 2.995×|28.4|
|16×2|39.776845|335 / 8.422×|124 / 3.117×|33.5|
|24×2|66.183856|528 / 7.978×|176 / 2.659×|52.8|
|Merge24×1|65.448733|507 / 7.747×|175 / 2.674×|50.7|

Source: [preceding accepted comparison](../2026-11-09-paired-top-query176/README.md). M5 references use different hardware. Merge54 remains 162.489760 ms without an M5 reference. No new SDK request, proof export, CPU/Metal timing, AWS/Aeglos measurement or cloud resource was added. The canonical ledger remains 38,668 proofs. The objective remains active: **0/8 accepted shapes reach CPU 10×**.

Private root: `/home/ubuntu/src/zopro-perf-work/prefix-short-composition186`. [Publication checks](artifacts/publication.json), [checksums](SHA256SUMS). The archive contains text deltas, helpers, pins and receipts; executable/library blobs, unchanged full-source copies and key/witness/proof data are excluded. No recording is live.
