# Gathering B2 scalars directly in packed key order

The solver extracts B2 scalars into their final packed order while it owns the full wire buffer. A parallel pass subtracts each representative from its tail members. The packed query counts nonzero differences in up to 128 blocks, converts its owned prefix in place and reads only blocks containing corrections on the sparse path. The dense and zero-optional-memory routes convert the whole owned scalar vector in place. All three implementations in this experiment use the same binary. The original image follows the ordinary B2 route. The two packed options share the exact same images and offset files from experiment 506. Ordinary proving workspace and key memory are still required.

Four randomized blocks compare the original image, the preceding packed-scalar extraction and direct extraction. Each process primes three and times eleven verified SDK proofs at 32 Ryzen workers. Peak RSS includes setup. Separate route probes cover forced dense and zero optional budget on each original request.

| Shape | Request | Ordinary / prior packed / direct ms | Direct vs prior | Direct vs ordinary | Peak MiB ordinary / prior / direct |
|---|---|---:|---:|---:|---:|
| 8x2 | Original | 23.907 / 23.707 / 23.127 | -2.55% (3/4) | -3.38% (4/4) | 439.0 / 479.5 / 435.5 |
| 8x2 | 153 | 24.132 / 23.013 / 23.744 | +3.59% (0/4) | -1.61% (3/4) | 442.5 / 459.0 / 444.2 |
| 12x2 | Original | 31.537 / 30.821 / 34.276 | +11.46% (0/4) | +8.71% (0/4) | 551.5 / 588.3 / 572.4 |
| 12x2 | 59 | 31.058 / 30.551 / 33.582 | +9.96% (0/4) | +7.98% (0/4) | 552.1 / 606.1 / 550.4 |
| 12x2 | 141 | 46.826 / 52.508 / 51.681 | -2.01% (4/4) | +10.37% (0/4) | 976.0 / 968.8 / 981.8 |
| 24x2 | Original | 63.213 / 66.729 / 61.202 | -7.99% (4/4) | -2.72% (4/4) | 1028.4 / 1060.0 / 1030.3 |
| 24x2 | 97 | 63.766 / 66.628 / 61.659 | -6.92% (4/4) | -2.74% (4/4) | 1050.7 / 1072.7 / 1001.9 |

All 97 exported proofs verified independently against the original key. [Process samples](artifacts/processes.json.gz) · [Execution routes](artifacts/probes.json) · [Source](artifacts/incremental.patch) · [Verification](artifacts/verification.json)
