# Early solver scheduling samples on C7a

The same 12 archived C7a processes supply 132 verified requests. At fixed circuit and worker count, does a higher fraction of samples in Crossbeam during the first 2 ms predict a longer remaining solve? A fixed prefix avoids counting extra scheduler samples simply because a solve lasted longer. Each correlation uses requests with at least one recorded prefix sample within a single process. The coverage column counts those requests across its three processes.

| Shape | Cores | Sampled / 33 | Median samples in first 2 ms | Median Crossbeam share | Median remaining solver ms | Within-process rank correlations |
|---|---:|---:|---:|---:|---:|---|
| 16x2 | 64 | 30 | 43 | 57.8% | 3.308 | -0.07, -0.55, -0.44 |
| 16x2 | 96 | 30 | 62 | 72.4% | 4.319 | -0.41, -0.27, +0.12 |
| merge-24x1 | 64 | 30 | 11 | 30.8% | 8.067 | +0.49, -0.33, +0.16 |
| merge-24x1 | 96 | 30 | 19 | 51.3% | 8.920 | -0.83, -0.17, -0.04 |

On 16×2 the median early Crossbeam share rose from 57.8% to 72.4% at 96 cores. Within that width a high early share did not consistently predict a slower finish. All twelve processes lacked CPU samples in the first 2 ms of their first request. The other ten requests in each process contributed to the correlations.

Sampling spans all threads during the solver call, including auxiliary pools on merge. Different pool widths and trace overhead can affect observed timings. Each process contributes only eleven requests. The analysis supplies no causal estimate of milliseconds recoverable by changing the queue.

[Per-request counts](artifacts/requests.json.gz) · [Protocol](artifacts/protocol.md) · [Trace identities](../2026-11-18-c7a-worker-attribution381/artifacts/inputs.json) · [Prior whole-solver attribution](../2026-11-18-wave-claims373/quintiles.md)
