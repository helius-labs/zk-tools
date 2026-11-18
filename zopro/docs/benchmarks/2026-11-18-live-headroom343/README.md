# Cost of live memory limit checks

The candidate parses cgroup mount lines without allocating token vectors, checks Linux address-space limits with getrlimit, and reuses the live headroom reading within one locked acquisition. The full-proof measurements use seven paired blocks on a Ryzen 9950X.

| Shape | Workers | Retained ms | Candidate ms | Paired change | Faster blocks |
|---|---:|---:|---:|---:|---:|
| 1x2 | 16 | 8.800 | 8.790 | -0.01% | 4/7 |
| 1x2 | 32 | 7.701 | 7.748 | +0.52% | 3/7 |
| 2x2 | 16 | 12.158 | 12.047 | -0.43% | 5/7 |
| 2x2 | 32 | 10.760 | 10.746 | -0.13% | 4/7 |

On three ordinary warm 1×2 requests, the retained executable opened /proc/self/limits 18 times. The candidate made 18 getrlimit calls and zero opens of that file. Both read /proc/meminfo 21 times. The file-read reduction gave little change in end-to-end latency. Under an 8 GiB address-space limit, both executables also read /proc/self/status to account for virtual memory already in use. Proofs verified with normal limits, finite address space and zero optional preparation bytes.

[Protocol](artifacts/protocol.md) · [File and syscall counts](artifacts/pressure.json) · [Timings](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
