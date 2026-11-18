# The 1×2 latency remaining after the FFT and curve work are separated

The 10× target for 1×2 is 3.7 ms. Using the earlier same-host C7a trace, remove the entire interval from the product FFT returning to the canonical core returning. What remains is startup and completion outside the canonical core plus the core prefix through the FFT. This is a counterfactual for the existing schedule. Changing the schedule or these stages changes the number. Instrumentation was enabled.

| Cores | G2 ranges | Core prefix through FFT ms | Outside core ms | Remaining ms | Remaining over 3.7 ms |
|---:|---:|---:|---:|---:|---:|
| 64 | 2 | 2.850 | 1.196 | 4.047 | 153/153 |
| 64 | 4 | 2.868 | 1.176 | 4.040 | 153/153 |
| 72 | 2 | 2.873 | 1.213 | 4.092 | 153/153 |
| 72 | 4 | 2.859 | 1.205 | 4.062 | 153/153 |
| 96 | 2 | 2.958 | 1.247 | 4.196 | 153/153 |
| 96 | 4 | 2.989 | 1.286 | 4.280 | 153/153 |

At 64 cores with two ranges, every request still exceeded 3.7 ms. The median remainder was 4.047 ms. Reaching the target requires reducing or overlapping frontend or completion work as well as shortening the curve critical path. The outside-core interval combines work before and after the canonical call.

[Original C7a study](../2026-11-18-factorial-panel333/README.md) · [Per-request calculation](artifacts/requests.json.gz)
