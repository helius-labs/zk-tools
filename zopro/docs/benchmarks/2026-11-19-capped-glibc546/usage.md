# Kernel activity within the capped runs

The warm counters come from each SDK process after its three priming proofs. The fault and CPU columns are medians across the four blocks. They exclude initial key loading. The two peak sizes are maxima over process lifetimes. The kernel charges pages in the cgroup that first acquires them, so pages already shared from the host file cache can appear in the prover’s RSS without adding the same bytes to its cgroup. Each case recorded zero cgroup max events and zero OOM kills.

| Shape | Limit MiB | Allocator | Scope peak / prover peak RSS MiB | cgroup max events | Minor faults per proof | Kernel / total CPU-ms per proof |
|---|---:|---|---:|---:|---:|---:|
| 1x2 | 56 | stock | 44.2 / 48.4 | 0 | 5723 | 13.9 / 155.8 |
| 1x2 | 56 | tuned | 43.7 / 47.9 | 0 | 5738 | 14.3 / 155.6 |
| 1x2 | 160 | stock | 56.8 / 61.2 | 0 | 1419 | 5.4 / 147.0 |
| 1x2 | 160 | tuned | 55.7 / 57.0 | 0 | 152 | 3.7 / 146.0 |
| 12x2 | 400 | stock | 164.5 / 287.0 | 0 | 48062 | 115.5 / 1470.1 |
| 12x2 | 400 | tuned | 164.1 / 286.9 | 0 | 48000 | 111.9 / 1468.5 |
| 12x2 | 768 | stock | 245.8 / 369.0 | 0 | 14424 | 78.6 / 1487.7 |
| 12x2 | 768 | tuned | 249.1 / 372.5 | 0 | 430 | 15.4 / 1471.5 |

On this warm-cache host, the 12×2 prover stayed below the kernel hard limit even as its latency and minor-fault counts varied with the limit. The resource admission and allocation paths both observe available headroom. Distinguishing their individual effects requires instrumenting those decisions.

[Latency](README.md) · [SDK summaries and proof identities](artifacts/usage.json.gz)
