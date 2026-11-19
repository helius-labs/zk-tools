# Dedicated workers for folded product evaluation

Each row shows five confirmation blocks after independent width selection. Times are medians of process means. Peak memory includes preparation. Each comparison uses equal main affinity in the paired processes.

| Shape | Product workers | Prior / main / idle pool / selected ms | Selected vs main | Selected vs idle | Peak main / selected MiB | Selected vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|---:|
| 12x2 | 24 | 28.638 / 28.976 / 28.865 / 28.541 | -1.00% (4/5) | -0.13% (3/5) | 597.8 / 457.6 | 9.95× / 3.54× |
| 16x2 | 28 | 37.386 / 37.103 / 37.432 / 37.411 | +2.07% (1/5) | -0.06% (3/5) | 770.7 / 588.3 | 8.95× / 3.31× |
| 24x2 | 28 | 58.517 / 59.134 / 59.349 / 61.693 | +4.58% (0/5) | +3.15% (0/5) | 1119.7 / 886.5 | 8.56× / 2.85× |

All 99 exported proofs verified. The three route probes each observed the declared worker count on all four calls. [Protocol](protocol.md) · [Confirmation order](artifacts/confirmation.json) · [Samples](artifacts/samples.json.gz) · [Source](artifacts/source.patch)
