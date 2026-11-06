# G1 binary-tree prefix records: native qualification

The existing two-vector binary-tree prefix path is enabled for native G1 in a private candidate. **Correctness and capacity qualification passes; performance has not been measured.** Accepted local 1×2 remains **7.928660 ms** and production is unchanged. The CPU 10× objective remains unmet for all eight supplied shapes.

## Change and rationale

The [preceding complete-caller study](../2026-11-06-triple-h-callers176/README.md) found a large working-set benefit but small-tile regressions for a ternary formula. That blanket ternary replacement remains closed. This candidate preserves binary pairs and uses the existing generic prefix/norm tree body, previously enabled only for G2. The earlier [G1 streaming-prefix result](../2026-10-25-prefix-records-component/README.md) and [old G2 tree confirmation](../2026-10-27-g2-tree-records/README.md) retain their original negative decisions. Actual inspected source distinguishes these placements; no exhaustive novelty claim is made.

G1 selects prefix records for every full native eight-lane group and reserves exactly the required count. The legacy Group vector stays unallocated throughout grow/shrink and exceptional workloads. This is a fixed storage-lifetime design: an amortized reserve could allocate a four-element minimum for one 640-byte record pair, exceeding the original 1,600-byte single group. Exact reservation avoids that overhead and amortized growth. G2 retains its threshold of 32 ordinary pairs and its original reservation policy.

The scalar tail, pair selection, exceptional handling, reverse writes, post-reverse compaction, weighted copied-even-index identity, inversion bodies, streaming kernels, field arithmetic and existing forecast bodies are unchanged. Binary prefix/division work remains four vector multiplications per full ordinary pair; two x arrays are packed forward and four x/y arrays reverse, versus four arrays forward originally. Extra packing and stack/register effects remain costs. No source byte saving implies a latency gain.

The candidate has 177 source members: 174 originals exact, two reversible parent edits and one cfg-only caller/test child. Reversing the edits recovers all 176 accepted originals. The default trait selector/reservation preserve G2 behavior; only experimental native G1 enables the new policy. The portable x86-64/no-IFMA selector is false and keeps original records. No key table, point/key copy, plan, new production pool, new field primitive, unsafe block or arbitrary RAM cap is added. Existing native and portable field/support libraries are reused without rebuild.

## Validation

All ten Rust/xtask workflow commands complete on their first attempt. Native executes **129 tests: 125 original and four new**, all passing. Portable executes four new focused tests, with 101 original tests filtered and 24 native-only originals absent from that executable. Each mode also passes two new independent Arkworks tests: 36 complete indexed/grouped fixtures including full 254-bit scalars, boundary values, mixed groups and fresh/reused buffers; plus 22 complete weighted cases including multiple and partial chunks.

Each mode checks 780 single-layer ordinary/partial/tail/pattern combinations and 120 weighted copied-even-identity cases, both inversion settings, and repeated increasing/decreasing scratch sizes. Ordinary, identity, doubling, opposite and mixed inputs match complete scalar/projective references. Separate std workers and Rayon pools each cover 1/4/16/32 workers with 512 KiB stacks. Grouped/indexed/weighted public paths are exercised inside the fixed-stack pools. Existing native forecast bounds remain conservative for the tested requested heap capacities. These are correctness/resource observations on this host, not CPU scaling or concurrent-proof throughput results.

The no-IFMA build uses x86-64 and matching frozen portable dependencies. It validates that code path on this host; it is not an ARM, mobile or TEE execution. Field tests are inherited by exact source/library identity and were not rerun.

## Capacity and emitted code

Representative repeated-scratch observations are below. The two modes have different lane counts and representations, so these are **not an old/new RAM comparison**. The entries include fixed scratch but exclude live input/output arrays, allocator overhead and the rest of the process. They are not peak RSS, minimum RAM or whole-thread stack bounds.

| Mode | Legacy record B | Prefix record B | Pair index B | Fixed scratch B |
|---|---:|---:|---:|---:|
|native|0|81920|8192|592|
|portable|163840|0|8192|336|

Enabled native G1 asserts zero legacy Group capacity and prefix capacity no larger than the largest full-group count seen. On this target a prefix/norm record is two 320-byte vectors instead of five, so the exact record bound is 640 versus 1,600 bytes per eight pairs. Retained common buffers and total process resources still need qualification. Existing admission estimates are unchanged.

Actual core commands use O3/oneCGU and explicit native or x86-64 targets. Final test/oracle consumers use fatLTO. Inspection of the native oracle consumer finds six selected functions with 1,200 static IFMA sites. Its G1 reduce_layer has 6,389 static instructions and a prologue with 3,840 bytes RSP subtraction plus 48 pushed-register bytes. These are emitted-code observations, not executed instruction counts, dynamic spill cost, total stack usage or timing results. No timing driver was built in this stage.

## Review correction and next stage

The main independent review passed once, but a separate derived field mistakenly hardcoded 125 portable filtered tests while its own actual summary contained 101. A narrow repair checked the successful stdout hash, parsed the actual count and corrected only that metadata. Original reviewer/source/JSON/logs are retained; the successful main review, tests and builds were not repeated. The published reviewer source derives the count for reproducibility. See the [correction record](artifacts/review-metadata-repair.json).

Next prepare matching frozen/fresh-OFF/new complete callers and a prospective performance protocol. Preserve all original unweighted cases and 1/16/32 contexts, and include weighted performance before SDK selection because the shared G1 tree affects weighted callers. Weighted point generation belongs outside the measured region. The existing weighted_words helper is a correctness boundary, not a ready timing caller. If an adapter extension is needed, keep qualified source/libraries immutable and qualify only the new wrapper/dependent artifacts.

Profile counts should use the previous control evidence prospectively to target thousands of samples. The prior study's millions of large-group samples are already complete and must not be repeated or pushed into Git. All new latency must finish before PMU/profiles. There is no positive-gain floor, threshold/layout sweep, favorable-subset promotion, extension or unchanged reroll. A qualified complete-caller gain can select only a separate verified all-eight+54/physical16/portable/concurrent/resource/Gnark SDK trial. Passing this qualification does not permit adoption.

No latency/PMU/profile capture, new SDK/Gnark proof export, AWS/Aeglos comparison, huge-circuit, device/pressure/swap, minimum-RAM or concurrent-proof throughput result is added.

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




[Review](artifacts/review.json), [codegen](artifacts/codegen.json), [design](artifacts/design.md), [source audit](artifacts/source-audit.json), [libraries](artifacts/libraries.json), [native/portable tests and caller](artifacts/probe.rs), [Arkworks oracle](artifacts/oracle.rs), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [metadata repair](artifacts/review-metadata-repair.json), [next stage](artifacts/next.md), [three exact source deltas](artifacts/source-deltas.json.gz), [exact helpers/receipts and retained original review](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS). Unchanged source members are bound by their accepted hashes; full snapshots/binaries stay local. No private inputs, witness/key/image/proof bytes, executable/library blobs or external paper text are included.
