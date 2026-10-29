# Prior experiments and new boundary

-64778d8ad6 tiled-top hybrid:10.325850ms vs8.036072ms accepted,0/15wins;
  CPU/instructions roughly41%higher. G2 reduce_layer and Fq inverse hot.
-419e0823b4 census:84.947% of ordinary GLS4 deferred mixed updates at top.
-72350556 all-ordinary GLS4:12.826593ms vs8.024429ms accepted; closed.
-26Oct g2-level-tree, g2-prefix-tree and g2-index-gather: ordinary phi2 windows
  changed; original narrow top explicitly preserved. Closed local losses.
-22Oct g2-gls4-sparse and parallel: signed indexed top, original tiled reducer.

This experiment changes only the GLS4 top reduction from per-bucket tile trees
to cross-bucket level batches. group_affine/reduce_layer/prefix/math are existing
and exact; their ordinary-window tree trials are not repeated. It can still lose
due to scratch/copying/parallel depth. No unchanged timing retry or group sweep.
Frozen historical lattice and GnarkCrypto attribution are inherited unchanged;
100767 old integer cases are not claimed fresh.
