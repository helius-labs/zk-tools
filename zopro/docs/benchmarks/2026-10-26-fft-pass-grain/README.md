# Task grain for Euler FFT packing and product passes

The fixed local screen fails. Primary at 32 workers improves from 8.598 to
8.544 ms, but the physical-16 case has a 7.17% paired regression against
accepted and substantial variability across all three variants. The
four-case paired geometric result also fails against accepted. The candidate
is preserved without adoption or an unchanged timing rerun. No AWS resource
is created for this experiment.

## Change

Two separate ON helpers add `with_min_len` only to Euler FFT input packing
and derivative output combination. Each floor covers 128 scalar positions:
32 packed-input items, 16 eight-row outputs or 32 four-row outputs. Short
inputs can be smaller. This is iterator grain, not a memory limit. The
transform, weights, arithmetic, vector lengths/capacities and reserve/drop
order retain their implementations. The original OFF bodies are preserved;
an audit recovers them byte-for-byte and recovers the ON product body after
removing its adapter and helper dispatch.

Private cfg `zopro_fft_pass_grain` and explicit
`ZOPRO_CPU_FFT_PASS_GRAIN=1` select ON. A process-local `OnceLock` reads the
option once; absence/zero select OFF. No new buffer, key table, point copy,
execution plan, dependency or per-proof allocation is introduced. The
bounded victim-scan prototype is excluded from this candidate.

The prior task census attributed roughly 8,755 primary and 130,234 merge-54
joins to the product FFT before the current source baseline. Those are
instrumented historical observations, not measured join counts for this
candidate. This experiment changes a different boundary from the accepted
canonical-conversion grain and the earlier rejected weighted-reducer grain.

## Complete SDK results

Ryzen 9950X; seven rotated/reversed fresh-process blocks, accepted versus
the same candidate binary OFF and ON. Each process runs three primes and
31 measured fresh SDK requests. The 32-worker cases use CPUs 0–31; the
16-worker case uses CPUs 0–15, one logical CPU per physical core. Loading
is excluded; solving, randomness, proving, internal verification and
serialization are included. All latency completes before separate profiles.
No owned build, profiler, CPU benchmark or bulk audit overlaps latency.

| Case | Accepted ms | OFF ms | ON ms | Paired ON/accepted | Paired ON/OFF |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2 / 32 | 8.598141 | 8.642184 | 8.544290 | 0.993840 | 0.989219 |
| 2x2 / 32 | 11.593448 | 11.708554 | 11.606481 | 0.996478 | 0.994186 |
| merge-54x1 / 32 | 177.372180 | 177.145223 | 176.377783 | 0.997651 | 0.995210 |
| 1x2 / 16 | 10.590246 | 12.129369 | 11.349478 | 1.071692 | 0.945212 |

Times are medians of process medians. Ratios pair matching blocks, so ratios
of displayed medians differ. Primary32 wins 6/7 against accepted and 5/7
against OFF. The predeclared gate requires positive primary and four-case
geometric results against both controls, at least four primary wins, every
case within 2% paired median and 5% per-block regression, and OFF calibration
within 1.03. There is no minimum gain floor.

The four-case ON/accepted paired geometric ratio is 1.007049; ON/OFF is
0.982923. Physical16 violates the regression bounds against accepted and
one per-block bound against OFF. Its process medians range from
10.46–15.85 ms accepted, 10.48–16.05 ms OFF and 10.50–14.02 ms ON. Worker
receipts consistently report 16. The variation limits causal interpretation;
no observations are removed or attributed solely to the code change. All
[failed checks](artifacts/pilot-result.json) and
[physical16 observations](artifacts/physical16-observations.json) are retained.

| Ordinary latency process | Accepted peak MiB | OFF peak MiB | ON peak MiB |
| --- | ---: | ---: | ---: |
| 1x2 / 32 | 111.50 | 112.25 | 114.00 |
| 2x2 / 32 | 276.00 | 268.25 | 268.75 |
| merge-54x1 / 32 | 1961.97 | 1862.58 | 1939.63 |
| 1x2 / 16 | 84.25 | 83.25 | 82.00 |

RSS includes loading and priming, and is not a minimum-RAM or pressure result.
Primary32 CPU time is 199.35 / 198.68 / 198.06 ms per request for
accepted / OFF / ON. Separate profiles show 198.95 ms OFF and 198.32 ms ON;
epoch share is 9.32% / 9.96%, scheduler share 4.49% / 2.30%, and FFT share
4.75% / 5.60%. These samples do not show a broad removal of scheduler cost.
Inlined work can be attributed to enclosing symbols, and shares are sampled
user-cycle weights rather than removable wall time. Nine monotonic-clock
999 Hz captures contain 239,637 samples, one process per capture, with zero
lost records. [Complete profile report](artifacts/perf/report.md).

## Validation and provenance

Both native OFF and ON pass all 111 core tests, including two new tests
against independent sparse-polynomial and scalar-product oracles, multiple
pool sizes, concurrent callers, boundary lengths and 512 KiB worker stacks.
Raw-image tests pass two and cubic tests pass six. Field kernels are
unchanged; the prior 33-test field qualification is inherited, with no new
field-suite execution claimed.

All nine real SDK shapes pass OFF/ON preflight at four workers: 72 requests
and 18 independently Gnark-verified exports. The fixture/freshness inventory
validator runs inside that workflow and again as a mandatory perf check
before timing. Latency adds 2,856 requests and 84 exports; profiles add
660 requests and nine exports. Total: **3,588 SDK requests, 111 distinct
verified exports and 222 proof/public files**. Public fixtures, fresh proof
identities against 2,934 declared priors, and all 452 performance input pins
pass. The final review rehashes every exported member and checks complete
process, worker, sample and verification receipts.

The first source-copy guard rejected the historical qualified snapshot
because its two trailing blank lines differ from the documented production
normalization. This occurred before creating the candidate, compiling or
proving. Copying from the verified current production tree resolved the
source selection; no source or manifest was changed. The final snapshot
contains the 170 accepted files plus the frozen generic CLI/helper, with
only the experimental FFT source changed. Libraries and executable are
pinned in the build/link receipts. Accepted production remains unchanged.

This is a failed local speed screen, with no all-nine latency, portable,
pressure, concurrent whole-SDK, large-circuit or C7a qualification. The
kernel concurrency tests do not establish concurrent proof behavior. The
candidate's 8.544290 ms primary is 4.33 times the supplied 37 ms M5 CPU
reference and 2.81 times the 24 ms Metal reference. Its 2x2 result is
5.08 / 2.58 times the supplied 59 / 30 ms references. These are historical
references, not matched-machine measurements. No merge-54 reference was
supplied. The original all-eight 10x CPU objective remains unmet.

[Protocol](artifacts/experiment.json), [closeout](artifacts/closeout.json),
[source change](artifacts/source.patch),
[build](artifacts/native/result.json),
[proof inventory](artifacts/proof-inventory.json),
[follow-up reading and limits](artifacts/followup-reading.md).

Full local experiment: `/tmp/zopro-fft-pass-grain/`.
Raw perf captures and expanded symbol streams are retained locally, omitted
from Git, and included in the artifact hash index. Executables and compiler
caches are represented by their source/build/link identities.
