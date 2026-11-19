# Dedicated workers for folded product evaluation

Each row shows five confirmation blocks after independent width selection. Times are medians of process means. Peak memory includes preparation. Each comparison uses equal main affinity in the paired processes.

| Shape | Product workers | Prior / main / idle pool / selected ms | Selected vs main | Selected vs idle | Peak main / selected MiB | Selected vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|---:|
| 12x2 | 24 | 28.638 / 28.976 / 28.865 / 28.541 | -1.00% (4/5) | -0.13% (3/5) | 597.8 / 457.6 | 9.95× / 3.54× |
| 16x2 | 28 | 37.386 / 37.103 / 37.432 / 37.411 | +2.07% (1/5) | -0.06% (3/5) | 770.7 / 588.3 | 8.95× / 3.31× |
| 24x2 | 28 | 58.517 / 59.134 / 59.349 / 61.693 | +4.58% (0/5) | +3.15% (0/5) | 1119.7 / 886.5 | 8.56× / 2.85× |

All 99 exported proofs verified. The three route probes each observed the declared worker count on all four calls. [Protocol](protocol.md) · [Confirmation order](artifacts/confirmation.json) · [Samples](artifacts/samples.json.gz) · [Source](artifacts/source.patch)


## Memory and worker activity

The three entries in each cell follow the same order as the control, idle auxiliary pool, and selected product pool. CPU occupancy is the sum of process user and system time divided by elapsed time during the timed loop. Faults and switches cover that same loop. RSS is the maximum over process lifetime.

| Transfer | Peak RSS, MiB | Mean busy logical CPUs, median process | Minor faults per proof | Voluntary switches per proof |
|---|---:|---:|---:|---:|
| 12x2 | 597.8 / 581.8 / 457.6 | 25.9 / 26.0 / 24.2 | 4106.9 / 2697.8 / 704.9 | 868.6 / 854.2 / 926.0 |
| 16x2 | 770.7 / 754.2 / 588.3 | 28.1 / 28.1 / 26.8 | 4778.5 / 4249.1 / 1887.2 | 958.1 / 968.4 / 1088.0 |
| 24x2 | 1119.7 / 1121.1 / 886.5 | 28.5 / 28.4 / 25.9 | 6020.2 / 6171.8 / 23750.3 | 1170.2 / 1241.3 / 1129.5 |
