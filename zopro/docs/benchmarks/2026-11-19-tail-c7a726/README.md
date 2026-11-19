# Locating the 1×2 proof tail on 96 C7a cores

On a fresh c7a.24xlarge, the second input added +2.746 ms inside the canonical solver after both observed curve calls returned. The following four intervals changed by -0.020, -0.019, -0.023 and -0.001 ms in order. These are within-block median differences on the ordinary server configuration. The probes changed second-input latency by +5.1%. Uninstrumented runs supply the latency comparison.

| Input | Zopro ms | Prepared Aeglos ms | Zopro wins | Zopro global dedup off ms | Peak MiB Zopro / Aeglos |
|---|---:|---:|---:|---:|---:|
| catalog | 8.073 | 8.236 | 4/5 | 9.903 | 45.2 / 458.3 |
| heldout | 11.241 | 8.194 | 0/5 | 9.768 | 39.4 / 459.3 |
| v27 | 11.270 | 8.349 | 0/5 | 9.936 | 39.2 / 462.0 |
| v28 | 11.162 | 8.415 | 0/5 | 9.486 | 42.2 / 456.1 |

On the original transaction the ordinary server was 4.58× faster than the supplied 37 ms M5 CPU reference and 2.97× faster than the supplied 24 ms M5 Metal reference. Each cell is the median of five process means with three warmups and fifteen timed, verified requests per process. Both tools had access to the same 96 physical cores. Zopro used a 16-core frontend subset. All timed Aeglos samples reported cache hits. The independent verifier passed all 180 exported proof and public-input pairs.

| Input | Zopro configuration | Tracing effect vs plain | Total after observed curves ms | To canonical return / to generation return / to final exponentiation / final exponentiation / to SDK return ms |
|---|---|---:|---:|---:|
| catalog | server | +6.7% | 0.923 | 0.066 / 0.234 / 0.287 / 0.315 / 0.021 |
| catalog | direct | +11.7% | 2.330 | 1.527 / 0.217 / 0.273 / 0.295 / 0.019 |
| heldout | server | +5.1% | 3.609 | 2.813 / 0.215 / 0.269 / 0.292 / 0.019 |
| heldout | direct | +10.3% | 2.366 | 1.564 / 0.217 / 0.270 / 0.294 / 0.019 |
| v27 | server | +5.7% | 3.538 | 2.743 / 0.219 / 0.268 / 0.292 / 0.019 |
| v27 | direct | +6.7% | 2.463 | 1.656 / 0.218 / 0.272 / 0.298 / 0.020 |
| v28 | server | +6.3% | 3.625 | 2.830 / 0.217 / 0.271 / 0.292 / 0.019 |
| v28 | direct | +12.2% | 2.083 | 1.275 / 0.219 / 0.273 / 0.296 / 0.019 |

All 600 timed traced SDK requests partition into non-overlapping intervals with matched entry and return events and zero lost records. The first interval begins at the later return of canonical G1 and exclusive G2. Additional proving work can run concurrently with either curve call. The measurements identify the enclosing function responsible for time after the two returns. They do not identify an individual worker or the specific operation inside that function.

The per-query controls below share one binary and retain the ordinary workspace forecast. Parentheses give the paired percentage change from the unmodified configuration of that binary. The global switch in the first table also changes other decisions in the prover.

| Input | Unmodified ms | Skip canonical G1 grouping ms | Skip exclusive G2 grouping ms | Skip both ms |
|---|---:|---:|---:|---:|
| catalog | 8.086 (+0.0%) | 10.329 (+27.4%) | 7.974 (-0.9%) | 10.245 (+26.0%) |
| heldout | 11.232 (+0.0%) | 9.983 (-10.7%) | 10.539 (-6.1%) | 9.615 (-14.4%) |
| v27 | 11.190 (+0.0%) | 9.713 (-13.5%) | 10.347 (-7.9%) | 9.745 (-12.6%) |
| v28 | 11.160 (+0.0%) | 9.565 (-14.2%) | 10.456 (-6.2%) | 9.722 (-13.0%) |

The instance terminated after retrieval. The cleanup check found zero tagged volumes and zero active tagged benchmark instances. The temporary bucket was removed.

[Samples and intervals](artifacts/rows.json.gz) · [Pinned inputs and binaries](artifacts/pins.json) · [Workflow](artifacts/workflow.json.gz) · [Probe sites](artifacts/symbols.json) · [Source of per-query controls](../2026-11-19-dedup-scope722/artifacts/source.patch) · [Verification](artifacts/verification.json) · [Full 41-shape comparison](../2026-11-19-group-pool-width714/artifacts/full-catalog.csv)
