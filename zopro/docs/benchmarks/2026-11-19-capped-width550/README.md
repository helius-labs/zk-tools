# Worker width under enforced memory limits

Each independent prover and its small supervising process ran in a fresh Linux cgroup with a hard memory.max and zero swap. The supervisor recorded peak charged bytes and local OOM events. Shared cached file pages may be charged outside the scope. These limits describe this host and warm file cache. The peak includes process setup. Five randomized blocks compare eight, sixteen and thirty-two workers on the same release and tuned glibc. Eight workers use alternate physical cores in each L3 group, sixteen use every physical core and thirty-two use both hardware threads throughout. At width eight the small circuit uses the main pool for frontend work. At sixteen and thirty-two it uses a separate frontend pool on eight physical cores across both L3 groups. The SDK requires a separately pinned frontend to be smaller than the main pool. Each process warms three proofs then times eleven. Latency is conditional on successful completion. Each completed export passed an independent verifier.

| Shape | Hard limit MiB | Workers | Completed | Warm ms | Peak charged MiB | OOM kills | M5 CPU / Metal speedup |
|---|---:|---|---:|---:|---:|---:|---:|
| 1x2 | 56 | 8 | 5/5 | 15.920 | 25.1 | 0 | 2.32× / 1.51× |
| 1x2 | 56 | 16 | 5/5 | 11.142 | 33.8 | 0 | 3.32× / 2.15× |
| 1x2 | 56 | 32 | 5/5 | 10.420 | 43.4 | 0 | 3.55× / 2.30× |
| 1x2 | 160 | 8 | 5/5 | 13.069 | 36.1 | 0 | 2.83× / 1.84× |
| 1x2 | 160 | 16 | 5/5 | 8.307 | 50.0 | 0 | 4.45× / 2.89× |
| 1x2 | 160 | 32 | 5/5 | 7.292 | 54.9 | 0 | 5.07× / 3.29× |
| 1x2 | 1024 | 8 | 5/5 | 12.997 | 35.9 | 0 | 2.85× / 1.85× |
| 1x2 | 1024 | 16 | 5/5 | 8.330 | 50.3 | 0 | 4.44× / 2.88× |
| 1x2 | 1024 | 32 | 5/5 | 7.275 | 58.6 | 0 | 5.09× / 3.30× |
| 12x2 | 400 | 8 | 5/5 | 145.557 | 150.4 | 0 | 1.95× / 0.69× |
| 12x2 | 400 | 16 | 5/5 | 99.807 | 155.4 | 0 | 2.85× / 1.01× |
| 12x2 | 400 | 32 | 5/5 | 87.452 | 164.0 | 0 | 3.25× / 1.15× |
| 12x2 | 768 | 8 | 5/5 | 116.203 | 236.9 | 0 | 2.44× / 0.87× |
| 12x2 | 768 | 16 | 5/5 | 68.958 | 250.1 | 0 | 4.12× / 1.46× |
| 12x2 | 768 | 32 | 5/5 | 56.535 | 298.8 | 0 | 5.02× / 1.79× |
| 12x2 | 4096 | 8 | 5/5 | 59.535 | 283.2 | 0 | 4.77× / 1.70× |
| 12x2 | 4096 | 16 | 5/5 | 36.967 | 368.9 | 0 | 7.68× / 2.73× |
| 12x2 | 4096 | 32 | 5/5 | 31.415 | 444.5 | 0 | 9.04× / 3.22× |

[Samples and cgroup events](artifacts/samples.json.gz) · [Design](artifacts/design.json) · [Scope monitor](artifacts/scoped.py) · [Verification](artifacts/verification.json)
