# Eight-window G2 SIMD scheduling screen

The fixed cross-window recipe is closed without native integration. On the saved 1×2 fixture, one tile has 226 active updates to the same window/bucket. A color admits at most one of those updates, so the proposed 200-color schedule cannot process that tile regardless of ordering. The first necessary violation occurs at tile row 403. The 2×2 fixture has no such obstruction in this bound; that is not proof of a feasible schedule or of a speedup.

The idea puts eight ordinary G2 Phi2 windows in SIMD lanes so one incoming point can be broadcast across them. It keeps 16 independent fill jobs by using 16 point parts, compared with 8 windows × 2 parts in the original path. The tradeoff is more first assignments, part merges and phi maps, plus inactive lane work and larger bucket storage. Width 12, windows 0–7, tile 1600, 200 colors and 200 rows/color were fixed before execution. No limit/partition/window sweep or native timing follows this refusal.

|Saved B fixture|Raw → effective rows|Tiles with proven cap obstruction / total|Largest same-bucket active group|Hypothetical fill lane use|
|---|---:|---:|---:|---:|
|1x2-B|30089 → 30089|1 / 64|226|56.92%|
|2x2-B|52274 → 28852|0 / 64|7|56.97%|

The effective 2×2 row count applies the unchanged scalar-grouping policy. The original raw points were checked nonidentity. Formal occupancy ignores actual point cancellation and equal-x exceptions. The lane-use counts describe the proposed masked fill with an unbounded schedule; they are not measurements of the accepted kernel. About 43% of those lane slots would execute suppressed updates. They exclude inverse bridges, part merges, scalar work, masking, broadcasts, gathers/scatters and coloring costs. No completed original-versus-candidate caller totals, inversion savings or latency claim are available.

Affected affine bucket payload rises from 4 MiB to 32 MiB. The proposed representation can additionally hold up to 504,896 bytes of scheduler payload and 128,000 bytes of prefix records per actor, before allocator overhead and other scratch. These are representation counts, not native allocations, prover RSS, minimum RAM or device qualification. Points remain borrowed and no key lookup table is proposed. The recipe's obstruction and added work/storage make a native trial unjustified as currently defined; other cross-window organizations remain unproven rather than ruled out.

## Validation and limitations

The original workflow compiled both targets and passed two new tests: 48 synthetic signed-token schedules across zero/sign/reuse/tile boundaries, plus explicit bounded-refusal checks. The real-fixture model then returned 101 at its fixed refusal; it did not write a completed result. Its source, pins and all command receipts remain intact. The prospective completed-census reviewer was never run. Token scheduling checks are not EC or proof correctness tests.

A separate full-fixture dependency audit counts unavoidable same-bucket conflicts without running the coloring algorithm. A separate reader directly gathers matching digits to verify the largest-clique certificates for both fixtures and the first necessary 1×2 violation. It also checks source/input integrity, exit/stream receipts, all 128 tile-bound identities, active-lane histograms and payload formulas. It shares the inherited scalar grouping and Phi2 split; it is not an independent cryptographic oracle.

Two audit corrections are retained: an explicit integer type fixed an initial compile error; a later correction removed the invalid expectation that every fixture must exceed 200 colors. The final audit and certificate reader passed. Neither the original model nor its passing tests were rerun. Compiler warnings about unused inherited helpers remain. No failure, zero-obstruction case or unfavorable estimate is excluded.

No production source, accepted SDK, proof ledger or proving-time result changes. No SDK proof or Gnark export was requested. The canonical verified-proof count remains 36,477. No new AWS, Aeglos, Gnark, huge-circuit, phone, memory-pressure, swap or throughput comparison is added.

## CPU and Metal comparisons, unchanged

Latest accepted local 32-worker full-proof observations, with the supplied historical M5 references, follow. These are inherited results from the [default-carry comparison](../2026-11-08-uniform-carry-default-performance183/README.md), not measurements from this scheduling screen.

|Shape|Accepted Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.018438|37 / 4.614×|24 / 2.993×|3.7|
|2×2|11.199751|59 / 5.268×|30 / 2.679×|5.9|
|4×2|15.957517|109 / 6.831×|48 / 3.008×|10.9|
|8×2|24.250900|189 / 7.794×|74 / 3.051×|18.9|
|12×2|33.735233|284 / 8.418×|101 / 2.994×|28.4|
|16×2|40.165709|335 / 8.340×|124 / 3.087×|33.5|
|24×2|65.802777|528 / 8.024×|176 / 2.675×|52.8|
|merge24|64.926690|507 / 7.809×|175 / 2.695×|50.7|

All eight CPU10× requirements remain active/unmet, 0/8. Merge54 remains 162.442299 ms without a supplied M5 reference. Earlier accepted 1×2 observations include 7.915095 ms; private experiments include 7.859756 and 7.869400 ms. Sub-8 ms is already historical evidence. Cross-campaign absolute variation does not establish an adopted regression or a new record.

## Artifacts

[Counts and certificates](artifacts/result.json), [certificate review](artifacts/review.json), [fixed design](artifacts/design.md), [original model](artifacts/model.rs), [final obstruction audit](artifacts/obstruction.rs), [final reader](artifacts/review.rs), [refusal note](artifacts/refusal-audit.md), [compile correction](artifacts/compile-repair.md), [assertion correction](artifacts/bound-assertion-repair.md), [sources, pins, plans and raw receipts](artifacts/sources-and-receipts.json.gz), [publication verification](artifacts/publication.json), [checksums](SHA256SUMS).

The archive retains original and corrected helpers and every captured workflow stream/exit, including failures. Each UTF8 member and the gzip roundtrip were hash checked. Executables, libraries, fixture/key/image/witness/proof bytes and the full proof ledger are excluded; their relevant identities remain in pins. Manually invoked preparer/reviewer commands do not have complete xtask compiler receipts; no claim is made that they do.
