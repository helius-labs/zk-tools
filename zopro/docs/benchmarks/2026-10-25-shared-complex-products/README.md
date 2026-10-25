# Shared raw products for quadratic-extension multiplication

A three-product native Fq2 implementation improved isolated arithmetic but
regressed complete SDK requests. The integration was not adopted. The accepted
implementation and its qualified 8.594267 ms local primary result are unchanged.

The prototype shares three integer Karatsuba products between the real and
imaginary outputs, followed by two Montgomery reductions. It adds no key table,
heap allocation or preparation. The [component report](artifacts/component-report.md)
contains the signed-carry range argument and independent field checks.

## Full request comparison

AMD Ryzen 9 9950X; 32 Rayon workers. Seven rotated/reversed fresh-process blocks,
three priming requests and 31 measured, internally verified requests per process.
Accepted, freshly compiled OFF and ON use the same SDK driver and configuration.
The only ON arithmetic switch is `zopro_shared_complex_products`.

| Shape | Accepted ms | Fresh OFF ms | ON ms | Paired ON/accepted |
| --- | ---: | ---: | ---: | ---: |
| 1x2 | 8.627238 | 8.661483 | 9.090140 | 1.051684 |
| 2x2 | 11.826572 | 11.786080 | 12.192210 | 1.032272 |
| 4x2 | 16.231862 | 16.106292 | 16.584181 | 1.022741 |
| 8x2 | 24.625740 | 24.644355 | 25.234214 | 1.020646 |
| 12x2 | 34.647865 | 34.798234 | 35.167746 | 1.015005 |
| 16x2 | 41.071827 | 40.819686 | 41.761045 | 1.014811 |
| 24x2 | 68.254036 | 68.576518 | 70.260092 | 1.024239 |
| merge-24x1 | 66.982068 | 68.464317 | 69.023978 | 1.031941 |
| merge-54x1 | 177.908327 | 179.571562 | 179.444866 | 1.010871 |

Times are medians of process medians. Ratios are medians of block-paired ratios.
Primary ON loses all seven blocks against both controls: +5.17% against accepted
and +4.16% against fresh OFF. The all-nine geometric means are 1.024845 and
1.021192 respectively. The original prospective gate rejects the integration;
there is no timing retry, changed threshold or source promotion.

An additional primary case uses 16 workers pinned to distinct physical cores.
Its OFF/accepted calibration factor is 1.160868, outside the declared 1.03.
All observations remain in the artifacts; that case supports no performance
improvement claim. It is not substituted for the stable 32-worker comparison.

## Component gain and integration costs

The isolated component uses 31 blocks with a single thread pinned to CPU 2.
Shared/fresh-old ratios are 0.804438 for a dependent chain, 0.786840 for four
independent streams, 0.889034 for product-plus-scaling and 0.961072 for an affine
arithmetic expression. Every pattern improves in all 31 blocks. These workloads
are not complete curve operations or verified proofs.

Static inspection of the actual linked G2 prefix caller shows fewer IFMAs but
more surrounding instructions and stack references. For the `true` specialization:

| Static observation | OFF | ON |
| --- | ---: | ---: |
| Instructions in symbol | 8,326 | 9,367 |
| IFMAs in symbol | 1,450 | 1,350 |
| Symbol bytes | 50,288 | 58,580 |
| Instructions referencing RSP-relative operands | 1,377 | 1,960 |
| Sum of literal prologue stack subtractions | 5,696 | 12,096 |

These are static counts, not execution counts, spill counts or total stack
bounds. The larger code and frames are consistent with register-pressure costs;
the measurements do not isolate a causal share of the request regression.
All six inspected G2 symbols and their disassemblies are retained.

Separate warm profiles cover primary and merge-54, three blocks per variant.
Primary summed CPU time is 199.044 / 199.517 / 208.073 ms per proof for
accepted / OFF / ON. Sampled G2 arithmetic shares are 29.28 / 28.74 / 32.04%.
Instrumented times and inlined symbol attribution do not replace the latency
comparison or identify removable wall time. Primary profile whole-process RSS
maxima are 129.25 / 130.50 / 132.25 MiB; they are not minimum RAM measurements.

## Validation and retained evidence

The exact field prototype passes 35 unfiltered native field tests. Two new named
tests check 729,088 result lanes against Arkworks, covering Fq/Fr, R256/R260,
four/eight lanes, lazy and boundary values, and chained results. Generator
consistency passes. Both SDK configurations pass 109 native core tests, two raw
prover tests and six cubic prover tests. The unchanged field source is inherited
from the component validation rather than counted as a fresh SDK test run.

The complete experiment performs 8,256 SDK requests and exports 246 distinct
proofs independently verified by Gnark. All 492 proof/public files retain fixture
bindings and freshness checks. Eighteen profiles contain 520,420 samples, one
sampled process per capture and no lost records. Source, executable, library,
image, request and verifier guards pass.

The field-test orchestration initially failed to recognize names split by
`--nocapture` output. The tests had passed; the corrected Rust parser reused the
successful command receipt. The original failure and subsequent attempt records
remain. The final Rust tooling passes 22 unit tests, formatting and Clippy.

No new portable, memory-pressure, concurrent-request, 250/4999 or AWS qualification
is attached to this rejected integration. The supplied M5 CPU/Metal comparison
for all eight shapes is in the [generated SDK report](artifacts/perf/report.md).
The original tenfold CPU objective remains unmet.

[SDK decision](artifacts/sdk-result.json),
[component decision](artifacts/component-result.json),
[prospective SDK protocol](artifacts/sdk-experiment.json),
[linked code generation](artifacts/linked-g2-codegen.json),
[C7a scaling audit](artifacts/c7a-scaling-review.json),
[artifact hashes](SHA256SUMS.json).

Numeric results, protocols, prototype sources, proof exports, command receipts
and disassembly are published here. Raw perf captures and expanded sample
streams remain in the local `artifacts` archive; `SHA256SUMS.json` covers that
complete archive. The Git checkout contains the remaining artifacts.
