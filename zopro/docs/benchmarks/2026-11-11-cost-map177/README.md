# Cost map and final-fold feasibility screen

The accepted binary-radix final fold takes **32–36 µs per query** in this standalone probe. Deprioritize moving its scaling into window tasks: this service scale is small beside the primary proof's remaining **4.122 ms** target gap, and distributing the fold would add doublings. This is a research-priority decision, not a measured candidate slowdown or a strict whole-proof bound.

The probe compiled in **2.339 seconds** and ran in **0.578 seconds**. These are command durations, excluding preparation and analysis. It required no prover rebuild, SDK capture or new proof. This applies the revised [playbook](../../cpu-performance-playbook.md): test the mechanism cheaply before expensive qualification.

## Current cost map

Reuse the three qualified OFF cycle profiles from the [G2 coordinate trial](../2026-11-11-g2-coordinate-leaf177/README.md). This fresh control shares the accepted release arithmetic but is not the exact accepted SDK binary. All **37,911 samples** are accounted for:

|Symbol category|Samples|Share|
|---|---:|---:|
|G1 streaming additions|7,744|20.43%|
|G2 streaming additions|6,069|16.01%|
|Affine reduction layers|3,846|10.14%|
|Inverse routines|3,244|8.56%|
|Epoch symbols|3,698|9.75%|
|Rayon symbols|1,696|4.47%|
|FFT symbols|1,910|5.04%|
|Solver symbols|621|1.64%|
|Unknown|652|1.72%|
|Other|8,431|22.24%|

These are symbol-resident CPU samples, including effects of inlining, not exclusive critical-path time. Scheduler and epoch samples are not automatically removable overhead. Halving only the two streaming categories would remove about 18.22% of this CPU-sample proxy; it does not predict an 18.22% proof latency reduction.

Five current source hashes exactly match the prior [stream arithmetic model](../2026-11-06-stream-work-census176/README.md). For `g = floor(n/8)` full groups and `t` tail points, that model remains:

- G1: `(6g+5)V + (5t+1)M + tS + I`.
- G2: `(7g+5)V + 5gF + (8t+1)M + 2tS + 4tFs + I`.

`V` is an eight-lane field multiplication, `F` a fused two-product operation, and `M`, `S`, `Fs`, `I` are distinct scalar operations. These counts are neither interchangeable units nor a latency lower bound. The bindings are retained in the artifacts; unchanged evidence was reused.

## Final-fold probe

One thread pinned to CPU2 on the local Ryzen 9950X; Rust 1.98.1, `znver4`, optimization level 3, fat LTO and one codegen unit. The probe links the retained coherent OFF field library. Six predetermined contexts use eight deterministic generator-weighted input banks each. Zero and mixed banks receive correctness checks; six ordinary banks supply timing inputs. Eighteen primes precede five blocks of 512 folds. Every output is black-boxed, with the final output checked after each timer.

|Curve|Window width|Windows|Median fold µs|
|---|---:|---:|---:|
|G1|12|22|33.124|
|G2|12|11|34.815|
|G1|13|20|32.176|
|G2|13|10|33.497|
|G1|14|19|32.524|
|G2|14|10|35.645|

All six output checks pass against a separately calculated scalar-power sum multiplied by the generator. The oracle shares the accepted field library; this is not independent field qualification. Inputs are synthetic projective window sums, not captured query outputs. Results establish standalone service scale, not SMT behavior, candidate performance, RSS, minimum RAM or integrated critical-path cost.

## Accepted full-proof reference

The following measurements are inherited from the latest matched accepted SDK campaign on this machine; none was rerun for this screen. M5 CPU and Metal references were supplied separately on different hardware.

|Shape|Accepted CPU ms|Peak RSS MiB|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|
|1×2|7.821726|127.50|37 / 4.730×|24 / 3.068×|3.7|
|2×2|10.887879|272.00|59 / 5.419×|30 / 2.755×|5.9|
|4×2|15.852187|306.75|109 / 6.876×|48 / 3.028×|10.9|
|8×2|24.217576|502.50|189 / 7.804×|74 / 3.056×|18.9|
|12×2|34.013966|719.00|284 / 8.350×|101 / 2.969×|28.4|
|16×2|40.059851|836.75|335 / 8.362×|124 / 3.095×|33.5|
|24×2|66.586815|1292.75|528 / 7.929×|176 / 2.643×|52.8|
|merge 24×1|66.228695|1183.32|507 / 7.655×|175 / 2.642×|50.7|

Merge 54×1 remains 163.713679 ms, with no supplied M5 reference. The goal remains active and unmet, 0/8. The primary needs a further 52.70% latency reduction. A small helper-boundary improvement alone cannot plausibly close that gap.

## Next research question

The earlier [packed ternary query](../2026-11-10-packed-ternary176/README.md) used about 8.15% fewer instructions and 8.23% fewer user cycles in its diagnostic panel, yet its primary 32-worker query was 5.64% slower. Its single-worker query improved about 10.1%. That is a useful work-versus-parallel-latency discrepancy to explain before selecting another implementation.

Review of the existing [ternary service traces](../2026-11-07-g2-ternary-span176/README.md) supplies a narrower question. Both algorithms have eleven windows and two point partitions; fewer top-level tasks do not explain the difference. In qualified single-worker traces of the earlier reciprocal variant, ternary removes 7.776 ms of indexed-top service but increases streaming service from 23.023 to 24.155 ms, weighted leaves from 3.047 to 4.719 ms, and merge leaves from 0.897 to 1.427 ms. These are separately calculated medians, not additive phase accounting. The later packed variant preserves that geometry but changes recoding.

Thus aggregate work savings can conceal a redistribution from one specialized window into the ordinary windows and their reductions. That is a hypothesis about the threaded loss, not its established cause: all eight threaded trace contexts failed calibration, and the observer even reversed the ordinary ranking. Do not repeat that intrusive observer or use its threaded timelines as unperturbed evidence.

The next useful question is whether the indexed-top saving can be retained without the added ordinary-window/reduction work. Review prior top-window and representation experiments before choosing an implementation; require a distinct operation or dependency reduction. This does not promote the old candidate or justify rerunning its unchanged codec. No new candidate is selected by this report.

The source/model bindings, workflow receipts, sample conservation, six fold outputs and unchanged proof ledger pass the retained Rust reader. The canonical ledger remains **53,917** verified exports, SHA-256 `a033132ee87cf5d5260c396acf1894bbb1afc95c07d8ba22c55ed9c8e5a85873`. No production code changed; no new SDK, AWS or Aeglos measurement was made. No push or publication is authorized.

[Results](artifacts/result.json), [protocol](artifacts/protocol.md), [probe source](artifacts/fold.rs), [reader source](artifacts/analyze.rs), [plan](artifacts/plan.json), [source bindings](artifacts/source-bindings.json), [pins](artifacts/pins.json), [workflow receipts](artifacts/run/result.json), [checksums](SHA256SUMS). Reproduction requires the pinned libraries and prior evidence under `/home/ubuntu/src/zopro-perf-work/`; binaries and private inputs are not included.
