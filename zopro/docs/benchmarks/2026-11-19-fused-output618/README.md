# Consuming Euler derivatives before final packed writeback

On 24×2, combining the final columns into the output took 57.804 ms on the reference and 57.366 ms on input 103. The respective paired gains over the frozen prover were 1.97% and 2.61%, each winning four of five blocks. The reference reached 9.13× the supplied M5 CPU and 3.04× Metal. Its 10× CPU cutoff is 52.8 ms. This run used the main worker pool. A factorial comparison with the separately measured lower-memory 30-worker pool is needed to quantify their combined effect. On 12×2, results against the frozen prover were mixed across inputs. [Independent pool confirmation](../2026-11-19-packed-width616/README.md)

The tiled setting combines two neighboring forward columns directly with the original rows. Timings are medians of five full-proof process means. Paired changes compare the tiled setting with the ordinary path in the same executable and with the previous frozen SDK.

| Shape | Request | Prior / ordinary / tiled ms | Tiled vs ordinary / prior | Peak prior / ordinary / tiled MiB | Tiled vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| 12x2 | Original | 28.844 / 28.924 / 28.722 | -0.70% (4/5) / -0.73% (3/5) | 585.3 / 583.5 / 598.6 | 9.89× / 3.52× |
| 12x2 | 57 | 28.601 / 29.571 / 28.556 | -3.67% (3/5) / -0.08% (3/5) | 581.4 / 583.5 / 578.0 | — |
| 12x2 | 139 | 45.504 / 45.260 / 45.641 | -0.99% (3/5) / +0.30% (2/5) | 1017.3 / 999.8 / 952.5 | — |
| 24x2 | Original | 58.682 / 60.685 / 57.804 | -3.85% (5/5) / -1.97% (4/5) | 1114.6 / 1131.9 / 1113.1 | 9.13× / 3.04× |
| 24x2 | 103 | 58.773 / 58.856 / 57.366 | -2.48% (4/5) / -2.61% (4/5) | 1124.6 / 1097.4 / 1097.2 | — |

All 77 exports verified independently. [Protocol](protocol.md) · [Implementation relative to SDK 612](artifacts/source.patch) · [Samples](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)
