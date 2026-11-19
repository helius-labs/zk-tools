# Frontend width and G2 geometry across three 1×2 transactions

The production binary ran on sixteen physical Ryzen cores or all thirty-two logical CPUs of the same host. At thirty-two workers the frontend used four or sixteen physical cores and G2 used its existing two- or four-range policy. The four-range case also compared parallel and serial scalar decomposition on the sixteen-core frontend setting. The sixteen-worker production control uses the ordinary two-range kernel due to its width gate. Every input was measured under every configuration, so the table compares the same transaction differences at each placement. Each of six blocks timed twenty-one complete proofs per process. The withdrawals and varied Merkle paths came from independently generated public requests. All 126 exports passed independent verification.

| Configuration | Catalog / second / withdrawal ms | Second vs catalog (faster blocks) | Withdrawal vs catalog (faster blocks) |
|---|---:|---:|---:|
| physical | 8.734 / 8.457 / 8.496 | -3.13% (6/6) | -2.40% (6/6) |
| f4-r2 | 7.607 / 7.510 / 7.502 | -1.34% (4/6) | -1.22% (6/6) |
| f4-r4 | 7.793 / 7.597 / 7.582 | -2.52% (6/6) | -3.01% (6/6) |
| f16-r2 | 6.945 / 6.960 / 6.928 | +0.41% (2/6) | +0.08% (2/6) |
| f16-r4 | 7.214 / 7.036 / 7.034 | -2.53% (5/6) | -2.21% (6/6) |
| f16-r4-parallel | 7.173 / 6.969 / 7.002 | -2.35% (5/6) | -1.78% (6/6) |
| aeglos32 | 12.771 / 12.845 / 12.990 | +0.05% (3/6) | +1.75% (1/6) |

[Same-input 16-core engine comparison](../2026-11-19-varied-ryzen719/README.md) · [Samples](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
