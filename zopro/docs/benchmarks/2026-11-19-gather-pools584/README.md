# Which pool issues the sampled gather notifications on C7a?

During the interval between solver return and FFT entry, two inlined Rayon task-publication sites were prominent in an earlier CPU profile. This analysis locates their samples by Linux thread ID and elapsed third of the interval. Main workers were pinned to a single CPU and recorded the same placement before and after each process. The caller is identified by its SDK entry event. The other-thread group comprises TIDs outside those two sets. The archived merge processes also configured a separate frontend pool of 32 and 48 workers at main widths 64 and 96 respectively.

The first two count columns show samples on main CPUs 0–31, 32–63, 64–95, other threads, and the caller, in that order. The last two columns show the share of samples in the first, middle and last equal-duration third of each handoff, pooled over 33 requests. A CPU sample at a notification test does not measure an actual wakeup or elapsed time spent waiting.

| Shape | Main workers | Notification samples by worker group | Crossbeam samples by worker group | Notification samples by third | Crossbeam samples by third |
|---|---:|---:|---:|---:|---:|
| 16×2 | 64 | 122 / 123 / 0 / 0 / 0 | 174 / 163 / 0 / 0 / 0 | 39% / 42% / 19% | 16% / 4% / 81% |
| 16×2 | 96 | 191 / 192 / 185 / 0 / 0 | 294 / 270 / 266 / 0 / 0 | 24% / 59% / 17% | 14% / 1% / 85% |
| Merge-24×1 | 64 | 149 / 159 / 0 / 0 / 0 | 103 / 99 / 0 / 3 / 0 | 10% / 57% / 33% | 4% / 2% / 94% |
| Merge-24×1 | 96 | 212 / 232 / 222 / 0 / 0 | 154 / 165 / 173 / 22 / 0 | 6% / 74% / 20% | 6% / 1% / 93% |

All 1,787 samples at the two notification tests fell on main-pool workers, including all 974 on merge. At 96 cores the last 32 main workers contributed 185 of 568 such samples on 16×2 and 222 of 666 on merge. The middle third contained 59% and 74% of the notification samples at this width. The final third contained 85% and 93% of the Crossbeam samples. This ordering supports varying the number of tasks in the main-pool gather while holding frontend width fixed. The archive cannot establish how that change would affect proof latency.

The reader verifies the trace, placement log, SDK and exported proof digests, checks each of the 132 intervals and sample totals against the prior decomposition, and reproduces each of the four published notification totals. [Reader](artifacts/read.py) · [Per-request counts](artifacts/observations.json.gz) · [Per-block placement](artifacts/blocks.json) · [Prior instruction attribution](../2026-11-19-c7a-previous-proof574/README.md)


## Do both scalar collectors publish jobs at the same time?

The ELF identifies two distinct indexed field-collection closures in the solver. Each has a sampled instruction that tests the job notification counter. The reader counts those instructions separately, verifies each request's counts against the earlier address analysis and divides the solver-to-FFT interval into equal thirds. For requests with samples at both instructions it also compares their first and last sample times. The binary names the enclosing closures 8 and 9. The counts alone cannot identify which scalar vector belongs to each closure.

| Shape | Cores | Closure 8 samples by third | Closure 9 samples by third | Requests with both sites sampled | Observed spans 8 first and disjoint / overlap / 9 first and disjoint |
|---|---:|---:|---:|---:|---:|
| 16×2 | 64 | 75 / 42 / 7 | 21 / 60 / 40 | 26/33 | 15 / 9 / 2 |
| 16×2 | 96 | 89 / 170 / 28 | 50 / 163 / 68 | 31/33 | 9 / 18 / 4 |
| Merge-24×1 | 64 | 31 / 81 / 13 | 0 / 94 / 89 | 28/33 | 14 / 14 / 0 |
| Merge-24×1 | 96 | 37 / 214 / 29 | 4 / 278 / 104 | 30/33 | 5 / 25 / 0 |

On 16×2 at 96 cores the sites contributed almost equally to the total (287 and 281 samples). On merge at 96 cores, closure 9 accounted for 104 of the 133 samples in the final third. Its share of that third was also higher at 64 cores (89 of 102). Removing one dense collector could leave substantial notification work in the other. Sampling at two instructions cannot establish when either collector finished, whether a worker woke or how many milliseconds an implementation would save. [Reader](artifacts/collector_sites.py) · [Per-request site counts and observed spans](artifacts/collector-sites.json.gz)
