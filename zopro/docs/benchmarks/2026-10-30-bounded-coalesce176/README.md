# Bounded full-scalar coalescing: point-work screen

**The necessary count screen passes; no speedup is measured.** After charging group formation, the modeled primary A fill loses 12,121 generic pairs (3.329%) and B2 loses 17,772 (3.104%). Both 2×2 populations already admit the original global grouping and bypass the proposed mechanism. Production and all proof measurements remain unchanged.

This differs from partial-window reuse: it combines equal entire scalars within consecutive tiles, without changing any digit. It also differs from the earlier raised grouping cutoff and sparse grouped-point trials: it preserves the original grouping decision, original point partitions and sequential key stream, and would retain only repeated-class sums and sparse skip/replacement indices. Those earlier trials retain their outcomes. This screen implements the mathematical coalescer only; the proposed streaming overlay is not implemented or qualified.

## Rule and counted work

When original global grouping declines, divide each original point partition into tiles of at most 1,024 rows, matching the existing G2 indexed-leaf geometry. Ignore zero scalars and identity inputs. Equal nonzero scalars within a tile form a class in first-occurrence order. Combine only classes with at least two active digits across the original query geometry. A one-digit class cannot save generic additions after paying for formation and therefore retains its original rows. There is one fixed geometry, with no size sweep or post-result selection.

Preserve signed width 12, A's one point partition and 21 ordinary windows plus narrow top, and B2's two partitions, Phi split, ten ordinary windows and signed/component narrow top. Tile groups never cross original part boundaries. Ordinary B2 high/low components share bucket seeds; a separate correction restores the seeds counted twice by the component tables. Indexed-top pairs retain separate sign/component bins. Formal occupied bins remain occupied even if token values cancel.

|Population|Original rows|Global grouping|Useful repeated classes|Formation additions|Original generic pairs|After coalescing, including formation|Net pairs saved|
|---|---:|---|---:|---:|---:|---:|---:|
|1×2 A|21,006|declined|227|583|364,096|351,975|12,121 (3.329%)|
|1×2 B2|30,089|declined|325|849|572,483|554,711|17,772 (3.104%)|
|2×2 A|36,637|admitted|—|—|—|—|bypass|
|2×2 B2|52,274|admitted|—|—|—|—|bypass|

Primary A arrivals fall from 407,098 to 394,394 and B2 from 613,891 to 595,270. Emitted nonzero terms are 18,690 and 27,640. Bucket occupancy is identical before and after, so saved arrivals equal saved generic pairs before formation. Each repeated class independently satisfies the identity `(m−1) × (active_digit_count−1)` for net saved pairs. The separate sorted-row checker reproduces the totals without importing the coalescer model.

These are noncancelling generic point-work counts, not actual curve operations, inversions, total query costs or CPU time. They exclude exceptional additions, classification, point gathering, packing, normalization, allocation, skip/replacement checks, component/part merges, weighting and scheduling. This modest opportunity cannot by itself establish the roughly twofold remaining proof improvement.

## Memory and next implementation

A hypothetical sparse overlay needs 227 G1 sums plus 583 skip and 227 representative u32 indices: 17,768 payload bytes. B2 needs 325 G2 sums plus 849 skip and 325 representative indices: 46,296 bytes. This excludes tile dictionaries, formation/reduction scratch, headers, capacity rounding, allocator overhead, stacks, inputs and the existing prover workspace. It is not a prover RAM measurement or an architectural cap. The model retains analysis row lists; those lists are not the proposed runtime representation.

An implementation should first reuse the classification already produced by the original declined grouping attempt. Hashing full scalars again would add work that this count does not cover. Tile-local class IDs could produce only the repeated sums and sorted sparse skip/replacement indices, then release the old classification before window execution. Each window would borrow original singleton points and the new sums. No singleton point array, retained key lookup table or duplicate solver plan is needed in that design. Checked admission and failure fallback remain required.

The next decision is a prospective complete-query trial including classification, all point formation, the overlay scan, unchanged top/reductions and allocation/drop. Exact curve oracles, actual linked code, 1/16/32-worker timing and profiles must precede any SDK evaluation. A count pass is not SDK eligibility or adoption, and old failed gates are not revised.

## Validation and provenance

Three focused tests pass: 4,096 boundary/full-width scalar cases reconstructed for both G1 and Phi2; bounded class/order/formation checks through 2,049 rows; and every signed bucket plus complete weighted sums on skewed/ragged populations. Each eligible real population checks two full-width deterministic Fr-token streams. Every original nonzero nonidentity row appears once, and every class stays inside its tile and point partition. These token checks are not curve or proof verifications.

The independent reader uses sorted `(part,tile,scalar,row)` records instead of the model's per-tile BTreeMap. It independently reconstructs every original scalar in all four populations, reproduces sampling/admission with HashSets, counts B2 ordinary buckets directly across both components, and checks all formation, arrival, emission and net totals. It shares the exact source-extracted digit/Phi helpers and accepted Fr arithmetic, so it is not an independent field oracle.

An initial test compilation failed because a test length needed an explicit `usize` type. The initial source, pins, plan and failed compiler receipt remain. Only that test annotation and driver filename changed in the repaired copy; no mathematical body or protocol changed, and no test or census had executed. The corrected seven-step workflow and separate two-step review finish once. No successful execution is repeated.

Cached A words/identity flags and the earlier independent-Arkworks-bound B2 fixtures are reused byte for byte. No witness solve, scalar export, old curve-oracle execution, native prover rebuild, SDK proof, Gnark export, timing or profile is added. All 176 production files, coherent accepted libraries, helpers and inputs remain pinned. The verified-proof ledger stays 27,353. Private inputs, point arrays, libraries and executables remain local.

## Accepted proof comparisons, unchanged

Inherited local 32-worker complete SDK measurements and supplied historical M5 references follow. This screen measures none of these timings. All eight CPU 10× targets remain unmet.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge 24→1|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Accepted merge 54→1 is 161.883899 ms without a supplied M5 reference. Accepted primary process RSS is 132.75 MiB. The [adoption report](../2026-10-30-g2-vector-sign-adoption176/README.md) retains its physical-16 calibration engineering exception and qualification scope. No new memory or platform result follows.

Latest AWS primary remains historical source 175 on c7a.24xlarge: Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96, each campaign's best tested configuration. That does not compare current source 176. No AWS resources were created here.

[Design](artifacts/design.md), [all counts](artifacts/analysis.json), [tested model](artifacts/model-fixed.rs), [driver](artifacts/probe-fixed.rs), [source-helper check](artifacts/source-review.json), [original helpers](artifacts/split.rs), [independent reader](artifacts/review.rs), [review](artifacts/review.json), [input provenance](artifacts/inputs.json), [original pins](artifacts/pins.json), [repaired pins](artifacts/pins-fixed.json), [repaired workflow](artifacts/plan-fixed.json), [review workflow](artifacts/review-plan.json), [all command receipts](artifacts/receipts.json.gz), [accepted source manifest](artifacts/accepted-manifest.json), [publication review](artifacts/publication-review.json), [checksums](SHA256SUMS).
