# Runtime-layout control diagnostic

Fixing the executable address layout did not remove the control discrepancy. The unchanged fresh-control executable remained about 14–15% slower than the accepted executable in this complete-subgroup-call harness, despite identical executable bytes and data. No arithmetic candidate or production change is adopted.

The preceding [subgroup comparison](../2026-11-10-seed-naf-callers176/README.md) failed its fresh-control calibration. This follow-up tests a different hypothesis: whether randomized runtime address placement explains that discrepancy. It reuses the exact two binaries and fixtures without rebuilding them. It is a diagnostic of the comparison, not another estimate of the signed-seed candidate's gain.

| Runtime layout | Accepted REF, µs | Fresh OFF, µs | Paired OFF/REF | 95% interval |
|---|---:|---:|---:|---:|
| Normal | 28.630 | 32.685 | 1.14363 | 1.14100–1.14649 |
| Fixed | 28.779 | 32.962 | 1.14595 | 1.14332–1.14834 |

Times are medians of 15 process medians. Ratios are paired geometric means, so they need not equal ratios of the rounded table entries. Each control lost all 15 matched blocks at both settings. Fixed/normal was 1.00315 for REF (1.00152–1.00477) and 1.00518 for OFF (1.00200–1.00803). The predeclared interaction, `(fixed OFF/REF)/(normal OFF/REF)`, was **1.00202** (0.99952–1.00442); its interval includes 1. The fixed-layout profiles' executable MMAP starts were equal at **0x55555556d000**. Different executable base addresses therefore do not suffice to explain this gap. Other process mappings, backing pages, runtime state and specific hardware mechanisms remain unisolated.

A direct ELF reader found byte-identical program headers, all executable load-segment bytes (444,240 bytes), read-only constants, relocation entries and initialized writable data. The first read-only load segment differed at only 23 bytes: the 20-byte build ID and three bytes of the ELF section-header offset. These bookkeeping differences are recorded, not assumed irrelevant to every possible tool. A separate GNU objcopy extraction confirms `.text` and `.rodata` equality; original files were checked unchanged afterward. Both binaries also have identical saved subgroup disassembly after removing the path header. Thus there is no identified arithmetic instruction or relative function-placement difference between the controls.

| Counter panel | Cycles, billion | Instructions, billion | Branch misses |
|---|---:|---:|---:|
| Normal REF | 5.01052 | 15.23552 | 39,477 |
| Normal OFF | 5.79314 | 15.23554 | 854,187 |
| Fixed REF | 5.01069 | 15.23552 | 51,970 |
| Fixed OFF | 5.79591 | 15.23551 | 833,227 |

These are single diagnostic panels. The instruction totals are almost equal, and the branch-miss discrepancy persists with equal executable addresses. The aggregate counters do not identify a responsible branch or establish the cycle cost of misprediction. The cycle difference must not be attributed entirely to the extra branch misses. No speculation about a particular predictor, cache or physical page is treated as a finding.

The prospective four-cell schedule, both readers, retained bootstrap code and input identities were frozen before preflight. All **92 serialized Rust xtask commands** completed on the first execution: four checked caller preflights, 60 latency processes, four PMU panels, four cycle profiles and 20 offline commands. Each timing process uses 31 samples, 1024 calls per sample, three primes and one worker on CPU2, with a requested 512 KiB stack. Input loading, allocation, oracles, startup, primes, barriers and checking are outside active-worker timing. Every boolean output is checked after clocks; coordinator wave time and process-scoped resources are retained separately. No native qualification was replayed.

Fixed layout uses `setarch x86_64 -R` only in this diagnostic. There is no recommendation to change production address randomization. PMU panels have 100% running time. Cycle profiles use a fixed user-cycle period of 16,000,000 and 262,144 calls; all four have at least 2,586 samples, at most 0.067% unknown symbols, zero lost records and checked PID/TID/CPU/executable-MMAP ownership. All recordings finish before offline decoding. No helper, build or analysis overlaps capture.

The separate same-agent reader reconstructs all 72 process medians, five declared contrasts and eight diagnostic panels, and performs the GNU section extraction. The 20,000-replicate paired-log bootstrap is shared unchanged; float serialization comparisons allow eight machine epsilons without changing statistics or thresholds. Both readers pass first execution. This is not an external audit.

[Static ELF audit](artifacts/audit.json), [runtime analysis](artifacts/analysis.json), [reader result](artifacts/review.json) and [complete evidence](artifacts/evidence.tar.gz) contain authored sources, fixed plans, pins, exact receipts, all latency samples, raw PMU data, all four raw perf captures, decoded samples, mappings and loss records. [Member hashes](artifacts/public-members.json) were checked after extraction. Exact original native binaries and synthetic point payloads remain at `/home/ubuntu/src/zopro-perf-work/seed-naf-callers176`, with their hashes bound by the archived pins. Their source and qualification are linked from the preceding report.

Production and the 47,437-entry proof ledger remain unchanged. This study produces zero SDK requests or Gnark proof exports and starts no AWS resource. Accepted 1×2 proof observations remain approximately 7.86 ms locally and 11.24 ms on c7a; these are not new proof measurements. All eight original CPU10× targets remain unmet.

Next: investigate executable backing or runtime state with a distinct controlled intervention, and inspect instruction-fetch/front-end evidence before attributing the gap to branches. Do not repeat this unchanged layout study or widen the earlier calibration band. A source optimization must reduce work in a material proof consumer and pass the existing coherent full-SDK, ALL8 plus 54, independent verification and memory checks before adoption.
