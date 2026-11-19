# Where the additional C7a wall time falls at 96 cores

Each traced SDK request is split into eight consecutive intervals. The sum for each request reconciles with the recorded SDK wall time to within one nanosecond. The table uses arithmetic means so the 96-minus-64 differences add up. The bracketed ranges cover the three individual block differences. Each block has eleven timed requests per width and shape on the same C7a.

| Interval | 16×2 at 64 / 96 (ms) | Difference [block range] (ms) | Merge-24×1 at 64 / 96 (ms) | Difference [block range] (ms) |
|---|---:|---:|---:|---:|
| SDK entry to solver | 1.210 / 1.286 | +0.076 [+0.054, +0.090] | 1.425 / 1.545 | +0.120 [+0.091, +0.142] |
| Solver | 5.295 / 6.293 | +0.998 [+0.859, +1.146] | 10.112 / 10.818 | +0.706 [+0.498, +1.013] |
| Solver return to FFT | 1.344 / 1.769 | +0.425 [+0.366, +0.483] | 1.165 / 1.655 | +0.490 [+0.465, +0.515] |
| FFT | 6.421 / 6.540 | +0.118 [+0.069, +0.156] | 10.254 / 9.569 | -0.685 [-0.751, -0.594] |
| FFT return to H | 0.343 / 0.359 | +0.015 [-0.007, +0.029] | 0.363 / 0.382 | +0.019 [-0.024, +0.046] |
| H call | 17.501 / 16.646 | -0.855 [-1.491, -0.053] | 22.546 / 22.445 | -0.100 [-1.807, +1.049] |
| H return to curve-query return | 3.084 / 3.590 | +0.506 [-1.738, +1.870] | 3.698 / 5.119 | +1.421 [-0.684, +4.272] |
| Curve-query return to SDK return | 1.068 / 1.196 | +0.128 [+0.094, +0.178] | 1.058 / 1.163 | +0.106 [+0.088, +0.121] |
| Full traced SDK | 36.267 / 37.678 | +1.412 [+0.039, +2.696] | 50.620 / 52.696 | +2.077 [+0.321, +3.041] |

Plain SDK process means across the same three blocks provide a check on the magnitude of observer effects.
On 16x2 these were 33.571 ms and 36.442 ms, a +2.871 ms change.
On merge-24x1 these were 48.380 ms and 50.419 ms, a +2.040 ms change.

## Sampled work after H returns

The interval starts when the H query returns and ends when the encompassing curve-query function returns. Samples cover every running thread in that interval. Each sample is approximately two CPU-ms. The most frequent symbol outside Crossbeam and Rayon is included to identify a candidate for a subsequent instrumentation point.

| Shape | Cores | Sampled CPU-ms/proof in interval | Crossbeam share | Most frequent other symbol | Share of all interval samples |
|---|---:|---:|---:|---|---:|
| 16x2 | 64 | 119.5 | 26.7% | `zopro_core::msm::g2_prefix::apply::<zopro_bn254::curve::G2Config, true>` | 24.4% |
| 16x2 | 96 | 217.9 | 41.3% | `zopro_core::msm::g2_prefix::apply::<zopro_bn254::curve::G2Config, true>` | 22.8% |
| merge-24x1 | 64 | 142.6 | 16.9% | `zopro_core::msm::g2_prefix::apply::<zopro_bn254::curve::G2Config, true>` | 28.9% |
| merge-24x1 | 96 | 286.1 | 33.7% | `zopro_core::msm::g2_prefix::apply::<zopro_bn254::curve::G2Config, true>` | 23.2% |

## Sampled work between solver return and FFT entry

Each of the six paired blocks across the two shapes spent longer in this interval at 96 cores. The symbol column names the most frequent instruction-pointer symbol outside the two scheduling libraries.

| Shape | Cores | CPU-ms/proof | Crossbeam share | Most frequent other symbol | Share of all samples in interval |
|---|---:|---:|---:|---|---:|
| 16x2 | 64 | 50.6 | 40.4% | `<std::sys::sync::mutex::futex::Mutex>::lock_contended` | 1.8% |
| 16x2 | 96 | 109.7 | 45.9% | `<std::sys::sync::mutex::futex::Mutex>::lock_contended` | 2.0% |
| merge-24x1 | 64 | 48.5 | 25.6% | `<std::sys::sync::mutex::futex::Mutex>::lock_contended` | 1.4% |
| merge-24x1 | 96 | 100.2 | 31.1% | `<std::sys::sync::mutex::futex::Mutex>::lock_contended` | 1.4% |

The Rayon iterator symbols also retain the type of the operation they execute. On 16×2, iterators over 32-bit wire indices collecting field elements in `Key::solve_inner` account for 41.8% of samples at 64 cores and 39.0% at 96. On merge they account for 58.2% and 51.7%. The two corresponding source expressions gather the A and B query scalars. These samples include inlined work and the surrounding Rayon iterator. They identify the gather as a concrete place to test a change in task size or data movement.

A function returning earlier can increase the time remaining until the enclosing query returns even if its peers run at the same speed. The post-H interval measures completion order as well as runtime. Samples are instruction-pointer observations across workers. They do not identify the task that gated the return. The recordings are from an earlier integrated executable. All 24 original exports were reverified.

[Per-request intervals](artifacts/requests.json.gz) · [Interval symbols](artifacts/symbols.json.gz) · [Verification](artifacts/verification.json) · [Original traces and whole-phase measurements](../2026-11-18-epoch-g2-c7a363/stages.md)
