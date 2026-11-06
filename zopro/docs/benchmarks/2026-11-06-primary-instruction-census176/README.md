# Accepted primary instruction census

The saved accepted-build profiles put more sampled instructions in borrowed G1 than borrowed G2. At 32 workers those functions contain 2,472 and 1,913 samples respectively. Instructions with explicit RSP operands account for 105/2,472 (4.25%) and 188/1,913 (9.83%). **These observations do not establish stack traffic as the dominant cost or identify removable latency.** No runtime change or new performance measurement is made.

This audit maps the twelve most-sampled defined executable functions to their exact linked instructions. The two existing accepted 1×2 profiles contain 8,601/12,236 samples at 16/32 workers; the selected functions cover 6,950/9,470. All 16,420 selected samples match exact instruction starts, with no interior or unresolved addresses. Every other sample remains accounted for.

## Sampled functions

Order is by combined sample count across the two profiles. Percentages and counts describe these captures, not proof-wall fractions, exclusive CPU service or cross-topology speedup.

| Function | 16-worker samples | 32-worker samples | 32-worker explicit RSP-operand samples |
|---|---:|---:|---:|
| Borrowed G1 stream | 2,160 | 2,472 | 105 |
| Borrowed G2 apply | 1,569 | 1,913 | 188 |
| Scalar Fq inverse | 742 | 934 | 49 |
| G2 tree layer | 560 | 713 | 180 |
| Epoch with_handle | 230 | 990 | 0 |
| G1 tree layer | 424 | 658 | 72 |
| G1 balanced feed | 422 | 567 | 29 |
| G1 projective add_affine | 278 | 358 | 22 |
| G2 window feed | 203 | 325 | 23 |
| G1 MSM closure | 161 | 230 | 22 |
| Vector inverse_nonzero | 125 | 146 | 16 |
| Worker take_local_job | 76 | 164 | 1 |

Borrowed G1 has 4,428 static instruction sites in 26,230 bytes; borrowed G2 has 8,591 in 51,862 bytes. These are the accepted SDK's functions, not the diagnostic adapters in the earlier control-code audit. Equal source does not imply identical code across those different consumers.

Within borrowed G1 at 32 workers, 1,013 samples land on IFMA instructions and 1,065 on other vector instructions. Borrowed G2 has 769 IFMA and 777 other-vector samples. The remaining classes include moves, shuffles, address calculation, control and scalar instructions. The full analysis preserves every sampled instruction and operand, per-thread counts, static classes and all unselected samples. It does not assign a causal cycle cost to any class.

RSP counts identify explicit stack-base operands only. They can include required data as well as spills. RBP is not assumed to be a frame pointer. Memory operands on IFMA instructions are kept separately; address calculation and NOPs are not classified as memory reads. Implicit stack operations are not included in these explicit-operand counts. Sampling skid, out-of-order execution and inlining prevent treating these categories as a stall decomposition or a bound on optimization gains.

The larger G1 presence does not establish that A or K/H determines wall time. Rayon callbacks can run one another's work while waiting. The [coarse phase report](../2026-11-06-coarse-proof-phase-universal176/README.md) remains a separate, limited observation of that behavior. Its combined qualification still fails at 16 workers; this census does not rescue it.

## Address and source checks

The executable is the accepted SDK, SHA-256 `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. Raw profile, decoded IP, MMAP/task/event/loss, producer analysis/review and production-source identities are bound by 218 pins. All 176 production files remain exact.

ELF64 executable load segments map file page 0xe4000 to virtual page 0xe5000. The helper accounts for that 4 KiB difference when deriving each PIE load bias. A separate reader independently intersects the valid bias intervals implied by 8,460/11,984 known Rust-symbol samples. Each intersection admits exactly one page-aligned bias, matching the ELF/MMAP derivation. Every own-executable sample resolves to the declared symbol range.

One source-pinned Rust/xtask command runs once. Its thirteen read-only subcommands are one symbol listing and twelve disassemblies; they add no profile recording. Independent review reproduces the ranking, 37,973 static instruction sites, every selected address/mnemonic/operand/class, all per-PC and per-profile counts and sample conservation. Both analysis and review succeed on their first executions. No native test, benchmark, successful decode or proof is rerun.

The original captures' PID/TID/CPU, period and zero-loss qualifications are rechecked against their saved data. Their absolute sampled-period coverage remains unvalidated. Single-capture shares supply no confidence interval. Source-line attribution is not claimed from the limited DWARF information. Existing raw profiles remain local with recorded hashes; exact decoded streams and disassemblies accompany this report.

## Research decision

This evidence does not support another blanket loading, packing, spill-removal or scheduler rewrite. The [direct gather trial](../2026-11-06-g2-aos-gather176/README.md), earlier paired packing, shared-point windows, native buckets, outlines and idle-policy results stay closed. A new candidate must remove specific arithmetic, dependency or data-lifetime work and then pass complete callers and proofs.

The current quotient code already uses the Euler derivative product and folded C correction. Earlier [native CRT-Euler](../2026-11-05-crt-euler-native176/README.md), [query rank](../2026-10-25-query-rank-census/README.md), [semantic expression](../2026-10-31-semantic-cse176/README.md), [solver-to-A streaming](../2026-11-06-solver-a-stream176/README.md) and derivative/evaluation-basis studies were checked before considering another algebraic change. None is reopened unchanged. This census establishes no new 2× path.

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

[Full analysis](artifacts/analysis.json.gz), [independent review](artifacts/review.json), [census helper](artifacts/census.rs), [reviewer](artifacts/review.rs), [design](artifacts/design.md), [pins](artifacts/pins.json), [workflow](artifacts/plan.json), [exact source/command/disassembly/decoded-profile archive](artifacts/source-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS). Raw profiles, executables, libraries and private proof/key/witness inputs are excluded.
