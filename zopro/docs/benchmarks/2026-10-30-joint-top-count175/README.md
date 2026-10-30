# Joint narrow-top G2 digit reduction: work screen

A new native prototype is selected for qualification: group points by the two existing signed six-bit top digits, then coalesce both reduction stages. On the two saved real inputs, this removes **20.6% / 20.2% of generic top-window additions**, and reduces modeled tree-layer calls **3,289→226 / 3,345→227**. These are algebra/topology counts, not measured speedups. Production and accepted proof latency stay unchanged.

## Algorithm

Within each existing G2 point-range half, group original points by their top digit pair (a,b), excluding (0,0). Reduce each occupied pair class once. Its derived sum then contributes to both original component marginals, with the original signs, phi map and bucket weights. The ten lower windows, Phi2 decomposition, 12-bit reader and two parts stay unchanged. No all-row-pair search, scalar recoding, shifted key, persistent point table or new field arithmetic is used.

The first tree groups 256 adjacent compact pair classes through the existing group_affine algorithm. The selected second stage uses that same algorithm across each 128-sign-bin marginal. This changes parallelism as well as work. A larger marginal leaf can reduce available parallelism, increase scratch and lose in practice; native timing is required. No generic frontend counter capture was repeated: retained IBS/dispatch studies already favor investigating arithmetic/resource costs over a frontend-only explanation. [Scope/history audit](source/history-audit.md).

## Both declared results

The first layout coalesced only joint classes and retained original per-bin indexed marginals. It **fails** its necessary gate: primary tree layers rise 3,289→3,301, despite fewer additions. A separate follow-up coalesces the marginals too; it uses the same count-gate conditions and **passes**. The old helper, protocol, observations and failed gate remain intact. No threshold changes, favorable subset, capture rerun or performance adoption occurs.

| Shape | Original top additions | Both layouts: additions after | Saved | First layout: tree layers | Follow-up: tree layers | Original / follow-up scalar tail pairs |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 54829 | 43531 | 20.606% | 3289 → 3301 | 3289 → 226 | 9333 / 787 |
| 2x2 | 56687 | 45209 | 20.248% | 3345 → 3316 | 3345 → 227 | 8719 / 777 |

Primary saves 11,298 generic additions; 2×2 saves 11,478. The exact identity is saved = input rows with both digits nonzero − occupied classes with both digits nonzero, applied separately to both halves. The separate first layout's scalar tails also fall, but its primary layer-count condition fails. Coalesced-independent counts without joint sharing are retained as attribution only; they do not remove additions or qualify a runtime alternative.

The necessary gate requires exact algebra/source/fixture checks, strictly fewer generic additions and no increase in tree-layer calls on both real shapes. It has no minimum percentage threshold. Tree calls are not measured inversions; prefix selection, scalar tails and lane groups are modeled from nonexceptional pairs, with the existing 32-pair threshold. Actual identities/cancellations/doublings, queue behavior, task cost, caches, inversion service and wall time remain unmodeled. Sign/component/part merges and final weighting are common and omitted from the addition totals. No conversion to removable proof time is justified.

## Correctness and provenance

The exact existing fixtures and their independent complete-MSM oracle receipts are reused. No old oracle repeats. Accepted stable zero-aware grouping declines primary (30,089 rows) and admits 2×2 (52,274→28,852). Grouped intermediate curve cancellation is unknown; the structural model assumes nonexceptional points. The current accepted source manifest, reused library source recovery, exact copied grouping/Phi/Booth functions, canonical scalar/point encoding and on-curve inputs are checked.

Four original model tests pass once, followed by two new follow-up tests. The latter invocation filters out the four already successful tests. Cases cover empty/odd/skewed/dense signed pair populations, zero pairs/digits, identities, digit extrema ±64,256-class group and 1024-point tile boundaries, structural occupancy, exact tree-edge counts and lane/tail accounting. Both real populations reconstruct Phi/Booth scalars and run two independent full-width Fr-token streams per layout, checking every signed marginal bin and the weighted top sum. These symbolic/field-token checks do not qualify a new real-curve kernel. Unchanged production suites are inherited, not rerun.

