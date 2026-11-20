# Readiness of 4×2 H coefficients on C7a

Half of H’s 98,674 coefficients were ready only **119 / 115 µs** before the quotient emitter returned on the original and held-out requests. The first completed leaf led emitter return by **230 / 217 µs**. H entered its ordinary query roughly **10 µs** after the full quotient returned on both inputs.

The test uses 64 main and 32 frontend C7a workers with the verified 16-core solver subset. The fused quotient emitter writes disjoint output stripes for each leaf. Each leaf records completion of its final coefficients. The row count is reconstructed from the stripe dimensions and checked independently for every leaf. The sensor stores times and counts only.

Lead is elapsed time from a completed leaf to emitter return. Quantiles are weighted by the number of real H coefficients written. Each input includes 104 timed trace records. Readiness measures the time available to overlap other work under this execution. A new consumer could affect FFT timing and worker contention.

| Input | Emission µs | First µs | 10% ready µs | Half ready µs | 90% ready µs | H dispatch after quotient µs | Fraction ready at least 100 µs early |
|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | 350 | 230 | 157 | 119 | 55 | 10.2 | 53.9% |
| heldout | 343 | 217 | 152 | 115 | 52 | 10.3 | 52.4% |

| Input | Exact prior ms | Trace off ms | Trace on ms | Off vs prior paired change | Trace vs off paired change |
|---|---:|---:|---:|---:|---:|
| catalog | 14.959 | 14.908 | 14.955 | -0.19% | +0.93% |
| heldout | 14.861 | 14.858 | 14.876 | -0.34% | +0.39% |

Both controls calibrated on both inputs. Six native tests passed. All 48 C7a and both local proof exports independently verified. The temporary AWS instance and its storage were removed. This is a timing diagnostic. [Eight-shape CPU and Metal scorecard](../2026-11-20-quartet-placement-factorial-c7a950/README.md#eight-supplied-m5-references)

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Raw trace samples](artifacts/rows.json.gz) · [Experimental source](artifacts/source.patch.gz) · [AWS cleanup](artifacts/cleanup.json)
