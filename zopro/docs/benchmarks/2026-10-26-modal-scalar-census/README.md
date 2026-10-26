# Global modal scalar translation census

Zero is already the most frequent scalar in all 27 fresh query vectors across nine fixtures. Changing the grouping origin to the modal scalar therefore removes no class-formation additions. No aggregate key-point preparation, curve implementation or latency trial follows. Production is unchanged.

The proposed identity is sum(s_i P_i)=c*sum(P_i)+sum((s_i-c)P_i). One key-only aggregate point could let grouping omit a frequent nonzero scalar class instead of zero. This differs from [opposite-scalar grouping](../2026-10-26-opposite-scalar-census/README.md): occurrence frequency, rather than unique class count, determines the potential saving. It requires no lookup table. The population screen rejects the global version before any key transformation.

| Shape | Query | Positions | Zero occurrences | One occurrences | Modal value | Modeled formation additions saved |
| --- | --- | ---: | ---: | ---: | --- | ---: |
| 1x2 | A_G1 | 21006 | 1733 | 646 | zero | 0 |
| 1x2 | B_G2 | 30089 | 1600 | 568 | zero | 0 |
| 1x2 | rawK_diagnostic | 30519 | 1864 | 669 | zero | 0 |
| 2x2 | A_G1 | 36637 | 3067 | 1231 | zero | 0 |
| 2x2 | B_G2 | 52274 | 2763 | 1066 | zero | 0 |
| 2x2 | rawK_diagnostic | 53062 | 3257 | 1267 | zero | 0 |
| 4x2 | A_G1 | 67154 | 5767 | 2450 | zero | 0 |
| 4x2 | B_G2 | 95537 | 5120 | 2111 | zero | 0 |
| 4x2 | rawK_diagnostic | 97044 | 6086 | 2510 | zero | 0 |
| 8x2 | A_G1 | 129064 | 11198 | 4897 | zero | 0 |
| 8x2 | B_G2 | 183347 | 9881 | 4210 | zero | 0 |
| 8x2 | rawK_diagnostic | 186352 | 11825 | 5005 | zero | 0 |
| 12x2 | A_G1 | 190734 | 16719 | 7314 | zero | 0 |
| 12x2 | B_G2 | 270757 | 14770 | 6279 | zero | 0 |
| 12x2 | rawK_diagnostic | 275340 | 17739 | 7470 | zero | 0 |
| 16x2 | A_G1 | 252548 | 22312 | 9867 | zero | 0 |
| 16x2 | B_G2 | 358343 | 19703 | 8484 | zero | 0 |
| 16x2 | rawK_diagnostic | 364584 | 23747 | 10071 | zero | 0 |
| 24x2 | A_G1 | 376992 | 33639 | 15072 | zero | 0 |
| 24x2 | B_G2 | 534619 | 29802 | 12993 | zero | 0 |
| 24x2 | rawK_diagnostic | 544416 | 36138 | 15372 | zero | 0 |
| merge-24x1 | A_G1 | 359220 | 31247 | 14270 | zero | 0 |
| merge-24x1 | B_G2 | 507895 | 26246 | 11917 | zero | 0 |
| merge-24x1 | rawK_diagnostic | 516151 | 31660 | 14295 | zero | 0 |
| merge-54x1 | A_G1 | 804765 | 71043 | 32019 | zero | 0 |
| merge-54x1 | B_G2 | 1137190 | 58977 | 26726 | zero | 0 |
| merge-54x1 | rawK_diagnostic | 1156561 | 71966 | 32074 | zero | 0 |

All 27 vectors pass two independent deterministic weighted-Fr reconstructions of the complete translation identity. Every row is accounted once. Four synthetic all-zero, all-one, one-dominant/random and zero-dominant/random populations add eight field checks. Nine raw/cached-image solution-vector comparisons agree, from 18 explicit public scalar solves. All 204 source/library/key/fixture pins and link/run receipts match after execution. The model counts grouping additions as sum(count-1); key-sum setup, changed scalar digits and output correction are not treated as free.

Raw K is diagnostic rather than accepted folded K/Euler H. Exact grouping populations do not reproduce all sampling/admission choices, and one fixture per shape does not establish a property of arbitrary witnesses. No private scalar, witness, point or proof arrays are exported. No new SDK proof, Gnark export, latency, minimum-RAM, physical-device or AWS result is claimed.

A possible next screen is a lossless local difference basis: replace (P,Q) by (P+Q,Q) and (a,b) by (a,b-a). The point count stays fixed, arbitrary inputs remain valid, and equal nearby scalars produce zeros. A recursive binary transform can generalize this to bounded blocks. This is not yet implemented or measured. It must count the effect on duplicate grouping and digit work, since extra zeros alone can conceal more distinct nonzero scalars.

Accepted primary remains historically 8.594267 ms and merge54 178.891799 ms. The unchanged [accepted CPU/Metal comparisons for all supplied shapes](../2026-10-25-grouping-module/artifacts/perf/report.md) remain authoritative; all eight CPU 10× targets are unmet.

[Protocol](artifacts/experiment.json), [results](artifacts/results.json), [driver](artifacts/probe.rs), [workflow](artifacts/plan.json), [input pins](artifacts/input-pins.json).