Both four-step source-pinned Rust xtask workflows complete on first executions: 425 original pins, 433 follow-up pins. Every command exit and stdout/stderr hash is checked. No native helper, test or successful count reruns. The follow-up inherits original baseline counts by exact saved-row identity. Its addition totals and occupied pair classes agree with the original layout; only reduction topology changes. The original top addition counts 54,829/56,687 also match the earlier balanced-window census.

## Memory limits of this screen

A 129×129 histogram admits at most 16,640 nonzero digit-pair classes. Compact u16 labels avoid allocating a full point grid. Original key points are borrowed; shared sums are derived proof-local points. Native routing can reuse the histogram as scatter cursors, then drop it; original order/starts/counts can drop after joint sums. That lifetime design is not implemented or qualified.

| Shape / half | Occupied classes U | Routing payload KiB | First reduction held KiB | Follow-up marginal retained KiB |
|---|---:|---:|---:|---:|
| 1x2 / 0 | 8159 | 332.21 | 1222.07 | 1070.69 |
| 1x2 / 1 | 8221 | 333.29 | 1230.91 | 1078.80 |
| 2x2 / 0 | 8556 | 336.77 | 1276.26 | 1122.64 |
| 2x2 / 1 | 8563 | 336.89 | 1277.26 | 1123.56 |

Requested 64-bit payload formulas are 8K+18U+8+4N for routing, 146U+8+4N for first-tree held arrays, and 134U+3080 before the selected marginal's leaf/output buffers. These are not total RAM or peak RSS. They exclude Vec headers/capacity growth, allocator overhead, all point-tree/normalization scratch, stacks and existing query workspace; sign/output/merge buffers remain separate. Coalescing may retain U local point copies in one leaf. Native integration must charge checked typed capacities, reserve optional memory and fall back to the unchanged top on denial. No synthetic global cap, phone/huge-key/concurrency/pressure/swap/minimum-RAM qualification is claimed.

## Accepted proof results, unchanged

All original eight CPU 10× targets remain unmet (0/8). The table compares inherited local 32-worker proof measurements with supplied M5 CPU/Metal references; these are not new Apple measurements. All nine local/AWS engine results, worker counts and RSS remain exact in [accepted rows](accepted-rows.json) and the [current benchmark snapshot](../2026-10-30-current-bench-snapshot175/README.md).

| Shape | Local ms | Peak RSS MiB | M5 CPU ms / speedup | M5 Metal ms / speedup | CPU 10× target ms |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.022165 | 134.00 | 37 / 4.61× | 24 / 2.99× | 3.7 |
| 2x2 | 11.335969 | 275.00 | 59 / 5.20× | 30 / 2.65× | 5.9 |
| 4x2 | 15.945544 | 315.00 | 109 / 6.84× | 48 / 3.01× | 10.9 |
| 8x2 | 24.090485 | 512.50 | 189 / 7.85× | 74 / 3.07× | 18.9 |
| 12x2 | 33.844855 | 707.25 | 284 / 8.39× | 101 / 2.98× | 28.4 |
| 16x2 | 39.931557 | 826.25 | 335 / 8.39× | 124 / 3.11× | 33.5 |
| 24x2 | 66.233309 | 1328.75 | 528 / 7.97× | 176 / 2.66× | 52.8 |
| merge-24x1 | 65.273224 | 1175.50 | 507 / 7.77× | 175 / 2.68× | 50.7 |
| merge-54x1 | 162.250464 | 1831.87 | — | — | — |

Latest retained C7a primary: Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96, best tested configurations in that campaign. No new AWS/Aeglos, CPU profile, performance capture, witness solve, SDK request or independent proof export occurs. Ledger remains 25,231. Passing this screen selects only the both-stages-coalesced native prototype. It requires complete-curve tests, resource/admission/release checks, controlled query/caller timing and profiles, then full verified SDK/all-shape/portable/resource/final-artifact qualification before adoption.

[All counts and gates](data.json), [original declaration](source/design.md), [separate follow-up declaration](source/followup-design.md), [original model](source/probe.rs), [follow-up model](source/followup.rs), [setup](source/setup.rs), [follow-up setup](source/followup-setup.rs), [publisher](source/publish.rs), [pins](source-hashes.json), [exact workflow receipts](workflow-receipts.tar.gz), [checksums](SHA256SUMS). Fixture points/scalars/tokens, unblinded query outputs, executables and libraries stay local.
