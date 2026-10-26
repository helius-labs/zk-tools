# Opposite-scalar grouping opportunity

Fresh accepted-library scalar solves show little additional grouping opportunity from opposite values. Primary A removes one class out of 17,454; primary B removes three out of 26,142. The largest A/B class reduction across nine fixtures is below 0.08%. No curve implementation or performance trial follows from this census. Production is unchanged.

Current grouping merges equal nonzero scalars. This read-only census also groups x with -x, independently computes canonical integer p-x, checks field negation and sign reconstruction, and validates two deterministic weighted field linear functionals per vector. A signed base sum would require extra point formation before windowing, so class reductions are not free point-operation savings or predicted latency.

| Shape | Query | Positions | Equal nonzero classes | Sign classes | Extra class reduction % |
| --- | --- | ---: | ---: | ---: | ---: |
| 1x2 | A_G1 | 21006 | 17454 | 17453 | 0.005729 |
| 1x2 | B_G2 | 30089 | 26142 | 26139 | 0.011476 |
| 1x2 | rawK_diagnostic | 30519 | 26194 | 26191 | 0.011453 |
| 2x2 | A_G1 | 36637 | 19268 | 19263 | 0.025950 |
| 2x2 | B_G2 | 52274 | 28852 | 28845 | 0.024262 |
| 2x2 | rawK_diagnostic | 53062 | 28910 | 28903 | 0.024213 |
| 4x2 | A_G1 | 67154 | 20386 | 20381 | 0.024527 |
| 4x2 | B_G2 | 95537 | 30544 | 30537 | 0.022918 |
| 4x2 | rawK_diagnostic | 97044 | 30614 | 30607 | 0.022865 |
| 8x2 | A_G1 | 129064 | 25236 | 25227 | 0.035663 |
| 8x2 | B_G2 | 183347 | 37850 | 37839 | 0.029062 |
| 8x2 | rawK_diagnostic | 186352 | 37940 | 37929 | 0.028993 |
| 12x2 | A_G1 | 190734 | 30704 | 30689 | 0.048854 |
| 12x2 | B_G2 | 270757 | 46062 | 46045 | 0.036907 |
| 12x2 | rawK_diagnostic | 275340 | 46167 | 46150 | 0.036823 |
| 16x2 | A_G1 | 252548 | 34326 | 34309 | 0.049525 |
| 16x2 | B_G2 | 358343 | 51597 | 51578 | 0.036824 |
| 16x2 | rawK_diagnostic | 364584 | 51727 | 51708 | 0.036731 |
| 24x2 | A_G1 | 376992 | 43958 | 43933 | 0.056872 |
| 24x2 | B_G2 | 534619 | 66221 | 66194 | 0.040773 |
| 24x2 | rawK_diagnostic | 544416 | 66391 | 66364 | 0.040668 |
| merge-24x1 | A_G1 | 359220 | 41272 | 41248 | 0.058151 |
| merge-24x1 | B_G2 | 507895 | 62005 | 61980 | 0.040319 |
| merge-24x1 | rawK_diagnostic | 516151 | 62259 | 62234 | 0.040155 |
| merge-54x1 | A_G1 | 804765 | 75952 | 75898 | 0.071098 |
| merge-54x1 | B_G2 | 1137190 | 114985 | 114930 | 0.047832 |
| merge-54x1 | rawK_diagnostic | 1156561 | 115539 | 115484 | 0.047603 |

All 27 vectors pass 54 weighted field checks and 25326731 integer/sign checks. Nine raw/cached-image output-vector comparisons agree, from 18 explicit public scalar solves. Synthetic zero, cancellation, duplicates and 128 random opposite pairs also pass. All 202 source/library/key/fixture pins and workflow receipts match after execution. No witness, private scalar, point or proof arrays are exported.

Raw K is diagnostic: it is not the accepted folded K/Euler H hot path. A/B scalars use the accepted solver, but this census models exact equality classes rather than reproducing every grouping sampling/admission choice. One established fixture per shape cannot prove absence of opportunity for arbitrary inputs. No new proof, Gnark export, benchmark latency, minimum-RAM, mobile or AWS result is claimed.

A distinct follow-up will examine occurrence frequencies for scalar translation. Given one key-only point sum S, the identity sum(s_i P_i)=cS+sum((s_i-c)P_i) can omit the most frequent scalar class rather than zero. Unlike opposite-value grouping, this could remove a large class-formation tree. This is unimplemented and has no performance result; input-dependent regrouping, key-sum setup, memory admission and full-prover costs must be included.

Accepted primary remains historically 8.594267 ms and merge54 178.891799 ms. All eight supplied CPU 10× targets remain unmet. The unchanged [accepted CPU/Metal comparisons](../2026-10-25-grouping-module/artifacts/perf/report.md) remain authoritative.

[Protocol](artifacts/experiment.json), [counts and closing audit](artifacts/results.json), [driver](artifacts/probe.rs), [workflow](artifacts/plan.json), [source/input pins](artifacts/input-pins.json).
