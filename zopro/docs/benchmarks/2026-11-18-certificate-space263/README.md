# Scratch space for the G2 diversity check

On the retained 1×2 input, a 32 KiB bitmap was sufficient to establish that scalar grouping would decline. The experimental prover currently uses 64 KiB for this input. Reducing the bitmap to 16 KiB lost the certificate through hash collisions. Under a tighter scratch budget, trying a smaller bitmap could preserve the latency benefit on some requests. A full-proof test with the actual admission policy would be needed to measure that benefit.

| Canonical G2 scalar fixture | 1×2 | 2×2 |
|---|---:|---:|
| Rows | 30,089 | 52,274 |
| Exact distinct nonzero scalars | 26,142 | 28,852 |
| Cutoff that must be exceeded | 24,071 | 49,660 |
| Distinct occupied bits at 16 KiB | 23,703 | 25,873 |
| Distinct occupied bits at 32 KiB | 24,883 | 27,284 |
| Distinct occupied bits at 64 KiB | 25,516 | 28,019 |
| Distinct occupied bits at 128 KiB | 25,814 | 28,451 |

Increasing the bitmap cannot make this certificate succeed on the 2×2 input because even its exact cardinality is below the cutoff. The existing grouping fallback serves this case. A zero-or-adjacent-repeat upper bound established that certification was impossible after reading row 37,778 on 2×2. The saved rows alone do not establish a latency improvement.

The fixture digests matched their original manifest. The reader checked every bitmap count against an independent ordered set of full nonzero scalars. These are counts for two specific requests. No proof timings or additional proof exports were taken. [Fixture results](artifacts/counts.json) · [Reader](artifacts/study.rs) · [End-to-end diversity experiment](../2026-11-18-early-diversity257/README.md)
