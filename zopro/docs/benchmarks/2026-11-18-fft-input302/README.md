# Reading inputs directly into the first inverse FFT butterflies

On the local 16-core Ryzen, the direct-input FFT passed the 24×2 screening threshold. It reached 72.998 ms against 74.848 ms disabled and 75.736 ms on the accepted SDK, winning all five within-block comparisons against disabled (2.62% median paired improvement). Maximum process RSS reached 999 MiB versus 950 MiB disabled. Merge24 improved in five pairs, with a 1.25% median reduction. The padding-aware variant missed the threshold on both primary shapes.

The existing path packs two scalar inputs into a full buffer, then reads them for the FFT. The candidate loads A and B while computing the first 32-row inverse butterfly and initializes each output stripe directly. It continues with the existing subtransforms and twiddle tables. A second mode skips work on padded high inputs. Both modes use the ordinary FFT path below a domain size of 8,192.

| Shape | Mode | Median ms | Paired change vs off | Wins | Median CPU ms | Peak MiB | M5 CPU / Metal speedup |
|---|---|---:|---:|---:|---:|---:|---:|
| 12x2 | accepted | 38.718 | -0.67% | 4/5 | 572 | 520 | 7.34× / 2.61× |
| 12x2 | off | 39.296 | +0.00% | 0/5 | 582 | 511 | 7.23× / 2.57× |
| 12x2 | direct | 38.420 | -2.23% | 5/5 | 570 | 514 | 7.39× / 2.63× |
| 12x2 | padded | 38.431 | -1.45% | 5/5 | 562 | 511 | 7.39× / 2.63× |
| 16x2 | accepted | 46.509 | -0.20% | 3/5 | 689 | 582 | 7.20× / 2.67× |
| 16x2 | off | 46.603 | +0.00% | 0/5 | 694 | 580 | 7.19× / 2.66× |
| 16x2 | direct | 46.275 | -0.69% | 3/5 | 680 | 614 | 7.24× / 2.68× |
| 16x2 | padded | 47.296 | -0.26% | 4/5 | 686 | 601 | 7.08× / 2.62× |
| 24x2 | accepted | 75.736 | +1.19% | 1/5 | 1137 | 979 | 6.97× / 2.32× |
| 24x2 | off | 74.848 | +0.00% | 0/5 | 1127 | 950 | 7.05× / 2.35× |
| 24x2 | direct | 72.998 | -2.62% | 5/5 | 1100 | 998 | 7.23× / 2.41× |
| 24x2 | padded | 75.202 | +1.57% | 2/5 | 1117 | 981 | 7.02× / 2.34× |
| merge-24x1 | accepted | 73.771 | +0.68% | 2/5 | 1081 | 879 | 6.87× / 2.37× |
| merge-24x1 | off | 72.964 | +0.00% | 0/5 | 1078 | 900 | 6.95× / 2.40× |
| merge-24x1 | direct | 71.697 | -1.25% | 5/5 | 1055 | 888 | 7.07× / 2.44× |
| merge-24x1 | padded | 71.411 | -1.32% | 3/5 | 1042 | 908 | 7.10× / 2.45× |

Each point uses five randomized process blocks, three primes, eleven timed requests and one externally verified export per process. Tests compared every output lane to the original inverse transform at two domain sizes under uneven and padded input lengths, with and without prepared column roots. The native core suite passed 142 tests and the generic core library compiled. RSS is the highest whole-process value within each cell. The observed 24×2 peak warrants checking on C7a before considering adoption.

[Source patch](artifacts/source.patch) · [Block and request data](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)
