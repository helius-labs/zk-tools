# Handing G2 work from the frontend to the main pool

The solver, gather and FFT stay on the frontend through a single call. The ready G2 scalar vector moves to a scoped main-pool job once. Low-memory calls keep the original dependency order.

| Shape | Selected mode | Release ms | Disabled ms | Defer ms | Selected ms | Paired vs release | Faster blocks | Paired vs defer | Faster blocks | Peak MiB | M5 CPU / Metal gain |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | handoff-early | 7.657 | 7.672 | 7.670 | 8.024 | +4.02% | 1/9 | +4.27% | 1/9 | 78 | 4.61× / 2.99× |
| 2x2 | handoff | 10.801 | 10.671 | 10.678 | 11.884 | +10.02% | 1/9 | +11.17% | 0/9 | 144 | 4.96× / 2.52× |

[Local process samples](artifacts/local.json.gz) · [Source](artifacts/source.patch.gz) · [Protocol](artifacts/protocol.md)

[C7a confirmation and Aeglos comparison](aws.md)
