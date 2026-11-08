# Four-pass wider-window MSM: operation-count tradeoff

A table-free, four-pass model with width 14 uses the same **2,048 ordinary buckets per pass** as the retained width 12 comparator. It reduces modeled additions by **5.34% on primary and 5.62% on grouped 2×2**, while increasing active doublings from 846 to 2,826 and logical input visits by 3.36×. **No native implementation or proof-speed improvement is established. Production is unchanged.**

The declared necessary screen requires fewer additions and no increase in active doublings on both populations. It fails the doubling condition; no native timing trial is selected under that screen. This is a conservative count-dominance decision, not proof that the tradeoff is slower. The positive addition savings remain a reason to examine complete costs in future research. No observation, width or subset is dropped or rerun.

## Construction

The indexed primary [OPTIMSM paper, section 4.4](https://eprint.iacr.org/2024/1827.pdf) describes bucket reuse across digit ranges. We independently apply that iteration identity without its fixed-point precomputation or FPGA pipeline. Direct full-PDF access returned 403; only the indexed section and abstract support this reading. No FPGA speedup is transferred to this CPU.

For a magnitude m=q·2048+j, with q∈{0,1,2,3} and 1≤j≤2048, let Wq be each band's locally weighted bucket sum and Tq its ordinary total. Then

    W = W0 + W1 + W2 + W3 + 2048·(T1 + 2T2 + 3T3).

The quotient term is evaluated as 2·(T2+T3)+T3+T1, then 11 doublings. Width 14 has ten positions for the signed 126-bit Phi2 components, including its final carry, versus eleven width 12 positions. Its narrow final carry has one bucket and one pass. Two source partitions, high/low component placement and the inherited 256-bucket weighted-tree graph remain charged. This is one fixed width/iteration geometry, without a sweep.

Both inputs are retained actual B-query scalar populations. Primary has 30,089 terms; the 2×2 population has 52,274 raw rows and 28,852 retained grouped terms, with inherited grouped-point nonidentity evidence. The grouping decision is the qualified data-only model; runtime preparation/admission is not replayed. The original width 12 counts are inherited from the [qualified fixed-window audit](../2026-11-06-g2-fixed-wnaf176/README.md), not recomputed by rerunning its producer.

## Complete count results

These are noncancelling abstract group-operation counts, **not CPU timings, native IFMA counts, exclusive wall time or a lower bound over all algorithms**. Fill includes free first writes and part/component merges. The weighted graph, band offsets/combines and Horner assembly are charged. Additions and doublings remain separate cost types.

|Population|Original additions|Four-pass additions|Change|Original active doublings|Four-pass active doublings|
|---|---:|---:|---:|---:|---:|
|1×2|634,486|600,576|−5.3445%|846|2,826|
|Grouped 2×2|654,290|617,488|−5.6247%|846|2,826|

Each population adds 1,980 doublings, a 3.3404× total. Band offset/combining alone costs 63 additions in either population; making all that work free would give 600,513/617,425 additions. The main tradeoff comes from the changed window/weighted geometry, not the small offset formula.

The saved 33,910/36,802 additions would pay for the extra doublings if their average cost were below **17.1263/18.5869 original additions per extra doubling**, before charging any other changes. This is a symbolic break-even budget, not a measured exchange rate. It neither establishes a speedup nor makes the additional input passes, inversions or memory work free.

Sequential band scans visit logical point/scalar records 2,226,586/2,135,048 times, versus 661,958/634,744 for the original 11-position two-component pass geometry: 37/11=3.3636×. This is a loop-visit count, not bytes transferred, cache misses or a bandwidth prediction. A native implementation could avoid some coordinate loads for discarded digits; none is implemented or priced.

Using 2,048 buckets per pass does not establish unchanged peak RAM. Running all bands concurrently would multiply live arrays; a sequential implementation must charge repeated scans and lifetime/scheduling effects. Histograms allocated by this Rust model are analysis storage, not a proposed runtime layout. No lookup table, copied key, retained solver plan, production worker pool or arbitrary memory cap is added.

## Qualification

All six serialized Rust/xtask commands succeed on their first executions: new test build/run, new count-model build/run, and offline reader build/run. Pinned Rust 1.98.1, O3 and overflow checks are used for mathematical tooling, not new AVX kernels. The 194 pins include all 176 unchanged production sources, retained inputs, inherited routines/counts and toolchain dependencies.

Two new focused tests pass:

- 2,560 modular comparisons of the four-band identity across two small cyclic groups, signed point labels, five geometries and 256 masks.
- 256 dense/sparse/boundary compositions of the inherited weighted graph with the new band-offset formula.

The new count pass reconstructs 117,882 signed components and checks every digit's unique band/local mapping. The offline reader independently uses quotient/remainder with overlap-sign carries to rebuild all histograms, first-occurrence grouping and scalar identities. It derives 74 weighted-band graphs from block occupancy rather than executing the producer reducer, and reconstructs all offset/Horner counts and the failed continuation decision. The exact already-qualified Phi2 split is shared; this is not a new split oracle, curve/point oracle or independent native arithmetic implementation.

The reader validates the preceding five command/exit/stream receipts; publication additionally validates its sixth receipt and the complete workflow. Original successful encoding, grouping, field/core/prover tests, captures, decoders and cryptographic verifiers are not replayed. No recording takes place, and no new SDK request or proof is generated. Canonical verified-proof inventory remains 35,314.

Unpriced work includes actual X-coordinate exceptions/cancellations, queues/projective spills, normalization, inverse topology, eight-lane groups and tails, packing/export, beta transforms, reads/metadata, allocation, scheduling and proof critical paths. Counts cannot determine which side wins on native CPU. The fixed screen's nonincreasing-doublings rule is sufficient count dominance, not a necessary condition for lower latency. Future selection should use a complete cost case rather than treating that failure as a universal rejection of iterative bucket methods.

## Accepted proof comparisons, inherited

These local 32-worker observations and supplied historical M5 references use different hardware; they are not measurements of this model. The latest primary 8.010122 ms and earlier 7.915095/7.928660 ms use the same accepted binary. Variation is unexplained without an adopted code regression.

|Shape|Accepted local CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.010122|37 /4.619×|24 /2.996×|3.7|
|2×2|11.241463|59 /5.248×|30 /2.669×|5.9|
|4×2|15.933816|109 /6.841×|48 /3.012×|10.9|
|8×2|24.234311|189 /7.799×|74 /3.054×|18.9|
|12×2|33.855565|284 /8.389×|101 /2.983×|28.4|
|16×2|40.071563|335 /8.360×|124 /3.094×|33.5|
|24×2|65.917462|528 /8.010×|176 /2.670×|52.8|
|Merge24×1|65.305023|507 /7.764×|175 /2.680×|50.7|

Merge54 remains 162.240109 ms without a supplied M5 reference. All-eight verified CPU 10× goal remains active and unmet, 0/8. No new CPU proof time/profile, AWS/Aeglos, huge-circuit, mobile/ARM/TEE, pressure/swap, RSS or minimum-RAM evidence is added.

[Design](artifacts/design.md),[reading scope](artifacts/reading.md),[analysis](artifacts/analysis.json),[review](artifacts/review.json),[publication](artifacts/publication.json),[evidence archive](artifacts/evidence.json.gz),[checksums](SHA256SUMS). The archive contains UTF8 sources, aggregate histograms, pins/plans and command receipts. Keys, witnesses, raw scalar records, proof/public payloads, ledgers, binaries/libraries and external full texts are excluded.
