# Page size of immutable proving-key images on Ryzen

This experiment changes the page tables used for the existing proving key. An aligned read-only shared mapping of one tmpfs copy uses 2 MiB pages. A separate byte-identical tmpfs copy uses base pages at the same alignment. Both are populated before setup. The benchmark checks the kernel’s ShmemPmdMapped count for the exact mapping before priming and requires full coverage of the aligned interior for the large-page arm and zero coverage for the base-page arm. A same-binary ordinary file mapping and the earlier experimental executable supply full-proof controls. All remaining key material uses its usual backing. Temporary copies are removed after each shape. Peak RSS is per process and excludes the other fixture’s tmpfs pages. Copying and page preparation are outside warm latency.

Five paired blocks per transaction screen three reference keys. Seven fresh paired blocks are scheduled on a key only when its large-page arm beats both base pages and the earlier executable on paired median and in at least three of five blocks on each transaction. Each process measures eleven verified requests after three warmups. The M5 CPU and Metal references apply to the catalog inputs.

| Shape | Stage | Input | Large / base / ordinary / earlier ms | vs base | vs earlier | Large-page MiB | Peak MiB large / base | vs M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 12x2 | screen | catalog | 28.450 / 28.377 / 28.650 / 28.317 | +0.26% (1/5) | +0.72% (1/5) | 166 | 571.7 / 570.8 | 9.98× / 3.55× |
| 12x2 | screen | second | 28.526 / 28.624 / 28.709 / 28.397 | +0.09% (2/5) | +0.03% (2/5) | 166 | 577.9 / 571.0 | — |
| 16x2 | screen | catalog | 37.137 / 36.557 / 37.164 / 37.215 | +1.09% (1/5) | -0.21% (3/5) | 210 | 781.9 / 785.2 | 9.02× / 3.34× |
| 16x2 | screen | second | 36.165 / 36.296 / 36.939 / 37.525 | -0.31% (3/5) | -3.30% (4/5) | 210 | 777.4 / 772.3 | — |
| merge-24x1 | screen | catalog | 60.866 / 61.833 / 61.626 / 62.477 | -1.25% (5/5) | -2.58% (5/5) | 318 | 1116.8 / 1096.0 | 8.33× / 2.88× |
| merge-24x1 | screen | second | 55.410 / 56.188 / 55.574 / 56.872 | -0.86% (4/5) | -0.88% (4/5) | 318 | 1080.1 / 1058.2 | — |
| merge-24x1 | confirm | catalog | 60.778 / 61.687 / 61.078 / 61.261 | -1.83% (7/7) | -0.77% (7/7) | 318 | 1116.9 / 1121.5 | 8.34× / 2.88× |
| merge-24x1 | confirm | second | 55.425 / 57.274 / 55.888 / 55.778 | -2.48% (7/7) | -0.80% (6/7) | 318 | 1089.2 / 1069.5 | — |

[Observations](artifacts/observations.json.gz) · [Mapping source](artifacts/source.patch) · [Verification](artifacts/verification.json)
