# Complete release proofs under cgroup memory limits

Each independent prover and its small supervising process ran in a fresh Linux cgroup with a hard memory.max and zero swap. The supervisor recorded peak charged bytes and local OOM events. Shared cached file pages may be charged outside the scope. These limits describe this host and warm file cache. The peak includes process setup. Four randomized blocks compare tuned and stock glibc on the same release, request, 32 workers, three warmups and eleven timed calls. Latency is conditional on successful completion. Each completed export passed an independent verifier.

| Shape | Hard limit MiB | Allocator | Completed | Warm ms | Peak charged MiB | OOM kills | M5 CPU / Metal speedup |
|---|---:|---|---:|---:|---:|---:|---:|
| 1x2 | 56 | tuned | 4/4 | 10.254 | 43.7 | 0 | 3.61× / 2.34× |
| 1x2 | 56 | stock | 4/4 | 9.988 | 44.2 | 0 | 3.70× / 2.40× |
| 1x2 | 160 | tuned | 4/4 | 7.307 | 55.7 | 0 | 5.06× / 3.28× |
| 1x2 | 160 | stock | 4/4 | 7.619 | 56.8 | 0 | 4.86× / 3.15× |
| 12x2 | 400 | tuned | 4/4 | 88.878 | 164.1 | 0 | 3.20× / 1.14× |
| 12x2 | 400 | stock | 4/4 | 87.416 | 164.5 | 0 | 3.25× / 1.16× |
| 12x2 | 768 | tuned | 4/4 | 56.888 | 249.1 | 0 | 4.99× / 1.78× |
| 12x2 | 768 | stock | 4/4 | 60.329 | 245.8 | 0 | 4.71× / 1.67× |

[Samples and cgroup events](artifacts/samples.json.gz) · [Design](artifacts/design.json) · [Scope monitor](artifacts/scoped.py) · [Verification](artifacts/verification.json)

[Kernel activity and charged memory compared with peak RSS](usage.md)
