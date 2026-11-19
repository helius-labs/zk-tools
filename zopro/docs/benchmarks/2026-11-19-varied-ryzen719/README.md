# Six 1×2 transactions on sixteen Ryzen cores

The same six public transactions used in the C7a phase study ran locally on sixteen physical Ryzen cores. Zopro used the production server settings with a four-core frontend pool. Aeglos used sixteen threads and a prepared key. Six paired blocks per input used seventeen timed, verified requests after three warmups. The driver independently checked every exported proof and the expected public input. These results qualify transaction sensitivity on the local machine.

| Transaction | Zopro / Aeglos ms | Paired difference and Zopro wins | Peak MiB |
|---|---:|---:|---:|
| catalog | 8.741 / 12.772 | -31.42% (6/6) | 54.2 / 410.9 |
| heldout | 8.533 / 12.870 | -34.00% (6/6) | 52.2 / 412.9 |
| v25 | 8.487 / 12.737 | -33.77% (6/6) | 52.5 / 411.8 |
| v26 | 8.570 / 12.970 | -33.45% (6/6) | 53.0 / 408.4 |
| v27 | 8.512 / 12.874 | -33.74% (6/6) | 52.2 / 413.8 |
| v28 | 8.479 / 12.766 | -33.69% (6/6) | 53.8 / 407.3 |

[AWS allocation and phase measurements](../2026-11-19-varied-phases716/README.md) · [Samples](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
