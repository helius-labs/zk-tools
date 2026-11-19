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
