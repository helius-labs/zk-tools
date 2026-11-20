# Curve work in the seeded 4×2 profile on C7a

The [K overlap test](../2026-11-20-four-by-two-early-k-c7a895/README.md) began K during the FFT and still waited on the main curve stage. This experiment counts operations with the same seeded solver, FFT and verification settings. It uses 64 main and 32 frontend physical cores on c7a.24xlarge.

After three ordinary SDK primes, each process captures one proof. The capture runs all four queries, replays all four, or evaluates one query while replaying the other three after comparing their complete scalar slices. The replay architecture and timing limits are described in the [phase study](../2026-11-20-four-by-two-phase-c7a891/README.md). All 36 exports independently verified. The temporary instance and storage were removed.

H performed 1,868,495–1,869,905 streaming additions. B2 performed 589,115–594,103 streaming additions. Together they accounted for 71.9–72.0% of all streaming additions before weighting the different curve costs. The four isolated counts reproduce every counter from the ordinary proof in all six blocks.

Rows are medians over three processes. Streaming counts come from the filled window states. Queue spills enter the projective path and can populate a new bucket or add to one that already exists. Merge rows count ordinary affine pairs joining window partitions. Tree rows count generic pairs in the shared reducer, including grouping, indexed leading windows and weighting. Tails are point pairs beyond a full eight-lane group in each batch or layer. One G1 pair and one G2 pair have different field costs. Instrumented latencies are excluded from the scorecard.

| Input | Query | Streaming adds | Batches | Streaming tails | Queue spills | Projective adds | Normalized buckets | Merge pairs | Tree pairs | Tree tails |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| catalog | A | 373,418 | 1,869 | 90 | 222 | 10 | 212 | 0 | 139,523 | 675 |
| catalog | K | 582,718 | 2,919 | 62 | 193 | 3 | 190 | 0 | 166,106 | 770 |
| catalog | H | 1,868,495 | 5,334 | 151 | 246 | 7 | 239 | 85,993 | 233,738 | 1,130 |
| catalog | B2 | 589,115 | 2,973 | 187 | 307 | 14 | 293 | 47,005 | 158,862 | 886 |
| heldout | A | 376,773 | 1,890 | 69 | 147 | 3 | 144 | 0 | 137,699 | 587 |
| heldout | K | 587,195 | 2,940 | 91 | 200 | 6 | 194 | 0 | 163,354 | 666 |
| heldout | H | 1,869,905 | 5,334 | 145 | 303 | 11 | 292 | 85,992 | 233,688 | 1,144 |
| heldout | B2 | 594,103 | 2,990 | 175 | 276 | 8 | 268 | 46,999 | 155,915 | 787 |

| Input | Query | Group inputs | Included terms | Classes | Streaming arrivals | Fills | Weighted windows | Weighted slots | Indexed terms |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| catalog | A | 67,154 | 58,424 | 19,851 | 416,647 | 21 | 22 | 43,012 | 16,616 |
| catalog | K | 97,075 | 86,732 | 29,828 | 625,919 | 21 | 22 | 43,012 | 24,866 |
| catalog | H | 0 | 0 | 0 | 2,040,729 | 42 | 22 | 86,020 | 65,087 |
| catalog | B2 | 95,537 | 86,163 | 29,738 | 683,508 | 46 | 24 | 47,106 | 10,093 |
| heldout | A | 67,154 | 56,690 | 20,014 | 419,927 | 21 | 22 | 43,012 | 16,693 |
| heldout | K | 97,075 | 83,995 | 30,052 | 630,403 | 21 | 22 | 43,012 | 25,075 |
| heldout | H | 0 | 0 | 0 | 2,042,195 | 42 | 22 | 86,020 | 65,037 |
| heldout | B2 | 95,537 | 83,409 | 29,961 | 688,459 | 46 | 24 | 47,106 | 10,123 |

[Counts per proof](artifacts/rows.json) · [Reconciliation](artifacts/reconciliation.json) · [Observer source](artifacts/source.patch) · [Workflow](artifacts/workflow.json.gz) · [Proof verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**5/8** original requests reach 10× M5 CPU on the recorded profiles.

| Shape | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.858 | 37 / 24 | 4.71× / 3.05× |
| 2×2 | 12.029 | 59 / 30 | 4.90× / 2.49× |
| 4×2 | 14.726 | 109 / 48 | 7.40× / 3.26× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |
