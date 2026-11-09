# Bounded fixed-factor FFT: matched consumer result

The corrected bounded fixed-factor FFT candidate is **rejected**. It regresses
in all six matched complete-Euler contexts. All six OFF/accepted-reference
calibrations pass; all 18 profiles meet the declared quality checks. Production
sources and the accepted SDK remain unchanged. This experiment adds no verified
full-proof latency or Aeglos/AWS comparison.

The table below measures the complete Euler consumer, including input clones
and factor expansion. Values are medians of seven process medians. The paired
ratio is computed within blocks and need not equal the ratio of the displayed
medians. These are consumer timings, not 1×2 proof timings.

|Domain|Workers|Accepted REF ms|OFF ms|ON ms|Paired ON/OFF|
|---:|---:|---:|---:|---:|---:|
|65536|1|6.311589|6.308944|8.321939|1.3193×|
|65536|16|1.061702|1.063054|1.158875|1.0901×|
|65536|32|1.157682|1.160207|1.198570|1.0359×|
|262144|1|33.186976|33.181486|41.186247|1.2427×|
|262144|16|7.041746|6.973437|7.500520|1.0721×|
|262144|32|7.116036|7.067654|7.373651|1.0435×|

The two 32-worker contexts' paired geometric aggregate is 1.038771 ON/OFF,
with descriptive ranks 1–5 of [1.035163, 1.043983] and one win in seven blocks.
Nineteen selection gates fail, including six clear matched regressions. The
fixed recipe is closed without extension, reroll, layout or subset rescue.

The arithmetic saved IFMA sites but increased total work. Single-worker PMU
instructions increase 22.9% at domain 65,536 and 19.9% at 262,144; user cycles
increase 32.6% and 24.1%. Both arms have zero measured minor faults in those
single-worker contexts. At 32 workers, instructions increase 18.1% and 16.9%.
Flat samples in the actual DIF/DIT leaf functions increase substantially.
Expansion, loads, normalization and carry work are plausible costs; this study
does not isolate a causal contribution for each. Generic cache counters do not
support attributing the regression solely to memory capacity or page faults.

The inherited compact-cache charge is at most 32,016 bytes across three
directions and 21,344 bytes after Euler conversion, plus optional pointers.
That allocation charge did not bound process RSS: the small-domain ON harness
peaks 1.75–2.25 MiB above OFF and fails five pairwise RSS checks. The harness
peak includes oracle and plan setup, allocator behavior and stacks; it is not
the prover's minimum RAM, nor a measurement of cache bytes alone.

This consumer also shows diminishing scaling: OFF takes 1.063/1.160 ms at
16/32 workers for the small domain and 6.973/7.068 ms for the large domain.
Summed phase CPU roughly doubles. Two named scheduling/epoch functions are
prominent in parallel flat profiles, but their samples do not measure all Rayon
overhead or an exclusive removable wall-time fraction. These findings do not
establish that the full prover should use 16 workers.

One favorable large-domain ON block is retained: 4.954 ms with 5.8 minor faults
per call, versus roughly 6,300 faults and 7.3–7.4 ms in its other blocks. The
single favorable block is included in the failed selection. Fault correlation
is a separate allocation/page-lifecycle question, not evidence for adopting the
lookup candidate. Earlier page-initialization, stack and scheduler experiments
must be checked before any distinct follow-up; this does not revive their
rejected variants.

All 270 capture/offline commands passed on first execution: 18 preflights,
126 latency processes, 18 gated counter panels, 18 gated cycle profiles, then
90 offline commands. The measured loops contain 24,696 complete consumer calls.
Every process checks a full independent Arkworks oracle before measurement and
a complete result digest afterwards. Workers are individually pinned with
512 KiB stacks. Profiles start disabled and gate only the warmed loop; this
includes result deallocation, whereas per-call wall timing excludes that drop.
All recordings finished before helper, analysis, hash, decode or publication
work. There are 4,236,256 samples, no LOST events, 100% assigned tasks and 100%
PMU running time. Unknown samples are 2.28% in aggregate and below 5% per profile.

The actual measurement executables first passed 2,949,120 new oracle outputs
and were decoded. One earlier link failed because a helper JSON library lacked
LTO bitcode; all arms were rebound to the same frozen JSON library before any
test or timing. The failure is retained. Two result readers independently
validated receipts, raw medians, paired ratios and the rejection without
repeating measurements.

[Capture analysis](artifacts/capture-analysis.json),
[profiles and counters](artifacts/diagnostic-analysis.json),
[interpretation and all large-32 blocks](artifacts/insights.json),
[independent review](artifacts/review.json),
[frozen protocol](artifacts/capture-protocol.json) and
[continuation notes](artifacts/next.md) retain the evidence. The
[release qualification](../2026-11-09-fixed-factor-fft-release176/README.md)
documents the admission-order fix used here. Full local results, including raw
perf recordings and decoded samples, remain under
`/home/ubuntu/src/zopro-perf-work/fixed-factor-fft-performance176`.

The archive contains source, receipts, timing samples, counters and profile
summaries. Large decoded logs and raw perf data remain local; the index lists
omissions and hashes. No private proving inputs or proof ledger are published.

The following local 9950X/32-worker full-proof results are inherited from the
accepted build. The supplied M5 CPU/Metal figures are reference measurements
from another machine. This candidate adds no full-proof row; the CPU 10× goal
remains active and unmet for all eight shapes.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.857170|37|24|4.709×|3.055×|3.7|
|2x2|11.052363|59|30|5.338×|2.714×|5.9|
|4x2|15.952018|109|48|6.833×|3.009×|10.9|
|8x2|24.080322|189|74|7.849×|3.073×|18.9|
|12x2|34.012417|284|101|8.350×|2.970×|28.4|
|16x2|40.128912|335|124|8.348×|3.090×|33.5|
|24x2|66.348981|528|176|7.958×|2.653×|52.8|
|merge-24x1|65.829291|507|175|7.702×|2.658×|50.7|

The canonical proof ledger remains 40,154. No candidate is adopted, and no owned job or cloud resource remains live.

[Source and receipt archive](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
