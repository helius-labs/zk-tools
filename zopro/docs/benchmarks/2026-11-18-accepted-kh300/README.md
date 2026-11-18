# K/H scheduling on the accepted prover source

The candidate lets a worker hand off a partially completed multi-scalar multiplication window after feeding K points, then after feeding H points. A pending window holds scratch across jobs. Each proof reserves a conservative estimate before taking the route. The original schedule handles denied reservations and teams smaller than 20. No additional proving-key table is built.

On this 16-core Ryzen with 32 logical workers the enabled candidate increased paired latency by 2.64% and won 1 of five blocks against its disabled control. The production default stays unchanged. C7a uses physical cores and is evaluated separately.

| 1×2 SDK | Median ms | Median CPU ms/proof | Max RSS MiB | M5 CPU / Metal speedup |
|---|---:|---:|---:|---:|
| accepted | 7.415 | 161.2 | 123.2 | 4.99× / 3.24× |
| off | 7.364 | 161.6 | 115.0 | 5.02× / 3.26× |
| on | 7.579 | 163.5 | 112.5 | 4.88× / 3.17× |

Five randomized blocks, three warmups and 51 timed warm verified SDK requests per process. Each process exported one proof for independent Gnark verification. The first 15 timing processes overlapped local cloud bundle preparation and were excluded. Five additional instrumented proofs measured 20 weighting callbacks on enabled 32-worker 1×2 and zero each when disabled, denied by a zero-byte allowance, using four workers, or running 2×2. Generic compilation and 142 native core tests passed. The four-worker fallback ran on x86.

[Source patch](artifacts/source.patch) · [Process and request data](artifacts/rows.json.gz) · [Route probes](artifacts/routes.json) · [Verification and build hashes](artifacts/verification.json)
